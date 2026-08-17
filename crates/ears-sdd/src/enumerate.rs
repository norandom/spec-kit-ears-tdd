//! The reference decision procedure: exhaustive enumeration over finitized domains.
//!
//! Deliberately the dull one. Exhaustive search over a finite space is a complete decision
//! procedure — there is no `unknown`, no heuristic, and no ordering to tune — so it is the thing a
//! cleverer procedure gets checked against. Its job is to be obviously correct, not fast.
//!
//! Assignments are identified by an index and decoded on demand in mixed radix, rather than
//! materialised. That keeps memory flat in the size of the state space and makes the enumeration
//! order a function of the variable order alone, which is what lets two runs produce byte-identical
//! witnesses.

use std::collections::BTreeMap;

use crate::guard::{Guard, Literal, Op};
use crate::model::{Component, ModelledRequirement, Value, Variable};
use crate::report::Finding;

/// A set of assignment indices, held as bits so a component near the budget costs kilobytes rather
/// than megabytes per requirement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexSet {
    words: Vec<u64>,
    length: u64,
}

impl IndexSet {
    fn empty(length: u64) -> Self {
        Self {
            words: vec![0; length.div_ceil(64) as usize],
            length,
        }
    }

    fn insert(&mut self, index: u64) {
        self.words[(index / 64) as usize] |= 1u64 << (index % 64);
    }

    pub fn contains(&self, index: u64) -> bool {
        self.words[(index / 64) as usize] & (1u64 << (index % 64)) != 0
    }

    pub fn is_empty(&self) -> bool {
        self.words.iter().all(|word| *word == 0)
    }

    /// The lowest index in both sets, which is the witness a finding reports. Lowest rather than
    /// any, so the same model always yields the same counterexample.
    pub fn first_common(&self, other: &IndexSet) -> Option<u64> {
        self.words
            .iter()
            .zip(other.words.iter())
            .enumerate()
            .find_map(|(word_index, (left, right))| {
                let both = left & right;
                (both != 0).then(|| word_index as u64 * 64 + both.trailing_zeros() as u64)
            })
            .filter(|index| *index < self.length)
    }

    pub fn first(&self) -> Option<u64> {
        self.first_common(&{
            let mut all = IndexSet::empty(self.length);
            all.words.iter_mut().for_each(|word| *word = u64::MAX);
            all
        })
    }

    /// Whether every index of `self` is also in `other`.
    pub fn is_subset_of(&self, other: &IndexSet) -> bool {
        self.words
            .iter()
            .zip(other.words.iter())
            .all(|(left, right)| left & !right == 0)
    }
}

/// Decode an index into the assignment it names, in mixed radix over the variable order.
pub fn assignment_at(index: u64, variables: &[Variable]) -> BTreeMap<String, Value> {
    let mut remaining = index;
    let mut assignment = BTreeMap::new();
    for variable in variables {
        let width = variable.values.len().max(1) as u64;
        let position = (remaining % width) as usize;
        remaining /= width;
        if let Some(value) = variable.values.get(position) {
            assignment.insert(variable.term.clone(), value.clone());
        }
    }
    assignment
}

/// Whether a guard holds under an assignment.
///
/// An integer region is tested through its representative. That is exact rather than approximate:
/// regions are cut at precisely the points where some comparison changes its answer, so every value
/// in a region agrees with the representative on every comparison the model makes.
pub fn evaluate(guard: &Guard, assignment: &BTreeMap<String, Value>) -> bool {
    match guard {
        Guard::Always => true,
        Guard::Term(name) => matches!(assignment.get(name), Some(Value::Bool(true))),
        Guard::Compare { term, op, value } => match (assignment.get(term), value) {
            (Some(Value::Bool(actual)), Literal::Bool(expected)) => match op {
                Op::Equal => actual == expected,
                Op::NotEqual => actual != expected,
                _ => false,
            },
            (Some(Value::Text(actual)), Literal::Text(expected)) => match op {
                Op::Equal => actual == expected,
                Op::NotEqual => actual != expected,
                _ => false,
            },
            (Some(Value::IntRegion { representative, .. }), Literal::Int(expected)) => match op {
                Op::Equal => representative == expected,
                Op::NotEqual => representative != expected,
                Op::Less => representative < expected,
                Op::LessOrEqual => representative <= expected,
                Op::Greater => representative > expected,
                Op::GreaterOrEqual => representative >= expected,
            },
            _ => false,
        },
        Guard::Not(inner) => !evaluate(inner, assignment),
        Guard::And(parts) => parts.iter().all(|part| evaluate(part, assignment)),
        Guard::Or(parts) => parts.iter().any(|part| evaluate(part, assignment)),
    }
}

/// Every assignment satisfying each requirement's guard, in requirement order.
pub fn satisfying_sets(component: &Component) -> Vec<IndexSet> {
    let states = component.state_space();
    let mut sets: Vec<IndexSet> = component
        .requirements
        .iter()
        .map(|_| IndexSet::empty(states))
        .collect();
    for index in 0..states {
        let assignment = assignment_at(index, &component.variables);
        for (position, requirement) in component.requirements.iter().enumerate() {
            if evaluate(&requirement.guard, &assignment) {
                sets[position].insert(index);
            }
        }
    }
    sets
}

