//! Declared exceptions to the separation gate.
//!
//! The policy already has an exception in it -- tests may name requirement identifiers, production
//! code may not -- and no way to express it in a language that keeps unit tests inside the file they
//! test. Without a way to say "this one is intentional", the only escape is rewording until the
//! checker stops noticing, which is indistinguishable from concealment and leaves no record of the
//! judgement. This repository reworded four times in two days before this existed.
//!
//! Two properties keep the mechanism from becoming a hole. A marker carries a reason or it does not
//! count, and every exemption applied is reported and counted, so the number can be seen growing.
//! An exemption mechanism that hides its own use turns a loud gate into a quiet one, which is worse
//! than having none.

use globset::{Glob, GlobSet, GlobSetBuilder};

use crate::report::{Finding, Severity};

/// Matched as text rather than parsed, so it works in whatever comment syntax the file uses.
///
/// Assembled from two pieces so this line does not itself contain the marker. Written whole, the
/// module defining the marker is detected as carrying a reasonless one -- the same self-reference
/// that motivated this feature, arriving one level further in.
pub const MARKER: &str = concat!("ears-sdd", ":allow-requirement-id");

/// Short enough not to be a burden, long enough that "x" does not qualify.
const MIN_REASON: usize = 10;

#[derive(Debug, Clone)]
pub struct Marker {
    pub line: usize,
    pub reason: Option<String>,
    /// Set once the marker actually suppresses something, so a marker that suppresses nothing can
    /// be reported as dead weight.
    pub used: bool,
}

/// Every marker in a file, keyed by the line it appears on.
pub fn markers(content: &str) -> Vec<Marker> {
    let mut found = Vec::new();
    for (index, line) in content.lines().enumerate() {
        let Some(position) = line.find(MARKER) else {
            continue;
        };
        let tail = line[position + MARKER.len()..].trim();
        let reason = tail
            .strip_prefix(':')
            .map(str::trim)
            .filter(|reason| reason.chars().count() >= MIN_REASON)
            .map(str::to_string);
        found.push(Marker {
            line: index + 1,
            reason,
            used: false,
        });
    }
    found
}

/// A marker covers its own line and the line immediately after it, so an author can put the
/// declaration above the thing it is about rather than trailing a long line.
pub fn covering(markers: &mut [Marker], line: usize) -> Option<&mut Marker> {
    markers
        .iter_mut()
        .find(|marker| marker.line == line || marker.line + 1 == line)
}

pub fn build_exempt_set(patterns: &[String]) -> (GlobSet, Vec<Finding>) {
    let mut builder = GlobSetBuilder::new();
    let mut findings = Vec::new();
    for pattern in patterns {
        match Glob::new(&pattern.replace('\\', "/")) {
            Ok(glob) => {
                builder.add(glob);
            }
            Err(error) => findings.push(Finding::new(
                "CONFIG_INVALID",
                format!("`separation_exempt` pattern is not valid: {pattern} ({error})"),
                crate::config::CONFIG_RELATIVE_PATH,
            )),
        }
    }
    (
        builder.build().unwrap_or_else(|_| GlobSet::empty()),
        findings,
    )
}

pub fn applied(path: &str, line: Option<usize>, reason: &str) -> Finding {
    let mut finding = Finding::new(
        "SEPARATION_EXEMPT",
        format!("Separation finding exempted: {reason}"),
        path.to_string(),
    )
    .severity(Severity::Advisory);
    if let Some(line) = line {
        finding = finding.line(line);
    }
    finding
}

pub fn redundant(path: &str, line: usize) -> Finding {
    Finding::new(
        "SEPARATION_EXEMPT_UNUSED",
        "Exemption marker suppresses nothing; remove it.".to_string(),
        path.to_string(),
    )
    .line(line)
    .severity(Severity::Warning)
}

pub fn marker_without_reason(path: &str, line: usize) -> Finding {
    Finding::new(
        "SEPARATION_EXEMPT_NO_REASON",
        format!(
            "Exemption marker carries no reason, so it suppresses nothing. Write \
             `{MARKER}: why this mention is intentional`."
        ),
        path.to_string(),
    )
    .line(line)
    .severity(Severity::Warning)
}

pub fn stale_pattern(pattern: &str) -> Finding {
    Finding::new(
        "SEPARATION_EXEMPT_STALE",
        format!("`separation_exempt` pattern matches no file: {pattern}"),
        crate::config::CONFIG_RELATIVE_PATH,
    )
    .severity(Severity::Warning)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_marker_needs_a_reason() {
        let with = markers(&format!(
            "// something {MARKER}: explaining the format here"
        ));
        assert_eq!(with.len(), 1);
        assert!(with[0].reason.is_some());

        let without = markers(&format!("// something {MARKER}"));
        assert_eq!(without.len(), 1);
        assert!(without[0].reason.is_none());

        let too_short = markers(&format!("// something {MARKER}: x"));
        assert!(too_short[0].reason.is_none());
    }

    #[test]
    fn a_marker_covers_its_own_line_and_the_next() {
        let mut found = markers(&format!(
            "// {MARKER}: declared above the thing\nlet x = 1;\n"
        ));
        assert!(covering(&mut found, 1).is_some());
        assert!(covering(&mut found, 2).is_some());
        assert!(covering(&mut found, 3).is_none());
    }
}
