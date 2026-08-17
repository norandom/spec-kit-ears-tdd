//! Classifying a conflict by the intentions its requirements serve.
//!
//! Knowing that two requirements contradict is half an answer. Joining each to its declared
//! intention separates two situations that need entirely different responses:
//!
//! * requirements serving **different** intentions are a genuine trade-off, and a declared
//!   precedence can say which one wins;
//! * requirements serving the **same** intention are a specification defect. Somebody wrote two
//!   contradictory rules in service of one goal, and no precedence can or should adjudicate that —
//!   offering to rank a goal above itself would just hide the mistake.
//!
//! The adjudication test is a **unique maximum**, not merely a maximal element or some ordered
//! pair. Under a partial order a set can have several maximal elements and no maximum, and in that
//! case there is still no mechanical answer to which requirements win. Requiring a unique maximum
//! is exactly the condition under which the tool can say "the ones serving `m` stand, change the
//! others".
//!
//! This is why the traceability entry carries a singular `intent`. With a list, the intention set
//! of a conflict becomes a union and the unique-maximum question stops meaning anything useful.

use std::collections::{BTreeMap, BTreeSet};

/// The declared precedence relation, transitively closed.
pub struct Precedence {
    closure: BTreeSet<(String, String)>,
}

impl Precedence {
    /// Closed by repeated relaxation. The relation is small — one entry per deliberate override —
    /// so the simplest correct algorithm is the right one.
    pub fn new(edges: &BTreeSet<(String, String)>) -> Self {
        let mut closure = edges.clone();
        loop {
            let mut added = Vec::new();
            for (over, under) in &closure {
                for (middle, bottom) in &closure {
                    if under == middle && !closure.contains(&(over.clone(), bottom.clone())) {
                        added.push((over.clone(), bottom.clone()));
                    }
                }
            }
            if added.is_empty() {
                return Self { closure };
            }
            closure.extend(added);
        }
    }

    pub fn outranks(&self, over: &str, under: &str) -> bool {
        self.closure
            .contains(&(over.to_string(), under.to_string()))
    }

    /// The single element outranking every other in the set, if there is one.
    pub fn unique_maximum(&self, intentions: &BTreeSet<String>) -> Option<String> {
        let maxima: Vec<&String> = intentions
            .iter()
            .filter(|candidate| {
                intentions
                    .iter()
                    .filter(|other| other != candidate)
                    .all(|other| self.outranks(candidate, other))
            })
            .collect();
        match maxima.as_slice() {
            [only] => Some((*only).clone()),
            _ => None,
        }
    }

    /// The comparisons that would have to be declared for a set to become adjudicable.
    ///
    /// Naming the missing pairs is the difference between a report someone can act on and one that
    /// says "add a precedence" without saying which.
    pub fn missing_comparisons(&self, intentions: &BTreeSet<String>) -> Vec<(String, String)> {
        let mut missing = Vec::new();
        for left in intentions {
            for right in intentions {
                if left < right && !self.outranks(left, right) && !self.outranks(right, left) {
                    missing.push((left.clone(), right.clone()));
                }
            }
        }
        missing
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Verdict {
    /// Every requirement in the conflict serves one intention. Unadjudicable by construction.
    Defect { intention: String },
    /// A trade-off with a declared winner.
    Adjudicated { winner: String },
    /// A trade-off nobody has decided.
    Unadjudicated { missing: Vec<(String, String)> },
    /// At least one requirement declares no intention, so there is nothing to join on.
    Unclassified,
}

pub fn classify(intentions: &[Option<String>], precedence: &Precedence) -> Verdict {
    if intentions.iter().any(Option::is_none) {
        return Verdict::Unclassified;
    }
    let distinct: BTreeSet<String> = intentions.iter().flatten().cloned().collect();
    match distinct.len() {
        0 => Verdict::Unclassified,
        1 => Verdict::Defect {
            intention: distinct.into_iter().next().expect("checked length"),
        },
        _ => match precedence.unique_maximum(&distinct) {
            Some(winner) => Verdict::Adjudicated { winner },
            None => Verdict::Unadjudicated {
                missing: precedence.missing_comparisons(&distinct),
            },
        },
    }
}

/// Which intention each requirement serves, keyed by feature and identifier.
pub type Intents = BTreeMap<(String, String), String>;

pub fn intent_of(intents: &Intents, feature: &str, identifier: &str) -> Option<String> {
    intents
        .get(&(feature.to_string(), identifier.to_string()))
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn edges(pairs: &[(&str, &str)]) -> BTreeSet<(String, String)> {
        pairs
            .iter()
            .map(|(over, under)| (over.to_string(), under.to_string()))
            .collect()
    }

    fn set(names: &[&str]) -> BTreeSet<String> {
        names.iter().map(|name| name.to_string()).collect()
    }

    #[test]
    fn precedence_is_transitive() {
        let precedence = Precedence::new(&edges(&[("a", "b"), ("b", "c")]));
        assert!(precedence.outranks("a", "c"));
    }

    #[test]
    fn one_intention_on_both_sides_is_a_defect() {
        let precedence = Precedence::new(&edges(&[]));
        let verdict = classify(&[Some("safety".into()), Some("safety".into())], &precedence);
        assert_eq!(
            verdict,
            Verdict::Defect {
                intention: "safety".into()
            }
        );
    }

    #[test]
    fn a_declared_winner_adjudicates_the_trade_off() {
        let precedence = Precedence::new(&edges(&[("safety", "convenience")]));
        let verdict = classify(
            &[Some("safety".into()), Some("convenience".into())],
            &precedence,
        );
        assert_eq!(
            verdict,
            Verdict::Adjudicated {
                winner: "safety".into()
            }
        );
    }

    #[test]
    fn an_undeclared_pair_leaves_the_conflict_unadjudicated() {
        let precedence = Precedence::new(&edges(&[]));
        let verdict = classify(
            &[Some("safety".into()), Some("convenience".into())],
            &precedence,
        );
        assert_eq!(
            verdict,
            Verdict::Unadjudicated {
                missing: vec![("convenience".into(), "safety".into())]
            }
        );
    }

    #[test]
    fn several_maximal_elements_are_not_a_maximum() {
        // a > c and b > c, but a and b are unordered. Two maximal elements, no maximum, so there is
        // still no mechanical answer to which requirements win.
        let precedence = Precedence::new(&edges(&[("a", "c"), ("b", "c")]));
        assert_eq!(precedence.unique_maximum(&set(&["a", "b", "c"])), None);
    }

    #[test]
    fn a_maximum_over_three_intentions_is_found() {
        let precedence = Precedence::new(&edges(&[("a", "b"), ("b", "c")]));
        assert_eq!(
            precedence.unique_maximum(&set(&["a", "b", "c"])),
            Some("a".into())
        );
    }

    #[test]
    fn a_missing_intention_leaves_the_conflict_unclassified() {
        let precedence = Precedence::new(&edges(&[]));
        assert_eq!(
            classify(&[Some("safety".into()), None], &precedence),
            Verdict::Unclassified
        );
    }

    #[test]
    fn missing_comparisons_name_the_pairs_to_declare() {
        let precedence = Precedence::new(&edges(&[("a", "b")]));
        let missing = precedence.missing_comparisons(&set(&["a", "b", "c"]));
        assert!(missing.contains(&("a".into(), "c".into())));
        assert!(missing.contains(&("b".into(), "c".into())));
        assert!(!missing.contains(&("a".into(), "b".into())));
    }
}
