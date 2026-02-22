use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Baseline {
    OnePage,
    TwoPage,
}

impl Baseline {
    pub fn as_cli_value(&self) -> &'static str {
        match self {
            Self::OnePage => "1pg",
            Self::TwoPage => "2pg",
        }
    }
}

impl std::str::FromStr for Baseline {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "1pg" | "one" | "onepage" | "one_page" => Ok(Self::OnePage),
            "2pg" | "two" | "twopage" | "two_page" => Ok(Self::TwoPage),
            _ => Err(format!("unsupported baseline: {s}")),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Track {
    SupportOpsCore,
    IdentityEndpoint,
    SecurityComplianceOps,
    AutomationAiOps,
    Managerish,
}

impl Track {
    pub fn precedence(&self) -> usize {
        match self {
            Self::SupportOpsCore => 0,
            Self::IdentityEndpoint => 1,
            Self::SecurityComplianceOps => 2,
            Self::AutomationAiOps => 3,
            Self::Managerish => 4,
        }
    }
}

impl Display for Track {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::SupportOpsCore => "Support/Ops Core",
            Self::IdentityEndpoint => "Identity & Endpoint",
            Self::SecurityComplianceOps => "Security & Compliance Ops",
            Self::AutomationAiOps => "Automation / AIOps-adjacent",
            Self::Managerish => "Manager-ish",
        };
        write!(f, "{s}")
    }
}

impl std::str::FromStr for Track {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "support" | "support_ops" | "support/ops core" | "support_ops_core" => {
                Ok(Self::SupportOpsCore)
            }
            "identity" | "identity_endpoint" | "identity & endpoint" => Ok(Self::IdentityEndpoint),
            "security" | "security_compliance_ops" | "security & compliance ops" => {
                Ok(Self::SecurityComplianceOps)
            }
            "automation" | "automation_aiops" | "automation / aiops-adjacent" => {
                Ok(Self::AutomationAiOps)
            }
            "manager" | "managerish" | "manager-ish" => Ok(Self::Managerish),
            _ => Err(format!("unsupported track override: {s}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ClaimLevel {
    Owned,
    Led,
    Partnered,
    Supported,
}

impl std::str::FromStr for ClaimLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "owned" => Ok(Self::Owned),
            "led" => Ok(Self::Led),
            "partnered" => Ok(Self::Partnered),
            "supported" => Ok(Self::Supported),
            _ => Err(format!("unsupported claim level: {s}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SkillLevel {
    Admin,
    Operator,
    Familiar,
}

impl SkillLevel {
    pub fn parse_input(level: &str) -> Result<Self, String> {
        match level.trim().to_ascii_lowercase().as_str() {
            "admin" | "strong" => Ok(Self::Admin),
            "operator" => Ok(Self::Operator),
            "familiar" => Ok(Self::Familiar),
            _ => Err(format!("unsupported skill level: {level}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateInput {
    pub company: String,
    pub role: String,
    pub source: String,
    pub baseline: Baseline,
    pub jd_text: String,
    pub outdir: Option<PathBuf>,
    pub run_date: Option<NaiveDate>,
    pub track_override: Option<Track>,
    pub allow_unapproved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedJd {
    pub normalized_text: String,
    pub keywords: Vec<String>,
    pub tools: Vec<String>,
    pub requirements: Vec<String>,
    pub scale_signals: Vec<String>,
    pub rigor_signals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ExtractionSource {
    #[default]
    Deterministic,
    LlmMerged,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExtractionDiagnostics {
    #[serde(default)]
    pub summarize_attempted: bool,
    #[serde(default)]
    pub summarize_merged: bool,
    #[serde(default)]
    pub summarize_fallback_reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackSelection {
    pub selected: Track,
    pub scores: Vec<TrackScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackScore {
    pub track: Track,
    pub score: i32,
    pub matched_terms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitScore {
    pub role_match: u8,
    pub stack_match: u8,
    pub scale_match: u8,
    pub rigor_match: u8,
    pub signal_boost: u8,
    pub total: u8,
    pub why_match: Vec<String>,
    pub gaps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TailorEdit {
    pub kind: String,
    pub target_section: String,
    pub reason: String,
    pub provenance_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TailorPlan {
    pub edits: Vec<TailorEdit>,
    pub max_resume_edits: usize,
    pub max_bullet_swaps: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedMessages {
    pub recruiter: String,
    pub hiring_manager: String,
    pub cover_short: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulletCandidate {
    pub id: String,
    pub text: String,
    pub tags: Vec<String>,
    pub track_hint: String,
    pub reason: String,
    pub approved: bool,
    pub score: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackerRow {
    pub date: String,
    pub company: String,
    pub role: String,
    pub source: String,
    pub track: String,
    pub fit_total: u8,
    pub status: String,
    pub next_action: String,
    pub packet_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruthValidationReport {
    pub passed: bool,
    pub violations: Vec<String>,
    pub unknown_tools: Vec<String>,
    pub claim_issues: Vec<String>,
    pub provenance_complete: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateResultData {
    pub extracted: ExtractedJd,
    #[serde(default)]
    pub extraction_source: ExtractionSource,
    #[serde(default)]
    pub extraction_diagnostics: ExtractionDiagnostics,
    pub track: TrackSelection,
    pub fit: FitScore,
    pub tailor_plan: TailorPlan,
    pub bullet_candidates: Vec<BulletCandidate>,
    pub resume_1pg: String,
    pub resume_2pg: Option<String>,
    pub recruiter_message: String,
    pub hiring_manager_message: String,
    pub cover_short_message: String,
    pub diff_md: String,
    pub tracker_row: TrackerRow,
    pub truth_report: TruthValidationReport,
    pub packet_dir: PathBuf,
    pub files_written: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketDetail {
    pub packet_dir: PathBuf,
    pub extracted: ExtractedJd,
    #[serde(default)]
    pub extraction_source: ExtractionSource,
    #[serde(default)]
    pub extraction_diagnostics: ExtractionDiagnostics,
    pub fit: FitScore,
    pub track: TrackSelection,
    pub tailor_plan: TailorPlan,
    pub bullet_candidates: Vec<BulletCandidate>,
    pub messages: GeneratedMessages,
    pub resume_1pg: String,
    pub resume_2pg: Option<String>,
    pub diff_md: String,
    pub tracker_row: TrackerRow,
    pub truth_report: TruthValidationReport,
}
