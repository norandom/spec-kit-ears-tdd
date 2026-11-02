//! The envelope interchange format (ears-sdd-envelope/1.0): a phase machine whose per-phase tool
//! allowlists and guarded transitions become containment effects.
//!
//! This is the "operating envelope the agent runs under" from the cited OpenShell Research dev
//! note: instead of asking whether a policy change is safe in prose, the envelope is encoded and
//! the containment query proves a candidate stays inside the approved reference. Here the
//! envelope is the validator's own interchange format; a native runtime adapter lands with
//! feature 010 task T009 from a checked-in sample.
//!
//! Derived effects, per the contract:
//! - tool `Read` permitted by phase `planning`  -> effect `tool:Read`, guard `envelope-phase == 'planning'`
//! - transition `planning` -> `implement`        -> effect `transition:planning:implement`,
//!   guard = source phase AND the declared transition guard
//!
//! Unknown fields are recorded by JSON-pointer path and reported as unmodelled constructs; they
//! never disable modelled parts. Structural invalidity withholds the whole definition.

use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

use crate::guard::{self, Guard, Literal, Op};
use crate::model::{EffectDeclaration, ModelFile, ModelledRequirement, RequirementEntry};
use crate::report::Finding;
use crate::vocabulary::{Domain, Term};

/// Reserved enum term derived from the phase names. Collisions with an envelope vocabulary term
/// of the same name are invalid.
pub const PHASE_TERM: &str = "envelope-phase";

