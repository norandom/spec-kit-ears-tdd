//! The second decision procedure: reduced ordered binary decision diagrams.
//!
//! Same questions as the enumerator, answered structurally instead of by search. A reduced ordered
//! diagram is canonical for a fixed variable order, so semantic identity becomes graph identity and
//! the queries collapse: unsatisfiable is "is this the zero terminal", contradiction is "is the
//! conjunction zero", subsumption is "is the implication one".
//!
//! **Encoding.** Each term becomes a one-hot block: one boolean per value, plus a constraint that
//! exactly one of them holds. One-hot rather than a log encoding because a log encoding makes every
//! comparison touch several variables, which increases interaction and so tends to grow the
//! diagram, and because a one-hot witness decodes back to a term and a value without arithmetic.
//!
//! **Order.** Each term's block is kept contiguous. That is the one ordering decision that matters
//! here: interleaving one term's bits with another's is precisely the shape that makes comparator
//! circuits blow up. Terms themselves are ordered as the component presents them, which is derived
//! from the vocabulary rather than from the formula, so adding a requirement does not reshuffle the
//! order and change the result.
//!
//! No dynamic reordering. It is the standard mitigation and the wrong one for a gate: it makes the
//! outcome depend on when the library decided to reorder, trading a performance problem for a
//! reproducibility problem.

use biodivine_lib_bdd::{Bdd, BddVariable, BddVariableSet, BddVariableSetBuilder};
use std::collections::BTreeMap;

use crate::guard::{Guard, Literal, Op};
use crate::model::{Component, Value};

pub struct Encoding {
    variables: BddVariableSet,
    /// Term to the one-hot variables of its values, in value order.
    blocks: BTreeMap<String, Vec<(BddVariable, Value)>>,
    /// Exactly-one over every block: the states that correspond to a real assignment.
    domain: Bdd,
}

fn variable_name(term: &str, position: usize) -> String {
    format!("{term}#{position}")
}

impl Encoding {
    pub fn new(component: &Component) -> Self {
        let mut builder = BddVariableSetBuilder::new();
        let mut blocks: BTreeMap<String, Vec<(BddVariable, Value)>> = BTreeMap::new();

        // Declared block by block, so each term's variables are adjacent in the order.
        for variable in &component.variables {
            let mut block = Vec::new();
            for (position, value) in variable.values.iter().enumerate() {
                let handle = builder.make_variable(&variable_name(&variable.term, position));
                block.push((handle, value.clone()));
            }
            blocks.insert(variable.term.clone(), block);
        }

        let variables = builder.build();
        let mut domain = variables.mk_true();
        for block in blocks.values() {
            domain = domain.and(&exactly_one(&variables, block));
        }
        Self {
            variables,
            blocks,
            domain,
        }
    }

    /// The constraint that a real assignment exists at all. Every query is asked relative to it,
    /// because without it the diagram also describes states where a term holds two values at once.
    pub fn domain(&self) -> &Bdd {
        &self.domain
    }

    pub fn encode(&self, guard: &Guard) -> Bdd {
        match guard {
            Guard::Always => self.variables.mk_true(),
            Guard::Term(name) => self.matching(name, |value| matches!(value, Value::Bool(true))),
            Guard::Compare { term, op, value } => self.compare(term, *op, value),
            Guard::Not(inner) => self.encode(inner).not(),
            Guard::And(parts) => parts.iter().fold(self.variables.mk_true(), |acc, part| {
                acc.and(&self.encode(part))
            }),
            Guard::Or(parts) => parts.iter().fold(self.variables.mk_false(), |acc, part| {
                acc.or(&self.encode(part))
            }),
        }
    }

    /// The disjunction of every value of a term satisfying a predicate.
    ///
    /// Comparisons are evaluated once here, at encoding time, against the same representative the
    /// enumerator uses — so both procedures agree by construction on what a region means, and can
    /// only disagree about the logic built on top.
    fn matching(&self, term: &str, predicate: impl Fn(&Value) -> bool) -> Bdd {
        let Some(block) = self.blocks.get(term) else {
            return self.variables.mk_false();
        };
        block
            .iter()
            .filter(|(_, value)| predicate(value))
            .fold(self.variables.mk_false(), |acc, (handle, _)| {
                acc.or(&self.variables.mk_var(*handle))
            })
    }

    fn compare(&self, term: &str, op: Op, literal: &Literal) -> Bdd {
        self.matching(term, |value| match (value, literal) {
            (Value::Bool(actual), Literal::Bool(expected)) => match op {
                Op::Equal => actual == expected,
                Op::NotEqual => actual != expected,
                _ => false,
            },
            (Value::Text(actual), Literal::Text(expected)) => match op {
                Op::Equal => actual == expected,
                Op::NotEqual => actual != expected,
                _ => false,
            },
            (Value::IntRegion { representative, .. }, Literal::Int(expected)) => match op {
                Op::Equal => representative == expected,
                Op::NotEqual => representative != expected,
                Op::Less => representative < expected,
                Op::LessOrEqual => representative <= expected,
                Op::Greater => representative > expected,
                Op::GreaterOrEqual => representative >= expected,
            },
            _ => false,
        })
    }

