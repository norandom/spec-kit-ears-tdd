//! Requirement discovery.
//!
//! The Python implementation recognized exactly two Markdown forms: a bullet and a bold bullet. A
//! requirement written as a heading, a numbered item, a table row, or a block quote was invisible,
//! and the gate reported PASS over content it had never read. It also had no notion of fenced code
//! blocks, so a specification illustrating a *bad* requirement had that example counted as real.

use regex::Regex;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::report::{relative, Finding};

#[derive(Debug, Clone)]
pub struct Requirement {
    pub identifier: String,
    pub text: String,
    pub feature: String,
    pub path: PathBuf,
    pub line: usize,
}

impl Requirement {
    /// Identifiers restart at `REQ-001` in every feature, so anything that compares requirements
    /// across features has to use this rather than the bare identifier.
    pub fn qualified(&self) -> String {
        format!("{}:{}", self.feature, self.identifier)
    }
}

fn identifier_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| Regex::new(r"^(?:\*\*)?(REQ-\d{3,})(?:\*\*)?$").unwrap())
}

fn inline_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
        Regex::new(r"^(?:\*\*)?(?P<id>REQ-\d{3,})(?:\*\*)?\s*[:\-\u{2013}\u{2014}]\s*(?P<text>.+?)$")
            .unwrap()
    })
}

/// Remove the Markdown structure that can precede a requirement identifier: block quote markers,
/// list bullets, ordered-list numbers, and heading hashes. Applied repeatedly because these nest.
fn strip_markers(line: &str) -> &str {
    let mut current = line.trim_start();
    loop {
        let before = current;
        if let Some(rest) = current.strip_prefix('>') {
            current = rest.trim_start();
        }
        if let Some(rest) = current
            .strip_prefix("- ")
            .or_else(|| current.strip_prefix("* "))
            .or_else(|| current.strip_prefix("+ "))
        {
            current = rest.trim_start();
        }
        if current.starts_with('#') {
            let rest = current.trim_start_matches('#');
            if rest.starts_with(' ') {
                current = rest.trim_start();
            }
        }
        current = strip_ordered_marker(current);
        if current == before {
            return current;
        }
    }
}

/// `1. ` and `1) ` only — a bare `1.` with no space is ordinary prose.
fn strip_ordered_marker(line: &str) -> &str {
    let digits = line.len() - line.trim_start_matches(|c: char| c.is_ascii_digit()).len();
    if digits == 0 {
        return line;
    }
    let rest = &line[digits..];
    for marker in [". ", ") "] {
        if let Some(stripped) = rest.strip_prefix(marker) {
            return stripped.trim_start();
        }
    }
    line
}

/// A table row: `| REQ-001 | The system shall ... |`. The identifier must be a cell of its own,
/// which keeps a prose table that merely mentions an identifier from being misread.
fn parse_table_row(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim();
    if !trimmed.starts_with('|') {
        return None;
    }
    let cells: Vec<&str> = trimmed
        .trim_matches('|')
        .split('|')
        .map(|cell| cell.trim())
        .collect();
    if cells.len() < 2 {
        return None;
    }
    let identifier = identifier_pattern().captures(cells[0])?.get(1)?.as_str();
    let text = cells[1].trim();
    if text.is_empty() {
        return None;
    }
    Some((identifier.to_string(), text.to_string()))
}

/// Tracks fenced code blocks so their contents are never mistaken for requirements. A fence opens
/// with three or more backticks or tildes and closes with at least as many of the same character.
#[derive(Default)]
struct FenceState {
    marker: Option<(char, usize)>,
}

impl FenceState {
    fn consume(&mut self, line: &str) -> bool {
        let trimmed = line.trim_start();
        let fence_char = trimmed.chars().next().filter(|c| *c == '`' || *c == '~');
        let Some(fence_char) = fence_char else {
            return self.marker.is_some();
        };
        let run = trimmed.chars().take_while(|c| *c == fence_char).count();
        if run < 3 {
            return self.marker.is_some();
        }
        match self.marker {
            None => {
                self.marker = Some((fence_char, run));
                true
            }
            Some((open_char, open_run)) if open_char == fence_char && run >= open_run => {
                self.marker = None;
                true
            }
            Some(_) => true,
        }
    }

    fn inside(&self) -> bool {
        self.marker.is_some()
    }
}

pub fn parse(root: &Path, spec_path: &Path, feature: &str) -> (Vec<Requirement>, Vec<Finding>) {
    let display = relative(spec_path, root);
    if !spec_path.is_file() {
        return (
            Vec::new(),
            vec![
                Finding::new("SPEC_MISSING", "Specification file not found.", display)
                    .feature(feature),
            ],
        );
    }
    let bytes = match std::fs::read(spec_path) {
        Ok(bytes) => bytes,
        Err(error) => {
            return (
                Vec::new(),
                vec![Finding::new("SPEC_UNREADABLE", error.to_string(), display).feature(feature)],
            )
        }
    };
    let text = match decode_utf8(&bytes) {
        Some(text) => text,
        None => {
            return (
                Vec::new(),
                vec![Finding::new(
                    "SPEC_UNREADABLE",
                    "Specification is not valid UTF-8.",
                    display,
                )
                .feature(feature)],
            )
        }
    };

    let mut requirements: Vec<Requirement> = Vec::new();
    let mut findings: Vec<Finding> = Vec::new();
    let mut seen: Vec<String> = Vec::new();
    let mut fence = FenceState::default();

    for (index, line) in text.lines().enumerate() {
        let number = index + 1;
        if fence.consume(line) || fence.inside() {
            continue;
        }
        let Some((identifier, body)) = parse_line(line) else {
            continue;
        };
        if seen.contains(&identifier) {
            findings.push(
                Finding::new(
                    "REQ_DUPLICATE",
                    "Requirement identifier is duplicated in this specification.",
                    display.clone(),
                )
                .feature(feature)
                .requirement(&identifier)
                .line(number),
            );
            continue;
        }
        seen.push(identifier.clone());
        let requirement = Requirement {
            identifier,
            text: body,
            feature: feature.to_string(),
            path: spec_path.to_path_buf(),
            line: number,
        };
        findings.extend(crate::ears::validate(root, &requirement));
        requirements.push(requirement);
    }

    if requirements.is_empty() {
        findings.push(
            Finding::new(
                "REQ_NONE",
                "No requirements matching `REQ-NNN: <EARS sentence>` were found.",
                display,
            )
            .feature(feature),
        );
    }
    (requirements, findings)
}

fn parse_line(line: &str) -> Option<(String, String)> {
    if let Some(parsed) = parse_table_row(line) {
        return Some(parsed);
    }
    let stripped = strip_markers(line);
    let captures = inline_pattern().captures(stripped)?;
    let identifier = captures.name("id")?.as_str().to_string();
    let text = captures
        .name("text")?
        .as_str()
        .trim()
        .trim_matches('*')
        .trim()
        .to_string();
    if text.is_empty() {
        return None;
    }
    Some((identifier, text))
}

/// Accepts a UTF-8 byte order mark, which Windows editors add routinely, rather than treating the
/// file as undecodable.
fn decode_utf8(bytes: &[u8]) -> Option<String> {
    let body = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(bytes);
    String::from_utf8(body.to_vec()).ok()
}
