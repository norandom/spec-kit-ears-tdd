//! The two decision procedures must agree.
//!
//! This is the reason for having two. The enumerator is obviously correct — exhaustive search over
//! a finite space — but does not scale; the decision diagram scales but is where the subtle bugs
//! live, in one-hot constraints, interval boundaries, and variable order. They share no encoding
//! code, so a disagreement between them is almost certainly a bug in the diagram rather than a
//! coincidence in both.
//!
//! Note what this does and does not cover. It catches wrong answers. It does not catch blowup: a
//! slow-but-correct diagram agrees perfectly. Tractability is defended by decomposition and the
//! state-space budget, which are separate mechanisms.

use ears_sdd::bdd::{implies, is_satisfiable, overlap, Encoding};
use ears_sdd::enumerate::satisfying_sets;
use ears_sdd::guard;
use ears_sdd::model::{finitize, Component, ModelledRequirement, Variable};
use ears_sdd::vocabulary::Domain;

fn boolean(term: &str) -> Variable {
    finitize(term, &Domain::Bool, &[])
}

fn enumeration(term: &str, values: &[&str]) -> Variable {
    let domain = Domain::Enum {
        values: values.iter().map(|value| value.to_string()).collect(),
    };
    finitize(term, &domain, &[])
}

fn integer(term: &str, min: i64, max: i64, cuts: &[(guard::Op, i64)]) -> Variable {
    let comparisons: Vec<(guard::Op, guard::Literal)> = cuts
        .iter()
        .map(|(op, value)| (*op, guard::Literal::Int(*value)))
        .collect();
    finitize(term, &Domain::Int { min, max }, &comparisons)
}

fn component(variables: Vec<Variable>, guards: &[&str]) -> Component {
    Component {
        index: 0,
        variables,
        requirements: guards
            .iter()
            .enumerate()
            .map(|(position, text)| ModelledRequirement {
                identifier: format!("REQ-{:03}", position + 1),
                feature: "specs/001-alpha".to_string(),
                guard: guard::parse(text)
                    .unwrap_or_else(|error| panic!("guard {text:?} should parse: {error}")),
                effect: "effect".to_string(),
            })
            .collect(),
    }
}

/// Ask both procedures every question the analysis asks, and require identical answers.
fn agree(component: &Component) {
    let sets = satisfying_sets(component);
    let encoding = Encoding::new(component);
    let diagrams: Vec<_> = component
        .requirements
        .iter()
        .map(|requirement| encoding.encode(&requirement.guard))
        .collect();

    for (position, requirement) in component.requirements.iter().enumerate() {
        assert_eq!(
            !sets[position].is_empty(),
            is_satisfiable(&encoding, &diagrams[position]),
            "satisfiability disagrees for {} in {:?}",
            requirement.identifier,
            component
                .requirements
                .iter()
                .map(|r| &r.guard)
                .collect::<Vec<_>>()
        );
    }

    for left in 0..component.requirements.len() {
        for right in 0..component.requirements.len() {
            if left == right {
                continue;
            }
            assert_eq!(
                sets[left].first_common(&sets[right]).is_some(),
                overlap(&encoding, &diagrams[left], &diagrams[right]),
                "overlap disagrees for {} and {}",
                component.requirements[left].identifier,
                component.requirements[right].identifier
            );
            // Subsumption is only meaningful for a guard that can hold at all: an unsatisfiable
            // guard is vacuously a subset of everything, which is true and useless.
            if !sets[left].is_empty() {
                assert_eq!(
                    sets[left].is_subset_of(&sets[right]),
                    implies(&encoding, &diagrams[left], &diagrams[right]),
                    "implication disagrees for {} and {}",
                    component.requirements[left].identifier,
                    component.requirements[right].identifier
                );
            }
        }
    }
}

#[test]
fn booleans_agree() {
    agree(&component(
        vec![boolean("a"), boolean("b"), boolean("c")],
        &[
            "a",
            "not a",
            "a and b",
            "a or b",
            "not (a and b)",
            "a and not a",
            "",
        ],
    ));
}

#[test]
fn enumerations_agree() {
    agree(&component(
        vec![enumeration("mode", &["normal", "maintenance", "degraded"])],
        &[
            "mode == 'normal'",
            "mode != 'normal'",
            "mode == 'maintenance' or mode == 'degraded'",
            "mode == 'normal' and mode == 'degraded'",
        ],
    ));
}

#[test]
fn integer_regions_agree() {
    let cuts = [
        (guard::Op::Less, 1000),
        (guard::Op::GreaterOrEqual, 5000),
        (guard::Op::Equal, 42),
    ];
    agree(&component(
        vec![integer("depth", 0, 10_000, &cuts)],
        &[
            "depth < 1000",
            "depth >= 5000",
            "depth == 42",
            "depth != 42",
            "depth < 1000 and depth >= 5000",
            "depth < 1000 or depth >= 5000",
        ],
    ));
}

#[test]
fn mixed_domains_agree() {
    let cuts = [(guard::Op::Less, 100)];
    agree(&component(
        vec![
            boolean("verified"),
            enumeration("mode", &["normal", "maintenance"]),
            integer("depth", 0, 500, &cuts),
        ],
        &[
            "verified and mode == 'maintenance'",
            "not verified or depth < 100",
            "mode == 'normal' and depth < 100 and verified",
            "not (verified and mode == 'normal')",
            "",
        ],
    ));
}

/// Exhaustive over a small space: every guard expressible from two booleans, so a systematic
/// encoding error cannot hide in a case nobody thought to write by hand.
#[test]
fn every_two_variable_guard_agrees() {
    let atoms = ["a", "b", "not a", "not b"];
    let mut guards: Vec<String> = atoms.iter().map(|atom| atom.to_string()).collect();
    for left in &atoms {
        for right in &atoms {
            guards.push(format!("{left} and {right}"));
            guards.push(format!("{left} or {right}"));
            guards.push(format!("not ({left} and {right})"));
        }
    }
    let borrowed: Vec<&str> = guards.iter().map(String::as_str).collect();
    agree(&component(vec![boolean("a"), boolean("b")], &borrowed));
}
