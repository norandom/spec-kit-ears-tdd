//! Turning components into findings.
//!
//! The budget check happens first and without enumerating anything. A component's state space is a
//! product of declared domain sizes, so it is known before any search starts; grinding through half
//! of one to report a limit would tell the reader nothing a multiplication could not.

use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::adjudicate::{classify, intent_of, Intents, Precedence, Verdict};
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

/// ears-sdd:allow-requirement-id: citing the requirement this contract enforces
/// The contract from REQ-026: a conditional requirement belongs to exactly one component.
///
/// Unconditional ones deliberately appear in every component, because their guard always holds and
/// isolating them would lose exactly the conflicts they can take part in.
///
/// Counting memberships is not enough on its own. A partition can give every requirement exactly
/// one component and still be unsound, by separating two requirements whose effects conflict: each
/// component then comes back satisfiable and the contradiction is never looked for. That is the
/// failure this function missed once, so the conflict reachability check below is the part that
/// actually earns it.
pub fn partition_is_sound(
    requirements: &[ModelledRequirement],
    components: &[Component],
    conflict_pairs: &BTreeSet<(String, String)>,
) -> Result<(), String> {
    for (index, left) in requirements.iter().enumerate() {
        for right in &requirements[index + 1..] {
            if !conflicts(conflict_pairs, &left.effect, &right.effect) {
                continue;
            }
            let together = components.iter().any(|component| {
                let has = |target: &ModelledRequirement| {
                    component.requirements.iter().any(|member| {
                        member.identifier == target.identifier && member.feature == target.feature
                    })
                };
                has(left) && has(right)
            });
            if !together {
                return Err(format!(
                    "{}:{} and {}:{} assert conflicting effects but share no component",
                    left.feature, left.identifier, right.feature, right.identifier
                ));
            }
        }
    }

    for requirement in requirements {
        let unconditional = requirement.guard.terms().is_empty();
        // Compared on feature and identifier together. Identifiers restart at the first number in
        // every feature, so matching on the identifier alone conflates one specification's
        // requirement with another's -- which is exactly what happens in a merge, and is how this
        // contract caught its own first real bug.
        let appearances = components
            .iter()
            .filter(|component| {
                component.requirements.iter().any(|member| {
                    member.identifier == requirement.identifier
                        && member.feature == requirement.feature
                })
            })
            .count();
        let expected = if unconditional { components.len() } else { 1 };
        if appearances != expected {
            return Err(format!(
                "{}:{} appears in {appearances} components, expected {expected}",
                requirement.feature, requirement.identifier
            ));
        }
    }
    Ok(())
}

fn defect_code(prefix: &str) -> &'static str {
    if prefix == "MERGE" {
        "MERGE_CONFLICT_DEFECT"
    } else {
        "MODEL_CONFLICT_DEFECT"
    }
}

fn adjudicated_code(prefix: &str) -> &'static str {
    if prefix == "MERGE" {
        "MERGE_CONFLICT_ADJUDICATED"
    } else {
        "MODEL_CONFLICT_ADJUDICATED"
    }
}

fn unadjudicated_code(prefix: &str) -> &'static str {
    if prefix == "MERGE" {
        "MERGE_CONFLICT_UNADJUDICATED"
    } else {
        "MODEL_CONFLICT_UNADJUDICATED"
    }
}

fn plain_code(prefix: &str) -> &'static str {
    if prefix == "MERGE" {
        "MERGE_CONFLICT"
    } else {
        "MODEL_CONFLICT"
    }
}

