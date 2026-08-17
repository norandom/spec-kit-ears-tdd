//! Turning components into findings.
//!
//! The budget check happens first and without enumerating anything. A component's state space is a
//! product of declared domain sizes, so it is known before any search starts; grinding through half
//! of one to report a limit would tell the reader nothing a multiplication could not.

use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::enumerate::{satisfying_sets, verify_witness};
use crate::guard::{self, Guard, Literal};
use crate::model::{decompose, Component, ModelFile, ModelledRequirement};
use crate::report::{Finding, Severity};
use crate::vocabulary::{Domain, Term};

/// Effect conflict, read symmetrically regardless of which side declared it.
///
/// Preference-based reasoning over an asymmetric attack relation is unsound, and requirement
/// contradiction genuinely is symmetric: A contradicts B exactly when B contradicts A. Reading a
/// one-sided declaration as one-sided would make the answer depend on which file was written first.
fn conflict_pairs(model: &ModelFile) -> BTreeSet<(String, String)> {
    let mut pairs = BTreeSet::new();
    for (name, declaration) in &model.effects {
        for other in &declaration.conflicts_with {
            let (low, high) = if name <= other {
                (name.clone(), other.clone())
            } else {
                (other.clone(), name.clone())
            };
            pairs.insert((low, high));
        }
    }
    pairs
}

fn conflicts(pairs: &BTreeSet<(String, String)>, left: &str, right: &str) -> bool {
    let key = if left <= right {
        (left.to_string(), right.to_string())
    } else {
        (right.to_string(), left.to_string())
    };
    pairs.contains(&key)
}

/// A guard may only use a term in a way its declared domain supports.
///
/// Without this, `evaluate` answers `false` for a mismatch and the model quietly describes
/// something nobody wrote -- a boolean test on an enumeration would simply never hold, and the
/// requirement would look dead rather than wrong.
fn check_types(
    requirement: &ModelledRequirement,
    terms: &BTreeMap<String, (Term, String)>,
    source: &str,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    let mut report = |message: String| {
        findings.push(
            Finding::new("MODEL_TYPE_MISMATCH", message, source.to_string())
                .feature(&requirement.feature)
                .requirement(&requirement.identifier),
        );
    };
    check_guard_types(&requirement.guard, terms, &mut report);
    findings
}

fn check_guard_types(
    guard: &Guard,
    terms: &BTreeMap<String, (Term, String)>,
    report: &mut impl FnMut(String),
) {
    match guard {
        Guard::Always => {}
        Guard::Term(name) => {
            if let Some((declaration, _)) = terms.get(name) {
                if !matches!(declaration.domain, Domain::Bool | Domain::Entity) {
                    report(format!(
                        "`{name}` is used as a boolean but its domain is not boolean; compare it \
                         against a value instead."
                    ));
                }
            }
        }
        Guard::Compare { term, value, .. } => {
            let Some((declaration, _)) = terms.get(term) else {
                return;
            };
            let ok = match (&declaration.domain, value) {
                (Domain::Bool | Domain::Entity, Literal::Bool(_)) => true,
                (Domain::Enum { values }, Literal::Text(text)) => {
                    if !values.contains(text) {
                        report(format!(
                            "`{term}` is compared against '{text}', which is not one of its \
                             declared values."
                        ));
                    }
                    true
                }
                (Domain::Int { min, max }, Literal::Int(number)) => {
                    if number < min || number > max {
                        report(format!(
                            "`{term}` is compared against {number}, which is outside its declared \
                             range [{min}, {max}]."
                        ));
                    }
                    true
                }
                _ => false,
            };
            if !ok {
                report(format!(
                    "`{term}` is compared against a value of the wrong kind for its declared domain."
                ));
            }
        }
        Guard::Not(inner) => check_guard_types(inner, terms, report),
        Guard::And(parts) | Guard::Or(parts) => {
            for part in parts {
                check_guard_types(part, terms, report);
            }
        }
    }
}

