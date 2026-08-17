//! Task coverage.
//!
//! `--phase tasks` used to be byte-identical to `--phase plan` -- it never opened `tasks.md`. The
//! one gate standing between an approved plan and production-code changes checked nothing about the
//! decomposition it exists to guard.
//!
//! The awkward part is spans. Real task lists reference a range of requirements as two identifiers
//! joined by a dash rather than listing every one inline, so a gate matching only literal
//! identifiers would report a false failure for each requirement in the middle of the range. A gate
//! that cries wolf gets switched off, so spans are expanded. They are matched numerically rather
//! than textually, which also makes a mismatched digit width between endpoints a non-issue.
//!
//! Note the identifiers in this file are written with an `NNN` placeholder rather than digits. The
//! separation gate forbids requirement identifiers in production code, and it is right to: the
//! alternative is an exemption mechanism that would be used to silence real leaks.

use regex::Regex;
use std::collections::BTreeSet;
use std::path::Path;
use std::sync::OnceLock;

use crate::report::{relative, Finding};
use crate::requirements::{FenceState, Requirement};

fn identifier_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| Regex::new(r"REQ-(\d{3,})").unwrap())
}

/// Two identifiers joined by a range marker.
///
/// Dash and word forms both occur in real task lists, sometimes in the same repository. Recognizing
/// only dashes produced seven false uncovered-requirement failures against a feature whose work was
/// fully decomposed, because its author wrote the range as `REQ-NNN through REQ-NNN`.
///
/// `to` is accepted despite being the most ambiguous, on the grounds that the two failure modes are
/// not symmetric: reading a non-range as a range silently misses one uncovered requirement, while
/// missing a real range reports several confident falsehoods, and a gate that cries wolf gets
/// switched off.
fn span_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
        Regex::new(
            r"(?i)REQ-(\d{3,})\s*(?:[-\u{2013}\u{2014}]|\.\.\.?|\u{2026}|through|to)\s*REQ-(\d{3,})",
        )
        .unwrap()
    })
}

#[derive(Default)]
struct References {
    /// Identifiers written out in full, exactly as they appear.
    literal: BTreeSet<String>,
    /// Numeric values covered by a span, including its endpoints.
    spanned: BTreeSet<u32>,
    /// Spans whose endpoints run backwards, reported rather than silently expanded to nothing.
    descending: Vec<String>,
    saw_any_line: bool,
}

fn collect(text: &str) -> References {
    let mut found = References::default();
    let mut fence = FenceState::default();
    for line in text.lines() {
        if fence.consume(line) || fence.inside() {
            continue;
        }
        if !line.trim().is_empty() {
            found.saw_any_line = true;
        }
        for capture in span_pattern().captures_iter(line) {
            let (Ok(from), Ok(to)) = (capture[1].parse::<u32>(), capture[2].parse::<u32>()) else {
                continue;
            };
            if from > to {
                found.descending.push(capture[0].to_string());
                continue;
            }
            found.spanned.extend(from..=to);
        }
        for capture in identifier_pattern().captures_iter(line) {
            found.literal.insert(capture[0].to_string());
        }
    }
    found
}

fn numeric(identifier: &str) -> Option<u32> {
    identifier_pattern()
        .captures(identifier)
        .and_then(|capture| capture[1].parse().ok())
}

pub struct Outcome {
    pub findings: Vec<Finding>,
    pub covered: usize,
}

pub fn validate(
    root: &Path,
    spec_path: &Path,
    feature: &str,
    requirements: &[Requirement],
) -> Outcome {
    let path = spec_path.with_file_name("tasks.md");
    let display = relative(&path, root);
    let finding = |code: &str, message: String, requirement: Option<&str>| {
        let mut item = Finding::new(code, message, display.clone()).feature(feature);
        if let Some(identifier) = requirement {
            item = item.requirement(identifier);
        }
        item
    };

    let Ok(text) = std::fs::read_to_string(&path) else {
        return Outcome {
            findings: vec![finding(
                "TASK_LIST_MISSING",
                "No task list beside this specification; requirements cannot reach implementation \
                 undecomposed."
                    .to_string(),
                None,
            )],
            covered: 0,
        };
    };

    let found = collect(&text);
    let mut findings = Vec::new();
    let mut covered = 0usize;

    for span in &found.descending {
        findings.push(finding(
            "TASK_SPAN",
            format!("Requirement span runs backwards and covers nothing: {span}"),
            None,
        ));
    }

    let declared: BTreeSet<&str> = requirements
        .iter()
        .map(|requirement| requirement.identifier.as_str())
        .collect();

    for identifier in &declared {
        let is_covered = found.literal.contains(*identifier)
            || numeric(identifier).is_some_and(|value| found.spanned.contains(&value));
        if is_covered {
            covered += 1;
        } else if found.saw_any_line {
            findings.push(finding(
                "TASK_UNCOVERED",
                "Requirement is declared but no task references it.".to_string(),
                Some(identifier),
            ));
        }
    }

    // Only literals are reported as unknown. A span stands for its endpoints and everything
    // between, and does not assert that every intermediate identifier was declared.
    for identifier in &found.literal {
        if !declared.contains(identifier.as_str()) {
            findings.push(finding(
                "TASK_UNKNOWN_REF",
                "Task references an identifier this specification does not declare.".to_string(),
                Some(identifier),
            ));
        }
    }

    Outcome { findings, covered }
}
