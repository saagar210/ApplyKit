use applykit_core::config::{
    load_config, load_runtime_settings, merge_config_with_runtime, resolve_output_base,
    save_runtime_settings, RuntimeSettings,
};
use applykit_core::insights::build_insights;
use applykit_core::pipeline::{
    list_packets_from_fs, read_packet_detail, read_packet_detail_by_job_id,
};
use applykit_core::source_preview::{
    create_bullet_value, create_skill_value, load_banks_preview, load_templates_preview,
    save_bullet_text_value, save_template_value, set_bullet_approved_value,
    set_skill_approved_value, set_skill_level_value,
};
use applykit_core::storage::{list_jobs, update_job_status};
use applykit_core::types::{Baseline, GenerateInput, Track};
use applykit_core::{generate_packet, GenerateOptions};
use applykit_export::{export_docx, export_markdown_bundle, export_pdf};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeneratePacketInput {
    company: String,
    role: String,
    source: String,
    baseline: String,
    jd_text: String,
    outdir: Option<String>,
    run_date: Option<String>,
    track_override: Option<String>,
    allow_unapproved: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PacketDetailInput {
    job_id: Option<String>,
    packet_dir: Option<String>,
    outdir: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MessagesResponse {
    recruiter: String,
    hiring_manager: String,
    cover_short: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct FitBreakdownResponse {
    role_match: u8,
    stack_match: u8,
    scale_match: u8,
    rigor_match: u8,
    signal_boost: u8,
    total: u8,
    why_match: Vec<String>,
    gaps: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TailorEditResponse {
    kind: String,
    target_section: String,
    reason: String,
    provenance_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TailorPlanResponse {
    edits: Vec<TailorEditResponse>,
    max_resume_edits: usize,
    max_bullet_swaps: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TruthReportResponse {
    passed: bool,
    violations: Vec<String>,
    unknown_tools: Vec<String>,
    claim_issues: Vec<String>,
    provenance_complete: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct BulletCandidateResponse {
    id: String,
    text: String,
    tags: Vec<String>,
    track_hint: String,
    reason: String,
    approved: bool,
    score: i32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TrackerRowResponse {
    date: String,
    company: String,
    role: String,
    source: String,
    track: String,
    fit_total: u8,
    status: String,
    next_action: String,
    packet_dir: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PacketDetailResponse {
    packet_dir: String,
    extraction_source: String,
    extraction_diagnostics: ExtractionDiagnosticsResponse,
    extracted_keywords: Vec<String>,
    extracted_tools: Vec<String>,
    extracted_requirements: Vec<String>,
    fit_breakdown: FitBreakdownResponse,
    track: String,
    track_scores: Vec<(String, i32, Vec<String>)>,
    tailor_plan: TailorPlanResponse,
    bullet_candidates: Vec<BulletCandidateResponse>,
    messages: MessagesResponse,
    resume_1pg: String,
    resume_2pg: Option<String>,
    diff: String,
    tracker_row: TrackerRowResponse,
    truth_report: TruthReportResponse,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ExtractionDiagnosticsResponse {
    summarize_attempted: bool,
    summarize_merged: bool,
    summarize_fallback_reasons: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GeneratePacketResponse {
    packet_dir: String,
    fit_total: u8,
    track: String,
    files_written: Vec<String>,
    truth_passed: bool,
    packet_detail: PacketDetailResponse,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct JobSummary {
    id: String,
    company: String,
    role: String,
    source: String,
    baseline: String,
    track: Option<String>,
    fit_total: Option<i64>,
    status: String,
    next_action: Option<String>,
    notes: Option<String>,
    output_dir: Option<String>,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListJobsInput {
    outdir: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ListJobsResponse {
    jobs: Vec<JobSummary>,
    fallback_packet_dirs: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateJobStatusInput {
    id: String,
    status: String,
    next_action: Option<String>,
    notes: Option<String>,
    outdir: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateJobStatusResponse {
    ok: bool,
    id: String,
    status: String,
    next_action: Option<String>,
    notes: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct InsightsResponse {
    replies_by_track: Vec<(String, usize)>,
    common_gaps: Vec<(String, usize)>,
    keyword_correlations: Vec<(String, usize)>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExportInput {
    packet_dir: String,
    out_dir: Option<String>,
    file_name: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ExportResponse {
    ok: bool,
    output_path: Option<String>,
    message: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SettingsResponse {
    allow_unapproved: bool,
    llm_enabled: bool,
    llm_provider: String,
    llm_base_url: String,
    llm_model: String,
    llm_allowed_tasks: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SaveSettingsInput {
    allow_unapproved: bool,
    llm_enabled: bool,
    llm_provider: String,
    llm_base_url: String,
    llm_model: String,
    llm_allowed_tasks: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct BanksPreviewResponse {
    bullet_bank_json: String,
    skills_bank_json: String,
    bullet_count: usize,
    approved_bullet_count: usize,
    skill_count: usize,
    approved_skill_count: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TemplatesPreviewResponse {
    resume_1pg_base: String,
    resume_2pg_base: String,
    recruiter_template: String,
    hiring_manager_template: String,
    cover_short_template: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SetBulletApprovedInput {
    id: String,
    approved: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SetSkillApprovedInput {
    name: String,
    approved: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SetSkillLevelInput {
    name: String,
    level: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SaveBulletTextInput {
    id: String,
    text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SaveTemplateInput {
    template_key: applykit_core::TemplateKey,
    content: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateBulletInput {
    id: String,
    scope: String,
    claim_level: String,
    text: String,
    seniority: String,
    category: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    tools: Option<Vec<String>>,
    approved: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateSkillInput {
    name: String,
    level: String,
    approved: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MutationResponse {
    ok: bool,
    message: String,
    updated_at: String,
}

fn to_mutation_response(resp: applykit_core::MutationResponse) -> MutationResponse {
    MutationResponse { ok: resp.ok, message: resp.message, updated_at: resp.updated_at }
}

fn to_packet_detail_response(detail: applykit_core::types::PacketDetail) -> PacketDetailResponse {
    PacketDetailResponse {
        packet_dir: detail.packet_dir.display().to_string(),
        extraction_source: match detail.extraction_source {
            applykit_core::types::ExtractionSource::Deterministic => "deterministic".to_string(),
            applykit_core::types::ExtractionSource::LlmMerged => "llm_merged".to_string(),
        },
        extraction_diagnostics: ExtractionDiagnosticsResponse {
            summarize_attempted: detail.extraction_diagnostics.summarize_attempted,
            summarize_merged: detail.extraction_diagnostics.summarize_merged,
            summarize_fallback_reasons: detail.extraction_diagnostics.summarize_fallback_reasons,
        },
        extracted_keywords: detail.extracted.keywords,
        extracted_tools: detail.extracted.tools,
        extracted_requirements: detail.extracted.requirements,
        fit_breakdown: FitBreakdownResponse {
            role_match: detail.fit.role_match,
            stack_match: detail.fit.stack_match,
            scale_match: detail.fit.scale_match,
            rigor_match: detail.fit.rigor_match,
            signal_boost: detail.fit.signal_boost,
            total: detail.fit.total,
            why_match: detail.fit.why_match,
            gaps: detail.fit.gaps,
        },
        track: detail.track.selected.to_string(),
        track_scores: detail
            .track
            .scores
            .into_iter()
            .map(|s| (s.track.to_string(), s.score, s.matched_terms))
            .collect(),
        tailor_plan: TailorPlanResponse {
            edits: detail
                .tailor_plan
                .edits
                .into_iter()
                .map(|e| TailorEditResponse {
                    kind: e.kind,
                    target_section: e.target_section,
                    reason: e.reason,
                    provenance_ids: e.provenance_ids,
                })
                .collect(),
            max_resume_edits: detail.tailor_plan.max_resume_edits,
            max_bullet_swaps: detail.tailor_plan.max_bullet_swaps,
        },
        bullet_candidates: detail
            .bullet_candidates
            .into_iter()
            .map(|c| BulletCandidateResponse {
                id: c.id,
                text: c.text,
                tags: c.tags,
                track_hint: c.track_hint,
                reason: c.reason,
                approved: c.approved,
                score: c.score,
            })
            .collect(),
        messages: MessagesResponse {
            recruiter: detail.messages.recruiter,
            hiring_manager: detail.messages.hiring_manager,
            cover_short: detail.messages.cover_short,
        },
        resume_1pg: detail.resume_1pg,
        resume_2pg: detail.resume_2pg,
        diff: detail.diff_md,
        tracker_row: TrackerRowResponse {
            date: detail.tracker_row.date,
            company: detail.tracker_row.company,
            role: detail.tracker_row.role,
            source: detail.tracker_row.source,
            track: detail.tracker_row.track,
            fit_total: detail.tracker_row.fit_total,
            status: detail.tracker_row.status,
            next_action: detail.tracker_row.next_action,
            packet_dir: detail.tracker_row.packet_dir,
        },
        truth_report: TruthReportResponse {
            passed: detail.truth_report.passed,
            violations: detail.truth_report.violations,
            unknown_tools: detail.truth_report.unknown_tools,
            claim_issues: detail.truth_report.claim_issues,
            provenance_complete: detail.truth_report.provenance_complete,
        },
    }
}

fn repo_root() -> Result<PathBuf, String> {
    std::env::current_dir().map_err(|e| format!("reading repo root: {e}"))
}

fn configured_output_base(repo_root: &Path) -> Result<PathBuf, String> {
    let cfg = load_config(repo_root).map_err(|e| e.to_string())?;
    let base = resolve_output_base(&cfg.output.base_dir);
    std::fs::create_dir_all(&base)
        .map_err(|e| format!("creating output base {}: {e}", base.display()))?;
    base.canonicalize().map_err(|e| format!("canonicalizing output base {}: {e}", base.display()))
}

fn canonicalize_candidate(path: &Path, must_exist: bool, label: &str) -> Result<PathBuf, String> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir().map_err(|e| format!("reading cwd for {label}: {e}"))?.join(path)
    };

    if must_exist || absolute.exists() {
        return absolute
            .canonicalize()
            .map_err(|e| format!("canonicalizing {label} {}: {e}", absolute.display()));
    }

    let mut cursor = absolute.as_path();
    let mut missing = Vec::new();
    while !cursor.exists() {
        let Some(segment) = cursor.file_name() else {
            return Err(format!("{label} has no existing ancestor: {}", absolute.display()));
        };
        if segment == OsStr::new(".") || segment == OsStr::new("..") {
            return Err(format!("{label} cannot contain '.' or '..' segments"));
        }
        missing.push(segment.to_os_string());
        cursor = cursor
            .parent()
            .ok_or_else(|| format!("{label} has no parent: {}", absolute.display()))?;
    }

    if !cursor.is_dir() {
        return Err(format!("{label} parent must be a directory: {}", cursor.display()));
    }

    let mut canonical = cursor
        .canonicalize()
        .map_err(|e| format!("canonicalizing parent for {label} {}: {e}", cursor.display()))?;
    for segment in missing.iter().rev() {
        canonical.push(segment);
    }
    Ok(canonical)
}

fn ensure_within_base(base: &Path, candidate: &Path, label: &str) -> Result<(), String> {
    if candidate == base || candidate.starts_with(base) {
        return Ok(());
    }
    Err(format!("{label} must stay under configured output base: {}", base.display()))
}

fn resolve_scoped_output_base(repo_root: &Path, outdir: Option<String>) -> Result<PathBuf, String> {
    let base = configured_output_base(repo_root)?;
    match outdir {
        Some(raw) => {
            let scoped = canonicalize_candidate(Path::new(&raw), false, "outdir")?;
            ensure_within_base(&base, &scoped, "outdir")?;
            Ok(scoped)
        }
        None => Ok(base),
    }
}

fn resolve_existing_output_path(
    repo_root: &Path,
    raw_path: &str,
    label: &str,
) -> Result<PathBuf, String> {
    let base = configured_output_base(repo_root)?;
    let path = canonicalize_candidate(Path::new(raw_path), true, label)?;
    ensure_within_base(&base, &path, label)?;
    Ok(path)
}

fn resolve_packet_dir(repo_root: &Path, raw_path: &str) -> Result<PathBuf, String> {
    let packet_dir = resolve_existing_output_path(repo_root, raw_path, "packet_dir")?;
    if !packet_dir.is_dir() {
        return Err(format!("packet_dir must be a directory: {}", packet_dir.display()));
    }
    if !packet_dir.join("ReviewData.json").exists() {
        return Err(format!("packet_dir is missing ReviewData.json: {}", packet_dir.display()));
    }
    Ok(packet_dir)
}

fn resolve_export_out_dir(
    repo_root: &Path,
    packet_dir: &Path,
    out_dir: Option<String>,
) -> Result<PathBuf, String> {
    let base = configured_output_base(repo_root)?;
    let candidate = match out_dir {
        Some(raw) => canonicalize_candidate(Path::new(&raw), false, "out_dir")?,
        None => packet_dir.parent().unwrap_or(Path::new(".")).to_path_buf().join("exports"),
    };
    ensure_within_base(&base, &candidate, "out_dir")?;
    Ok(candidate)
}

fn sanitize_export_file_name(
    file_name: Option<String>,
    default_name: String,
    required_extension: &str,
) -> Result<String, String> {
    let mut raw = file_name.unwrap_or(default_name);
    raw = raw.trim().to_string();
    if raw.is_empty() {
        return Err("file_name cannot be empty".to_string());
    }

    let path = Path::new(&raw);
    if path.components().count() != 1 || path.file_name().is_none() {
        return Err("file_name must not include path separators".to_string());
    }

    if !raw.to_ascii_lowercase().ends_with(&required_extension.to_ascii_lowercase()) {
        raw.push_str(required_extension);
    }

    Ok(raw)
}

fn required_trimmed_field(name: &str, value: String) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(format!("{name} is required"));
    }
    Ok(trimmed.to_string())
}

fn ensure_non_empty_text(name: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err(format!("{name} is required"));
    }
    Ok(())
}

#[tauri::command]
fn generate_packet_cmd(input: GeneratePacketInput) -> Result<GeneratePacketResponse, String> {
    let repo_root = repo_root()?;
    let company = required_trimmed_field("company", input.company)?;
    let role = required_trimmed_field("role", input.role)?;
    let source = required_trimmed_field("source", input.source)?;
    ensure_non_empty_text("jdText", &input.jd_text)?;
    let baseline =
        input.baseline.parse::<Baseline>().map_err(|e| format!("invalid baseline: {e}"))?;

    let run_date = match input.run_date {
        Some(date) => Some(
            NaiveDate::parse_from_str(&date, "%Y-%m-%d")
                .map_err(|e| format!("invalid date format (expected YYYY-MM-DD): {e}"))?,
        ),
        None => None,
    };

    let track_override = match input.track_override {
        Some(value) => Some(value.parse::<Track>().map_err(|e| format!("invalid track: {e}"))?),
        None => None,
    };
    let scoped_outdir = match input.outdir {
        Some(raw) => Some(resolve_scoped_output_base(&repo_root, Some(raw))?),
        None => None,
    };

    let result = generate_packet(
        GenerateInput {
            company,
            role,
            source,
            baseline,
            jd_text: input.jd_text,
            outdir: scoped_outdir,
            run_date,
            track_override,
            allow_unapproved: input.allow_unapproved.unwrap_or(false),
        },
        GenerateOptions { repo_root },
    )
    .map_err(|e| format!("generation failed: {e:#}"))?;

    let packet_detail = to_packet_detail_response(applykit_core::types::PacketDetail {
        packet_dir: result.packet_dir.clone(),
        extracted: result.extracted.clone(),
        extraction_source: result.extraction_source.clone(),
        extraction_diagnostics: result.extraction_diagnostics.clone(),
        fit: result.fit.clone(),
        track: result.track.clone(),
        tailor_plan: result.tailor_plan.clone(),
        bullet_candidates: result.bullet_candidates.clone(),
        messages: applykit_core::types::GeneratedMessages {
            recruiter: result.recruiter_message.clone(),
            hiring_manager: result.hiring_manager_message.clone(),
            cover_short: result.cover_short_message.clone(),
        },
        resume_1pg: result.resume_1pg.clone(),
        resume_2pg: result.resume_2pg.clone(),
        diff_md: result.diff_md.clone(),
        tracker_row: result.tracker_row.clone(),
        truth_report: result.truth_report.clone(),
    });

    Ok(GeneratePacketResponse {
        packet_dir: result.packet_dir.display().to_string(),
        fit_total: result.fit.total,
        track: result.track.selected.to_string(),
        files_written: result.files_written.iter().map(|path| path.display().to_string()).collect(),
        truth_passed: result.truth_report.passed,
        packet_detail,
    })
}

#[tauri::command]
fn get_packet_detail_cmd(input: PacketDetailInput) -> Result<PacketDetailResponse, String> {
    let repo_root = repo_root()?;
    let base = resolve_scoped_output_base(&repo_root, input.outdir)?;

    let detail = if let Some(job_id) = input.job_id {
        read_packet_detail_by_job_id(&base, &job_id).map_err(|e| e.to_string())?
    } else if let Some(packet_dir) = input.packet_dir {
        let scoped_packet_dir = resolve_packet_dir(&repo_root, &packet_dir)?;
        read_packet_detail(&scoped_packet_dir).map_err(|e| e.to_string())?
    } else {
        return Err("jobId or packetDir is required".to_string());
    };

    Ok(to_packet_detail_response(detail))
}

#[tauri::command]
fn list_jobs_cmd(input: Option<ListJobsInput>) -> Result<ListJobsResponse, String> {
    let repo_root = repo_root()?;
    let base = resolve_scoped_output_base(&repo_root, input.and_then(|i| i.outdir))?;

    let db_path = base.join("applykit.db");
    let jobs = list_jobs(&db_path)
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|j| JobSummary {
            id: j.id,
            company: j.company,
            role: j.role,
            source: j.source,
            baseline: j.baseline,
            track: j.track,
            fit_total: j.fit_total,
            status: j.status,
            next_action: j.next_action,
            notes: j.notes,
            output_dir: j.output_dir,
            updated_at: j.updated_at,
        })
        .collect::<Vec<_>>();

    let fallback_packet_dirs = list_packets_from_fs(&base)
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|p| p.display().to_string())
        .collect::<Vec<_>>();

    Ok(ListJobsResponse { jobs, fallback_packet_dirs })
}

#[tauri::command]
fn update_job_status_cmd(input: UpdateJobStatusInput) -> Result<UpdateJobStatusResponse, String> {
    let repo_root = repo_root()?;
    let base = resolve_scoped_output_base(&repo_root, input.outdir.clone())?;
    let db_path = base.join("applykit.db");
    let status = input.status.to_ascii_lowercase();

    update_job_status(
        &db_path,
        &input.id,
        &status,
        input.next_action.as_deref(),
        input.notes.as_deref(),
    )
    .map_err(|e| e.to_string())?;

    Ok(UpdateJobStatusResponse {
        ok: true,
        id: input.id,
        status,
        next_action: input.next_action,
        notes: input.notes,
    })
}

#[tauri::command]
fn insights_cmd(input: Option<ListJobsInput>) -> Result<InsightsResponse, String> {
    let repo_root = repo_root()?;
    let base = resolve_scoped_output_base(&repo_root, input.and_then(|i| i.outdir))?;

    let db_path = base.join("applykit.db");
    let jobs = list_jobs(&db_path).map_err(|e| e.to_string())?;
    let insights = build_insights(&jobs);
    Ok(InsightsResponse {
        replies_by_track: insights.replies_by_track,
        common_gaps: insights.common_gaps,
        keyword_correlations: insights.keyword_correlations,
    })
}

#[tauri::command]
fn export_markdown_cmd(input: ExportInput) -> Result<ExportResponse, String> {
    let repo_root = repo_root()?;
    let packet_dir = resolve_packet_dir(&repo_root, &input.packet_dir)?;
    let out_dir = resolve_export_out_dir(&repo_root, &packet_dir, input.out_dir)?;
    let output = export_markdown_bundle(&packet_dir, &out_dir).map_err(|e| e.to_string())?;
    Ok(ExportResponse {
        ok: true,
        output_path: Some(output.display().to_string()),
        message: "Markdown export complete".to_string(),
    })
}

#[tauri::command]
fn export_docx_cmd(input: ExportInput) -> Result<ExportResponse, String> {
    let repo_root = repo_root()?;
    let packet_dir = resolve_packet_dir(&repo_root, &input.packet_dir)?;
    let out_dir = resolve_export_out_dir(&repo_root, &packet_dir, input.out_dir)?;
    std::fs::create_dir_all(&out_dir).map_err(|e| e.to_string())?;
    let default_name = format!(
        "{}.docx",
        packet_dir
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "packet".to_string())
    );
    let file_name = sanitize_export_file_name(input.file_name, default_name, ".docx")?;
    let out_path = out_dir.join(file_name);

    match export_docx(&packet_dir, &out_path) {
        Ok(()) => Ok(ExportResponse {
            ok: true,
            output_path: Some(out_path.display().to_string()),
            message: "DOCX export complete".to_string(),
        }),
        Err(err) => Ok(ExportResponse { ok: false, output_path: None, message: err.to_string() }),
    }
}

#[tauri::command]
fn export_pdf_cmd(input: ExportInput) -> Result<ExportResponse, String> {
    let repo_root = repo_root()?;
    let packet_dir = resolve_packet_dir(&repo_root, &input.packet_dir)?;
    let out_dir = resolve_export_out_dir(&repo_root, &packet_dir, input.out_dir)?;
    std::fs::create_dir_all(&out_dir).map_err(|e| e.to_string())?;
    let default_name = format!(
        "{}.pdf",
        packet_dir
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "packet".to_string())
    );
    let file_name = sanitize_export_file_name(input.file_name, default_name, ".pdf")?;
    let out_path = out_dir.join(file_name);

    match export_pdf(&packet_dir, &out_path) {
        Ok(()) => Ok(ExportResponse {
            ok: true,
            output_path: Some(out_path.display().to_string()),
            message: "PDF export complete".to_string(),
        }),
        Err(err) => Ok(ExportResponse {
            ok: false,
            output_path: None,
            message: format!("PDF export unavailable: {err}"),
        }),
    }
}

#[tauri::command]
fn get_settings_cmd() -> Result<SettingsResponse, String> {
    let repo_root = repo_root()?;
    let cfg = load_config(&repo_root).map_err(|e| e.to_string())?;
    let runtime = load_runtime_settings(&repo_root).map_err(|e| e.to_string())?;
    let merged = merge_config_with_runtime(cfg, &runtime);

    Ok(SettingsResponse {
        allow_unapproved: runtime.allow_unapproved,
        llm_enabled: merged.llm.enabled,
        llm_provider: merged.llm.provider,
        llm_base_url: merged.llm.base_url,
        llm_model: merged.llm.model,
        llm_allowed_tasks: merged.llm.allowed_tasks,
    })
}

#[tauri::command]
fn save_settings_cmd(input: SaveSettingsInput) -> Result<SettingsResponse, String> {
    let repo_root = repo_root()?;
    let runtime = RuntimeSettings {
        allow_unapproved: input.allow_unapproved,
        llm_enabled: Some(input.llm_enabled),
        llm_provider: Some(input.llm_provider),
        llm_base_url: Some(input.llm_base_url),
        llm_model: Some(input.llm_model),
        llm_allowed_tasks: input.llm_allowed_tasks,
    };
    save_runtime_settings(&repo_root, &runtime).map_err(|e| e.to_string())?;
    get_settings_cmd()
}

#[tauri::command]
fn get_banks_preview_cmd() -> Result<BanksPreviewResponse, String> {
    let repo_root = repo_root()?;
    let preview = load_banks_preview(&repo_root).map_err(|e| e.to_string())?;
    Ok(BanksPreviewResponse {
        bullet_bank_json: preview.bullet_bank_json,
        skills_bank_json: preview.skills_bank_json,
        bullet_count: preview.bullet_count,
        approved_bullet_count: preview.approved_bullet_count,
        skill_count: preview.skill_count,
        approved_skill_count: preview.approved_skill_count,
    })
}

#[tauri::command]
fn get_templates_preview_cmd() -> Result<TemplatesPreviewResponse, String> {
    let repo_root = repo_root()?;
    let preview = load_templates_preview(&repo_root).map_err(|e| e.to_string())?;
    Ok(TemplatesPreviewResponse {
        resume_1pg_base: preview.resume_1pg_base,
        resume_2pg_base: preview.resume_2pg_base,
        recruiter_template: preview.recruiter_template,
        hiring_manager_template: preview.hiring_manager_template,
        cover_short_template: preview.cover_short_template,
    })
}

#[tauri::command]
fn set_bullet_approved_cmd(input: SetBulletApprovedInput) -> Result<MutationResponse, String> {
    let repo_root = repo_root()?;
    let resp = set_bullet_approved_value(&repo_root, &input.id, input.approved)
        .map_err(|e| e.to_string())?;
    Ok(to_mutation_response(resp))
}

#[tauri::command]
fn set_skill_approved_cmd(input: SetSkillApprovedInput) -> Result<MutationResponse, String> {
    let repo_root = repo_root()?;
    let resp = set_skill_approved_value(&repo_root, &input.name, input.approved)
        .map_err(|e| e.to_string())?;
    Ok(to_mutation_response(resp))
}

#[tauri::command]
fn set_skill_level_cmd(input: SetSkillLevelInput) -> Result<MutationResponse, String> {
    let repo_root = repo_root()?;
    let resp =
        set_skill_level_value(&repo_root, &input.name, &input.level).map_err(|e| e.to_string())?;
    Ok(to_mutation_response(resp))
}

#[tauri::command]
fn save_bullet_text_cmd(input: SaveBulletTextInput) -> Result<MutationResponse, String> {
    let repo_root = repo_root()?;
    let resp =
        save_bullet_text_value(&repo_root, &input.id, &input.text).map_err(|e| e.to_string())?;
    Ok(to_mutation_response(resp))
}

#[tauri::command]
fn save_template_cmd(input: SaveTemplateInput) -> Result<MutationResponse, String> {
    let repo_root = repo_root()?;
    let resp = save_template_value(&repo_root, &input.template_key, &input.content)
        .map_err(|e| e.to_string())?;
    Ok(to_mutation_response(resp))
}

#[tauri::command]
fn create_bullet_cmd(input: CreateBulletInput) -> Result<MutationResponse, String> {
    let repo_root = repo_root()?;
    let resp = create_bullet_value(
        &repo_root,
        applykit_core::CreateBulletInput {
            id: input.id,
            scope: input.scope,
            claim_level: input.claim_level,
            text: input.text,
            seniority: input.seniority,
            category: input.category.unwrap_or_default(),
            tags: input.tags.unwrap_or_default(),
            tools: input.tools.unwrap_or_default(),
            approved: input.approved.unwrap_or(false),
        },
    )
    .map_err(|e| e.to_string())?;
    Ok(to_mutation_response(resp))
}

#[tauri::command]
fn create_skill_cmd(input: CreateSkillInput) -> Result<MutationResponse, String> {
    let repo_root = repo_root()?;
    let resp =
        create_skill_value(&repo_root, &input.name, &input.level, input.approved.unwrap_or(false))
            .map_err(|e| e.to_string())?;
    Ok(to_mutation_response(resp))
}

#[tauri::command]
fn open_output_folder(path: String) -> Result<(), String> {
    let repo_root = repo_root()?;
    let p = resolve_existing_output_path(&repo_root, &path, "path")?;
    if !p.is_dir() {
        return Err(format!("path must be a directory: {}", p.display()));
    }

    let status = {
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open").arg(&p).status().map_err(|e| e.to_string())?
        }

        #[cfg(target_os = "linux")]
        {
            std::process::Command::new("xdg-open").arg(&p).status().map_err(|e| e.to_string())?
        }

        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("explorer").arg(&p).status().map_err(|e| e.to_string())?
        }
    };
    if !status.success() {
        return Err(format!("failed to open output folder: {}", p.display()));
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_packet_cmd,
            get_packet_detail_cmd,
            list_jobs_cmd,
            update_job_status_cmd,
            insights_cmd,
            export_markdown_cmd,
            export_docx_cmd,
            export_pdf_cmd,
            get_settings_cmd,
            save_settings_cmd,
            get_banks_preview_cmd,
            get_templates_preview_cmd,
            set_bullet_approved_cmd,
            set_skill_approved_cmd,
            set_skill_level_cmd,
            save_bullet_text_cmd,
            save_template_cmd,
            create_bullet_cmd,
            create_skill_cmd,
            open_output_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_test_config(repo_root: &Path, base_dir: &Path) {
        let config_dir = repo_root.join("config");
        std::fs::create_dir_all(&config_dir).expect("create config dir");
        let escaped_base = base_dir.to_string_lossy().replace('\\', "\\\\");
        let content = format!(
            r#"[output]
base_dir = "{escaped_base}"
date_format = "%Y-%m-%d"

[determinism]
sort_tools = "jd_priority_then_alpha"
sort_bullets = "score_then_id"
max_resume_edits = 3
max_bullet_swaps = 2

[scoring]
role_match = 30
stack_match = 30
scale_match = 20
rigor_match = 10
signal_boost = 10

[tracks]
support_ops = []
identity_endpoint = []
security_compliance_ops = []
automation_aiops = []
managerish = []

[llm]
enabled = false
provider = "ollama"
base_url = "http://127.0.0.1:11434"
model = "none"
allowed_tasks = []
"#
        );
        std::fs::write(config_dir.join("applykit.toml"), content).expect("write config");
    }

    #[test]
    fn sanitize_export_file_name_rejects_path_segments() {
        let err = sanitize_export_file_name(
            Some("../escape".to_string()),
            "packet.docx".to_string(),
            ".docx",
        )
        .expect_err("path separators should fail");
        assert!(err.contains("path separators"));
    }

    #[test]
    fn sanitize_export_file_name_appends_extension() {
        let file = sanitize_export_file_name(
            Some("packet_export".to_string()),
            "packet.docx".to_string(),
            ".docx",
        )
        .expect("name");
        assert_eq!(file, "packet_export.docx");
    }

    #[test]
    fn resolve_scoped_output_base_rejects_escape() {
        let repo = tempfile::tempdir().expect("repo");
        let base = repo.path().join("allowed");
        let outside = repo.path().join("outside");
        std::fs::create_dir_all(&base).expect("base");
        std::fs::create_dir_all(&outside).expect("outside");
        write_test_config(repo.path(), &base);

        let err =
            resolve_scoped_output_base(repo.path(), Some(outside.to_string_lossy().to_string()))
                .expect_err("outside outdir should fail");
        assert!(err.contains("must stay under configured output base"));
    }

    #[test]
    fn required_trimmed_field_rejects_blank() {
        let err = required_trimmed_field("company", "   ".to_string()).expect_err("blank");
        assert!(err.contains("company is required"));
    }
}