fn over_budget(component: &Component, budget: u64, source: &str) -> Finding {
    let states = component.state_space();
    let contributors: Vec<serde_json::Value> = component
        .largest_contributors(3)
        .into_iter()
        .map(|(term, size)| json!({ "term": term, "values": size }))
        .collect();
    Finding::new(
        "MODEL_BUDGET_EXCEEDED",
        format!(
            "Component {} has {} states against a budget of {}. Narrow the guards on its largest \
             terms, or split it so fewer terms interact; raising the budget converts a known gap \
             into an invisible one.",
            component.index, states, budget
        ),
        source.to_string(),
    )
    .detail("component", component.index as u64)
    .detail("variables", component.variables.len() as u64)
    .detail("states", states)
    .detail("budget", budget)
    .detail(
        "largest_contributors",
        serde_json::Value::Array(contributors),
    )
}

/// The contract from REQ-026: a conditional requirement belongs to exactly one component.
///
/// Unconditional ones deliberately appear in every component, because their guard always holds and
/// isolating them would lose exactly the conflicts they can take part in.
pub fn partition_is_sound(
    requirements: &[ModelledRequirement],
    components: &[Component],
) -> Result<(), String> {
    for requirement in requirements {
        let unconditional = requirement.guard.terms().is_empty();
        let appearances = components
            .iter()
            .filter(|component| {
                component
                    .requirements
                    .iter()
                    .any(|member| member.identifier == requirement.identifier)
            })
            .count();
        let expected = if unconditional { components.len() } else { 1 };
        if appearances != expected {
            return Err(format!(
                "{} appears in {appearances} components, expected {expected}",
                requirement.identifier
            ));
        }
    }
    Ok(())
}

fn witness_detail(finding: Finding, description: &str) -> Finding {
    finding.detail("witness", description.to_string())
}

fn analyze_component(
    component: &Component,
    pairs: &BTreeSet<(String, String)>,
    budget: u64,
    source: &str,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    if component.state_space() > budget {
        findings.push(over_budget(component, budget, source));
        return findings;
    }

    let sets = satisfying_sets(component);

    for (position, requirement) in component.requirements.iter().enumerate() {
        if sets[position].is_empty() {
            findings.push(
                Finding::new(
                    "MODEL_DEAD_GUARD",
                    "No assignment satisfies this requirement's guard, so it can never apply."
                        .to_string(),
                    source.to_string(),
                )
                .feature(&requirement.feature)
                .requirement(&requirement.identifier),
            );
        }
    }

    for left in 0..component.requirements.len() {
        for right in (left + 1)..component.requirements.len() {
            let (first, second) = (
                &component.requirements[left],
                &component.requirements[right],
            );
            if sets[left].is_empty() || sets[right].is_empty() {
                continue;
            }

            if conflicts(pairs, &first.effect, &second.effect) {
                if let Some(index) = sets[left].first_common(&sets[right]) {
                    match verify_witness(index, component, &[first, second]) {
                        Ok(witness) => findings.push(witness_detail(
                            Finding::new(
                                "MODEL_CONFLICT",
                                format!(
                                    "{} and {} both apply, but `{}` and `{}` cannot both hold.",
                                    first.identifier,
                                    second.identifier,
                                    first.effect,
                                    second.effect
                                ),
                                source.to_string(),
                            )
                            .feature(&first.feature)
                            .requirement(&first.identifier)
                            .detail("with", second.identifier.clone())
                            .detail("component", component.index as u64),
                            &witness.description,
                        )),
                        Err(message) => {
                            findings.push(crate::enumerate::internal_error(message, source))
                        }
                    }
                }
                continue;
            }

            if first.effect == second.effect {
                let (stricter, looser) = if sets[left].is_subset_of(&sets[right]) {
                    (first, second)
                } else if sets[right].is_subset_of(&sets[left]) {
                    (second, first)
                } else {
                    continue;
                };
                findings.push(
                    Finding::new(
                        "MODEL_SUBSUMED",
                        format!(
                            "{} applies only where {} already does, and both assert `{}`.",
                            stricter.identifier, looser.identifier, stricter.effect
                        ),
                        source.to_string(),
                    )
                    .feature(&stricter.feature)
                    .requirement(&stricter.identifier)
                    .detail("subsumed_by", looser.identifier.clone())
                    .severity(Severity::Warning),
                );
            }
        }
    }
    findings
}

pub struct Outcome {
    pub findings: Vec<Finding>,
    pub modelled: usize,
    pub components: usize,
}

