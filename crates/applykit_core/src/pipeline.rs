use crate::banks::load_banks;
use crate::classify::classify_track;
use crate::config::{
    load_config, load_runtime_settings, merge_config_with_runtime, resolve_output_base,
};
use crate::diff::inline_diff_md;
use crate::jd::{extract_structured, merge_extracted_with_summary, parse_llm_jd_summary};
use crate::messages::generate_messages;
use crate::packet::{write_packet, PacketWriteInput};
use crate::resume::{load_resume_template, tailor_resume};
use crate::score::compute_fit_score;
use crate::storage::{get_job_by_id, upsert_job_record, UpsertJobRecordInput};
use crate::truth_gate::validate;
use crate::types::{
    ExtractionDiagnostics, ExtractionSource, GenerateInput, GenerateResultData, PacketDetail,
    TrackerRow,
};
use anyhow::Context;
use applykit_llm::{LlmAdapter, LlmRequest, LlmTask, OllamaAdapter, OpenAiCompatAdapter};
use chrono::{Local, NaiveDate};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct GenerateOptions {
    pub repo_root: PathBuf,
}

pub type GenerateResult = anyhow::Result<GenerateResultData>;

fn hash_jd(jd_text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(jd_text.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn output_date(input: Option<NaiveDate>) -> NaiveDate {
    input.unwrap_or_else(|| Local::now().date_naive())
}

fn task_allowed(cfg: &crate::config::LlmConfig, task: &str) -> bool {
    cfg.allowed_tasks.iter().any(|t| t.eq_ignore_ascii_case(task))
}

fn llm_rewrite(
    cfg: &crate::config::LlmConfig,
    task: LlmTask,
    task_name: &str,
    prompt: &str,
) -> anyhow::Result<Option<String>> {
    if !cfg.enabled || !task_allowed(cfg, task_name) {
        return Ok(None);
    }

    let request = LlmRequest { task, prompt: prompt.to_string() };
    let provider = cfg.provider.to_ascii_lowercase();

    let output = if provider == "ollama" {
        let adapter = OllamaAdapter { base_url: cfg.base_url.clone(), model: cfg.model.clone() };
        adapter.rewrite(&request)?.output
    } else {
        let adapter = OpenAiCompatAdapter {
            provider_name: cfg.provider.clone(),
            base_url: cfg.base_url.clone(),
            model: cfg.model.clone(),
        };
        adapter.rewrite(&request)?.output
    };

    Ok(Some(output.trim().to_string()))
}

pub fn generate_packet(input: GenerateInput, options: GenerateOptions) -> GenerateResult {
    let runtime_settings = load_runtime_settings(&options.repo_root)?;
    let cfg = merge_config_with_runtime(load_config(&options.repo_root)?, &runtime_settings);
    let banks = load_banks(&options.repo_root)?;
    let allow_unapproved = input.allow_unapproved || runtime_settings.allow_unapproved;

    let deterministic_extracted = extract_structured(&input.jd_text, &banks);
    let mut extracted = deterministic_extracted.clone();
    let mut extraction_source = ExtractionSource::Deterministic;
    let mut extraction_diagnostics = ExtractionDiagnostics::default();
    if cfg.llm.enabled && task_allowed(&cfg.llm, "summarize_jd") {
        extraction_diagnostics.summarize_attempted = true;
        let summarize_prompt = format!(
            "Summarize the following job description into strict JSON with keys: keywords, requirements, tools, scale_signals, rigor_signals. Return JSON only.\n\n{}",
            deterministic_extracted.normalized_text
        );
        match llm_rewrite(&cfg.llm, LlmTask::SummarizeJd, "summarize_jd", &summarize_prompt) {
            Ok(Some(summary_text)) => match parse_llm_jd_summary(&summary_text) {
                Ok(summary) => {
                    match merge_extracted_with_summary(&deterministic_extracted, &summary, &banks) {
                        Ok(merged) => {
                            extracted = merged;
                            extraction_source = ExtractionSource::LlmMerged;
                            extraction_diagnostics.summarize_merged = true;
                        }
                        Err(_) => extraction_diagnostics
                            .summarize_fallback_reasons
                            .push("merge_rejected".to_string()),
                    }
                }
                Err(_) => extraction_diagnostics
                    .summarize_fallback_reasons
                    .push("parse_failed".to_string()),
            },
            Ok(None) => {
                extraction_diagnostics.summarize_fallback_reasons.push("empty_response".to_string())
            }
            Err(_) => {
                extraction_diagnostics.summarize_fallback_reasons.push("request_failed".to_string())
            }
        }
    }
    let track = classify_track(&extracted.normalized_text, &cfg, input.track_override);
    let fit = compute_fit_score(&extracted, &track, &banks, &cfg);

    let baseline_template = load_resume_template(
        &options.repo_root,
        matches!(input.baseline, crate::types::Baseline::TwoPage),
    )?;
    let (resume_primary, tailor_plan, provenance_ids, bullet_candidates) = tailor_resume(
        &baseline_template,
        &extracted,
        track.selected,
        &banks,
        &cfg,
        allow_unapproved,
    )?;

    let resume_1pg = if matches!(input.baseline, crate::types::Baseline::OnePage) {
        resume_primary.clone()
    } else {
        load_resume_template(&options.repo_root, false)?
    };

    let resume_2pg = if matches!(input.baseline, crate::types::Baseline::TwoPage) {
        Some(resume_primary)
    } else {
        None
    };

    let top_matches = tailor_plan
        .edits
        .iter()
        .filter(|e| e.kind == "bullet_swap")
        .map(|e| e.reason.clone())
        .collect::<Vec<_>>();

    let (det_recruiter_message, det_hiring_manager_message, det_cover_short_message) =
        generate_messages(
            &options.repo_root,
            &input.company,
            &input.role,
            &fit,
            track.selected,
            &top_matches,
        )?;

    let mut resume_1pg = resume_1pg;
    let mut resume_2pg = resume_2pg;
    let mut recruiter_message = det_recruiter_message.clone();
    let mut hiring_manager_message = det_hiring_manager_message.clone();
    let mut cover_short_message = det_cover_short_message.clone();

    let deterministic_resume_1pg = resume_1pg.clone();
    let deterministic_resume_2pg = resume_2pg.clone();
    let deterministic_recruiter_message = recruiter_message.clone();
    let deterministic_hiring_manager_message = hiring_manager_message.clone();
    let deterministic_cover_short_message = cover_short_message.clone();

    let mut llm_applied = false;
    if cfg.llm.enabled {
        let style_prompt = "Rewrite for clarity and tone only. Preserve all claims, tools, and metrics exactly. Do not add any new achievements, tools, titles, or stronger ownership language.";

        if let Ok(Some(rewrite)) = llm_rewrite(
            &cfg.llm,
            LlmTask::RewriteMessage,
            "rewrite_message",
            &format!("{style_prompt}\n\n{}", recruiter_message),
        ) {
            recruiter_message = rewrite;
            llm_applied = true;
        }

        if let Ok(Some(rewrite)) = llm_rewrite(
            &cfg.llm,
            LlmTask::RewriteMessage,
            "rewrite_message",
            &format!("{style_prompt}\n\n{}", hiring_manager_message),
        ) {
            hiring_manager_message = rewrite;
            llm_applied = true;
        }

        if let Ok(Some(rewrite)) = llm_rewrite(
            &cfg.llm,
            LlmTask::RewriteMessage,
            "rewrite_message",
            &format!("{style_prompt}\n\n{}", cover_short_message),
        ) {
            cover_short_message = rewrite;
            llm_applied = true;
        }

        if task_allowed(&cfg.llm, "rewrite_bullet") {
            let selected_ids = provenance_ids.iter().collect::<Vec<_>>();
            for id in selected_ids {
                if let Some(candidate) = bullet_candidates.iter().find(|b| &b.id == id) {
                    let prompt = format!(
                        "Rewrite this resume bullet for concise clarity only. Keep tools/metrics/claim exactly.\n\n{}",
                        candidate.text
                    );
                    if let Ok(Some(rewrite)) =
                        llm_rewrite(&cfg.llm, LlmTask::RewriteBullet, "rewrite_bullet", &prompt)
                    {
                        if resume_1pg.contains(&candidate.text) {
                            resume_1pg = resume_1pg.replace(&candidate.text, &rewrite);
                            llm_applied = true;
                        }
                        if let Some(resume_two) = &resume_2pg {
                            if resume_two.contains(&candidate.text) {
                                resume_2pg = Some(resume_two.replace(&candidate.text, &rewrite));
                                llm_applied = true;
                            }
                        }
                    }
                }
            }
        }
    }

    let before = load_resume_template(
        &options.repo_root,
        matches!(input.baseline, crate::types::Baseline::TwoPage),
    )?;
    let output_base =
        input.outdir.clone().unwrap_or_else(|| resolve_output_base(&cfg.output.base_dir));
    let date = output_date(input.run_date);
    let mut diff_md =
        inline_diff_md(&before, &resume_2pg.clone().unwrap_or_else(|| resume_1pg.clone()));

    let make_validation_candidate =
        |extraction_source_value: &ExtractionSource,
         resume_1pg_value: &str,
         resume_2pg_value: Option<String>,
         recruiter_message_value: &str,
         hiring_manager_message_value: &str,
         cover_short_message_value: &str,
         diff_md_value: &str| GenerateResultData {
            extracted: extracted.clone(),
            extraction_source: extraction_source_value.clone(),
            extraction_diagnostics: extraction_diagnostics.clone(),
            track: track.clone(),
            fit: fit.clone(),
            tailor_plan: tailor_plan.clone(),
            bullet_candidates: bullet_candidates.clone(),
            resume_1pg: resume_1pg_value.to_string(),
            resume_2pg: resume_2pg_value,
            recruiter_message: recruiter_message_value.to_string(),
            hiring_manager_message: hiring_manager_message_value.to_string(),
            cover_short_message: cover_short_message_value.to_string(),
            diff_md: diff_md_value.to_string(),
            tracker_row: TrackerRow {
                date: date.format("%Y-%m-%d").to_string(),
                company: input.company.clone(),
                role: input.role.clone(),
                source: input.source.clone(),
                track: track.selected.to_string(),
                fit_total: fit.total,
                status: "new".to_string(),
                next_action: String::new(),
                packet_dir: String::new(),
            },
            truth_report: crate::types::TruthValidationReport {
                passed: true,
                violations: vec![],
                unknown_tools: vec![],
                claim_issues: vec![],
                provenance_complete: true,
            },
            packet_dir: PathBuf::new(),
            files_written: Vec::new(),
        };

    let mut validation_candidate = make_validation_candidate(
        &extraction_source,
        &resume_1pg,
        resume_2pg.clone(),
        &recruiter_message,
        &hiring_manager_message,
        &cover_short_message,
        &diff_md,
    );
    let mut truth_report = validate(&validation_candidate, &banks, &provenance_ids);
    if !truth_report.passed && llm_applied {
        resume_1pg = deterministic_resume_1pg;
        resume_2pg = deterministic_resume_2pg;
        recruiter_message = deterministic_recruiter_message;
        hiring_manager_message = deterministic_hiring_manager_message;
        cover_short_message = deterministic_cover_short_message;
        diff_md =
            inline_diff_md(&before, &resume_2pg.clone().unwrap_or_else(|| resume_1pg.clone()));
        validation_candidate = make_validation_candidate(
            &extraction_source,
            &resume_1pg,
            resume_2pg.clone(),
            &recruiter_message,
            &hiring_manager_message,
            &cover_short_message,
            &diff_md,
        );
        truth_report = validate(&validation_candidate, &banks, &provenance_ids);
    }

    if !truth_report.passed {
        anyhow::bail!("truth gate failed: {}", truth_report.violations.join(", "));
    }

    let (packet_dir, files_written, tracker_row) = write_packet(PacketWriteInput {
        output_base: &output_base,
        date,
        company: &input.company,
        role: &input.role,
        source: &input.source,
        jd_text: &extracted.normalized_text,
        extracted: &extracted,
        fit: &fit,
        tailor_plan: &tailor_plan,
        resume_1pg: &resume_1pg,
        resume_2pg: resume_2pg.as_deref(),
        recruiter_message: &recruiter_message,
        hiring_manager_message: &hiring_manager_message,
        cover_short_message: &cover_short_message,
        diff_md: &diff_md,
        track: track.selected,
    })?;

    let generated = GenerateResultData {
        extracted,
        extraction_source,
        extraction_diagnostics,
        track,
        fit,
        tailor_plan,
        bullet_candidates,
        resume_1pg,
        resume_2pg,
        recruiter_message,
        hiring_manager_message,
        cover_short_message,
        diff_md,
        tracker_row,
        truth_report,
        packet_dir: packet_dir.clone(),
        files_written,
    };

    let review_data_path = generated.packet_dir.join("ReviewData.json");
    let review_data = serde_json::to_string_pretty(&generated)?;
    std::fs::write(&review_data_path, review_data)
        .with_context(|| format!("writing {}", review_data_path.display()))?;

    let id = format!(
        "{}:{}:{}:{}",
        input.company,
        input.role,
        date.format("%Y-%m-%d"),
        hash_jd(&input.jd_text)
    );
    let jd_hash = hash_jd(&input.jd_text);
    let track_label = generated.track.selected.to_string();
    let packet_dir_string = packet_dir.to_string_lossy().to_string();
    let db_path = output_base.join("applykit.db");
    upsert_job_record(
        &db_path,
        UpsertJobRecordInput {
            id: &id,
            company: &input.company,
            role: &input.role,
            source: &input.source,
            baseline: input.baseline.as_cli_value(),
            jd_text: &input.jd_text,
            jd_hash: &jd_hash,
            track: Some(&track_label),
            fit_total: Some(generated.fit.total as i64),
            output_dir: Some(&packet_dir_string),
        },
    )
    .with_context(|| format!("recording job in {}", db_path.display()))?;

    Ok(generated)
}

pub fn list_packets_from_fs(base_dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    if !base_dir.exists() {
        return Ok(Vec::new());
    }
    let mut dirs = Vec::new();
    for entry in std::fs::read_dir(base_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir()
            && !path.file_name().map(|n| n.to_string_lossy().starts_with('.')).unwrap_or(false)
        {
            dirs.push(path);
        }
    }
    dirs.sort();
    Ok(dirs)
}

pub fn read_packet_detail(packet_dir: &Path) -> anyhow::Result<PacketDetail> {
    let review_data_path = packet_dir.join("ReviewData.json");
    if review_data_path.exists() {
        let raw = std::fs::read_to_string(&review_data_path)
            .with_context(|| format!("reading {}", review_data_path.display()))?;
        let data: GenerateResultData = serde_json::from_str(&raw)
            .with_context(|| format!("parsing {}", review_data_path.display()))?;
        return Ok(PacketDetail {
            packet_dir: data.packet_dir,
            extracted: data.extracted,
            extraction_source: data.extraction_source,
            extraction_diagnostics: data.extraction_diagnostics,
            fit: data.fit,
            track: data.track,
            tailor_plan: data.tailor_plan,
            bullet_candidates: data.bullet_candidates,
            messages: crate::types::GeneratedMessages {
                recruiter: data.recruiter_message,
                hiring_manager: data.hiring_manager_message,
                cover_short: data.cover_short_message,
            },
            resume_1pg: data.resume_1pg,
            resume_2pg: data.resume_2pg,
            diff_md: data.diff_md,
            tracker_row: data.tracker_row,
            truth_report: data.truth_report,
        });
    }

    anyhow::bail!("packet detail unavailable: {} missing", review_data_path.display())
}

pub fn read_packet_detail_by_job_id(
    output_base: &Path,
    job_id: &str,
) -> anyhow::Result<PacketDetail> {
    let db_path = output_base.join("applykit.db");
    let job = get_job_by_id(&db_path, job_id)?
        .ok_or_else(|| anyhow::anyhow!("job not found: {job_id}"))?;
    let packet_dir = job
        .output_dir
        .map(PathBuf::from)
        .ok_or_else(|| anyhow::anyhow!("job has no output_dir: {job_id}"))?;
    read_packet_detail(&packet_dir)
}
