//! Production-code separation: requirement identifiers and copied requirement prose may not appear
//! in production source.
//!
//! Three changes from the Python implementation. It compiled a fresh regex per (line, requirement)
//! pair, which is O(requirements x lines) per file; one Aho-Corasick automaton scans each file once
//! for every identifier at the same time. It walked every file under a production root including
//! `.venv` and `node_modules`; walking now honours ignore rules. And it skipped a missing production
//! root, an undecodable file, and an oversize file in silence — each is now a warning, because a
//! check that did not run must not look like a check that passed.

use aho_corasick::AhoCorasick;
use std::collections::BTreeSet;
use std::path::Path;

use crate::config::Config;
use crate::exemptions;
use crate::report::{relative, Finding, Severity};
use crate::requirements::Requirement;

const MAX_FILE_BYTES: u64 = 2_000_000;
const MIN_PROSE_CHARS: usize = 40;

pub struct Outcome {
    pub findings: Vec<Finding>,
    pub files_scanned: usize,
    /// How many separation findings an exemption removed. Reported so the mechanism cannot quietly
    /// grow: an escape hatch that hides its own use turns a loud gate into a silent one.
    pub exempted: usize,
}

/// Where copied prose begins, approximately.
///
/// Detection runs over the whole file normalized, so that a requirement sentence reformatted across
/// several lines is still caught. That costs the line number, which an exemption marker needs. The
/// first line substantial enough to be part of the sentence is close enough to point a reader at,
/// and a wrong-by-one line is better than a finding nobody can locate.
fn locate_prose(content: &str, normalized_requirement: &str) -> Option<usize> {
    content.lines().enumerate().find_map(|(index, line)| {
        let normalized = normalized_prose(line);
        if normalized.chars().count() < 20 {
            return None;
        }
        // Both directions matter. The whole sentence on one line means that line contains the
        // requirement; a sentence reflowed across several lines means each line is a fragment of
        // it. Checking only the second case misses the common one entirely.
        let whole_sentence_here = normalized.contains(normalized_requirement);
        let fragment_of_sentence = normalized_requirement.contains(&normalized);
        (whole_sentence_here || fragment_of_sentence).then_some(index + 1)
    })
}