/// Read a feature's model, decompose it, and analyse every component.
pub fn validate(
    root: &Path,
    feature: &str,
    directory: &Path,
    declared: &BTreeSet<String>,
    terms: &BTreeMap<String, (Term, String)>,
    budget: u64,
) -> Outcome {
    let path = directory.join("model.toml");
    let source = crate::report::relative(&path, root);
    let mut findings = Vec::new();

    let Ok(text) = std::fs::read_to_string(&path) else {
        return Outcome {
            findings,
            modelled: 0,
            components: 0,
        };
    };
    let model: ModelFile = match toml::from_str(&text) {
        Ok(model) => model,
        Err(error) => {
            findings.push(Finding::new(
                "MODEL_INVALID",
                error.message().to_string(),
                source,
            ));
            return Outcome {
                findings,
                modelled: 0,
                components: 0,
            };
        }
    };

    let mut requirements = Vec::new();
    for (identifier, entry) in &model.requirements {
        if !declared.contains(identifier) {
            findings.push(
                Finding::new(
                    "MODEL_UNKNOWN_REQ",
                    "Model entry refers to a requirement this specification does not declare."
                        .to_string(),
                    source.clone(),
                )
                .feature(feature)
                .requirement(identifier),
            );
            continue;
        }
        if !model.effects.contains_key(&entry.then) {
            findings.push(
                Finding::new(
                    "MODEL_EFFECT_UNDECLARED",
                    format!("Effect `{}` is not declared in this model.", entry.then),
                    source.clone(),
                )
                .feature(feature)
                .requirement(identifier),
            );
            continue;
        }
        let guard = match entry.when.as_deref().map(guard::parse).transpose() {
            Ok(parsed) => parsed.unwrap_or(Guard::Always),
            Err(error) => {
                findings.push(
                    Finding::new("MODEL_GUARD_INVALID", error.to_string(), source.clone())
                        .feature(feature)
                        .requirement(identifier),
                );
                continue;
            }
        };
        requirements.push(ModelledRequirement {
            identifier: identifier.clone(),
            feature: feature.to_string(),
            guard,
            effect: entry.then.clone(),
        });
    }

    for requirement in &requirements {
        findings.extend(check_types(requirement, terms, &source));
    }

    let components = decompose(&requirements, terms);
    if let Err(message) = partition_is_sound(&requirements, &components) {
        findings.push(crate::enumerate::internal_error(message, &source));
        return Outcome {
            findings,
            modelled: requirements.len(),
            components: components.len(),
        };
    }

    let pairs = conflict_pairs(&model);
    for component in &components {
        findings.extend(analyze_component(component, &pairs, budget, &source));
    }

    // Recorded rather than reported as an error: the model layer is opt-in per requirement, and a
    // partially modelled feature is a normal state to be in rather than a defect.
    for identifier in declared {
        if !model.requirements.contains_key(identifier) {
            findings.push(
                Finding::new(
                    "MODEL_UNMODELLED",
                    "Requirement has no constraint model entry and was not analysed.".to_string(),
                    source.clone(),
                )
                .feature(feature)
                .requirement(identifier)
                .severity(Severity::Advisory),
            );
        }
    }

    Outcome {
        findings,
        modelled: requirements.len(),
        components: components.len(),
    }
}

