use crate::banks::{Banks, Bullet};
use crate::config::ApplykitConfig;
use crate::determinism::cmp_score_desc_id_asc;
use crate::types::{BulletCandidate, ExtractedJd, TailorEdit, TailorPlan, Track};
use anyhow::{anyhow, Context};
use std::path::Path;

pub fn load_resume_template(repo_root: &Path, baseline_two_page: bool) -> anyhow::Result<String> {
    let file = if baseline_two_page { "resume_2pg_base.md" } else { "resume_1pg_base.md" };
    let path = repo_root.join("templates").join("resume").join(file);
    std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))
}

fn relevance_score(bullet: &Bullet, extracted: &ExtractedJd, track: Track) -> i32 {
    let mut score = 0;
    for tag in &bullet.tags {
        if extracted.keywords.iter().any(|k| k == &tag.to_ascii_lowercase()) {
            score += 6;
        }
    }
    for tool in &bullet.tools {
        if extracted.tools.iter().any(|jt| jt.eq_ignore_ascii_case(tool.as_str())) {
            score += 9;
        }
    }
    if bullet.category.iter().any(|c| c.eq_ignore_ascii_case("security"))
        && matches!(track, Track::SecurityComplianceOps)
    {
        score += 5;
    }
    if bullet.category.iter().any(|c| c.eq_ignore_ascii_case("automation"))
        && matches!(track, Track::AutomationAiOps)
    {
        score += 5;
    }
    if bullet.category.iter().any(|c| c.eq_ignore_ascii_case("support"))
        && matches!(track, Track::SupportOpsCore)
    {
        score += 4;
    }
    score
}

fn track_hint_for_bullet(bullet: &Bullet) -> String {
    if bullet.category.iter().any(|c| c.eq_ignore_ascii_case("security")) {
        return "security".to_string();
    }
    if bullet
        .category
        .iter()
        .any(|c| c.eq_ignore_ascii_case("automation") || c.eq_ignore_ascii_case("ai"))
    {
        return "automation".to_string();
    }
    if bullet
        .category
        .iter()
        .any(|c| c.eq_ignore_ascii_case("support") || c.eq_ignore_ascii_case("ops"))
    {
        return "support".to_string();
    }
    if bullet
        .category
        .iter()
        .any(|c| c.eq_ignore_ascii_case("management") || c.eq_ignore_ascii_case("leadership"))
    {
        return "managerish".to_string();
    }
    "general".to_string()
}

fn find_section_range(lines: &[String], section_anchor: &str) -> Option<(usize, usize)> {
    let start_marker = format!("<!--SECTION:{section_anchor}-->");
    let start_idx = lines.iter().position(|line| line.trim() == start_marker)?;

    let mut end_idx = lines.len();
    for (idx, line) in lines.iter().enumerate().skip(start_idx + 1) {
        if line.trim().starts_with("<!--SECTION:") {
            end_idx = idx;
            break;
        }
    }
    Some((start_idx + 1, end_idx))
}

pub fn tailor_resume(
    template: &str,
    extracted: &ExtractedJd,
    track: Track,
    banks: &Banks,
    cfg: &ApplykitConfig,
    allow_unapproved: bool,
) -> anyhow::Result<(String, TailorPlan, Vec<String>, Vec<BulletCandidate>)> {
    let mut candidates =
        banks.bullets.iter().filter(|b| allow_unapproved || b.approved).collect::<Vec<_>>();

    candidates.sort_by(|a, b| {
        let a_score = relevance_score(a, extracted, track);
        let b_score = relevance_score(b, extracted, track);
        cmp_score_desc_id_asc(a_score, &a.id, b_score, &b.id)
    });

    let ranked_candidates = candidates
        .iter()
        .take(20)
        .map(|bullet| {
            let score = relevance_score(bullet, extracted, track);
            let mut reasons = Vec::new();
            if bullet
                .tools
                .iter()
                .any(|tool| extracted.tools.iter().any(|jt| jt.eq_ignore_ascii_case(tool)))
            {
                reasons.push("tool overlap".to_string());
            }
            if bullet
                .tags
                .iter()
                .any(|tag| extracted.keywords.iter().any(|k| k.eq_ignore_ascii_case(tag)))
            {
                reasons.push("keyword overlap".to_string());
            }
            if reasons.is_empty() {
                reasons.push("track alignment".to_string());
            }

            BulletCandidate {
                id: bullet.id.clone(),
                text: bullet.text.clone(),
                tags: bullet.tags.clone(),
                track_hint: track_hint_for_bullet(bullet),
                reason: reasons.join(", "),
                approved: bullet.approved,
                score,
            }
        })
        .collect::<Vec<_>>();

    let selected =
        candidates.into_iter().take(cfg.determinism.max_bullet_swaps).cloned().collect::<Vec<_>>();

    let mut lines = template.lines().map(|line| line.to_string()).collect::<Vec<_>>();
    let (start, end) = find_section_range(&lines, "BOX_BULLETS")
        .ok_or_else(|| anyhow!("resume template missing SECTION:BOX_BULLETS"))?;

    let bullet_line_indices =
        (start..end).filter(|idx| lines[*idx].trim_start().starts_with("- ")).collect::<Vec<_>>();

    let mut edits = Vec::new();
    let mut provenance_ids = Vec::new();
    for (idx, bullet) in selected.iter().enumerate() {
        if idx >= bullet_line_indices.len() {
            break;
        }
        let line_idx = bullet_line_indices[idx];
        lines[line_idx] = format!("- {}", bullet.text);
        provenance_ids.push(bullet.id.clone());
        edits.push(TailorEdit {
            kind: "bullet_swap".to_string(),
            target_section: "BOX_BULLETS".to_string(),
            reason: format!("Matched JD using tags/tools overlap for {}", bullet.id),
            provenance_ids: vec![bullet.id.clone()],
        });
    }

    edits.push(TailorEdit {
        kind: "summary_alignment".to_string(),
        target_section: "SUMMARY".to_string(),
        reason: format!("Summary kept truth-first and aligned to selected track: {track}"),
        provenance_ids: vec![],
    });

    edits.push(TailorEdit {
        kind: "stack_focus".to_string(),
        target_section: "STACK".to_string(),
        reason: "Stack emphasis ordered by JD overlap and approved skills".to_string(),
        provenance_ids: provenance_ids.clone(),
    });

    let edits = edits.into_iter().take(cfg.determinism.max_resume_edits).collect::<Vec<_>>();

    let tailored = lines.join("\n");
    let plan = TailorPlan {
        edits,
        max_resume_edits: cfg.determinism.max_resume_edits,
        max_bullet_swaps: cfg.determinism.max_bullet_swaps,
    };
    Ok((tailored, plan, provenance_ids, ranked_candidates))
}
