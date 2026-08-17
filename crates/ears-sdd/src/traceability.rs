//! Requirement-to-verification mapping.
//!
//! One behaviour is deliberately different from the Python implementation. It normalized a selector
//! with `lstrip("./")`, which strips *characters* rather than a prefix: `../tests/x.py` became
//! `tests/x.py`, so a traversal selector passed the test-root check and was then existence-checked
//! at the wrong path. A selector that escapes the project is now rejected outright.

use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

use crate::config::Config;
use crate::report::{relative, Finding};
use crate::requirements::Requirement;

#[derive(Debug, Deserialize)]
struct TraceabilityFile {
    #[serde(default)]
    requirements: BTreeMap<String, Entry>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    verification: String,
    #[serde(default)]
    tests: Vec<String>,
    #[serde(default)]
    rationale: String,
}

/// Strip the selector suffix that names a test inside a file: `::name` for pytest-shaped selectors,
/// `#name` for the anchor style the reference project uses.
fn selector_path(selector: &str) -> &str {
    let without_test = selector.split("::").next().unwrap_or(selector);
    without_test.split('#').next().unwrap_or(without_test)
}

fn normalize(path: &str) -> String {
    let slashed = path.replace('\\', "/");
    slashed
        .strip_prefix("./")
        .unwrap_or(&slashed)
        .trim_start_matches('/')
        .to_string()
}

pub fn validate(
    root: &Path,
    spec_path: &Path,
    feature: &str,
    requirements: &[Requirement],
    config: &Config,
) -> Vec<Finding> {
    let path = spec_path
        .parent()
        .map(|parent| parent.join(&config.traceability_file))
        .unwrap_or_else(|| root.join(&config.traceability_file));
    let display = relative(&path, root);

    let finding = |code: &str, message: String, requirement: Option<&str>| {
        let mut item = Finding::new(code, message, display.clone()).feature(feature);
        if let Some(identifier) = requirement {
            item = item.requirement(identifier);
        }
        item
    };

    if !path.is_file() {
        return vec![finding(
            "TRACE_MISSING",
            "Traceability file not found.".to_string(),
            None,
        )];
    }
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) => return vec![finding("TRACE_INVALID", error.to_string(), None)],
    };
    let parsed: TraceabilityFile = match toml::from_str(&text) {
        Ok(parsed) => parsed,
        Err(error) => return vec![finding("TRACE_INVALID", error.message().to_string(), None)],
    };

    let mut findings = Vec::new();
    let declared: Vec<&str> = requirements
        .iter()
        .map(|requirement| requirement.identifier.as_str())
        .collect();

    for identifier in &declared {
        if !parsed.requirements.contains_key(*identifier) {
            findings.push(finding(
                "TRACE_MISSING_REQ",
                "Requirement has no verification mapping.".to_string(),
                Some(identifier),
            ));
        }
    }
    for identifier in parsed.requirements.keys() {
        if !declared.contains(&identifier.as_str()) {
            findings.push(finding(
                "TRACE_UNKNOWN_REQ",
                "Mapping refers to an unknown requirement.".to_string(),
                Some(identifier),
            ));
        }
    }

    let test_roots = config.test_root_prefixes();
    for identifier in &declared {
        let Some(entry) = parsed.requirements.get(*identifier) else {
            continue;
        };
        match entry.verification.as_str() {
            "automated" => {
                if entry.tests.is_empty() || entry.tests.iter().any(|test| test.trim().is_empty()) {
                    findings.push(finding(
                        "TRACE_TESTS",
                        "Automated verification requires a non-empty `tests` list.".to_string(),
                        Some(identifier),
                    ));
                    continue;
                }
                for selector in &entry.tests {
                    let candidate = normalize(selector_path(selector));
                    if candidate.split('/').any(|segment| segment == "..") {
                        findings.push(finding(
                            "TRACE_TEST_ROOT",
                            format!("Test selector escapes the project: {selector}"),
                            Some(identifier),
                        ));
                        continue;
                    }
                    if !test_roots.is_empty()
                        && !test_roots
                            .iter()
                            .any(|prefix| candidate.starts_with(prefix))
                    {
                        findings.push(finding(
                            "TRACE_TEST_ROOT",
                            format!("Test selector is outside configured test roots: {selector}"),
                            Some(identifier),
                        ));
                    }
                    if config.require_test_files && !root.join(&candidate).is_file() {
                        findings.push(finding(
                            "TRACE_TEST_FILE",
                            format!("Referenced test file does not exist: {candidate}"),
                            Some(identifier),
                        ));
                    }
                }
            }
            "manual" => {
                if entry.rationale.trim().chars().count() < 20 {
                    findings.push(finding(
                        "TRACE_MANUAL",
                        "Manual verification requires a concrete rationale of at least 20 characters."
                            .to_string(),
                        Some(identifier),
                    ));
                }
            }
            _ => findings.push(finding(
                "TRACE_MODE",
                "`verification` must be `automated` or `manual`.".to_string(),
                Some(identifier),
            )),
        }
    }
    findings
}
