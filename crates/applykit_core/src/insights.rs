use crate::storage::JobRecord;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insights {
    pub replies_by_track: Vec<(String, usize)>,
    pub common_gaps: Vec<(String, usize)>,
    pub keyword_correlations: Vec<(String, usize)>,
}

pub fn build_insights(jobs: &[JobRecord]) -> Insights {
    let mut replies = BTreeMap::<String, usize>::new();
    let mut gaps = BTreeMap::<String, usize>::new();
    let mut correlations = BTreeMap::<String, usize>::new();

    for job in jobs {
        if job.status.eq_ignore_ascii_case("reply") || job.status.eq_ignore_ascii_case("interview")
        {
            let key = job.track.clone().unwrap_or_else(|| "unknown".to_string());
            *replies.entry(key).or_insert(0) += 1;
        }

        let role_lower = job.role.to_ascii_lowercase();
        for token in ["python", "sql", "okta", "jamf", "audit", "incident", "change"] {
            if role_lower.contains(token) {
                *correlations.entry(token.to_string()).or_insert(0) += 1;
            }
        }

        if let Some(notes) = &job.notes {
            for marker in ["gap:", "missing:"] {
                if let Some(idx) = notes.to_ascii_lowercase().find(marker) {
                    let value = notes[idx + marker.len()..].trim().to_string();
                    if !value.is_empty() {
                        *gaps.entry(value).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    let mut replies_by_track = replies.into_iter().collect::<Vec<_>>();
    replies_by_track.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let mut common_gaps = gaps.into_iter().collect::<Vec<_>>();
    common_gaps.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let mut keyword_correlations = correlations.into_iter().collect::<Vec<_>>();
    keyword_correlations.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    Insights { replies_by_track, common_gaps, keyword_correlations }
}
