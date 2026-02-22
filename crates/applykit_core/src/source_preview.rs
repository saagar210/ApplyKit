use crate::banks::{
    add_bullet, add_skill, load_banks, save_bullet_text, set_bullet_approved, set_skill_approved,
    set_skill_level, BulletRecord,
};
use crate::config::atomic_write_text;
use anyhow::Context;
use chrono::Utc;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BanksPreview {
    pub bullet_bank_json: String,
    pub skills_bank_json: String,
    pub bullet_count: usize,
    pub approved_bullet_count: usize,
    pub skill_count: usize,
    pub approved_skill_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatesPreview {
    pub resume_1pg_base: String,
    pub resume_2pg_base: String,
    pub recruiter_template: String,
    pub hiring_manager_template: String,
    pub cover_short_template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateKey {
    Resume1pgBase,
    Resume2pgBase,
    Recruiter,
    HiringManager,
    CoverShort,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationResponse {
    pub ok: bool,
    pub message: String,
    pub updated_at: String,
}

fn now_iso() -> String {
    Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

pub fn load_banks_preview(repo_root: &Path) -> anyhow::Result<BanksPreview> {
    let bullet_path = repo_root.join("data").join("bullet_bank.json");
    let skills_path = repo_root.join("data").join("skills_bank.json");
    let bullet_bank_json = std::fs::read_to_string(&bullet_path)
        .with_context(|| format!("reading {}", bullet_path.display()))?;
    let skills_bank_json = std::fs::read_to_string(&skills_path)
        .with_context(|| format!("reading {}", skills_path.display()))?;

    let banks = load_banks(repo_root)?;
    let bullet_count = banks.bullets.len();
    let approved_bullet_count = banks.bullets.iter().filter(|b| b.approved).count();
    let skill_count = banks.skills.len();
    let approved_skill_count = banks.skills.values().filter(|s| s.approved).count();

    Ok(BanksPreview {
        bullet_bank_json,
        skills_bank_json,
        bullet_count,
        approved_bullet_count,
        skill_count,
        approved_skill_count,
    })
}

fn response_ok(message: impl Into<String>) -> MutationResponse {
    MutationResponse { ok: true, message: message.into(), updated_at: now_iso() }
}

pub fn set_bullet_approved_value(
    repo_root: &Path,
    id: &str,
    approved: bool,
) -> anyhow::Result<MutationResponse> {
    set_bullet_approved(repo_root, id, approved)?;
    Ok(response_ok(format!("bullet {id} {}", if approved { "approved" } else { "unapproved" })))
}

pub fn set_skill_approved_value(
    repo_root: &Path,
    name: &str,
    approved: bool,
) -> anyhow::Result<MutationResponse> {
    set_skill_approved(repo_root, name, approved)?;
    Ok(response_ok(format!("skill {name} {}", if approved { "approved" } else { "unapproved" })))
}

pub fn set_skill_level_value(
    repo_root: &Path,
    name: &str,
    level: &str,
) -> anyhow::Result<MutationResponse> {
    set_skill_level(repo_root, name, level)?;
    Ok(response_ok(format!("skill {name} level updated")))
}

pub fn save_bullet_text_value(
    repo_root: &Path,
    id: &str,
    text: &str,
) -> anyhow::Result<MutationResponse> {
    save_bullet_text(repo_root, id, text)?;
    Ok(response_ok(format!("bullet {id} text updated")))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBulletInput {
    pub id: String,
    pub scope: String,
    pub claim_level: String,
    pub text: String,
    pub seniority: String,
    #[serde(default)]
    pub category: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub tools: Vec<String>,
    #[serde(default)]
    pub approved: bool,
}

pub fn create_bullet_value(
    repo_root: &Path,
    input: CreateBulletInput,
) -> anyhow::Result<MutationResponse> {
    add_bullet(
        repo_root,
        BulletRecord {
            id: input.id,
            scope: input.scope,
            category: input.category,
            tags: input.tags,
            tools: input.tools,
            seniority: input.seniority,
            approved: input.approved,
            claim_level: input.claim_level,
            text: input.text,
        },
    )?;
    Ok(response_ok("bullet created"))
}

pub fn create_skill_value(
    repo_root: &Path,
    name: &str,
    level: &str,
    approved: bool,
) -> anyhow::Result<MutationResponse> {
    add_skill(repo_root, name, level, approved)?;
    Ok(response_ok(format!("skill {name} created")))
}

fn template_path(repo_root: &Path, key: &TemplateKey) -> std::path::PathBuf {
    match key {
        TemplateKey::Resume1pgBase => {
            repo_root.join("templates").join("resume").join("resume_1pg_base.md")
        }
        TemplateKey::Resume2pgBase => {
            repo_root.join("templates").join("resume").join("resume_2pg_base.md")
        }
        TemplateKey::Recruiter => repo_root.join("templates").join("messages").join("recruiter.md"),
        TemplateKey::HiringManager => {
            repo_root.join("templates").join("messages").join("hiring_manager.md")
        }
        TemplateKey::CoverShort => {
            repo_root.join("templates").join("messages").join("cover_short.md")
        }
    }
}

fn normalize_markdown(content: &str) -> String {
    let normalized = content.replace("\r\n", "\n").replace('\r', "\n");
    let trimmed_end = normalized.trim_end();
    format!("{trimmed_end}\n")
}

fn required_resume_anchors() -> [&'static str; 7] {
    [
        "<!--SECTION:HEADLINE-->",
        "<!--SECTION:SUMMARY-->",
        "<!--SECTION:STACK-->",
        "<!--SECTION:CORE_STRENGTHS-->",
        "<!--SECTION:EXPERIENCE-->",
        "<!--SECTION:EDUCATION-->",
        "<!--SECTION:CERTS-->",
    ]
}

fn required_message_placeholders(key: &TemplateKey) -> &'static [&'static str] {
    match key {
        TemplateKey::Recruiter => {
            &["company", "role", "top_match_1", "top_match_2", "proof_metric", "cta"]
        }
        TemplateKey::HiringManager => {
            &["company", "role", "how_i_help", "proof_metric", "problem_1", "problem_2", "cta"]
        }
        TemplateKey::CoverShort => {
            &["company", "role", "top_match_1", "top_match_2", "proof_metric"]
        }
        _ => &[],
    }
}

fn placeholder_set(content: &str) -> BTreeSet<String> {
    let re = Regex::new(r"\{\{\s*([a-zA-Z0-9_]+)\s*\}\}").expect("regex");
    re.captures_iter(content).filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string())).collect()
}

fn validate_template(key: &TemplateKey, content: &str) -> anyhow::Result<()> {
    match key {
        TemplateKey::Resume1pgBase | TemplateKey::Resume2pgBase => {
            let missing = required_resume_anchors()
                .into_iter()
                .filter(|anchor| !content.contains(anchor))
                .collect::<Vec<_>>();
            if !missing.is_empty() {
                anyhow::bail!("template missing required anchors: {}", missing.join(", "));
            }
        }
        _ => {
            let placeholders = placeholder_set(content);
            let missing = required_message_placeholders(key)
                .iter()
                .filter(|name| !placeholders.contains(**name))
                .map(|name| (*name).to_string())
                .collect::<Vec<_>>();
            if !missing.is_empty() {
                anyhow::bail!("template missing required placeholders: {}", missing.join(", "));
            }
        }
    }
    Ok(())
}

pub fn save_template_value(
    repo_root: &Path,
    key: &TemplateKey,
    content: &str,
) -> anyhow::Result<MutationResponse> {
    validate_template(key, content)?;
    let path = template_path(repo_root, key);
    atomic_write_text(&path, &normalize_markdown(content))
        .with_context(|| format!("writing {}", path.display()))?;
    Ok(response_ok("template saved"))
}

pub fn load_templates_preview(repo_root: &Path) -> anyhow::Result<TemplatesPreview> {
    let resume_1pg_path = repo_root.join("templates").join("resume").join("resume_1pg_base.md");
    let resume_2pg_path = repo_root.join("templates").join("resume").join("resume_2pg_base.md");
    let recruiter_path = repo_root.join("templates").join("messages").join("recruiter.md");
    let hiring_path = repo_root.join("templates").join("messages").join("hiring_manager.md");
    let cover_path = repo_root.join("templates").join("messages").join("cover_short.md");

    Ok(TemplatesPreview {
        resume_1pg_base: std::fs::read_to_string(&resume_1pg_path)
            .with_context(|| format!("reading {}", resume_1pg_path.display()))?,
        resume_2pg_base: std::fs::read_to_string(&resume_2pg_path)
            .with_context(|| format!("reading {}", resume_2pg_path.display()))?,
        recruiter_template: std::fs::read_to_string(&recruiter_path)
            .with_context(|| format!("reading {}", recruiter_path.display()))?,
        hiring_manager_template: std::fs::read_to_string(&hiring_path)
            .with_context(|| format!("reading {}", hiring_path.display()))?,
        cover_short_template: std::fs::read_to_string(&cover_path)
            .with_context(|| format!("reading {}", cover_path.display()))?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use walkdir::WalkDir;

    fn repo_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .canonicalize()
            .expect("repo root")
    }

    fn copy_tree(src: &Path, dest: &Path) {
        for entry in WalkDir::new(src) {
            let entry = entry.expect("walk entry");
            let rel = entry.path().strip_prefix(src).expect("relative path");
            let target = dest.join(rel);
            if entry.file_type().is_dir() {
                std::fs::create_dir_all(&target).expect("mkdir");
            } else {
                if let Some(parent) = target.parent() {
                    std::fs::create_dir_all(parent).expect("mkdir parent");
                }
                std::fs::copy(entry.path(), &target).expect("copy file");
            }
        }
    }

    fn prepare_temp_repo() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().expect("temp repo");
        let root = repo_root();
        for folder in ["data", "templates"] {
            copy_tree(&root.join(folder), &tmp.path().join(folder));
        }
        tmp
    }

    #[test]
    fn rejects_unknown_skill_level() {
        let tmp = prepare_temp_repo();
        let err =
            set_skill_level_value(tmp.path(), "Okta", "wizard").expect_err("invalid skill level");
        assert!(err.to_string().contains("unsupported skill level"));
    }

    #[test]
    fn rejects_unknown_bullet_id() {
        let tmp = prepare_temp_repo();
        let err =
            set_bullet_approved_value(tmp.path(), "missing", true).expect_err("unknown bullet");
        assert!(err.to_string().contains("bullet not found"));
    }

    #[test]
    fn rejects_template_missing_required_anchor() {
        let tmp = prepare_temp_repo();
        let err = save_template_value(tmp.path(), &TemplateKey::Resume1pgBase, "# resume")
            .expect_err("missing anchors");
        assert!(err.to_string().contains("required anchors"));
    }

    #[test]
    fn rejects_message_template_missing_required_placeholder() {
        let tmp = prepare_temp_repo();
        let bad = "Hi {{company}}, role {{role}} only.";
        let err = save_template_value(tmp.path(), &TemplateKey::Recruiter, bad)
            .expect_err("missing placeholders");
        assert!(err.to_string().contains("required placeholders"));
    }

    #[test]
    fn rejects_duplicate_skill_create() {
        let tmp = prepare_temp_repo();
        let err = create_skill_value(tmp.path(), "Okta", "operator", true).expect_err("duplicate");
        assert!(err.to_string().contains("already exists"));
    }

    #[test]
    fn creates_new_bullet_with_valid_input() {
        let tmp = prepare_temp_repo();
        let resp = create_bullet_value(
            tmp.path(),
            CreateBulletInput {
                id: "custom_new_001".to_string(),
                scope: "Custom".to_string(),
                claim_level: "supported".to_string(),
                text: "Supported a controlled rollout with documented outcomes.".to_string(),
                seniority: "mid".to_string(),
                category: vec!["ops".to_string()],
                tags: vec!["rollout".to_string()],
                tools: vec!["Jira".to_string()],
                approved: false,
            },
        )
        .expect("create bullet");
        assert!(resp.ok);

        let banks = load_banks(tmp.path()).expect("banks");
        assert!(banks.bullets.iter().any(|b| b.id == "custom_new_001"));
    }
}
