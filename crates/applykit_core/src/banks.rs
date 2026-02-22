use crate::config::atomic_write_text;
use crate::determinism::sorted_unique;
use crate::types::{ClaimLevel, SkillLevel};
use anyhow::{bail, Context};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulletBankFile {
    pub version: String,
    pub generated_at: Option<String>,
    pub bullets: Vec<BulletRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulletRecord {
    pub id: String,
    pub scope: String,
    pub category: Vec<String>,
    pub tags: Vec<String>,
    pub tools: Vec<String>,
    pub seniority: String,
    pub approved: bool,
    pub claim_level: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillsBankFile {
    pub version: String,
    pub generated_at: Option<String>,
    pub skills: BTreeMap<String, SkillRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillRecord {
    pub level: String,
    pub approved: bool,
}

#[derive(Debug, Clone)]
pub struct Banks {
    pub bullets: Vec<Bullet>,
    pub skills: BTreeMap<String, Skill>,
}

#[derive(Debug, Clone)]
pub struct Bullet {
    pub id: String,
    pub scope: String,
    pub category: Vec<String>,
    pub tags: Vec<String>,
    pub tools: Vec<String>,
    pub seniority: String,
    pub approved: bool,
    pub claim_level: ClaimLevel,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct Skill {
    pub level: SkillLevel,
    pub approved: bool,
}

fn bullet_bank_path(repo_root: &Path) -> PathBuf {
    repo_root.join("data").join("bullet_bank.json")
}

fn skills_bank_path(repo_root: &Path) -> PathBuf {
    repo_root.join("data").join("skills_bank.json")
}

pub fn load_bullet_bank_file(repo_root: &Path) -> anyhow::Result<BulletBankFile> {
    let bullet_path = bullet_bank_path(repo_root);
    let bullet_raw = std::fs::read_to_string(&bullet_path)
        .with_context(|| format!("reading {}", bullet_path.display()))?;
    let mut bullet_file: BulletBankFile =
        serde_json::from_str(&bullet_raw).context("parsing bullet bank")?;
    bullet_file.bullets.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(bullet_file)
}

pub fn load_skills_bank_file(repo_root: &Path) -> anyhow::Result<SkillsBankFile> {
    let skills_path = skills_bank_path(repo_root);
    let skills_raw = std::fs::read_to_string(&skills_path)
        .with_context(|| format!("reading {}", skills_path.display()))?;
    let skill_file: SkillsBankFile =
        serde_json::from_str(&skills_raw).context("parsing skills bank")?;
    Ok(skill_file)
}

fn claim_level_valid(level: &str) -> bool {
    level.parse::<ClaimLevel>().is_ok()
}

fn normalize_bullet_id(id: &str) -> anyhow::Result<String> {
    let normalized = id.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        bail!("bullet id cannot be empty");
    }
    let pattern = Regex::new(r"^[a-z0-9_]+$").expect("regex");
    if !pattern.is_match(&normalized) {
        bail!("bullet id must match ^[a-z0-9_]+$");
    }
    Ok(normalized)
}

fn normalize_text(input: &str) -> anyhow::Result<String> {
    let out = input.split_whitespace().collect::<Vec<_>>().join(" ");
    if out.is_empty() {
        bail!("text cannot be empty");
    }
    Ok(out)
}

fn normalize_list(values: Vec<String>, lowercase: bool) -> Vec<String> {
    let items = values
        .into_iter()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .map(|value| if lowercase { value.to_ascii_lowercase() } else { value })
        .collect::<Vec<_>>();
    sorted_unique(items)
}

fn canonical_skill_level(level: SkillLevel) -> &'static str {
    match level {
        SkillLevel::Admin => "admin",
        SkillLevel::Operator => "operator",
        SkillLevel::Familiar => "familiar",
    }
}

pub fn save_bullet_bank_file(repo_root: &Path, mut file: BulletBankFile) -> anyhow::Result<()> {
    let mut seen = BTreeMap::new();
    for row in &file.bullets {
        if !claim_level_valid(&row.claim_level) {
            bail!("unsupported claim level in bullet {}: {}", row.id, row.claim_level);
        }
        if seen.insert(row.id.clone(), true).is_some() {
            bail!("duplicate bullet id: {}", row.id);
        }
    }
    file.bullets.sort_by(|a, b| a.id.cmp(&b.id));
    let raw = serde_json::to_string_pretty(&file).context("serializing bullet bank")?;
    atomic_write_text(&bullet_bank_path(repo_root), &(raw + "\n"))
}

pub fn save_skills_bank_file(repo_root: &Path, mut file: SkillsBankFile) -> anyhow::Result<()> {
    for row in file.skills.values_mut() {
        let parsed = SkillLevel::parse_input(&row.level).map_err(anyhow::Error::msg)?;
        row.level = canonical_skill_level(parsed).to_string();
    }
    let raw = serde_json::to_string_pretty(&file).context("serializing skills bank")?;
    atomic_write_text(&skills_bank_path(repo_root), &(raw + "\n"))
}

pub fn set_bullet_approved(repo_root: &Path, id: &str, approved: bool) -> anyhow::Result<()> {
    let mut file = load_bullet_bank_file(repo_root)?;
    let Some(row) = file.bullets.iter_mut().find(|row| row.id == id) else {
        bail!("bullet not found: {id}");
    };
    row.approved = approved;
    save_bullet_bank_file(repo_root, file)
}

pub fn save_bullet_text(repo_root: &Path, id: &str, text: &str) -> anyhow::Result<()> {
    let mut file = load_bullet_bank_file(repo_root)?;
    let Some(row) = file.bullets.iter_mut().find(|row| row.id == id) else {
        bail!("bullet not found: {id}");
    };
    let clean = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if clean.is_empty() {
        bail!("bullet text cannot be empty");
    }
    row.text = clean;
    save_bullet_bank_file(repo_root, file)
}

pub fn add_bullet(repo_root: &Path, mut bullet: BulletRecord) -> anyhow::Result<()> {
    let mut file = load_bullet_bank_file(repo_root)?;
    bullet.id = normalize_bullet_id(&bullet.id)?;
    if file.bullets.iter().any(|row| row.id == bullet.id) {
        bail!("duplicate bullet id: {}", bullet.id);
    }
    bullet.scope = normalize_text(&bullet.scope)?;
    bullet.seniority = normalize_text(&bullet.seniority)?.to_ascii_lowercase();
    bullet.claim_level = bullet.claim_level.trim().to_ascii_lowercase();
    if !claim_level_valid(&bullet.claim_level) {
        bail!("unsupported claim level: {}", bullet.claim_level);
    }
    bullet.text = normalize_text(&bullet.text)?;
    bullet.category = normalize_list(bullet.category, true);
    bullet.tags = normalize_list(bullet.tags, true);
    bullet.tools = normalize_list(bullet.tools, false);
    file.bullets.push(bullet);
    save_bullet_bank_file(repo_root, file)
}

pub fn set_skill_approved(repo_root: &Path, name: &str, approved: bool) -> anyhow::Result<()> {
    let mut file = load_skills_bank_file(repo_root)?;
    let Some(row) = file.skills.get_mut(name) else {
        bail!("skill not found: {name}");
    };
    row.approved = approved;
    save_skills_bank_file(repo_root, file)
}

pub fn set_skill_level(repo_root: &Path, name: &str, level: &str) -> anyhow::Result<()> {
    let mut file = load_skills_bank_file(repo_root)?;
    let Some(row) = file.skills.get_mut(name) else {
        bail!("skill not found: {name}");
    };
    let parsed = SkillLevel::parse_input(level).map_err(anyhow::Error::msg)?;
    row.level = canonical_skill_level(parsed).to_string();
    save_skills_bank_file(repo_root, file)
}

pub fn add_skill(repo_root: &Path, name: &str, level: &str, approved: bool) -> anyhow::Result<()> {
    let mut file = load_skills_bank_file(repo_root)?;
    let normalized_name = name.trim();
    if normalized_name.is_empty() {
        bail!("skill name cannot be empty");
    }
    if file.skills.keys().any(|existing| existing.eq_ignore_ascii_case(normalized_name)) {
        bail!("skill already exists: {normalized_name}");
    }
    let parsed = SkillLevel::parse_input(level).map_err(anyhow::Error::msg)?;
    file.skills.insert(
        normalized_name.to_string(),
        SkillRecord { level: canonical_skill_level(parsed).to_string(), approved },
    );
    save_skills_bank_file(repo_root, file)
}

pub fn load_banks(repo_root: &Path) -> anyhow::Result<Banks> {
    let bullet_file = load_bullet_bank_file(repo_root)?;
    let skill_file = load_skills_bank_file(repo_root)?;

    let mut bullets = Vec::with_capacity(bullet_file.bullets.len());
    for row in bullet_file.bullets {
        let claim_level = row.claim_level.parse::<ClaimLevel>().map_err(anyhow::Error::msg)?;
        bullets.push(Bullet {
            id: row.id,
            scope: row.scope,
            category: row.category,
            tags: row.tags,
            tools: row.tools,
            seniority: row.seniority,
            approved: row.approved,
            claim_level,
            text: row.text,
        });
    }

    let mut skills = BTreeMap::new();
    for (name, value) in skill_file.skills {
        let level = SkillLevel::parse_input(&value.level).map_err(anyhow::Error::msg)?;
        skills.insert(name, Skill { level, approved: value.approved });
    }

    if skills.is_empty() {
        bail!("skills bank is empty");
    }

    Ok(Banks { bullets, skills })
}
