use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplykitConfig {
    pub output: OutputConfig,
    pub determinism: DeterminismConfig,
    pub scoring: ScoringConfig,
    pub tracks: TrackTermsConfig,
    pub llm: LlmConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub base_dir: String,
    pub date_format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeterminismConfig {
    pub sort_tools: String,
    pub sort_bullets: String,
    pub max_resume_edits: usize,
    pub max_bullet_swaps: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringConfig {
    pub role_match: u8,
    pub stack_match: u8,
    pub scale_match: u8,
    pub rigor_match: u8,
    pub signal_boost: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackTermsConfig {
    pub support_ops: Vec<String>,
    pub identity_endpoint: Vec<String>,
    pub security_compliance_ops: Vec<String>,
    pub automation_aiops: Vec<String>,
    pub managerish: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub enabled: bool,
    pub provider: String,
    pub base_url: String,
    pub model: String,
    pub allowed_tasks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuntimeSettings {
    pub allow_unapproved: bool,
    pub llm_enabled: Option<bool>,
    pub llm_provider: Option<String>,
    pub llm_base_url: Option<String>,
    pub llm_model: Option<String>,
    pub llm_allowed_tasks: Option<Vec<String>>,
}

pub fn load_config(repo_root: &Path) -> anyhow::Result<ApplykitConfig> {
    let path = repo_root.join("config").join("applykit.toml");
    let raw =
        std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let config: ApplykitConfig = toml::from_str(&raw).context("parsing applykit.toml")?;
    Ok(config)
}

pub fn runtime_settings_path(repo_root: &Path) -> PathBuf {
    repo_root.join("config").join("applykit.user.toml")
}

pub fn load_runtime_settings(repo_root: &Path) -> anyhow::Result<RuntimeSettings> {
    let path = runtime_settings_path(repo_root);
    if !path.exists() {
        return Ok(RuntimeSettings::default());
    }
    let raw =
        std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let settings: RuntimeSettings =
        toml::from_str(&raw).with_context(|| format!("parsing {}", path.display()))?;
    Ok(settings)
}

pub fn save_runtime_settings(repo_root: &Path, settings: &RuntimeSettings) -> anyhow::Result<()> {
    let path = runtime_settings_path(repo_root);
    let raw = toml::to_string_pretty(settings).context("serializing runtime settings")?;
    atomic_write_text(&path, &(raw + "\n"))
}

pub fn atomic_write_text(path: &Path, raw: &str) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let tmp_path = path.with_extension("tmp");
    {
        use std::io::Write;
        let mut file = std::fs::File::create(&tmp_path)
            .with_context(|| format!("creating {}", tmp_path.display()))?;
        file.write_all(raw.as_bytes())
            .with_context(|| format!("writing {}", tmp_path.display()))?;
        file.sync_all().with_context(|| format!("syncing {}", tmp_path.display()))?;
    }
    std::fs::rename(&tmp_path, path)
        .with_context(|| format!("renaming {} -> {}", tmp_path.display(), path.display()))?;
    #[cfg(unix)]
    if let Some(parent) = path.parent() {
        let dir =
            std::fs::File::open(parent).with_context(|| format!("opening {}", parent.display()))?;
        dir.sync_all().with_context(|| format!("syncing {}", parent.display()))?;
    }
    Ok(())
}

pub fn merge_config_with_runtime(
    mut base: ApplykitConfig,
    runtime: &RuntimeSettings,
) -> ApplykitConfig {
    if let Some(enabled) = runtime.llm_enabled {
        base.llm.enabled = enabled;
    }
    if let Some(provider) = &runtime.llm_provider {
        base.llm.provider = provider.clone();
    }
    if let Some(base_url) = &runtime.llm_base_url {
        base.llm.base_url = base_url.clone();
    }
    if let Some(model) = &runtime.llm_model {
        base.llm.model = model.clone();
    }
    if let Some(tasks) = &runtime.llm_allowed_tasks {
        let mut normalized = tasks
            .iter()
            .map(|s| s.trim().to_ascii_lowercase())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        normalized.sort();
        normalized.dedup();
        if !normalized.is_empty() {
            base.llm.allowed_tasks = normalized;
        }
    }
    base
}

pub fn resolve_output_base(base_dir: &str) -> PathBuf {
    if let Some(stripped) = base_dir.strip_prefix("~/") {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home).join(stripped);
        }
    }
    PathBuf::from(base_dir)
}

pub fn scoring_total_weights(scoring: &ScoringConfig) -> BTreeMap<&'static str, u8> {
    let mut map = BTreeMap::new();
    map.insert("role_match", scoring.role_match);
    map.insert("stack_match", scoring.stack_match);
    map.insert("scale_match", scoring.scale_match);
    map.insert("rigor_match", scoring.rigor_match);
    map.insert("signal_boost", scoring.signal_boost);
    map
}
