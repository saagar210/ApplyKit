use crate::config::ApplykitConfig;
use crate::types::{Track, TrackScore, TrackSelection};

fn score_terms(text: &str, terms: &[String]) -> (i32, Vec<String>) {
    let mut matched = Vec::new();
    for term in terms {
        if text.contains(&term.to_ascii_lowercase()) {
            matched.push(term.clone());
        }
    }
    let score = (matched.len() as i32) * 10;
    (score, matched)
}

pub fn classify_track(
    normalized_text: &str,
    cfg: &ApplykitConfig,
    override_track: Option<Track>,
) -> TrackSelection {
    if let Some(track) = override_track {
        return TrackSelection {
            selected: track,
            scores: vec![TrackScore {
                track,
                score: 100,
                matched_terms: vec!["manual_override".to_string()],
            }],
        };
    }

    let text = normalized_text.to_ascii_lowercase();
    let mut scores = vec![];

    let tracks = vec![
        (Track::SupportOpsCore, cfg.tracks.support_ops.clone()),
        (Track::IdentityEndpoint, cfg.tracks.identity_endpoint.clone()),
        (Track::SecurityComplianceOps, cfg.tracks.security_compliance_ops.clone()),
        (Track::AutomationAiOps, cfg.tracks.automation_aiops.clone()),
        (Track::Managerish, cfg.tracks.managerish.clone()),
    ];

    for (track, terms) in tracks {
        let (score, matched_terms) = score_terms(&text, &terms);
        scores.push(TrackScore { track, score, matched_terms });
    }

    scores.sort_by(|a, b| {
        b.score.cmp(&a.score).then_with(|| a.track.precedence().cmp(&b.track.precedence()))
    });

    let selected = scores.first().map(|r| r.track).unwrap_or(Track::SupportOpsCore);

    TrackSelection { selected, scores }
}