/// Exposed for the differential harness, which needs to build components without reading files.
pub fn analyze_for_test(
    component: &Component,
    pairs: &BTreeSet<(String, String)>,
    budget: u64,
) -> Vec<Finding> {
    analyze_component(component, pairs, budget, "model.toml")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{finitize, Variable};
    use crate::vocabulary::Domain;

    fn requirement(identifier: &str, when: &str, effect: &str) -> ModelledRequirement {
        ModelledRequirement {
            identifier: identifier.to_string(),
            feature: "specs/001-alpha".to_string(),
            guard: guard::parse(when).expect("test guard parses"),
            effect: effect.to_string(),
        }
    }

    fn component(variables: Vec<Variable>, requirements: Vec<ModelledRequirement>) -> Component {
        Component {
            index: 0,
            variables,
            requirements,
        }
    }

    fn boolean(term: &str) -> Variable {
        finitize(term, &Domain::Bool, &[])
    }

    fn conflicting(left: &str, right: &str) -> BTreeSet<(String, String)> {
        let mut pairs = BTreeSet::new();
        let (low, high) = if left <= right {
            (left, right)
        } else {
            (right, left)
        };
        pairs.insert((low.to_string(), high.to_string()));
        pairs
    }

    fn codes(findings: &[Finding]) -> Vec<String> {
        findings.iter().map(|f| f.code.clone()).collect()
    }

    #[test]
    fn conflicting_effects_on_overlapping_guards_are_reported_with_a_witness() {
        let component = component(
            vec![boolean("a"), boolean("b")],
            vec![
                requirement("REQ-001", "a", "persist"),
                requirement("REQ-002", "a and b", "reject"),
            ],
        );
        let findings = analyze_for_test(&component, &conflicting("persist", "reject"), 1000);

        assert_eq!(codes(&findings), vec!["MODEL_CONFLICT"]);
        let witness = findings[0].detail.as_ref().and_then(|d| d.get("witness"));
        assert!(
            witness
                .and_then(|w| w.as_str())
                .is_some_and(|w| w.contains("a = true")),
            "{witness:?}"
        );
    }

    #[test]
    fn conflicting_effects_that_never_overlap_are_not_reported() {
        // The point of guards: two contradictory effects are fine if they cannot both apply.
        let component = component(
            vec![boolean("a")],
            vec![
                requirement("REQ-001", "a", "persist"),
                requirement("REQ-002", "not a", "reject"),
            ],
        );
        assert!(analyze_for_test(&component, &conflicting("persist", "reject"), 1000).is_empty());
    }

    #[test]
    fn a_guard_that_can_never_hold_is_reported() {
        let component = component(
            vec![boolean("a")],
            vec![requirement("REQ-001", "a and not a", "persist")],
        );
        assert_eq!(
            codes(&analyze_for_test(&component, &BTreeSet::new(), 1000)),
            vec!["MODEL_DEAD_GUARD"]
        );
    }

    #[test]
    fn a_stricter_requirement_with_the_same_effect_is_subsumed() {
        let component = component(
            vec![boolean("a"), boolean("b")],
            vec![
                requirement("REQ-001", "a and b", "persist"),
                requirement("REQ-002", "a", "persist"),
            ],
        );
        let findings = analyze_for_test(&component, &BTreeSet::new(), 1000);
        assert_eq!(codes(&findings), vec!["MODEL_SUBSUMED"]);
        assert_eq!(findings[0].requirement.as_deref(), Some("REQ-001"));
    }

    #[test]
    fn an_over_budget_component_reports_the_lever_rather_than_being_evaluated() {
        let wide = Domain::Int {
            min: 0,
            max: 1_000_000,
        };
        let mut comparisons = Vec::new();
        for value in 1..40 {
            comparisons.push((crate::guard::Op::Less, Literal::Int(value * 1000)));
        }
        let variables = vec![
            finitize("depth", &wide, &comparisons),
            finitize("other", &wide, &comparisons),
            boolean("a"),
        ];
        let component = component(variables, vec![requirement("REQ-001", "a", "persist")]);

        let findings = analyze_for_test(&component, &BTreeSet::new(), 100);
        assert_eq!(codes(&findings), vec!["MODEL_BUDGET_EXCEEDED"]);

        let detail = findings[0]
            .detail
            .as_ref()
            .expect("budget findings carry detail");
        assert_eq!(detail.get("budget").and_then(|v| v.as_u64()), Some(100));
        assert!(detail.get("states").and_then(|v| v.as_u64()).unwrap() > 100);
        // The lever: which terms to narrow, largest first.
        let contributors = detail
            .get("largest_contributors")
            .and_then(|v| v.as_array())
            .unwrap();
        assert_eq!(contributors[0]["term"], "depth");
    }

    #[test]
    fn an_unconditional_requirement_joins_every_component() {
        // Two disjoint conditional requirements make two components; the unconditional one can
        // conflict with either, so it has to appear in both or those conflicts are lost.
        let terms = BTreeMap::new();
        let requirements = vec![
            requirement("REQ-001", "a", "persist"),
            requirement("REQ-002", "b", "persist"),
            requirement("REQ-003", "", "reject"),
        ];
        let components = decompose(&requirements, &terms);
        assert_eq!(components.len(), 2);
        assert!(partition_is_sound(&requirements, &components).is_ok());
        for component in &components {
            assert!(component
                .requirements
                .iter()
                .any(|r| r.identifier == "REQ-003"));
        }
    }
}
