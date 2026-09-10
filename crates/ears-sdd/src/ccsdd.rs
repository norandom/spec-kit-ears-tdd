//! cc-sdd (`.kiro/specs/<name>/requirements.md`) requirement discovery.
//!
//! facdrone's cc-sdd specs use several coexisting requirement grammars, all of which normalize
//! into the dotted-identifier IR consumed downstream by the EARS checks:
//!
//! * **G1 canonical** — `### Requirement N: <title>` then numbered `M.` criteria under
//!   `#### Acceptance Criteria`. The dominant form.
//! * **G1 dotted** — same headings, but criteria are self-numbered `N.M When…` (a dotted marker
//!   that repeats the requirement number), e.g. facdrone-performance-attribution.
//! * **G2 R-style** — `### R1 — <title>` then bulleted `- R1.1 …` criteria with no criteria
//!   heading, uppercase `SHALL/WHEN` (facdrone-results-and-service). The bullet label is stripped.
//! * **G3 amendment** — `### Amendment A1 — <title> (extends Requirement 4)` then numbered `M.`
//!   criteria. Amendments refine a frozen baseline; they parse into `A1.M` with an `extends` link.
//! * **G4 empty/prose** — a `## Requirements` section holding only an HTML-comment placeholder, or
//!   no requirement headings at all (reserved / initialized specs). Yields no requirements and no
//!   error; phase reconciliation is the ledger's job, not the parser's.
//!
//! The parser is structural and marker-driven. A criterion is recognized in one of two ways:
//!
//! 1. **Self-identifying dotted marker** — a list item whose marker is a dotted id (`1.1 ` or
//!    `- R1.1 `). These are unambiguous anywhere inside a requirement block, so they need no
//!    criteria heading. This covers G1-dotted and G2.
//! 2. **Bare numbered marker inside a criteria region** — `M. ` is only a criterion when it
//!    appears under an `Acceptance Criteria` heading, because a bare `1. ` is otherwise ordinary
//!    prose numbering. This covers G1-canonical and G3.
//!
//! Dotted references such as `R3.4` or `Requirement 6.7` in prose are never definitions: they do
//! not sit at a list-marker position. Multi-line criteria are joined to one logical sentence;
//! findings cite the physical start line.

use std::path::{Path, PathBuf};

/// Whether a criterion belongs to the approved baseline or to a post-approval amendment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReqKind {
    Baseline,
    Amendment,
}

/// One acceptance criterion, normalized to the dotted-identifier IR.
///
/// The identifier is `N.M` for baseline requirements (G1/G2) and `A{amendment}.M` for amendment
/// criteria (G3), so every criterion in a spec has a unique, citable, sortable identifier.
#[derive(Debug, Clone)]
pub struct CcsddRequirement {
    /// Dotted identifier: `5.6` (G1), `1.1` (G2 from `R1.1`), `A1.3` (G3).
    pub id: String,
    pub kind: ReqKind,
    /// The parent requirement heading number: `5` (G1), `1` (G2), `A1` (G3).
    pub requirement_no: String,
    /// The criterion number within its parent: `6`, `1`, `3`.
    pub criterion_no: String,
    /// Title of the parent requirement / amendment heading.
    pub title: String,
    /// Amendment-only: requirement numbers this extends, from `(extends Requirement 4 / 12)`.
    pub extends: Vec<String>,
    /// The joined, single-sentence criterion text (continuation lines merged).
    pub ears_text: String,
    pub feature: String,
    pub path: PathBuf,
    /// Physical line where the criterion starts (1-based).
    pub line: usize,
    /// Physical line where the criterion ends, after joining continuations.
    pub end_line: usize,
}

impl CcsddRequirement {
    /// Feature-qualified identifier, matching `Requirement::qualified` for the spec-kit path so
    /// cross-spec references are unambiguous: `facdrone-ledger-core:3.4`.
    pub fn qualified(&self) -> String {
        format!("{}:{}", self.feature, self.id)
    }
}

/// A non-fatal structural observation produced while parsing. These surface grammar drift a
/// reader should know about (e.g. a requirement heading with no criteria) without failing the parse.
#[derive(Debug, Clone)]
pub struct ParseNote {
    pub code: &'static str,
    pub message: String,
    pub line: usize,
}