    /// Decode a satisfying valuation back into terms and values.
    pub fn witness(&self, formula: &Bdd) -> Option<BTreeMap<String, Value>> {
        let constrained = formula.and(&self.domain);
        let valuation = constrained.sat_witness()?;
        let mut assignment = BTreeMap::new();
        for (term, block) in &self.blocks {
            for (handle, value) in block {
                if valuation.value(*handle) {
                    assignment.insert(term.clone(), value.clone());
                    break;
                }
            }
        }
        Some(assignment)
    }

    pub fn node_count(&self, formula: &Bdd) -> usize {
        formula.size()
    }
}

fn exactly_one(variables: &BddVariableSet, block: &[(BddVariable, Value)]) -> Bdd {
    let at_least_one = block.iter().fold(variables.mk_false(), |acc, (handle, _)| {
        acc.or(&variables.mk_var(*handle))
    });
    let mut at_most_one = variables.mk_true();
    for (index, (left, _)) in block.iter().enumerate() {
        for (right, _) in block.iter().skip(index + 1) {
            let both = variables.mk_var(*left).and(&variables.mk_var(*right));
            at_most_one = at_most_one.and(&both.not());
        }
    }
    at_least_one.and(&at_most_one)
}

/// Whether a guard can hold at all, given the domain constraint.
pub fn is_satisfiable(encoding: &Encoding, guard: &Bdd) -> bool {
    !guard.and(encoding.domain()).is_false()
}

/// Whether two guards can hold together.
pub fn overlap(encoding: &Encoding, left: &Bdd, right: &Bdd) -> bool {
    !left.and(right).and(encoding.domain()).is_false()
}

/// Whether the first guard holds only where the second does.
pub fn implies(encoding: &Encoding, left: &Bdd, right: &Bdd) -> bool {
    encoding.domain().and(left).and(&right.not()).is_false()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guard;
    use crate::model::{finitize, ModelledRequirement, Variable};
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
    fn a_contradiction_is_unsatisfiable() {
        let component = component(vec![boolean("a")], &[("R1", "a and not a")]);
        let encoding = Encoding::new(&component);
        let guard = encoding.encode(&component.requirements[0].guard);
        assert!(!is_satisfiable(&encoding, &guard));
    }

    #[test]
    fn overlapping_guards_overlap() {
        let component = component(
            vec![boolean("a"), boolean("b")],
            &[("R1", "a"), ("R2", "a and b")],
        );
        let encoding = Encoding::new(&component);
        let left = encoding.encode(&component.requirements[0].guard);
        let right = encoding.encode(&component.requirements[1].guard);
        assert!(overlap(&encoding, &left, &right));
    }

    #[test]
    fn disjoint_guards_do_not_overlap() {
        let component = component(vec![boolean("a")], &[("R1", "a"), ("R2", "not a")]);
        let encoding = Encoding::new(&component);
        let left = encoding.encode(&component.requirements[0].guard);
        let right = encoding.encode(&component.requirements[1].guard);
        assert!(!overlap(&encoding, &left, &right));
    }

    #[test]
    fn a_stricter_guard_implies_a_looser_one() {
        let component = component(
            vec![boolean("a"), boolean("b")],
            &[("R1", "a and b"), ("R2", "a")],
        );
        let encoding = Encoding::new(&component);
        let stricter = encoding.encode(&component.requirements[0].guard);
        let looser = encoding.encode(&component.requirements[1].guard);
        assert!(implies(&encoding, &stricter, &looser));
        assert!(!implies(&encoding, &looser, &stricter));
    }

    #[test]
    fn the_domain_forbids_a_term_holding_two_values() {
        // Without the exactly-one constraint the diagram also describes states where an enumeration
        // is simultaneously two of its values, and every answer built on it would be wrong.
        let domain = Domain::Enum {
            values: vec!["normal".into(), "maintenance".into()],
        };
        let component = component(vec![finitize("mode", &domain, &[])], &[]);
        let encoding = Encoding::new(&component);
        let block = &encoding.blocks["mode"];
        let both = encoding
            .variables
            .mk_var(block[0].0)
            .and(&encoding.variables.mk_var(block[1].0));
        assert!(both.and(encoding.domain()).is_false());
    }

    #[test]
    fn a_witness_decodes_back_to_a_term_and_value() {
        let component = component(vec![boolean("a"), boolean("b")], &[("R1", "a and b")]);
        let encoding = Encoding::new(&component);
        let guard = encoding.encode(&component.requirements[0].guard);
        let witness = encoding.witness(&guard).expect("the guard is satisfiable");
        assert_eq!(witness.get("a"), Some(&Value::Bool(true)));
        assert_eq!(witness.get("b"), Some(&Value::Bool(true)));
    }

    #[test]
    fn integer_regions_encode_as_their_range() {
        let domain = Domain::Int {
            min: 0,
            max: 10_000,
        };
        let variable = finitize("depth", &domain, &[(Op::Less, Literal::Int(1000))]);
        let component = component(vec![variable], &[("R1", "depth < 1000")]);
        let encoding = Encoding::new(&component);
        let guard = encoding.encode(&component.requirements[0].guard);
        let witness = encoding.witness(&guard).expect("satisfiable");
        assert_eq!(
            witness.get("depth"),
            Some(&Value::IntRegion {
                representative: 0,
                low: 0,
                high: 999
            })
        );
    }
}