/// Render an assignment the way a reader can act on it.
pub fn describe(assignment: &BTreeMap<String, Value>) -> String {
    assignment
        .iter()
        .map(|(term, value)| format!("{term} = {value}"))
        .collect::<Vec<_>>()
        .join(", ")
}

pub struct Witness {
    pub index: u64,
    pub description: String,
}

/// Decode a witness and confirm it satisfies the guards attributed to it.
///
/// The contract from REQ-024. A witness that does not reproduce is worse than none, because someone
/// will act on it; this turns an encoding bug into a loud internal error rather than a plausible
/// counterexample that wastes an afternoon.
pub fn verify_witness(
    index: u64,
    component: &Component,
    must_hold: &[&ModelledRequirement],
) -> Result<Witness, String> {
    let assignment = assignment_at(index, &component.variables);
    for requirement in must_hold {
        if !evaluate(&requirement.guard, &assignment) {
            return Err(format!(
                "witness for {} does not satisfy its own guard; the encoding is wrong",
                requirement.identifier
            ));
        }
    }
    Ok(Witness {
        index,
        description: describe(&assignment),
    })
}

pub fn internal_error(message: String, path: &str) -> Finding {
    Finding::new("MODEL_INTERNAL", message, path.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guard;
    use crate::model::{finitize, ModelledRequirement};
    use crate::vocabulary::Domain;

    fn component(variables: Vec<Variable>, guards: &[(&str, &str)]) -> Component {
        Component {
            index: 0,
            variables,
            requirements: guards
                .iter()
                .map(|(identifier, text)| ModelledRequirement {
                    identifier: (*identifier).to_string(),
                    feature: "specs/001-alpha".to_string(),
                    guard: guard::parse(text).expect("test guard parses"),
                    effect: "effect".to_string(),
                })
                .collect(),
        }
    }

    fn boolean(term: &str) -> Variable {
        finitize(term, &Domain::Bool, &[])
    }

    #[test]
    fn a_contradictory_guard_satisfies_nothing() {
        let component = component(vec![boolean("a")], &[("R1", "a and not a")]);
        assert!(satisfying_sets(&component)[0].is_empty());
    }

    #[test]
    fn an_unconditional_guard_satisfies_everything() {
        let component = component(vec![boolean("a")], &[("R1", "")]);
        let sets = satisfying_sets(&component);
        assert_eq!(component.state_space(), 2);
        assert!(sets[0].contains(0) && sets[0].contains(1));
    }

    #[test]
    fn overlap_is_detected_and_the_witness_reproduces() {
        let component = component(
            vec![boolean("a"), boolean("b")],
            &[("R1", "a"), ("R2", "a and b")],
        );
        let sets = satisfying_sets(&component);
        let index = sets[0]
            .first_common(&sets[1])
            .expect("the guards overlap where a and b both hold");

        let witness = verify_witness(
            index,
            &component,
            &[&component.requirements[0], &component.requirements[1]],
        )
        .expect("a reported witness must satisfy the guards attributed to it");
        assert!(
            witness.description.contains("a = true"),
            "{}",
            witness.description
        );
        assert!(
            witness.description.contains("b = true"),
            "{}",
            witness.description
        );
    }

    #[test]
    fn a_stricter_guard_is_a_subset_of_a_looser_one() {
        let component = component(
            vec![boolean("a"), boolean("b")],
            &[("R1", "a and b"), ("R2", "a")],
        );
        let sets = satisfying_sets(&component);
        assert!(sets[0].is_subset_of(&sets[1]));
        assert!(!sets[1].is_subset_of(&sets[0]));
    }

    #[test]
    fn integer_regions_evaluate_as_their_whole_range() {
        let domain = Domain::Int {
            min: 0,
            max: 10_000,
        };
        let variable = finitize("depth", &domain, &[(Op::Less, Literal::Int(1000))]);
        let component = component(vec![variable], &[("R1", "depth < 1000")]);

        // Two regions, and exactly the one standing for [0, 999] satisfies the guard.
        assert_eq!(component.state_space(), 2);
        let sets = satisfying_sets(&component);
        assert!(sets[0].contains(0));
        assert!(!sets[0].contains(1));
    }

    #[test]
    fn the_lowest_common_index_is_chosen_so_witnesses_are_stable() {
        let component = component(vec![boolean("a"), boolean("b")], &[("R1", ""), ("R2", "")]);
        let sets = satisfying_sets(&component);
        assert_eq!(sets[0].first_common(&sets[1]), Some(0));
    }

    #[test]
    fn an_enumeration_value_compares_by_name() {
        let domain = Domain::Enum {
            values: vec!["normal".into(), "maintenance".into()],
        };
        let component = component(
            vec![finitize("mode", &domain, &[])],
            &[("R1", "mode == 'maintenance'")],
        );
        let sets = satisfying_sets(&component);
        assert!(!sets[0].contains(0));
        assert!(sets[0].contains(1));
    }
}