/// The requirement currently being read: its identity plus whether it has produced any criteria.
#[derive(Debug, Clone)]
struct RequirementContext {
    no: String,
    kind: ReqKind,
    title: String,
    extends: Vec<String>,
    heading_line: usize,
    has_criteria: bool,
}

/// A criterion being assembled from its first line plus any continuation lines, carrying the full
/// parent context captured at the moment its marker was seen.
struct PendingCriterion {
    ctx: RequirementContext,
    criterion_no: String,
    text: String,
    line: usize,
    end_line: usize,
}

impl PendingCriterion {
    fn push_continuation(&mut self, text: &str, line: usize) {
        if !self.text.is_empty() && !text.is_empty() {
            self.text.push(' ');
        }
        self.text.push_str(text);
        self.end_line = line;
    }

    fn finish(self, feature: &str, path: &Path) -> CcsddRequirement {
        let id = format!("{}.{}", self.ctx.no, self.criterion_no);
        CcsddRequirement {
            id,
            kind: self.ctx.kind,
            requirement_no: self.ctx.no.clone(),
            criterion_no: self.criterion_no,
            title: self.ctx.title.clone(),
            extends: self.ctx.extends.clone(),
            ears_text: self.text.trim().to_string(),
            feature: feature.to_string(),
            path: path.to_path_buf(),
            line: self.line,
            end_line: self.end_line,
        }
    }
}

pub struct Parser<'a> {
    feature: &'a str,
    path: &'a Path,
}

impl<'a> Parser<'a> {
    pub fn new(feature: &'a str, path: &'a Path) -> Self {
        Self { feature, path }
    }

    /// Parse a `requirements.md` body into normalized criteria plus structural notes.
    pub fn parse(&self, text: &str) -> (Vec<CcsddRequirement>, Vec<ParseNote>) {
        let mut out: Vec<CcsddRequirement> = Vec::new();
        let mut notes: Vec<ParseNote> = Vec::new();

        let mut fence = Fence::default();
        let mut ctx: Option<RequirementContext> = None;
        let mut in_criteria_heading = false;
        let mut pending: Option<PendingCriterion> = None;

        for (index, line) in text.lines().enumerate() {
            let number = index + 1;
            if fence.consume(line) || fence.inside() {
                continue;
            }
            let trimmed = line.trim();

            if let Some(heading) = Heading::parse(line) {
                if let Some(p) = pending.take() {
                    out.push(p.finish(self.feature, self.path));
                }
                match heading.kind {
                    HeadingKind::Requirement { no, title } => {
                        self.close_requirement(&ctx, &mut notes);
                        ctx = Some(RequirementContext {
                            no,
                            kind: ReqKind::Baseline,
                            title,
                            extends: Vec::new(),
                            heading_line: number,
                            has_criteria: false,
                        });
                        in_criteria_heading = false;
                    }
                    HeadingKind::Amendment { no, title, extends } => {
                        self.close_requirement(&ctx, &mut notes);
                        ctx = Some(RequirementContext {
                            no,
                            kind: ReqKind::Amendment,
                            title,
                            extends,
                            heading_line: number,
                            has_criteria: false,
                        });
                        in_criteria_heading = false;
                    }
                    HeadingKind::AcceptanceCriteria => {
                        in_criteria_heading = true;
                    }
                    HeadingKind::Other => {
                        self.close_requirement(&ctx, &mut notes);
                        ctx = None;
                        in_criteria_heading = false;
                    }
                }
                continue;
            }

            // We only collect criteria while inside a requirement block.
            let Some(current) = ctx.as_mut() else {
                continue;
            };

            // A blank line closes the open criterion but not the requirement block.
            if trimmed.is_empty() {
                if let Some(p) = pending.take() {
                    out.push(p.finish(self.feature, self.path));
                }
                continue;
            }

            // Try to open a new criterion. Self-identifying dotted markers match anywhere in the
            // block; bare numbered markers match under an Acceptance Criteria heading (baseline)
            // or anywhere in an amendment block (amendments carry no other numbered prose).
            let bare_ok = in_criteria_heading || current.kind == ReqKind::Amendment;
            if let Some((crit_no, first_text)) =
                parse_criterion_marker(trimmed, &current.no, bare_ok)
            {
                if let Some(p) = pending.take() {
                    out.push(p.finish(self.feature, self.path));
                }
                current.has_criteria = true;
                pending = Some(PendingCriterion {
                    ctx: current.clone(),
                    criterion_no: crit_no,
                    text: first_text,
                    line: number,
                    end_line: number,
                });
                continue;
            }

            // Otherwise, an indented/non-marker line continues the open criterion.
            if let Some(p) = pending.as_mut() {
                p.push_continuation(trimmed, number);
            }
        }
        if let Some(p) = pending.take() {
            out.push(p.finish(self.feature, self.path));
        }
        self.close_requirement(&ctx, &mut notes);

        (out, notes)
    }

