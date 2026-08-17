//! Grounded vocabulary and intentions.
//!
//! One rule carries most of the value: a tag or intention is declared before it is used, and an
//! undeclared one fails the gate. That turns vocabulary drift from a code-review argument into a
//! deterministic failure, which is what "grounding" actually buys a small team.
//!
//! There is no reasoner here and none is needed. The three things a description logic would be
//! reached for all collapse: subsumption over `broader` is transitive closure, equivalence is
//! `alt_labels`, and disjointness belongs to the constraint layer. The standard property this
//! mirrors is itself explicitly non-transitive, so computing the closure here is the conformant
//! reading rather than a shortcut.
//!
//! Term identity is a join key, which is why a collision between two declarations of the same
//! identifier is an error rather than a merge. Anything that later compares requirements across
//! features depends on two specifications meaning the same thing by the same name.

use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::report::{Finding, Severity};

pub const PROJECT_VOCABULARY: &str = ".specify/vocabulary.toml";
pub const PROJECT_INTENTIONS: &str = ".specify/intentions.toml";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Domain {
    Entity,
    Bool,
    Enum { values: Vec<String> },
    Int { min: i64, max: i64 },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Term {
    pub label: String,
    pub definition: String,
    pub domain: Domain,
    #[serde(default)]
    pub broader: Vec<String>,
    #[serde(default)]
    pub alt_labels: Vec<String>,
    #[serde(default)]
    pub deprecated: bool,
    #[serde(default)]
    pub replaced_by: Option<String>,
}

#[derive(Debug, Deserialize)]
struct VocabularyFile {
    #[serde(default)]
    terms: BTreeMap<String, Term>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Intention {
    #[allow(dead_code)]
    statement: String,
    #[allow(dead_code)]
    rationale: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Precedence {
    over: String,
    under: String,
    #[serde(default)]
    #[allow(dead_code)]
    reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct IntentionsFile {
    #[serde(default)]
    intentions: BTreeMap<String, Intention>,
    #[serde(default)]
    precedence: Vec<Precedence>,
}

/// What a requirement claims, gathered from its traceability entry.
pub struct Mapping {
    pub feature: String,
    pub requirement: String,
    pub tags: Vec<String>,
    pub intent: Option<String>,
    /// The traceability file the claim came from, so findings point at something editable.
    pub source: String,
}

/// Labels differ only by case and spacing far more often than by meaning.
fn normalized_label(label: &str) -> String {
    label
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

fn read_toml<T: serde::de::DeserializeOwned>(
    path: &Path,
    display: &str,
    code: &str,
) -> Result<Option<T>, Box<Finding>> {
    // Boxed: a Finding is large enough that returning one by value in the error position makes
    // every call site pay for the failure path.
    if !path.is_file() {
        return Ok(None);
    }
    let text = std::fs::read_to_string(path)
        .map_err(|error| Box::new(Finding::new(code, error.to_string(), display.to_string())))?;
    toml::from_str(&text).map(Some).map_err(|error| {
        Box::new(Finding::new(
            code,
            error.message().to_string(),
            display.to_string(),
        ))
    })
}

pub fn validate(
    root: &Path,
    mappings: &[Mapping],
    feature_dirs: &[(String, &Path)],
) -> Vec<Finding> {
    let (terms, mut findings) = load_terms(root, feature_dirs);
    findings.extend(check_labels(&terms));
    findings.extend(check_broader(&terms));
    findings.extend(check_tags(&terms, mappings));
    findings.extend(check_intentions(root, mappings));
    findings
}

/// The merged vocabulary, project-level then feature-local.
///
/// Split out of `validate` because the constraint model needs the same term map to know each
/// term's domain, and reading the files twice would let the two layers disagree about what was
/// declared.
pub fn load_terms(
    root: &Path,
    feature_dirs: &[(String, &Path)],
) -> (BTreeMap<String, (Term, String)>, Vec<Finding>) {
    let mut findings = Vec::new();
    let mut terms: BTreeMap<String, (Term, String)> = BTreeMap::new();

    // Project vocabulary first, then any feature-local one. A feature may add terms only it uses;
    // redeclaring a project term with a different domain is the drift this file exists to prevent.
    let mut sources: Vec<(String, std::path::PathBuf)> = vec![(
        PROJECT_VOCABULARY.to_string(),
        root.join(".specify").join("vocabulary.toml"),
    )];
    for (feature, directory) in feature_dirs {
        sources.push((
            format!("{feature}/vocabulary.toml"),
            directory.join("vocabulary.toml"),
        ));
    }

    for (display, path) in &sources {
        match read_toml::<VocabularyFile>(path, display, "VOCAB_INVALID") {
            Err(finding) => findings.push(*finding),
            Ok(None) => {}
            Ok(Some(file)) => {
                for (identifier, term) in file.terms {
                    if let Some((existing, first_seen)) = terms.get(&identifier) {
                        if existing.domain != term.domain {
                            findings.push(Finding::new(
                                "TERM_COLLISION",
                                format!(
                                    "Term `{identifier}` is declared with a different domain in \
                                     {first_seen}; a term with two meanings cannot be a join key."
                                ),
                                display.clone(),
                            ));
                        }
                        continue;
                    }
                    if term.definition.trim().is_empty() {
                        findings.push(Finding::new(
                            "TERM_NO_DEFINITION",
                            format!(
                                "Term `{identifier}` has no definition; a label without one is a \
                                 tag pretending to be a concept."
                            ),
                            display.clone(),
                        ));
                    }
                    terms.insert(identifier, (term, display.clone()));
                }
            }
        }
    }

    (terms, findings)
}

fn check_labels(terms: &BTreeMap<String, (Term, String)>) -> Vec<Finding> {
    let mut seen: BTreeMap<String, String> = BTreeMap::new();
    let mut findings = Vec::new();
    for (identifier, (term, display)) in terms {
        for label in std::iter::once(&term.label).chain(term.alt_labels.iter()) {
            let key = normalized_label(label);
            if key.is_empty() {
                continue;
            }
            match seen.get(&key) {
                Some(other) if other != identifier => findings.push(Finding::new(
                    "TERM_DUPLICATE_LABEL",
                    format!("Terms `{other}` and `{identifier}` share the label \"{label}\"."),
                    display.clone(),
                )),
                _ => {
                    seen.insert(key, identifier.clone());
                }
            }
        }
    }
    findings
}

fn check_broader(terms: &BTreeMap<String, (Term, String)>) -> Vec<Finding> {
    let mut findings = Vec::new();
    for (identifier, (term, display)) in terms {
        for parent in &term.broader {
            if !terms.contains_key(parent) {
                findings.push(Finding::new(
                    "TERM_BROADER_UNDECLARED",
                    format!(
                        "Term `{identifier}` is broadened by `{parent}`, which is not declared."
                    ),
                    display.clone(),
                ));
            }
        }
    }
    // Depth-first cycle detection over the broader relation. Reported once per participating term
    // so the message is a work list rather than a single opaque "there is a cycle somewhere".
    for identifier in terms.keys() {
        if reaches_itself(identifier, terms) {
            let display = &terms[identifier].1;
            findings.push(Finding::new(
                "VOCAB_CYCLE",
                format!("Term `{identifier}` is transitively broader than itself."),
                display.clone(),
            ));
        }
    }
    findings
}

fn reaches_itself(start: &str, terms: &BTreeMap<String, (Term, String)>) -> bool {
    let mut stack: Vec<&str> = terms
        .get(start)
        .map(|(term, _)| term.broader.iter().map(String::as_str).collect())
        .unwrap_or_default();
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    while let Some(current) = stack.pop() {
        if current == start {
            return true;
        }
        if !seen.insert(current) {
            continue;
        }
        if let Some((term, _)) = terms.get(current) {
            stack.extend(term.broader.iter().map(String::as_str));
        }
    }
    false
}

fn check_tags(terms: &BTreeMap<String, (Term, String)>, mappings: &[Mapping]) -> Vec<Finding> {
    let mut findings = Vec::new();
    let mut used: BTreeSet<&str> = BTreeSet::new();

    for mapping in mappings {
        for tag in &mapping.tags {
            let Some((term, _)) = terms.get(tag) else {
                findings.push(
                    Finding::new(
                        "TERM_UNDECLARED",
                        format!("Tag `{tag}` is not declared in the vocabulary."),
                        mapping.source.clone(),
                    )
                    .feature(&mapping.feature)
                    .requirement(&mapping.requirement),
                );
                continue;
            };
            used.insert(tag.as_str());
            if term.deprecated {
                let replacement = term
                    .replaced_by
                    .as_deref()
                    .map(|to| format!(" Use `{to}` instead."))
                    .unwrap_or_default();
                findings.push(
                    Finding::new(
                        "TERM_DEPRECATED",
                        format!("Tag `{tag}` is deprecated.{replacement}"),
                        mapping.source.clone(),
                    )
                    .feature(&mapping.feature)
                    .requirement(&mapping.requirement)
                    .severity(Severity::Warning),
                );
            }
        }
    }

    // A term reached through `broader` from a tagged term is in use, even though no requirement
    // names it directly -- it is what the tagged term means. Without the closure every interior
    // node of a hierarchy is reported as unused, and the advisory becomes noise people filter out.
    // This is where the transitive closure earns its place rather than being decoration.
    let mut reachable: BTreeSet<String> = used.iter().map(|tag| tag.to_string()).collect();
    let mut frontier: Vec<String> = reachable.iter().cloned().collect();
    while let Some(current) = frontier.pop() {
        let Some((term, _)) = terms.get(&current) else {
            continue;
        };
        for parent in &term.broader {
            if reachable.insert(parent.clone()) {
                frontier.push(parent.clone());
            }
        }
    }

    // Advisory rather than an error. Without it a vocabulary decays into singletons that ground
    // nothing; as an error it would block work for a bookkeeping problem.
    for identifier in terms.keys() {
        if !reachable.contains(identifier.as_str()) {
            findings.push(
                Finding::new(
                    "TERM_UNUSED",
                    format!("Term `{identifier}` is declared but no requirement uses it."),
                    terms[identifier].1.clone(),
                )
                .severity(Severity::Advisory),
            );
        }
    }
    findings
}

/// Declared precedence between intentions, as `(over, under)` pairs.
///
/// Exposed because the constraint layer needs it to decide whether a conflict has been adjudicated,
/// and reading the file twice would let the two layers disagree about what was declared.
pub fn load_precedence(root: &Path) -> BTreeSet<(String, String)> {
    let path = root.join(".specify").join("intentions.toml");
    match read_toml::<IntentionsFile>(&path, PROJECT_INTENTIONS, "INTENT_INVALID") {
        Ok(Some(file)) => file
            .precedence
            .iter()
            .map(|edge| (edge.over.clone(), edge.under.clone()))
            .collect(),
        _ => BTreeSet::new(),
    }
}

fn check_intentions(root: &Path, mappings: &[Mapping]) -> Vec<Finding> {
    let mut findings = Vec::new();
    let path = root.join(".specify").join("intentions.toml");
    let file = match read_toml::<IntentionsFile>(&path, PROJECT_INTENTIONS, "INTENT_INVALID") {
        Err(finding) => return vec![*finding],
        Ok(None) => IntentionsFile {
            intentions: BTreeMap::new(),
            precedence: Vec::new(),
        },
        Ok(Some(file)) => file,
    };

    let mut served: BTreeSet<&str> = BTreeSet::new();
    for mapping in mappings {
        let Some(intent) = mapping.intent.as_deref() else {
            continue;
        };
        if file.intentions.contains_key(intent) {
            served.insert(intent);
        } else {
            findings.push(
                Finding::new(
                    "INTENT_UNDECLARED",
                    format!("Intention `{intent}` is not declared."),
                    mapping.source.clone(),
                )
                .feature(&mapping.feature)
                .requirement(&mapping.requirement),
            );
        }
    }

    for name in file.intentions.keys() {
        if !served.contains(name.as_str()) {
            findings.push(
                Finding::new(
                    "INTENT_UNSERVED",
                    format!("Intention `{name}` is declared but no requirement serves it."),
                    PROJECT_INTENTIONS.to_string(),
                )
                .severity(Severity::Advisory),
            );
        }
    }

    for edge in &file.precedence {
        for endpoint in [&edge.over, &edge.under] {
            if !file.intentions.contains_key(endpoint) {
                findings.push(Finding::new(
                    "INTENT_PRECEDENCE_UNDECLARED",
                    format!("Precedence names `{endpoint}`, which is not a declared intention."),
                    PROJECT_INTENTIONS.to_string(),
                ));
            }
        }
    }
    findings.extend(check_precedence_cycles(&file.precedence));
    findings
}

/// Precedence has to be a strict partial order. A cycle means the declaration resolves nothing,
/// which is worse than leaving the pair unordered: unordered honestly records that nobody decided.
fn check_precedence_cycles(edges: &[Precedence]) -> Vec<Finding> {
    let mut graph: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for edge in edges {
        graph
            .entry(edge.over.as_str())
            .or_default()
            .push(edge.under.as_str());
    }
    let mut findings = Vec::new();
    for start in graph.keys() {
        let mut stack: Vec<&str> = graph.get(start).cloned().unwrap_or_default();
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        while let Some(current) = stack.pop() {
            if current == *start {
                findings.push(Finding::new(
                    "INTENT_PRECEDENCE_CYCLE",
                    format!("Intention `{start}` transitively takes precedence over itself."),
                    PROJECT_INTENTIONS.to_string(),
                ));
                break;
            }
            if !seen.insert(current) {
                continue;
            }
            stack.extend(graph.get(current).cloned().unwrap_or_default());
        }
    }
    findings
}

/// Propose vocabulary stubs from existing specification prose.
///
/// A blank vocabulary file is the most likely way this feature dies in adoption, so the tool pays
/// the authoring cost rather than the author. The candidates are the subjects of existing EARS
/// requirements -- the thing each one is *about* -- plus anything already written in backticks,
/// which is how authors mark a term they consider significant.
///
/// Definitions are emitted empty on purpose. An empty definition fails the gate, so a scaffold
/// cannot be committed unread; the alternative is a file of plausible-looking terms nobody has
/// actually agreed on.
pub fn scaffold(requirements: &[crate::requirements::Requirement]) -> String {
    let mut candidates: BTreeMap<String, String> = BTreeMap::new();

    for requirement in requirements {
        for phrase in subject_phrases(&requirement.text) {
            let slug = slugify(&phrase);
            if slug.len() >= 3 {
                candidates.entry(slug).or_insert(phrase);
            }
        }
    }

    let mut out = String::from(
        "# Proposed vocabulary, derived from existing specification prose.\n\
         #\n\
         # Every definition is empty and every empty definition fails the gate. That is deliberate:\n\
         # a scaffold you can commit without reading is a scaffold that grounds nothing. Delete the\n\
         # terms that are not real concepts, then write a sentence for each one that survives.\n\n\
         schema_version = \"1.0\"\n",
    );
    for (slug, label) in &candidates {
        out.push_str(&format!(
            "\n[terms.{slug}]\nlabel = \"{label}\"\ndefinition = \"\"\ndomain = {{ kind = \"entity\" }}\n"
        ));
    }
    if candidates.is_empty() {
        out.push_str("\n# No candidates found. Requirements may not be in EARS form yet.\n");
    }
    out
}

/// The noun phrase a requirement is about: what stands between the EARS condition and `shall`,
/// plus any backticked span anywhere in the sentence.
fn subject_phrases(sentence: &str) -> Vec<String> {
    let mut phrases = Vec::new();

    let lower = sentence.to_lowercase();
    if let Some(shall) = lower.find(" shall ") {
        let head = &sentence[..shall];
        // Drop the leading EARS clause, which describes the trigger rather than the subject.
        let subject = head.rsplit(", then ").next().unwrap_or(head);
        let subject = subject.rsplit(',').next().unwrap_or(subject);
        let cleaned = subject
            .trim()
            .trim_start_matches("The ")
            .trim_start_matches("the ")
            .trim();
        if !cleaned.is_empty() && cleaned.split_whitespace().count() <= 6 {
            phrases.push(cleaned.to_string());
        }
    }

    let mut rest = sentence;
    while let Some(open) = rest.find('`') {
        let after = &rest[open + 1..];
        let Some(close) = after.find('`') else { break };
        let span = after[..close].trim();
        if !span.is_empty() && span.split_whitespace().count() <= 6 {
            phrases.push(span.to_string());
        }
        rest = &after[close + 1..];
    }

    phrases
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut previous_dash = true;
    for character in value.chars() {
        if character.is_alphanumeric() {
            slug.extend(character.to_lowercase());
            previous_dash = false;
        } else if !previous_dash {
            slug.push('-');
            previous_dash = true;
        }
    }
    slug.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::requirements::Requirement;
    use std::path::PathBuf;

    fn requirement(text: &str) -> Requirement {
        Requirement {
            // Not a real identifier: crates/ is a production root, and Rust keeps unit tests in the
            // same file as the code they test, so the separation gate cannot tell them apart.
            identifier: "example".to_string(),
            text: text.to_string(),
            feature: "specs/001-alpha".to_string(),
            path: PathBuf::from("spec.md"),
            line: 1,
        }
    }

    #[test]
    fn scaffold_proposes_terms_from_prose() {
        let requirements = vec![
            requirement("When the digest differs, the workstation manager shall stop."),
            requirement("The `captured policy` shall be pinned with its digest."),
        ];
        let out = scaffold(&requirements);

        // the subject of an event-driven requirement, taken from after the condition clause
        assert!(out.contains("[terms.workstation-manager]"), "{out}");
        // an author-marked term in backticks
        assert!(out.contains("[terms.captured-policy]"), "{out}");
        // definitions are empty on purpose, so a scaffold cannot be committed unread
        assert!(out.contains("definition = \"\""), "{out}");
    }

    #[test]
    fn scaffold_says_so_when_it_finds_nothing() {
        assert!(scaffold(&[]).contains("No candidates found"));
    }
}