fn normalized_prose(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// A match is only an identifier reference when it is not embedded in a longer token, so a longer identifier
/// is never read as a reference to a shorter one that prefixes it.
fn is_standalone(haystack: &str, start: usize, end: usize) -> bool {
    let before = haystack[..start].chars().next_back();
    let after = haystack[end..].chars().next();
    let boundary = |c: Option<char>| match c {
        None => true,
        Some(c) => !(c.is_alphanumeric() || c == '-' || c == '_'),
    };
    boundary(before) && boundary(after)
}

pub fn validate(root: &Path, requirements: &[Requirement], config: &Config) -> Outcome {
    let mut findings = Vec::new();
    let mut files_scanned = 0usize;

    if requirements.is_empty() {
        return Outcome {
            findings,
            files_scanned,
            exempted: 0,
        };
    }

    let identifiers: Vec<String> = requirements
        .iter()
        .map(|requirement| requirement.identifier.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let automaton = match AhoCorasick::new(&identifiers) {
        Ok(automaton) => automaton,
        Err(error) => {
            findings.push(Finding::new("SEPARATION_INTERNAL", error.to_string(), "."));
            return Outcome {
                findings,
                files_scanned,
                exempted: 0,
            };
        }
    };

    let (exempt_paths, mut pattern_findings) =
        exemptions::build_exempt_set(&config.separation_exempt);
    findings.append(&mut pattern_findings);
    let mut pattern_matched = false;
    let mut exempted = 0usize;

    for declared_root in &config.production_roots {
        let production_root = root.join(declared_root);
        if !production_root.is_dir() {
            findings.push(
                Finding::new(
                    "PRODUCTION_ROOT_MISSING",
                    format!(
                        "Configured production root `{declared_root}` does not exist; \
                         nothing was scanned for it."
                    ),
                    declared_root.clone(),
                )
                .severity(Severity::Warning),
            );
            continue;
        }

        for entry in ignore::WalkBuilder::new(&production_root)
            .hidden(true)
            .git_ignore(true)
            .build()
            .flatten()
        {
            let path = entry.path();
            if !path.is_file() || !config.matches_source_extension(path) {
                continue;
            }
            let display = relative(path, root);
            match std::fs::metadata(path) {
                Ok(metadata) if metadata.len() > MAX_FILE_BYTES => {
                    findings.push(
                        Finding::new(
                            "SOURCE_SKIPPED",
                            "File exceeds the separation-scan size limit and was not scanned."
                                .to_string(),
                            display,
                        )
                        .severity(Severity::Warning),
                    );
                    continue;
                }
                Ok(_) => {}
                Err(error) => {
                    findings.push(
                        Finding::new("SOURCE_SKIPPED", error.to_string(), display)
                            .severity(Severity::Warning),
                    );
                    continue;
                }
            }
            let bytes = match std::fs::read(path) {
                Ok(bytes) => bytes,
                Err(error) => {
                    findings.push(
                        Finding::new("SOURCE_SKIPPED", error.to_string(), display)
                            .severity(Severity::Warning),
                    );
                    continue;
                }
            };
            let Ok(content) = String::from_utf8(
                bytes
                    .strip_prefix(&[0xEF, 0xBB, 0xBF])
                    .unwrap_or(&bytes)
                    .to_vec(),
            ) else {
                // Windows tooling emits UTF-16 for .ps1 routinely. The Python implementation
                // dropped these silently, so the separation gate passed without reading them.
                findings.push(
                    Finding::new(
                        "SOURCE_SKIPPED",
                        "File is not valid UTF-8 and was not scanned for requirement leakage."
                            .to_string(),
                        display,
                    )
                    .severity(Severity::Warning),
                );
                continue;
            };
            files_scanned += 1;
            let mut raw = Vec::new();
            scan(
                &mut raw,
                &automaton,
                &identifiers,
                requirements,
                &content,
                &display,
            );

            let path_exempt = exempt_paths.is_match(&display);
            pattern_matched |= path_exempt;
            let mut markers = exemptions::markers(&content);

            for finding in raw {
                if path_exempt {
                    exempted += 1;
                    findings.push(exemptions::applied(
                        &display,
                        finding.line,
                        "matched a configured separation_exempt pattern",
                    ));
                    continue;
                }
                // A marker with no reason is not a marker. Without that rule the mechanism becomes
                // a one-word way to silence the gate, which is the outcome it exists to prevent.
                let reason = finding
                    .line
                    .and_then(|line| exemptions::covering(&mut markers, line))
                    .and_then(|marker| {
                        marker.reason.clone().inspect(|_| {
                            marker.used = true;
                        })
                    });
                match reason {
                    Some(reason) => {
                        exempted += 1;
                        findings.push(exemptions::applied(&display, finding.line, &reason));
                    }
                    None => findings.push(finding),
                }
            }

            for marker in &markers {
                match (&marker.reason, marker.used) {
                    (None, _) => {
                        findings.push(exemptions::marker_without_reason(&display, marker.line))
                    }
                    (Some(_), false) => findings.push(exemptions::redundant(&display, marker.line)),
                    (Some(_), true) => {}
                }
            }
        }
    }

    if !pattern_matched {
        for pattern in &config.separation_exempt {
            findings.push(exemptions::stale_pattern(pattern));
        }
    }

    findings.sort_by_key(|finding| finding.dedupe_key());
    findings.dedup_by_key(|finding| finding.dedupe_key());
    Outcome {
        findings,
        files_scanned,
        exempted,
    }
}

fn scan(
    findings: &mut Vec<Finding>,
    automaton: &AhoCorasick,
    identifiers: &[String],
    requirements: &[Requirement],
    content: &str,
    display: &str,
) {
    let mut line_starts = vec![0usize];
    for (index, byte) in content.bytes().enumerate() {
        if byte == b'\n' {
            line_starts.push(index + 1);
        }
    }
    let line_of = |offset: usize| match line_starts.binary_search(&offset) {
        Ok(index) => index + 1,
        Err(index) => index,
    };

    for hit in automaton.find_iter(content) {
        if !is_standalone(content, hit.start(), hit.end()) {
            continue;
        }
        let identifier = &identifiers[hit.pattern().as_usize()];
        // Every feature that declares this identifier is a candidate owner, so the finding is
        // qualified by feature and deduplicated afterwards.
        for requirement in requirements
            .iter()
            .filter(|requirement| &requirement.identifier == identifier)
        {
            findings.push(
                Finding::new(
                    "CODE_REQ_ID",
                    "Production code contains a requirement ID; keep traceability in tests and \
                     artifacts.",
                    display.to_string(),
                )
                .feature(&requirement.feature)
                .requirement(identifier)
                .line(line_of(hit.start())),
            );
        }
    }

    let normalized_content = normalized_prose(content);
    for requirement in requirements {
        let normalized = normalized_prose(&requirement.text);
        if normalized.chars().count() >= MIN_PROSE_CHARS && normalized_content.contains(&normalized)
        {
            let mut finding = Finding::new(
                "CODE_REQ_PROSE",
                "Production code contains copied requirement prose.",
                display.to_string(),
            )
            .feature(&requirement.feature)
            .requirement(&requirement.identifier);
            if let Some(line) = locate_prose(content, &normalized) {
                finding = finding.line(line);
            }
            findings.push(finding);
        }
    }
}