    /// Report a requirement that ended with no criteria, then drop it. Reserved/initialized specs
    /// legitimately have prose-only requirement blocks, so this is a note, not an error.
    fn close_requirement(&self, ctx: &Option<RequirementContext>, notes: &mut Vec<ParseNote>) {
        if let Some(c) = ctx {
            if !c.has_criteria && c.kind == ReqKind::Baseline {
                notes.push(ParseNote {
                    code: "REQ_NO_CRITERIA",
                    message: format!(
                        "Requirement {} (“{}”) has no acceptance criteria.",
                        c.no, c.title
                    ),
                    line: c.heading_line,
                });
            }
        }
    }
}

/// Tracks fenced code blocks so example requirements inside ``` fences are never parsed.
#[derive(Default)]
struct Fence {
    marker: Option<(char, usize)>,
}

impl Fence {
    fn consume(&mut self, line: &str) -> bool {
        let trimmed = line.trim_start();
        let ch = trimmed.chars().next().filter(|c| *c == '`' || *c == '~');
        let Some(ch) = ch else {
            return self.marker.is_some();
        };
        let run = trimmed.chars().take_while(|c| *c == ch).count();
        if run < 3 {
            return self.marker.is_some();
        }
        match self.marker {
            None => {
                self.marker = Some((ch, run));
                true
            }
            Some((open_ch, open_run)) if open_ch == ch && run >= open_run => {
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

enum HeadingKind {
    Requirement {
        no: String,
        title: String,
    },
    Amendment {
        no: String,
        title: String,
        extends: Vec<String>,
    },
    AcceptanceCriteria,
    Other,
}

struct Heading {
    kind: HeadingKind,
}

impl Heading {
    /// Classify a heading line. Only `### Requirement`, `### R<n>`, `### Amendment`, and
    /// `#### Acceptance Criteria` are structurally meaningful; everything else is `Other`.
    fn parse(line: &str) -> Option<Heading> {
        let trimmed = line.trim_start();
        if !trimmed.starts_with('#') {
            return None;
        }
        let level = trimmed.chars().take_while(|c| *c == '#').count();
        let rest = &trimmed[level..];
        if !rest.is_empty() && !rest.starts_with(' ') {
            return None; // `##tag` is not a heading
        }
        let body = rest.trim_start();

        if body.eq_ignore_ascii_case("acceptance criteria") {
            return Some(Heading {
                kind: HeadingKind::AcceptanceCriteria,
            });
        }
        if let Some(r) = strip_ci_prefix(body, "requirement ") {
            if let Some((no, title)) = split_number_title(r) {
                return Some(Heading {
                    kind: HeadingKind::Requirement { no, title },
                });
            }
        }
        if let Some(r) = strip_ci_prefix(body, "amendment ") {
            if let Some((no, title, extends)) = split_amendment_title(r) {
                return Some(Heading {
                    kind: HeadingKind::Amendment { no, title, extends },
                });
            }
        }
        // `### R1 — <title>` (G2): a bare `R<integer>` followed by a real title separator.
        if let Some(r) = strip_ci_prefix(body, "r") {
            if let Some((no, title)) = split_r_title(r) {
                return Some(Heading {
                    kind: HeadingKind::Requirement { no, title },
                });
            }
        }
        Some(Heading {
            kind: HeadingKind::Other,
        })
    }
}

/// Case-insensitive prefix strip returning the remainder.
fn strip_ci_prefix<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    let head = s.get(..prefix.len())?;
    if head.eq_ignore_ascii_case(prefix) {
        Some(&s[prefix.len()..])
    } else {
        None
    }
}

/// Split `<int>[:.—-] <title>` for G1 headings. The separator may be a colon, em/en dash, or hyphen.
fn split_number_title(rest: &str) -> Option<(String, String)> {
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return None;
    }
    let after = &rest[digits.len()..];
    let title = after
        .trim_start()
        .trim_start_matches([':', '—', '–', '-'])
        .trim();
    Some((digits, title.to_string()))
}

/// Split `1 — <title>` for G2 headings, where the number follows the `R`. Requires a real
/// separator (dash/colon) so a bare `# R1 discussion` is not treated as a requirement heading.
fn split_r_title(rest: &str) -> Option<(String, String)> {
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return None;
    }
    let after = rest[digits.len()..].trim_start();
    if !(after.starts_with('—')
        || after.starts_with('–')
        || after.starts_with(':')
        || after.starts_with("- "))
    {
        return None;
    }
    let title = after.trim_start_matches([':', '—', '–', '-']).trim();
    Some((digits, title.to_string()))
}

/// Split `A1 — <title> (extends Requirement 4 / Requirement 12)` for G3 headings, extracting the
/// amendment id, the title, and the list of extended requirement numbers.
fn split_amendment_title(rest: &str) -> Option<(String, String, Vec<String>)> {
    let mut chars = rest.chars();
    if !matches!(chars.next(), Some('A') | Some('a')) {
        return None;
    }
    let digits: String = chars
        .as_str()
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    if digits.is_empty() {
        return None;
    }
    let no = format!("A{digits}");
    let after = &rest[1 + digits.len()..];
    let title_full = after
        .trim_start()
        .trim_start_matches([':', '—', '–', '-'])
        .trim();

    let mut extends = Vec::new();
    let mut title = title_full.to_string();
    if let Some(open) = title_full.find("(extends") {
        let close = title_full[open..].find(')').map(|i| open + i);
        if let Some(close) = close {
            let inner = &title_full[open + "(extends".len()..close];
            for tok in inner.split(['/', ',', '&']) {
                let t = tok.trim();
                if let Some(num) = strip_ci_prefix(t, "requirement ") {
                    extends.push(num.trim().to_string());
                } else if !t.is_empty() {
                    extends.push(t.to_string());
                }
            }
            title = title_full[..open].trim().to_string();
        }
    }
    Some((no, title, extends))
}

/// Recognize the first line of a criterion and return its number and the text following the marker.
///
/// Two marker shapes are accepted, gated differently:
///
/// * **Dotted self-identifying** — `1.1 <text>` or `- R1.1 <text>`. The marker repeats the parent
///   requirement number, so it is unambiguous and accepted anywhere in the requirement block. The
///   returned criterion number is the part after the dot (`1` from `1.1`).
/// * **Bare numbered** — `1. <text>` or `1) <text>`. Accepted only when `bare_ok` is true (the
///   caller sets it for an Acceptance Criteria region or an amendment block), because a bare
///   `1. ` is otherwise ordinary prose numbering.
fn parse_criterion_marker(
    trimmed: &str,
    requirement_no: &str,
    bare_ok: bool,
) -> Option<(String, String)> {
    // Bulleted form: `- <marker> <text>`. The marker may be `R1.1` (G2) or a bare dotted `1.1`.
    for bullet in ["- ", "* ", "+ "] {
        if let Some(rest) = trimmed.strip_prefix(bullet) {
            let body = strip_ci_prefix(rest, "r").unwrap_or(rest);
            let (label, after) = take_dotted(body);
            if is_dotted_of(&label, requirement_no)
                && after.chars().next().is_some_and(char::is_whitespace)
            {
                return Some((criterion_part(&label).to_string(), after.trim().to_string()));
            }
            return None; // a bullet that is not a criterion marker is not a criterion
        }
    }

    // Numbered form. First try the dotted self-identifying shape `1.1 <text>`.
    let (label, after) = take_dotted(trimmed);
    if is_dotted_of(&label, requirement_no) && after.starts_with(' ') {
        return Some((criterion_part(&label).to_string(), after.trim().to_string()));
    }

    // Then the bare numbered shape `1. <text>` / `1) <text>`, gated on `bare_ok`.
    if bare_ok {
        let digits: String = trimmed.chars().take_while(|c| c.is_ascii_digit()).collect();
        if !digits.is_empty() {
            let rest = &trimmed[digits.len()..];
            if let Some(text) = rest.strip_prefix(". ") {
                return Some((digits, text.trim().to_string()));
            }
            if let Some(text) = rest.strip_prefix(") ") {
                return Some((digits, text.trim().to_string()));
            }
        }
    }
    None
}

/// Whether a dotted label (`1.1`) belongs to the given requirement number (`1`).
fn is_dotted_of(label: &str, requirement_no: &str) -> bool {
    label
        .split_once('.')
        .is_some_and(|(req, _crit)| req == requirement_no)
}

/// The criterion number is the part of a dotted label after the dot (`1` from `1.1`).
fn criterion_part(label: &str) -> &str {
    label.split_once('.').map(|(_, c)| c).unwrap_or(label)
}

/// Take a dotted `1.1` label from the front of a string, returning `(label, rest)`. The label must
/// have digits on both sides of a single dot; otherwise returns an empty label and the input.
fn take_dotted(s: &str) -> (String, &str) {
    let bytes = s.as_bytes();
    let mut end = 0;
    let mut seen_dot = false;
    while end < bytes.len() {
        let c = bytes[end] as char;
        if c.is_ascii_digit() {
            end += 1;
        } else if c == '.' && !seen_dot {
            seen_dot = true;
            end += 1;
        } else {
            break;
        }
    }
    let candidate = &s[..end];
    if !seen_dot || candidate.ends_with('.') || candidate.starts_with('.') {
        return (String::new(), s);
    }
    (candidate.to_string(), &s[end..])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn parse(text: &str) -> (Vec<CcsddRequirement>, Vec<ParseNote>) {
        Parser::new("demo", Path::new("requirements.md")).parse(text)
    }

    #[test]
    fn g1_canonical_numbered_criteria() {
        let text = "\
### Requirement 1: Alpha start
#### Acceptance Criteria
1. When the start event occurs, the Alpha Service shall record the start.
2. The Alpha Service shall expose the recorded start.
";
        let (reqs, notes) = parse(text);
        assert!(notes.is_empty());
        assert_eq!(
            reqs.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(),
            ["1.1", "1.2"]
        );
        assert_eq!(reqs[0].kind, ReqKind::Baseline);
        assert!(reqs[0].qualified().starts_with("demo:"));
    }

    #[test]
    fn g2_r_style_strips_the_r_prefix() {
        let text = "\
### R1 — Bravo pulse
- R1.1 The Bravo Service shall emit a pulse.
";
        let (reqs, notes) = parse(text);
        assert!(notes.is_empty());
        assert_eq!(reqs.len(), 1);
        assert_eq!(reqs[0].id, "1.1");
        assert_eq!(reqs[0].requirement_no, "1");
    }

    #[test]
    fn g3_amendment_extends_and_keeps_parent_feature() {
        let text = "\
### Requirement 1: Baseline
#### Acceptance Criteria
1. The Ported Service shall record a decision.

### Amendment A1 — Extra logging (extends Requirement 1)
1. The Ported Service shall also log the decision.
";
        let (reqs, notes) = parse(text);
        assert!(notes.is_empty());
        assert_eq!(reqs.len(), 2);
        assert_eq!(reqs[1].id, "A1.1");
        assert_eq!(reqs[1].kind, ReqKind::Amendment);
        assert_eq!(reqs[1].extends, ["1"]);
        assert_eq!(reqs[1].feature, "demo");
    }

    #[test]
    fn g4_empty_document_is_not_an_error() {
        let text =
            "# Requirements Document\n\nStatus: reserved. No requirements are written yet.\n";
        let (reqs, notes) = parse(text);
        assert!(reqs.is_empty());
        assert!(notes.is_empty());
    }

    #[test]
    fn baseline_heading_without_criteria_is_noted() {
        let text = "\
### Requirement 1: Title only

### Requirement 2: With a criterion
#### Acceptance Criteria
1. The Partial Service shall beep.
";
        let (reqs, notes) = parse(text);
        assert_eq!(reqs.len(), 1);
        assert_eq!(reqs[0].id, "2.1");
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].code, "REQ_NO_CRITERIA");
        assert_eq!(notes[0].line, 1);
    }

    #[test]
    fn fenced_examples_are_not_criteria() {
        let text = "\
### Requirement 1: Real obligation
#### Acceptance Criteria
1. The Illustrated Service shall beep.

```markdown
### Requirement 99: Fenced example
#### Acceptance Criteria
1. The Illustrated Service shall ignore this example.
```
";
        let (reqs, notes) = parse(text);
        assert!(notes.is_empty());
        assert_eq!(reqs.len(), 1);
        assert_eq!(reqs[0].id, "1.1");
    }
}