/// A conflict, classified by the intentions its two requirements serve.
///
/// Note the conflict itself is always a *pair* here, and its minimal form is therefore the pair.
/// That is a property of the model shape rather than a shortcut: requirements react to state, they
/// do not constrain it, so there is no way for three guards to be jointly unsatisfiable while every
/// pair of them is fine. Larger minimal sets need declared invariants — facts that restrict which
/// states are reachable — which this version does not have.
#[allow(clippy::too_many_arguments)]
fn conflict_finding(
    prefix: &str,
    first: &ModelledRequirement,
    second: &ModelledRequirement,
    witness: &str,
    component: usize,
    intents: &Intents,
    precedence: &Precedence,
    source: &str,
) -> Finding {
    let verdict = classify(
        &[
            intent_of(intents, &first.feature, &first.identifier),
            intent_of(intents, &second.feature, &second.identifier),
        ],
        precedence,
    );

    let (code, message, severity) = match &verdict {
        Verdict::Defect { intention } => (
            defect_code(prefix),
            format!(
                "{} and {} contradict each other and both serve `{intention}`. No precedence can                  adjudicate this: one goal cannot outrank itself, so one of the two rules is wrong.",
                first.identifier, second.identifier
            ),
            Severity::Error,
        ),
        Verdict::Adjudicated { winner } => (
            adjudicated_code(prefix),
            format!(
                "{} and {} contradict each other; `{winner}` takes precedence, so this is a                  recorded decision rather than a defect.",
                first.identifier, second.identifier
            ),
            Severity::Advisory,
        ),
        Verdict::Unadjudicated { missing } => (
            unadjudicated_code(prefix),
            format!(
                "{} and {} contradict each other and nothing says which wins. Declare precedence                  between {}.",
                first.identifier,
                second.identifier,
                missing
                    .iter()
                    .map(|(left, right)| format!("`{left}` and `{right}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Severity::Error,
        ),
        Verdict::Unclassified => (
            plain_code(prefix),
            format!(
                "{} and {} both apply, but `{}` and `{}` cannot both hold.",
                first.identifier, second.identifier, first.effect, second.effect
            ),
            Severity::Error,
        ),
    };

    let mut finding = Finding::new(code, message, source.to_string())
        .feature(&first.feature)
        .requirement(&first.identifier)
        .severity(severity)
        .detail("with", second.identifier.clone())
        .detail("component", component as u64)
        .detail("witness", witness.to_string());
    if let Verdict::Unadjudicated { missing } = &verdict {
        finding = finding.detail(
            "declare_precedence_between",
            serde_json::Value::Array(
                missing
                    .iter()
                    .map(|(left, right)| json!({ "a": left, "b": right }))
                    .collect(),
            ),
        );
    }
    finding
}

#[allow(clippy::too_many_arguments)]
fn analyze_component(
    component: &Component,
    pairs: &BTreeSet<(String, String)>,
    budget: u64,
    source: &str,
    intents: &Intents,
    precedence: &Precedence,
    prefix: &str,
    cross_feature_only: bool,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    if component.state_space() > budget {
        findings.push(over_budget(component, budget, source));
        return findings;
    }

    let sets = satisfying_sets(component);

    for (position, requirement) in component.requirements.iter().enumerate() {
        if cross_feature_only {
            break;
        }
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
            // In merge mode a same-specification pair has already been reported by that
            // specification's own run. Repeating it would double every finding and bury the
            // cross-specification ones this gate exists to surface.
            if cross_feature_only && first.feature == second.feature {
                continue;
            }

            if conflicts(pairs, &first.effect, &second.effect) {
                if let Some(index) = sets[left].first_common(&sets[right]) {
                    match verify_witness(index, component, &[first, second]) {
                        Ok(witness) => findings.push(conflict_finding(
                            prefix,
                            first,
                            second,
                            &witness.description,
                            component.index,
                            intents,
                            precedence,
                            source,
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
                let code = if prefix == "MERGE" {
                    "MERGE_SHADOW"
                } else {
                    "MODEL_SUBSUMED"
                };
                findings.push(
                    Finding::new(
                        code,
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

/// Everything the model layer needs that is the same for every feature.
///
/// Bundled rather than passed one by one: these four are loaded once per run and travel together,
/// and threading them separately made the entry point's signature a list nobody could read.
pub struct ModelContext<'a> {
    pub terms: &'a BTreeMap<String, (Term, String)>,
    pub budget: u64,
    pub intents: &'a Intents,
    pub precedence: &'a Precedence,
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
    context: &ModelContext<'_>,
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
        findings.extend(check_types(requirement, context.terms, &source));
    }

    // Built before decomposition, which needs it: requirements asserting conflicting effects have
    // to land in the same component or the conflict is never looked for.
    let pairs = conflict_pairs(&model);
    let components = decompose(&requirements, context.terms, &pairs);
    if let Err(message) = partition_is_sound(&requirements, &components, &pairs) {
        findings.push(crate::enumerate::internal_error(message, &source));
        return Outcome {
            findings,
            modelled: requirements.len(),
            components: components.len(),
        };
    }

    for component in &components {
        findings.extend(analyze_component(
            component,
            &pairs,
            context.budget,
            &source,
            context.intents,
            context.precedence,
            "MODEL",
            false,
        ));
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
    analyze_component(
        component,
        pairs,
        budget,
        "model.toml",
        &Intents::new(),
        &Precedence::new(&BTreeSet::new()),
        "MODEL",
        false,
    )
}

/// As above, with the intention layer supplied.
pub fn analyze_with_intents(
    component: &Component,
    pairs: &BTreeSet<(String, String)>,
    intents: &Intents,
    precedence: &Precedence,
) -> Vec<Finding> {
    analyze_component(
        component,
        pairs,
        1_000_000,
        "model.toml",
        intents,
        precedence,
        "MODEL",
        false,
    )
}

/// One specification's contribution to the merge.
pub struct FeatureModel {
    pub feature: String,
    pub directory: std::path::PathBuf,
    pub declared: BTreeSet<String>,
}

/// Check every specification's constraints together.
///
/// This is the gate the whole layer exists to feed. Specifications that each pass on their own can
/// still describe a system that cannot exist, and no per-feature check can see it: the contradiction
/// only appears once both sets of constraints are in the same room.
///
/// The merge is a graph union on shared terms, which is why grounded vocabulary is a precondition
/// rather than a nicety. Two features naming the same setting differently never share a component,
/// so their conflict stays structurally invisible and the gate reports a confident pass.
pub fn validate_merged(features: &[FeatureModel], context: &ModelContext<'_>) -> Outcome {
    let mut findings = Vec::new();
    let mut requirements: Vec<ModelledRequirement> = Vec::new();
    let mut unmerged: Vec<String> = Vec::new();
    let mut pairs: BTreeSet<(String, String)> = BTreeSet::new();
    let source = ".specify/ears-sdd.toml".to_string();

    for entry in features {
        let path = entry.directory.join("model.toml");
        let Ok(text) = std::fs::read_to_string(&path) else {
            unmerged.push(entry.feature.clone());
            continue;
        };
        let Ok(model) = toml::from_str::<ModelFile>(&text) else {
            continue; // already reported by the per-feature run
        };
        pairs.extend(conflict_pairs(&model));
        for (identifier, requirement_entry) in &model.requirements {
            if !entry.declared.contains(identifier) {
                continue;
            }
            if !model.effects.contains_key(&requirement_entry.then) {
                continue;
            }
            let Ok(parsed) = requirement_entry
                .when
                .as_deref()
                .map(guard::parse)
                .transpose()
            else {
                continue;
            };
            requirements.push(ModelledRequirement {
                identifier: identifier.clone(),
                feature: entry.feature.clone(),
                guard: parsed.unwrap_or(Guard::Always),
                effect: requirement_entry.then.clone(),
            });
        }
    }

    let contributing: BTreeSet<&str> = requirements
        .iter()
        .map(|requirement| requirement.feature.as_str())
        .collect();
    if contributing.len() < 2 {
        // Nothing to merge: one specification's constraints have already been checked against
        // themselves, and a project using none of this layer should hear nothing from it.
        return Outcome {
            findings,
            modelled: requirements.len(),
            components: 0,
        };
    }

    // Reported only once a merge actually happened. A specification left out of a real merge is a
    // gap in what was checked, and a gap nobody can see is the failure this project exists to
    // prevent -- but saying so in a project that declares no models at all is just noise.
    for feature in &unmerged {
        findings.push(
            Finding::new(
                "MERGE_UNMERGED",
                "Specification declares no constraint model and was excluded from the merge."
                    .to_string(),
                source.clone(),
            )
            .feature(feature)
            .severity(Severity::Advisory),
        );
    }

    let components = decompose(&requirements, context.terms, &pairs);
    if let Err(message) = partition_is_sound(&requirements, &components, &pairs) {
        findings.push(crate::enumerate::internal_error(message, &source));
        return Outcome {
            findings,
            modelled: requirements.len(),
            components: components.len(),
        };
    }

    for component in &components {
        let spanning: BTreeSet<&str> = component
            .requirements
            .iter()
            .map(|requirement| requirement.feature.as_str())
            .collect();
        if spanning.len() < 2 {
            continue;
        }
        let shared: Vec<String> = component
            .variables
            .iter()
            .map(|variable| variable.term.clone())
            .collect();
        for mut finding in analyze_component(
            component,
            &pairs,
            context.budget,
            &source,
            context.intents,
            context.precedence,
            "MERGE",
            true,
        ) {
            finding = finding
                .detail("component", component.index as u64)
                .detail(
                    "specifications",
                    serde_json::Value::Array(spanning.iter().map(|name| json!(name)).collect()),
                )
                .detail(
                    "shared_terms",
                    serde_json::Value::Array(shared.iter().map(|term| json!(term)).collect()),
                );
            findings.push(finding);
        }
    }

    Outcome {
        findings,
        modelled: requirements.len(),
        components: components.len(),
    }
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
                requirement("R-001", "a", "persist"),
                requirement("R-002", "a and b", "reject"),
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
                requirement("R-001", "a", "persist"),
                requirement("R-002", "not a", "reject"),
            ],
        );
        assert!(analyze_for_test(&component, &conflicting("persist", "reject"), 1000).is_empty());
    }

    #[test]
    fn a_guard_that_can_never_hold_is_reported() {
        let component = component(
            vec![boolean("a")],
            vec![requirement("R-001", "a and not a", "persist")],
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
                requirement("R-001", "a and b", "persist"),
                requirement("R-002", "a", "persist"),
            ],
        );
        let findings = analyze_for_test(&component, &BTreeSet::new(), 1000);
        assert_eq!(codes(&findings), vec!["MODEL_SUBSUMED"]);
        assert_eq!(findings[0].requirement.as_deref(), Some("R-001"));
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
        let component = component(variables, vec![requirement("R-001", "a", "persist")]);

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
            requirement("R-001", "a", "persist"),
            requirement("R-002", "b", "persist"),
            requirement("R-003", "", "reject"),
        ];
        let components = decompose(&requirements, &terms, &BTreeSet::new());
        assert_eq!(components.len(), 2);
        assert!(partition_is_sound(&requirements, &components, &BTreeSet::new()).is_ok());
        for component in &components {
            assert!(component
                .requirements
                .iter()
                .any(|r| r.identifier == "R-003"));
        }
    }

    /// The cross-specification shape: two features guard on different conditions and assert effects
    /// that cannot both hold. Decomposing by shared guard terms alone puts them in separate
    /// components, each satisfiable, and the merge reports a pass it has not earned.
    ///
    /// This is not hypothetical. It shipped, and a two-specification project modelling an exploit
    /// mitigation against a native toolchain reported no findings at all.
    #[test]
    fn conflicting_effects_share_a_component_even_with_disjoint_guards() {
        let terms = BTreeMap::new();
        let requirements = vec![
            requirement("R-001", "mitigation-enforced", "block_dynamic_code"),
            requirement("R-002", "toolchain-building", "permit_dynamic_code"),
        ];
        let pairs = conflicting("block_dynamic_code", "permit_dynamic_code");

        let components = decompose(&requirements, &terms, &pairs);

        assert_eq!(
            components.len(),
            1,
            "conflicting requirements must be evaluated together"
        );
        assert!(partition_is_sound(&requirements, &components, &pairs).is_ok());
    }

    /// Non-conflicting requirements must still decompose, or the optimisation is gone and every
    /// project pays the state space of its whole specification set at once.
    #[test]
    fn disjoint_guards_still_decompose_when_no_effects_conflict() {
        let terms = BTreeMap::new();
        let requirements = vec![
            requirement("R-001", "mitigation-enforced", "block_dynamic_code"),
            requirement("R-002", "toolchain-building", "emit_audit_entry"),
        ];

        let components = decompose(&requirements, &terms, &BTreeSet::new());

        assert_eq!(components.len(), 2);
    }

    /// The contract has to reject the partition the old decomposition produced, not merely accept
    /// the new one. Without this, a future change could reintroduce the split silently.
    #[test]
    fn the_contract_rejects_a_partition_that_separates_conflicting_requirements() {
        let terms = BTreeMap::new();
        let requirements = vec![
            requirement("R-001", "mitigation-enforced", "block_dynamic_code"),
            requirement("R-002", "toolchain-building", "permit_dynamic_code"),
        ];
        let pairs = conflicting("block_dynamic_code", "permit_dynamic_code");
        // Exactly what decompose returned before the fix: split by guard term.
        let split = decompose(&requirements, &terms, &BTreeSet::new());
        assert_eq!(
            split.len(),
            2,
            "precondition: the old partition splits them"
        );

        let verdict = partition_is_sound(&requirements, &split, &pairs);

        assert!(
            verdict.is_err(),
            "a partition hiding a conflict must not be reported as sound"
        );
        assert!(verdict.unwrap_err().contains("share no component"));
    }
}