#[derive(Debug, Deserialize)]
struct EnvelopeFile {
    schema_version: String,
    id: String,
    initial: String,
    phases: BTreeMap<String, PhaseDef>,
    #[serde(flatten)]
    extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct PhaseDef {
    allowed_tools: Vec<String>,
    #[serde(default)]
    transitions: Vec<TransitionDef>,
    #[serde(flatten)]
    extra: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct TransitionDef {
    to: String,
    #[serde(default)]
    when: Option<String>,
    #[serde(flatten)]
    extra: BTreeMap<String, serde_json::Value>,
}

/// What the reader produced: a model-shaped side plus the unmodelled JSON-pointer paths.
pub struct EnvelopeSide {
    pub model: ModelFile,
    pub modelled: Vec<ModelledRequirement>,
    pub terms: BTreeMap<String, (Term, String)>,
    pub unmodelled: Vec<String>,
    /// Non-fatal guard problems (undeclared term, invalid literal): the affected transition
    /// effect was withheld, but the rest of the definition still compares.
    pub findings: Vec<Finding>,
}

fn invalid(message: String, display: &str) -> Finding {
    Finding::new(
        "CONTAIN_DEFINITION_UNREADABLE",
        message,
        display.to_string(),
    )
}

fn phase_name_ok(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
}

/// Check one guard against the envelope vocabulary: undeclared terms and out-of-domain enum
/// literals are errors, collected so the whole transition effect can be withheld at once.
fn check_guard(
    guard: &Guard,
    terms: &BTreeMap<String, (Term, String)>,
    display: &str,
) -> Vec<Finding> {
    fn walk(
        g: &Guard,
        terms: &BTreeMap<String, (Term, String)>,
        display: &str,
        findings: &mut Vec<Finding>,
    ) {
        match g {
            Guard::Always => {}
            Guard::Term(name) => {
                if !terms.contains_key(name) {
                    findings.push(Finding::new(
                        "CONTAIN_UNDECLARED_TERM",
                        format!("Envelope guard references undeclared term `{name}`."),
                        display.to_string(),
                    ));
                }
            }
            Guard::Compare { term, value, .. } => match (terms.get(term), value) {
                (None, _) => findings.push(Finding::new(
                    "CONTAIN_UNDECLARED_TERM",
                    format!("Envelope guard references undeclared term `{term}`."),
                    display.to_string(),
                )),
                (Some((declaration, _)), Literal::Text(text)) => {
                    if let Domain::Enum { values } = &declaration.domain {
                        if !values.contains(text) {
                            findings.push(Finding::new(
                                "CONTAIN_GUARD_INVALID",
                                format!(
                                    "`{term}` compared against '{text}', which is not one of its \
                                     declared values."
                                ),
                                display.to_string(),
                            ));
                        }
                    }
                }
                (Some((declaration, _)), Literal::Int(number)) => {
                    if let Domain::Int { min, max } = &declaration.domain {
                        if *number < *min || *number > *max {
                            findings.push(Finding::new(
                                "CONTAIN_GUARD_INVALID",
                                format!(
                                    "`{term}` compared against {number}, outside its declared range \
                                     [{min}, {max}]."
                                ),
                                display.to_string(),
                            ));
                        }
                    }
                }
                _ => {}
            },
            Guard::Not(inner) => walk(inner, terms, display, findings),
            Guard::And(parts) | Guard::Or(parts) => {
                for part in parts {
                    walk(part, terms, display, findings);
                }
            }
        }
    }
    let mut findings = Vec::new();
    walk(guard, terms, display, &mut findings);
    findings
}

/// Read and validate one envelope definition, deriving effects and the reserved phase term.
/// `vocab_terms` is the envelope vocabulary (already loaded); the reserved `envelope-phase`
/// enum is added to it.
pub fn read_definition(
    text: &str,
    mut vocab_terms: BTreeMap<String, (Term, String)>,
    display: &str,
) -> Result<EnvelopeSide, Vec<Finding>> {
    let file: EnvelopeFile = match serde_json::from_str(text) {
        Ok(file) => file,
        Err(error) => {
            return Err(vec![invalid(
                format!("Envelope definition does not parse as JSON: {error}"),
                display,
            )])
        }
    };
    if file.schema_version != "1.0" {
        return Err(vec![invalid(
            format!(
                "Unsupported envelope schema_version `{}`.",
                file.schema_version
            ),
            display,
        )]);
    }
    if file.phases.is_empty() {
        return Err(vec![invalid(
            "Envelope declares no phases.".to_string(),
            display,
        )]);
    }
    if file.id.trim().is_empty() {
        return Err(vec![invalid(
            "Envelope declares an empty `id`.".to_string(),
            display,
        )]);
    }
    if !file.phases.contains_key(&file.initial) {
        return Err(vec![invalid(
            format!(
                "`initial` names phase `{}`, which does not exist.",
                file.initial
            ),
            display,
        )]);
    }
    for name in file.phases.keys() {
        if !phase_name_ok(name) {
            return Err(vec![invalid(
                format!("Phase name `{name}` must match [a-z][a-z0-9_-]*."),
                display,
            )]);
        }
    }
    if vocab_terms.contains_key(PHASE_TERM) {
        return Err(vec![invalid(
            format!("Envelope vocabulary collides with the reserved term `{PHASE_TERM}`."),
            display,
        )]);
    }

    let mut unmodelled: Vec<String> = Vec::new();
    for key in file.extra.keys() {
        unmodelled.push(format!("/{key}"));
    }

    // Validate structure: tools and transition endpoints.
    for (phase, def) in &file.phases {
        let mut seen_tools: BTreeSet<&str> = BTreeSet::new();
        for tool in &def.allowed_tools {
            if tool.trim().is_empty() {
                return Err(vec![invalid(
                    format!("Phase `{phase}` declares an empty tool name."),
                    display,
                )]);
            }
            if !seen_tools.insert(tool.as_str()) {
                return Err(vec![invalid(
                    format!("Phase `{phase}` lists tool `{tool}` twice."),
                    display,
                )]);
            }
        }
        for key in def.extra.keys() {
            unmodelled.push(format!("/phases/{phase}/{key}"));
        }
        for (index, transition) in def.transitions.iter().enumerate() {
            if !file.phases.contains_key(&transition.to) {
                return Err(vec![invalid(
                    format!(
                        "Transition from `{phase}` names destination `{}`, which does not exist.",
                        transition.to
                    ),
                    display,
                )]);
            }
            for key in transition.extra.keys() {
                unmodelled.push(format!("/phases/{phase}/transitions/{index}/{key}"));
            }
        }
    }
    unmodelled.sort();

    // Reserved phase enum, canonically ordered.
    let mut phases: Vec<String> = file.phases.keys().cloned().collect();
    phases.sort();
    vocab_terms.insert(
        PHASE_TERM.to_string(),
        (
            Term {
                label: "Envelope phase".to_string(),
                definition: "Reserved enum of the envelope's phase names.".to_string(),
                domain: Domain::Enum {
                    values: phases.clone(),
                },
                broader: Vec::new(),
                alt_labels: Vec::new(),
                deprecated: false,
                replaced_by: None,
            },
            display.to_string(),
        ),
    );

    // Derive effects and modelled requirements.
    let mut effects: BTreeMap<String, EffectDeclaration> = BTreeMap::new();
    let mut requirements: BTreeMap<String, RequirementEntry> = BTreeMap::new();
    let mut modelled: Vec<ModelledRequirement> = Vec::new();
    let mut findings: Vec<Finding> = Vec::new();

    let phase_guard = |phase: &str| Guard::Compare {
        term: PHASE_TERM.to_string(),
        op: Op::Equal,
        value: Literal::Text(phase.to_string()),
    };

    for phase in &phases {
        let def = &file.phases[phase];
        let phase_id = format!("PHASE-{phase}");
        for tool in &def.allowed_tools {
            let effect = format!("tool:{tool}");
            effects.insert(
                effect.clone(),
                EffectDeclaration {
                    conflicts_with: Vec::new(),
                },
            );
            modelled.push(ModelledRequirement {
                identifier: phase_id.clone(),
                feature: display.to_string(),
                guard: phase_guard(phase),
                effect,
            });
        }
        requirements.insert(
            phase_id.clone(),
            RequirementEntry {
                when: None,
                then: format!("phase:{phase}"),
            },
        );
        for transition in &def.transitions {
            let effect = format!("transition:{phase}:{}", transition.to);
            effects.insert(
                effect.clone(),
                EffectDeclaration {
                    conflicts_with: Vec::new(),
                },
            );
            let guard = match &transition.when {
                Some(text) => match guard::parse(text) {
                    Ok(parsed) => {
                        let problems = check_guard(&parsed, &vocab_terms, display);
                        if !problems.is_empty() {
                            findings.extend(problems);
                            continue; // withhold the affected transition effect
                        }
                        Guard::And(vec![phase_guard(phase), parsed])
                    }
                    Err(error) => {
                        findings.push(Finding::new(
                            "CONTAIN_GUARD_INVALID",
                            format!("Transition guard `{text}` does not parse: {error}"),
                            display.to_string(),
                        ));
                        continue;
                    }
                },
                None => phase_guard(phase),
            };
            modelled.push(ModelledRequirement {
                identifier: phase_id.clone(),
                feature: display.to_string(),
                guard,
                effect,
            });
        }
    }
    modelled.sort_by(|a, b| {
        (a.effect.as_str(), a.identifier.as_str()).cmp(&(b.effect.as_str(), b.identifier.as_str()))
    });

    Ok(EnvelopeSide {
        model: ModelFile {
            schema_version: Some("1.0".to_string()),
            effects,
            requirements,
            derogations: BTreeMap::new(),
        },
        modelled,
        terms: vocab_terms,
        unmodelled,
        findings,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env_vocab() -> BTreeMap<String, (Term, String)> {
        let mut terms = BTreeMap::new();
        terms.insert(
            "test-result".to_string(),
            (
                Term {
                    label: "Test result".to_string(),
                    definition: "Outcome of the last test run.".to_string(),
                    domain: Domain::Enum {
                        values: vec!["pass".to_string(), "fail".to_string()],
                    },
                    broader: Vec::new(),
                    alt_labels: Vec::new(),
                    deprecated: false,
                    replaced_by: None,
                },
                "envelope-vocabulary.toml".to_string(),
            ),
        );
        terms
    }

    const DEF: &str = r#"{
        "schema_version": "1.0",
        "id": "bugfix",
        "initial": "planning",
        "phases": {
            "planning": { "allowed_tools": ["Read"], "transitions": [{"to": "implement", "when": "test-result == 'fail'"}] },
            "implement": { "allowed_tools": ["Read", "Edit"], "transitions": [{"to": "planning", "when": "test-result == 'pass'"}] }
        }
    }"#;

    #[test]
    fn local_permissions_preserve_phase_identity() {
        // The same tool permitted in two phases is two distinct effects with phase guards.
        let side = read_definition(DEF, env_vocab(), "envelope.json").expect("parses");
        let read_effects: Vec<_> = side
            .modelled
            .iter()
            .filter(|r| r.effect == "tool:Read")
            .map(|r| format!("{:?}", r.guard))
            .collect();
        assert_eq!(read_effects.len(), 2);
        assert_ne!(read_effects[0], read_effects[1]);
        assert!(side
            .modelled
            .iter()
            .any(|r| r.effect == "transition:planning:implement"));
    }

    #[test]
    fn invalid_definitions_are_withheld() {
        // Structural invalidity withholds the whole definition.
        assert!(read_definition("{ not json", env_vocab(), "d").is_err());
        let bad_version = DEF.replace("\"1.0\"", "\"2.0\"");
        assert!(read_definition(&bad_version, env_vocab(), "d").is_err());
        let no_phases = r#"{"schema_version": "1.0", "id": "x", "initial": "a", "phases": {}}"#;
        assert!(read_definition(no_phases, env_vocab(), "d").is_err());
    }

    #[test]
    fn envelope_vocabulary_is_not_checked_for_use() {
        // A term declared only in the envelope vocabulary resolves in a guard and produces no
        // unused-term finding; the product vocabulary gate never sees this file.
        let side = read_definition(DEF, env_vocab(), "d").expect("parses");
        assert!(side.terms.contains_key("test-result"));
        assert!(side.terms.contains_key(PHASE_TERM));
        assert!(side.findings.is_empty());
    }
}
