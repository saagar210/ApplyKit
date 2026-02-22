use crate::banks::Banks;
use crate::types::{GenerateResultData, TruthValidationReport};
use regex::Regex;
use std::collections::BTreeSet;

fn normalize_token(input: &str) -> String {
    input
        .to_ascii_lowercase()
        .replace(|ch: char| !ch.is_ascii_alphanumeric() && ch != '+' && ch != '/' && ch != '.', "")
}

fn approved_tool_set(banks: &Banks) -> BTreeSet<String> {
    let mut set = BTreeSet::new();
    for (skill, record) in &banks.skills {
        if !record.approved {
            continue;
        }
        let normalized = normalize_token(skill);
        set.insert(normalized.clone());
        if normalized == "microsoft365" {
            set.insert("m365".to_string());
        }
        if normalized == "googleworkspace" {
            set.insert("gws".to_string());
        }
        for piece in skill.split([' ', '/', '+', '-', '.']) {
            let token = normalize_token(piece);
            if token.len() > 2 {
                set.insert(token);
            }
        }
    }
    for bullet in &banks.bullets {
        if !bullet.approved {
            continue;
        }
        for tool in &bullet.tools {
            let normalized = normalize_token(tool);
            if !normalized.is_empty() {
                set.insert(normalized.clone());
            }
            for piece in tool.split([' ', '/', '+', '-', '.']) {
                let token = normalize_token(piece);
                if token.len() > 2 {
                    set.insert(token);
                }
            }
        }
    }
    set
}

fn technology_lexicon() -> BTreeSet<String> {
    [
        "kubernetes",
        "terraform",
        "ansible",
        "servicenow",
        "splunk",
        "sentinelone",
        "crowdstrike",
        "datadog",
        "newrelic",
        "snowflake",
        "tableau",
        "mongodb",
        "postgres",
        "redis",
        "jenkins",
        "gitlab",
        "github",
        "prometheus",
        "grafana",
        "elk",
        "elasticsearch",
        "logstash",
        "kibana",
        "kafka",
        "airflow",
        "notion",
        "workday",
        "azure",
        "aws",
        "gcp",
        "okta",
        "jamf",
        "intune",
        "kandji",
        "duo",
        "m365",
        "googleworkspace",
        "jira",
        "confluence",
        "zendesk",
        "salesforce",
        "cloudflare",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect()
}

fn scan_unknown_tools(text: &str, banks: &Banks) -> Vec<String> {
    let known = approved_tool_set(banks);
    let lexicon = technology_lexicon();
    let token_re = Regex::new(r"[A-Za-z][A-Za-z0-9+./-]{2,}").expect("regex");

    let mut unknown = BTreeSet::new();
    for cap in token_re.captures_iter(text) {
        if let Some(m) = cap.get(0) {
            let token = normalize_token(m.as_str());
            if token.is_empty() {
                continue;
            }
            if lexicon.contains(&token) && !known.contains(&token) {
                unknown.insert(token);
            }
        }
    }

    unknown.into_iter().collect()
}

fn claim_level_constraints(resume: &str) -> Vec<String> {
    let mut issues = Vec::new();
    let lower = resume.to_ascii_lowercase();

    let banned_phrases = [
        "single-handedly",
        "invented",
        "authored patents",
        "world-class",
        "industry-leading",
        "best-in-class",
        "guaranteed",
        "100% uptime forever",
    ];
    for phrase in banned_phrases {
        if lower.contains(phrase) {
            issues.push(format!("disallowed_claim_phrase:{phrase}"));
        }
    }

    let escalation_verbs = [
        "spearheaded",
        "orchestrated",
        "transformed",
        "revolutionized",
        "owned entire",
        "ran global program",
    ];
    for verb in escalation_verbs {
        if lower.contains(verb) {
            issues.push(format!("claim_escalation_verb:{verb}"));
        }
    }

    let disallowed_titles =
        ["director", "vp", "vice president", "staff engineer", "principal engineer"];
    for title in disallowed_titles {
        if lower.contains(title) {
            issues.push(format!("disallowed_title_injection:{title}"));
        }
    }

    issues
}

fn provenance_checks(
    data: &GenerateResultData,
    banks: &Banks,
    provenance_ids: &[String],
) -> Vec<String> {
    let mut issues = Vec::new();

    if provenance_ids.is_empty() {
        issues.push("missing_bullet_provenance".to_string());
        return issues;
    }

    let ids = banks
        .bullets
        .iter()
        .map(|b| (b.id.as_str(), b.approved))
        .collect::<std::collections::BTreeMap<_, _>>();
    for id in provenance_ids {
        match ids.get(id.as_str()) {
            None => issues.push(format!("provenance_unknown_id:{id}")),
            Some(approved) if !approved => issues.push(format!("provenance_unapproved_id:{id}")),
            _ => {}
        }
    }

    let edit_provenance_missing = data
        .tailor_plan
        .edits
        .iter()
        .filter(|e| e.kind == "bullet_swap")
        .any(|e| e.provenance_ids.is_empty());
    if edit_provenance_missing {
        issues.push("tailor_edit_missing_provenance".to_string());
    }

    issues
}

pub fn validate(
    data: &GenerateResultData,
    banks: &Banks,
    provenance_ids: &[String],
) -> TruthValidationReport {
    let mut violations = Vec::new();

    let combined = format!(
        "{}\n{}\n{}\n{}\n{}",
        data.resume_1pg,
        data.resume_2pg.clone().unwrap_or_default(),
        data.recruiter_message,
        data.hiring_manager_message,
        data.cover_short_message
    );

    let unknown_tools = scan_unknown_tools(&combined, banks);
    if !unknown_tools.is_empty() {
        violations.push("unknown_tools_detected".to_string());
    }

    let claim_issues = claim_level_constraints(&combined);
    if !claim_issues.is_empty() {
        violations.push("claim_level_constraint_failed".to_string());
    }

    let provenance_issues = provenance_checks(data, banks, provenance_ids);
    if !provenance_issues.is_empty() {
        violations.push("provenance_validation_failed".to_string());
    }

    let provenance_complete = provenance_issues.is_empty();

    let mut detailed_claim_issues = claim_issues;
    detailed_claim_issues.extend(provenance_issues);

    TruthValidationReport {
        passed: violations.is_empty(),
        violations,
        unknown_tools,
        claim_issues: detailed_claim_issues,
        provenance_complete,
    }
}
