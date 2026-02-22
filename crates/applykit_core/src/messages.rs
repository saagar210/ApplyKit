use crate::types::{FitScore, Track};
use anyhow::Context;
use std::collections::BTreeMap;
use std::path::Path;

fn render_template(template: &str, vars: &BTreeMap<&str, String>) -> String {
    let mut out = template.to_string();
    for (key, value) in vars {
        let needle = format!("{{{{{key}}}}}");
        out = out.replace(&needle, value);
    }
    out
}

fn load_template(repo_root: &Path, file: &str) -> anyhow::Result<String> {
    let path = repo_root.join("templates").join("messages").join(file);
    std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))
}

pub fn generate_messages(
    repo_root: &Path,
    company: &str,
    role: &str,
    fit: &FitScore,
    track: Track,
    top_matches: &[String],
) -> anyhow::Result<(String, String, String)> {
    let recruiter_template = load_template(repo_root, "recruiter.md")?;
    let hiring_template = load_template(repo_root, "hiring_manager.md")?;
    let cover_template = load_template(repo_root, "cover_short.md")?;

    let top1 = top_matches
        .first()
        .cloned()
        .unwrap_or_else(|| "Delivered reliable support at scale".to_string());
    let top2 = top_matches
        .get(1)
        .cloned()
        .unwrap_or_else(|| "Applied automation and metrics for proactive remediation".to_string());

    let mut vars = BTreeMap::new();
    vars.insert("company", company.to_string());
    vars.insert("role", role.to_string());
    vars.insert("name", "Hiring Team".to_string());
    vars.insert("how_i_help", format!("Track focus: {track}"));
    vars.insert("top_match_1", top1.clone());
    vars.insert("top_match_2", top2.clone());
    vars.insert("proof_metric", format!("fit score {} / 100", fit.total));
    vars.insert("problem_1", "operational rigor".to_string());
    vars.insert("problem_2", "support-to-systems leverage".to_string());
    vars.insert("cta", "Happy to share details and tailored materials.".to_string());

    let recruiter = render_template(&recruiter_template, &vars);
    let hiring = render_template(&hiring_template, &vars);
    let cover = render_template(&cover_template, &vars);

    Ok((recruiter, hiring, cover))
}
