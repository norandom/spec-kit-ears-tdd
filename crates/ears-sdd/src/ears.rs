//! EARS form checking.
//!
//! Two false positives are fixed here relative to the Python implementation. It searched the raw
//! sentence, so `shall log the message "operation may fail"` was reported as using a competing
//! modal, and `shall reject must-gather bundles` tripped on a hyphenated compound because `\b`
//! matches at a hyphen. Both are ordinary English that the gate had no business rejecting.
//!
//! The clause check is also stricter: the Python version took the *first* comma anywhere in the
//! sentence, so a parenthetical before the response satisfied it. This uses the last comma that
//! precedes `shall`.

use regex::Regex;
use std::path::Path;
use std::sync::OnceLock;

use crate::report::{relative, Finding};
use crate::requirements::Requirement;

fn shall_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| Regex::new(r"(?i)\bshall\b").unwrap())
}

fn modal_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| Regex::new(r"(?i)\b(should|may|must)\b").unwrap())
}

fn closing_delimiter(open: char) -> char {
    match open {
        '\u{201c}' => '\u{201d}',
        '\u{2018}' => '\u{2019}',
        other => other,
    }
}

/// An apostrophe inside a word is possessive, not a quote. Without this, `the research tree's
/// physical root changes, ... shall recreate ...` opens a literal at `tree's` that never closes,
/// masks the rest of the sentence, and reports a requirement containing `shall` as containing none.
fn is_word_internal(characters: &[char], index: usize) -> bool {
    let before = index.checked_sub(1).and_then(|i| characters.get(i));
    let after = characters.get(index + 1);
    matches!(before, Some(c) if c.is_alphanumeric()) && matches!(after, Some(c) if c.is_alphanumeric())
}

fn is_delimiter(characters: &[char], index: usize) -> bool {
    match characters[index] {
        '"' | '`' | '\u{201c}' | '\u{201d}' => true,
        '\'' | '\u{2018}' | '\u{2019}' => !is_word_internal(characters, index),
        _ => false,
    }
}

/// Replace the contents of *balanced* quoted literals with spaces, preserving byte offsets so
/// positions found in the masked text still index the original sentence. An unterminated quote is
/// left alone rather than swallowing the remainder of the sentence.
fn mask_quoted(sentence: &str) -> String {
    let characters: Vec<char> = sentence.chars().collect();
    let mut hidden = vec![false; characters.len()];
    let mut index = 0usize;
    while index < characters.len() {
        if is_delimiter(&characters, index) {
            let wanted = closing_delimiter(characters[index]);
            let close = (index + 1..characters.len())
                .find(|&j| characters[j] == wanted && is_delimiter(&characters, j));
            if let Some(close) = close {
                for item in hidden.iter_mut().take(close).skip(index + 1) {
                    *item = true;
                }
                index = close + 1;
                continue;
            }
        }
        index += 1;
    }

    let mut masked = String::with_capacity(sentence.len());
    for (position, character) in characters.iter().enumerate() {
        if hidden[position] {
            for _ in 0..character.len_utf8() {
                masked.push(' ');
            }
        } else {
            masked.push(*character);
        }
    }
    masked
}

/// `\b` treats a hyphen as a boundary, so `must-gather` and `may-fail` would otherwise be read as
/// normative modals.
fn is_hyphen_adjacent(text: &str, start: usize, end: usize) -> bool {
    let before = text[..start].chars().next_back();
    let after = text[end..].chars().next();
    before == Some('-') || after == Some('-')
}

pub fn validate(root: &Path, requirement: &Requirement) -> Vec<Finding> {
    let mut findings = Vec::new();
    let sentence = requirement.text.trim();
    let masked = mask_quoted(sentence);
    let display = relative(&requirement.path, root);

    let finding = |code: &str, message: &str| {
        Finding::new(code, message, display.clone())
            .feature(&requirement.feature)
            .requirement(&requirement.identifier)
            .line(requirement.line)
    };

    let shall_matches: Vec<_> = shall_pattern().find_iter(&masked).collect();
    if shall_matches.len() != 1 {
        findings.push(finding(
            "EARS_SHALL",
            &format!(
                "EARS requires exactly one `shall`; found {}.",
                shall_matches.len()
            ),
        ));
    }

    if let Some(modal) = modal_pattern()
        .find_iter(&masked)
        .find(|m| !is_hyphen_adjacent(&masked, m.start(), m.end()))
    {
        findings.push(finding(
            "EARS_MODAL",
            &format!("Use `shall`, not `{}`.", &sentence[modal.start()..modal.end()]),
        ));
    }

    let Some(shall) = shall_matches.first() else {
        return findings;
    };

    let subject = sentence[..shall.start()].trim_matches(|c: char| c == ',' || c.is_whitespace());
    let response = sentence[shall.end()..].trim_matches(|c: char| c == '.' || c.is_whitespace());
    if subject.is_empty() || response.is_empty() {
        findings.push(finding(
            "EARS_INCOMPLETE",
            "The requirement needs both a system subject and an observable response.",
        ));
    }

    let lower = sentence.to_lowercase();
    let last_comma_before_shall = masked[..shall.start()].rfind(',');
    if lower.starts_with("when ") || lower.starts_with("while ") || lower.starts_with("where ") {
        if last_comma_before_shall.is_none() {
            findings.push(finding(
                "EARS_CLAUSE",
                "The EARS condition must end with a comma before the system response.",
            ));
        }
    } else if lower.starts_with("if ") {
        let head = &lower[..shall.start().min(lower.len())];
        if !head.contains(", then ") {
            findings.push(finding(
                "EARS_UNWANTED",
                "Unwanted-behavior form must use `If <condition>, then <system> shall ...`.",
            ));
        }
    } else if ["when", "while", "where", "if"]
        .iter()
        .any(|prefix| lower.starts_with(prefix))
    {
        findings.push(finding(
            "EARS_PREFIX",
            "Use a complete EARS prefix followed by a space.",
        ));
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::mask_quoted;

    #[test]
    fn possessive_apostrophe_is_not_a_quote() {
        let sentence =
            "When the research tree's physical root changes, the manager shall recreate it.";
        assert!(mask_quoted(sentence).contains("shall"));
    }

    #[test]
    fn balanced_literal_is_masked() {
        let sentence = "The service shall log the message \"operation may fail\".";
        let masked = mask_quoted(sentence);
        assert!(masked.contains("shall"));
        assert!(!masked.contains("may"));
    }

    #[test]
    fn unterminated_quote_does_not_swallow_the_sentence() {
        let sentence = "The service shall emit a \" character.";
        assert!(mask_quoted(sentence).contains("character"));
    }
}
