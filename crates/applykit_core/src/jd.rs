use crate::banks::Banks;
use crate::determinism::sorted_unique;
use crate::types::ExtractedJd;
use anyhow::bail;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

fn normalize_whitespace(input: &str) -> String {
    let nfc: String = unicode_normalization::UnicodeNormalization::nfc(input).collect();
    let normalized_newlines = nfc.replace("\r\n", "\n").replace('\r', "\n");
    let mut out = String::new();
    let mut blank_streak = 0;
    for raw_line in normalized_newlines.lines() {
        let line = raw_line.trim_end();
        if line.is_empty() {
            blank_streak += 1;
            if blank_streak <= 1 {
                out.push('\n');
            }
            continue;
        }
        blank_streak = 0;
        out.push_str(line);
        out.push('\n');
    }
    out.trim().to_string()
}

pub fn normalize_jd(input: &str) -> String {
    normalize_whitespace(input)
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct LlmJdSummary {
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub requirements: Vec<String>,
    #[serde(default)]
    pub tools: Vec<String>,
    #[serde(default)]
    pub scale_signals: Vec<String>,
    #[serde(default)]
    pub rigor_signals: Vec<String>,
}

fn extract_json_object(raw: &str) -> &str {
    let trimmed = raw.trim();
    if let (Some(start), Some(end)) = (trimmed.find('{'), trimmed.rfind('}')) {
        if start < end {
            return &trimmed[start..=end];
        }
    }
    trimmed
}

pub fn parse_llm_jd_summary(raw: &str) -> anyhow::Result<LlmJdSummary> {
    let candidate = extract_json_object(raw);
    let parsed: LlmJdSummary =
        serde_json::from_str(candidate).map_err(|e| anyhow::anyhow!("summary parse error: {e}"))?;
    Ok(parsed)
}

fn normalize_line(input: &str) -> Option<String> {
    let collapsed = input.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.is_empty() {
        None
    } else {
        Some(collapsed)
    }
}

fn keyword_tokens(entries: &[String]) -> Vec<String> {
    let token_re = Regex::new(r"[a-zA-Z0-9+.#/-]{2,}").expect("regex");
    let mut out = Vec::new();
    for entry in entries {
        let lower = entry.to_ascii_lowercase();
        for cap in token_re.captures_iter(&lower) {
            if let Some(m) = cap.get(0) {
                out.push(m.as_str().to_string());
            }
        }
    }
    sorted_unique(out)
}

fn sanitize_summary_list(
    values: &[String],
    jd_lower: &str,
    max: usize,
) -> anyhow::Result<Vec<String>> {
    let mut filtered = Vec::new();
    let mut unknown = Vec::new();
    for value in values {
        if let Some(cleaned) = normalize_line(value) {
            if jd_lower.contains(&cleaned.to_ascii_lowercase()) {
                filtered.push(cleaned);
            } else {
                unknown.push(cleaned);
            }
        }
    }
    if !unknown.is_empty() {
        bail!("summary content not present in JD: {}", unknown.join(", "));
    }
    let mut deduped = sorted_unique(filtered);
    deduped.truncate(max);
    Ok(deduped)
}

fn canonical_tool_map(banks: &Banks) -> BTreeMap<String, String> {
    banks.skills.keys().map(|name| (name.to_ascii_lowercase(), name.clone())).collect()
}

pub fn merge_extracted_with_summary(
    base: &ExtractedJd,
    summary: &LlmJdSummary,
    banks: &Banks,
) -> anyhow::Result<ExtractedJd> {
    const KEYWORDS_MAX: usize = 30;
    const REQUIREMENTS_MAX: usize = 30;
    const TOOLS_MAX: usize = 30;
    const SCALE_MAX: usize = 20;
    const RIGOR_MAX: usize = 20;

    let jd_lower = base.normalized_text.to_ascii_lowercase();
    let summary_keywords = keyword_tokens(&summary.keywords);
    if summary_keywords.iter().any(|token| !jd_lower.contains(token)) {
        bail!("summary keywords not present in JD");
    }

    let mut merged_keywords = base.keywords.clone();
    merged_keywords.extend(summary_keywords);
    merged_keywords = sorted_unique(merged_keywords);
    merged_keywords.truncate(KEYWORDS_MAX);

    let mut merged_requirements = base.requirements.clone();
    merged_requirements.extend(sanitize_summary_list(
        &summary.requirements,
        &jd_lower,
        REQUIREMENTS_MAX,
    )?);
    merged_requirements = sorted_unique(merged_requirements);
    merged_requirements.truncate(REQUIREMENTS_MAX);

    let known_tools = canonical_tool_map(banks);
    let mut tool_unknowns = Vec::new();
    let mut summary_tools = Vec::new();
    for raw_tool in &summary.tools {
        if let Some(cleaned) = normalize_line(raw_tool) {
            let key = cleaned.to_ascii_lowercase();
            let Some(canonical) = known_tools.get(&key).cloned() else {
                tool_unknowns.push(cleaned);
                continue;
            };
            if !jd_lower.contains(&key) {
                tool_unknowns.push(canonical);
                continue;
            }
            summary_tools.push(canonical);
        }
    }
    if !tool_unknowns.is_empty() {
        bail!("summary tools not allowed or not present in JD: {}", tool_unknowns.join(", "));
    }
    let mut merged_tools = base.tools.clone();
    merged_tools.extend(summary_tools);
    merged_tools = sorted_unique(merged_tools);
    merged_tools.truncate(TOOLS_MAX);

    let mut merged_scale = base.scale_signals.clone();
    merged_scale.extend(sanitize_summary_list(&summary.scale_signals, &jd_lower, SCALE_MAX)?);
    merged_scale = sorted_unique(merged_scale);
    merged_scale.truncate(SCALE_MAX);

    let mut merged_rigor = base.rigor_signals.clone();
    merged_rigor.extend(sanitize_summary_list(&summary.rigor_signals, &jd_lower, RIGOR_MAX)?);
    merged_rigor = sorted_unique(merged_rigor);
    merged_rigor.truncate(RIGOR_MAX);

    Ok(ExtractedJd {
        normalized_text: base.normalized_text.clone(),
        keywords: merged_keywords,
        tools: merged_tools,
        requirements: merged_requirements,
        scale_signals: merged_scale,
        rigor_signals: merged_rigor,
    })
}

pub fn extract_structured(jd_raw: &str, banks: &Banks) -> ExtractedJd {
    let normalized = normalize_jd(jd_raw);
    let lower = normalized.to_ascii_lowercase();

    let token_re = Regex::new(r"[a-zA-Z0-9+.#/-]{2,}").expect("regex");
    let mut terms = Vec::new();
    for cap in token_re.captures_iter(&lower) {
        if let Some(m) = cap.get(0) {
            terms.push(m.as_str().to_string());
        }
    }

    let stop_words: BTreeSet<&'static str> = [
        "and",
        "the",
        "with",
        "for",
        "you",
        "our",
        "are",
        "will",
        "this",
        "that",
        "from",
        "into",
        "have",
        "has",
        "your",
        "role",
        "team",
        "years",
        "plus",
        "req",
        "required",
        "responsibilities",
        "requirements",
    ]
    .into_iter()
    .collect();

    let mut keyword_freq = std::collections::BTreeMap::<String, usize>::new();
    for term in terms {
        if !stop_words.contains(term.as_str()) {
            *keyword_freq.entry(term).or_insert(0) += 1;
        }
    }

    let mut keywords: Vec<(String, usize)> = keyword_freq.into_iter().collect();
    keywords.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let keywords = keywords.into_iter().take(30).map(|(k, _)| k).collect::<Vec<_>>();

    let mut tools = Vec::new();
    for skill_name in banks.skills.keys() {
        if lower.contains(&skill_name.to_ascii_lowercase()) {
            tools.push(skill_name.clone());
        }
    }

    let mut requirements = Vec::new();
    for line in normalized.lines() {
        let trimmed = line.trim_start_matches('-').trim();
        let lc = trimmed.to_ascii_lowercase();
        if lc.starts_with("require")
            || lc.contains("experience")
            || lc.contains("familiar")
            || lc.contains("add skills")
        {
            requirements.push(trimmed.to_string());
        }
    }

    let mut scale_signals = Vec::new();
    let mut rigor_signals = Vec::new();
    for line in normalized.lines() {
        let lc = line.to_ascii_lowercase();
        if lc.contains("scale")
            || lc.contains("global")
            || lc.contains("3,000")
            || lc.contains("high volume")
        {
            scale_signals.push(line.trim().to_string());
        }
        if lc.contains("incident")
            || lc.contains("change")
            || lc.contains("audit")
            || lc.contains("controls")
        {
            rigor_signals.push(line.trim().to_string());
        }
    }

    ExtractedJd {
        normalized_text: normalized,
        keywords,
        tools: sorted_unique(tools),
        requirements: sorted_unique(requirements),
        scale_signals: sorted_unique(scale_signals),
        rigor_signals: sorted_unique(rigor_signals),
    }
}
