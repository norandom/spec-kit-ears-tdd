//! Turning a declared model into something finite, then into independent pieces.
//!
//! Two steps, in this order, and the order is the whole trick.
//!
//! **Finitize.** A boolean term has two values, an enumeration has its declared ones, and a bounded
//! integer is cut at the constants its guards compare against. A term declared over `[0, 10000]`
//! whose guards only mention `< 1000` has three regions, not ten thousand values — every value
//! inside a region makes every comparison in the model give the same answer, so one representative
//! stands for all of them. That is sound and complete precisely because comparisons are against
//! literals, which is why the guard grammar refuses anything else.
//!
//! **Decompose.** Two requirements can only interact if they mention a term in common, so a
//! union-find over shared terms splits one problem into many independent ones. This is what keeps
//! every later decision procedure cheap, and it is why the state space of a *component* — rather
//! than of the whole model — is the number that matters.
//!
//! Because a component's size is a product of declared domain sizes, it is known before any search
//! begins. An over-budget component is reported without evaluating a single state of it.

use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

use crate::guard::{Guard, Literal, Op};
use crate::vocabulary::{Domain, Term};

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EffectDeclaration {
    #[serde(default)]
    pub conflicts_with: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequirementEntry {
    /// Absent means unconditional. Not the same as a condition that happens to hold.
    #[serde(default)]
    pub when: Option<String>,
    pub then: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelFile {
    #[serde(default)]
    pub schema_version: Option<String>,
    #[serde(default)]
    pub effects: BTreeMap<String, EffectDeclaration>,
    #[serde(default)]
    pub requirements: BTreeMap<String, RequirementEntry>,
}

/// One concrete value a variable may take.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Bool(bool),
    Text(String),
    /// A representative and the closed region it stands for. Carrying the bounds lets a witness say
    /// `queue-depth ∈ [0, 999]` rather than naming an arbitrary number the reader has to interpret.
    IntRegion {
        representative: i64,
        low: i64,
        high: i64,
    },
}

impl std::fmt::Display for Value {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(value) => write!(formatter, "{value}"),
            Value::Text(value) => write!(formatter, "'{value}'"),
            Value::IntRegion { low, high, .. } if low == high => write!(formatter, "{low}"),
            Value::IntRegion { low, high, .. } => write!(formatter, "[{low}, {high}]"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub term: String,
    pub values: Vec<Value>,
}

/// Split points for an integer domain: the values at which some comparison changes its answer.
///
/// For `< k` the answer changes at `k`; for `<= k` at `k + 1`; equality changes twice, at `k` and
/// again at `k + 1`. Everything between two consecutive split points is indistinguishable to every
/// guard in the model, which is what licenses collapsing it to one representative.
fn split_points(comparisons: &[(Op, i64)], low: i64, high: i64) -> Vec<i64> {
    let mut points = BTreeSet::new();
    for (op, value) in comparisons {
        let starts: &[i64] = match op {
            Op::Less | Op::GreaterOrEqual => &[*value],
            Op::LessOrEqual | Op::Greater => &[*value + 1],
            Op::Equal | Op::NotEqual => &[*value, *value + 1],
        };
        for start in starts {
            if *start > low && *start <= high {
                points.insert(*start);
            }
        }
    }
    points.into_iter().collect()
}

fn integer_regions(low: i64, high: i64, comparisons: &[(Op, i64)]) -> Vec<Value> {
    let mut regions = Vec::new();
    let mut start = low;
    for point in split_points(comparisons, low, high) {
        regions.push(Value::IntRegion {
            representative: start,
            low: start,
            high: point - 1,
        });
        start = point;
    }
    regions.push(Value::IntRegion {
        representative: start,
        low: start,
        high,
    });
    regions
}

/// ears-sdd:allow-requirement-id: citing the requirement this contract enforces
/// The contract from REQ-025: the regions of an integer domain cover it exactly once. A gap hides
/// whole assignments, and a conflict living in the gap would never be found.
pub fn regions_cover_domain(regions: &[Value], low: i64, high: i64) -> bool {
    let mut expected = low;
    for region in regions {
        let Value::IntRegion {
            low: region_low,
            high: region_high,
            ..
        } = region
        else {
            return false;
        };
        if *region_low != expected || region_high < region_low {
            return false;
        }
        expected = region_high + 1;
    }
    expected == high.saturating_add(1)
}

/// Finitize one term against every comparison the model makes on it.
pub fn finitize(term: &str, domain: &Domain, comparisons: &[(Op, Literal)]) -> Variable {
    let values = match domain {
        Domain::Bool => vec![Value::Bool(false), Value::Bool(true)],
        Domain::Entity => vec![Value::Bool(false), Value::Bool(true)],
        Domain::Enum { values } => values.iter().cloned().map(Value::Text).collect(),
        Domain::Int { min, max } => {
            let integers: Vec<(Op, i64)> = comparisons
                .iter()
                .filter_map(|(op, literal)| match literal {
                    Literal::Int(value) => Some((*op, *value)),
                    _ => None,
                })
                .collect();
            integer_regions(*min, *max, &integers)
        }
    };
    Variable {
        term: term.to_string(),
        values,
    }
}

#[derive(Debug, Clone)]
pub struct ModelledRequirement {
    pub identifier: String,
    pub feature: String,
    pub guard: Guard,
    pub effect: String,
}

#[derive(Debug)]
pub struct Component {
    pub index: usize,
    pub variables: Vec<Variable>,
    pub requirements: Vec<ModelledRequirement>,
}

impl Component {
    /// The product of the domain sizes. Saturating, because the point of computing it is to refuse
    /// the cases that would overflow, and an overflow panic is a worse answer than a large number.
    pub fn state_space(&self) -> u64 {
        self.variables
            .iter()
            .try_fold(1u64, |total, variable| {
                total.checked_mul(variable.values.len().max(1) as u64)
            })
            .unwrap_or(u64::MAX)
    }

    /// This component restricted to the variables the given requirements actually mention.
    ///
    /// Grouping and deciding are different questions, and conflating them is what made the merged
    /// analysis exponential. A component exists so that requirements which can interact are
    /// considered together; it is not the space any single question ranges over. Every question the
    /// analysis asks is at most pairwise, and a guard constrains only the terms it names, so an
    /// assignment satisfying two guards over their own variables always extends to the rest of the
    /// component: the remaining domains are non-empty and nothing constrains them.
    ///
    /// Projecting onto that cone of influence is therefore exact rather than an approximation. On
    /// the twelve-feature example it takes the merged question from 100,663,296 states to at most
    /// 64, because no pair of requirements there mentions more than six terms between them.
    pub fn restricted_to(&self, requirements: &[&ModelledRequirement]) -> Component {
        let mentioned: BTreeSet<String> = requirements
            .iter()
            .flat_map(|requirement| requirement.guard.terms())
            .collect();
        Component {
            index: self.index,
            variables: self
                .variables
                .iter()
                .filter(|variable| mentioned.contains(&variable.term))
                .cloned()
                .collect(),
            requirements: requirements.iter().map(|r| (*r).clone()).collect(),
        }
    }

    /// The terms contributing most to that product, largest first. This is the lever a report hands
    /// back: narrowing the guards on these is what makes an over-budget component tractable.
    pub fn largest_contributors(&self, count: usize) -> Vec<(String, usize)> {
        let mut sizes: Vec<(String, usize)> = self
            .variables
            .iter()
            .map(|variable| (variable.term.clone(), variable.values.len()))
            .collect();
        sizes.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        sizes.truncate(count);
        sizes
    }
}

/// Union-find over terms, joined by co-occurrence in a requirement's guard.
struct DisjointSets {
    parent: BTreeMap<String, String>,
}

impl DisjointSets {
    fn new() -> Self {
        Self {
            parent: BTreeMap::new(),
        }
    }

    fn find(&mut self, item: &str) -> String {
        let current = self
            .parent
            .entry(item.to_string())
            .or_insert_with(|| item.to_string())
            .clone();
        if current == item {
            return current;
        }
        let root = self.find(&current);
        self.parent.insert(item.to_string(), root.clone());
        root
    }

    fn union(&mut self, left: &str, right: &str) {
        let (left_root, right_root) = (self.find(left), self.find(right));
        if left_root != right_root {
            // Joined in a fixed direction so the result never depends on iteration order.
            let (keep, merge) = if left_root <= right_root {
                (left_root, right_root)
            } else {
                (right_root, left_root)
            };
            self.parent.insert(merge, keep);
        }
    }
}

/// Split requirements into independent components, and finitize each component's variables.
///
/// An unconditional requirement joins *every* component. Its guard always holds, so it can conflict
/// with anything, and isolating it in a component of its own would silently lose exactly those
/// conflicts. That is the one case where the "shares a term" rule is not the whole story.
/// Whether two requirements assert effects that cannot both hold.
///
/// `conflicts` is the symmetric, normalized pair set built from the effect declarations.
fn effects_conflict(
    conflicts: &BTreeSet<(String, String)>,
    left: &ModelledRequirement,
    right: &ModelledRequirement,
) -> bool {
    let key = if left.effect <= right.effect {
        (left.effect.clone(), right.effect.clone())
    } else {
        (right.effect.clone(), left.effect.clone())
    };
    conflicts.contains(&key)
}

pub fn decompose(
    requirements: &[ModelledRequirement],
    terms: &BTreeMap<String, (Term, String)>,
    conflicts: &BTreeSet<(String, String)>,
) -> Vec<Component> {
    let mut sets = DisjointSets::new();
    for requirement in requirements {
        let mentioned: Vec<String> = requirement.guard.terms().into_iter().collect();
        for window in mentioned.windows(2) {
            sets.union(&window[0], &window[1]);
        }
        for term in &mentioned {
            sets.find(term);
        }
    }

    // Sharing a guard term is not the only way two requirements can depend on each other. Two that
    // assert conflicting effects must be evaluated together, because the assignment making both
    // guards true forces both effects, and that is a contradiction no matter how unrelated the
    // guards look.
    //
    // Splitting them leaves each component satisfiable and the contradiction invisible, which is
    // the worst available outcome: the gate reports a pass it has not earned. It is also the
    // ordinary cross-specification shape, since two features naturally guard on different
    // conditions -- one on `mitigation-enforced`, the other on `toolchain-building`.
    //
    // Requirements with empty guards need no union: they already join every component.
    for (index, left) in requirements.iter().enumerate() {
        for right in &requirements[index + 1..] {
            if !effects_conflict(conflicts, left, right) {
                continue;
            }
            let (Some(left_term), Some(right_term)) = (
                left.guard.terms().into_iter().next(),
                right.guard.terms().into_iter().next(),
            ) else {
                continue;
            };
            sets.union(&left_term, &right_term);
        }
    }

    let (unconditional, conditional): (Vec<_>, Vec<_>) = requirements
        .iter()
        .cloned()
        .partition(|requirement| requirement.guard.terms().is_empty());

    let mut grouped: BTreeMap<String, Vec<ModelledRequirement>> = BTreeMap::new();
    for requirement in conditional {
        let representative = requirement
            .guard
            .terms()
            .into_iter()
            .next()
            .map(|term| sets.find(&term))
            .unwrap_or_default();
        grouped.entry(representative).or_default().push(requirement);
    }

    // No conditional requirements at all: the unconditional ones still have to be checked against
    // each other, so they need somewhere to live.
    if grouped.is_empty() && !unconditional.is_empty() {
        grouped.insert(String::new(), Vec::new());
    }

    grouped
        .into_iter()
        .enumerate()
        .map(|(index, (_, mut members))| {
            members.extend(unconditional.iter().cloned());
            members.sort_by(|a, b| {
                (a.feature.as_str(), a.identifier.as_str())
                    .cmp(&(b.feature.as_str(), b.identifier.as_str()))
            });
            let variables = variables_for(&members, terms);
            Component {
                index,
                variables,
                requirements: members,
            }
        })
        .collect()
}

fn variables_for(
    members: &[ModelledRequirement],
    terms: &BTreeMap<String, (Term, String)>,
) -> Vec<Variable> {
    let mut comparisons: Vec<(String, Op, Literal)> = Vec::new();
    for requirement in members {
        requirement.guard.comparisons(&mut comparisons);
    }

    let mut mentioned: BTreeSet<String> = BTreeSet::new();
    for requirement in members {
        mentioned.extend(requirement.guard.terms());
    }

    mentioned
        .into_iter()
        .filter_map(|term| {
            let (declaration, _) = terms.get(&term)?;
            let relevant: Vec<(Op, Literal)> = comparisons
                .iter()
                .filter(|(name, _, _)| *name == term)
                .map(|(_, op, literal)| (*op, literal.clone()))
                .collect();
            Some(finitize(&term, &declaration.domain, &relevant))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn int_domain(min: i64, max: i64) -> Domain {
        Domain::Int { min, max }
    }

    #[test]
    fn an_unconstrained_integer_is_one_region() {
        let variable = finitize("depth", &int_domain(0, 10_000), &[]);
        assert_eq!(variable.values.len(), 1);
        assert!(regions_cover_domain(&variable.values, 0, 10_000));
    }

    #[test]
    fn a_single_less_than_cuts_the_domain_in_two() {
        // The whole point of the abstraction: ten thousand values become two.
        let variable = finitize(
            "depth",
            &int_domain(0, 10_000),
            &[(Op::Less, Literal::Int(1000))],
        );
        assert_eq!(variable.values.len(), 2);
        assert!(regions_cover_domain(&variable.values, 0, 10_000));
        assert_eq!(
            variable.values[0],
            Value::IntRegion {
                representative: 0,
                low: 0,
                high: 999
            }
        );
    }

    #[test]
    fn equality_isolates_the_value_it_names() {
        let variable = finitize("depth", &int_domain(0, 10), &[(Op::Equal, Literal::Int(5))]);
        assert!(regions_cover_domain(&variable.values, 0, 10));
        assert!(variable.values.contains(&Value::IntRegion {
            representative: 5,
            low: 5,
            high: 5
        }));
    }

    #[test]
    fn regions_always_cover_the_domain_exactly_once() {
        let comparisons = [
            (Op::Less, Literal::Int(3)),
            (Op::LessOrEqual, Literal::Int(7)),
            (Op::Greater, Literal::Int(5)),
            (Op::Equal, Literal::Int(9)),
            (Op::NotEqual, Literal::Int(1)),
        ];
        let variable = finitize("depth", &int_domain(0, 10), &comparisons);
        assert!(regions_cover_domain(&variable.values, 0, 10));
    }

    #[test]
    fn a_comparison_outside_the_domain_adds_no_region() {
        let variable = finitize(
            "depth",
            &int_domain(0, 10),
            &[(Op::Less, Literal::Int(500))],
        );
        assert_eq!(variable.values.len(), 1);
    }

    #[test]
    fn an_enumeration_uses_its_declared_values() {
        let domain = Domain::Enum {
            values: vec!["normal".into(), "maintenance".into(), "degraded".into()],
        };
        assert_eq!(finitize("mode", &domain, &[]).values.len(), 3);
    }
}
