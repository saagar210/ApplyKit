use crate::banks::Banks;
use crate::config::ApplykitConfig;
use crate::types::{ExtractedJd, FitScore, TrackSelection};

fn capped_ratio(numerator: usize, denominator: usize, max_points: u8) -> u8 {
    if denominator == 0 {
        return 0;
    }
    let ratio = (numerator as f32 / denominator as f32).min(1.0);
    (ratio * max_points as f32).round() as u8
}

pub fn compute_fit_score(
    extracted: &ExtractedJd,
    track: &TrackSelection,
    banks: &Banks,
    cfg: &ApplykitConfig,
) -> FitScore {
    let top_track_terms = track
        .scores
        .iter()
        .find(|r| r.track == track.selected)
        .map(|r| r.matched_terms.len())
        .unwrap_or(0);

    let role_match = capped_ratio(top_track_terms, 6, cfg.scoring.role_match);

    let approved_skills = banks.skills.iter().filter(|(_, v)| v.approved).count();
    let matched_skill_count = extracted
        .tools
        .iter()
        .filter(|tool| banks.skills.get(*tool).map(|s| s.approved).unwrap_or(false))
        .count();
    let stack_match =
        capped_ratio(matched_skill_count, approved_skills.max(1), cfg.scoring.stack_match);

    let scale_match = capped_ratio(extracted.scale_signals.len(), 3, cfg.scoring.scale_match);
    let rigor_match = capped_ratio(extracted.rigor_signals.len(), 3, cfg.scoring.rigor_match);

    let signal_hits = extracted
        .keywords
        .iter()
        .filter(|k| {
            matches!(
                k.as_str(),
                "python" | "sql" | "incident" | "change" | "audit" | "okta" | "jamf" | "intune"
            )
        })
        .count();
    let signal_boost = capped_ratio(signal_hits, 5, cfg.scoring.signal_boost);

    let total = role_match + stack_match + scale_match + rigor_match + signal_boost;

    let mut why_match = vec![format!("Primary track aligned: {}", track.selected)];
    if !extracted.tools.is_empty() {
        why_match.push(format!("Known tools overlap: {}", extracted.tools.join(", ")));
    }
    if !extracted.scale_signals.is_empty() {
        why_match.push("JD includes scale indicators that match prior operating scope".to_string());
    }

    let mut gaps = Vec::new();
    for req in &extracted.requirements {
        let lower = req.to_ascii_lowercase();
        let matched = banks.skills.keys().any(|k| lower.contains(&k.to_ascii_lowercase()));
        if !matched {
            gaps.push(req.clone());
        }
    }

    FitScore {
        role_match,
        stack_match,
        scale_match,
        rigor_match,
        signal_boost,
        total,
        why_match,
        gaps,
    }
}
