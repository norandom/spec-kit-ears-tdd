//! Containment comparisons: is a candidate model's set of permitted states inside a reference's?
//!
//! The query is the one the cited OpenShell Research dev note states for agent policy
//! (`Allowed(candidate) ∖ Allowed(reference) = ∅`): `sat` means a widening with a concrete
//! witness, `unsat` means contained, and anything the encoding cannot decide is refused, never
//! reported as contained. This crate answers the same question over requirement models with BDDs
//! instead of an SMT solver, per the project constitution (single binary, deterministic verdicts).
//!
//! This slice implements the `revision` and `policy` kinds. `supersession` and `amendment`
//! comparisons are accepted in the manifest but reported `uncompared`; the `envelope` kind is
//! reserved for feature 010 and refused by name.

use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use biodivine_lib_bdd::Bdd;

use crate::bdd::Encoding;
use crate::config::Config;
use crate::guard::{self, Guard, Literal, Op};
use crate::model::{self, Component, ModelFile, ModelledRequirement, Value, Variable};
use crate::report::{ComparisonRecord, Finding, Severity};
use crate::vocabulary::{self, Domain, Term};

pub const MANIFEST_RELATIVE_PATH: &str = ".specify/containment.toml";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    #[serde(default)]
    schema_version: Option<String>,
    #[serde(default)]
    comparisons: Vec<ComparisonDecl>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ComparisonDecl {
    id: String,
    kind: String,
    reference: SourceDecl,
    #[serde(default)]
    candidates: Vec<SourceDecl>,
    // Amendment-pair selectors land with US4; parsed now so the manifest shape is stable.
    #[serde(default)]
    #[allow(dead_code)]
    reference_requirement: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    candidate_requirement: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct SourceDecl {
    /// Logical feature label. Optional for envelope sources, which name a definition instead.
    #[serde(default)]
    feature: Option<String>,
    #[serde(default)]
    spec: Option<String>,
    #[serde(default)]
    model: Option<String>,
    #[serde(default)]
    vocabulary: Option<String>,
    /// Envelope sources replace spec/model with a definition file and optional format.
    #[serde(default)]
    definition: Option<String>,
    #[serde(default)]
    format: Option<String>,
    #[serde(default)]
    project: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Revision,
    Policy,
    Supersession,
    Amendment,
    Envelope,
}

impl Kind {
    fn parse(text: &str, id: &str) -> Result<Kind, Box<Finding>> {
        match text {
            "revision" => Ok(Kind::Revision),
            "policy" => Ok(Kind::Policy),
            "supersession" => Ok(Kind::Supersession),
            "amendment" => Ok(Kind::Amendment),
            "envelope" => Ok(Kind::Envelope),
            other => Err(Box::new(Finding::new(
                "CONTAIN_INPUT_INVALID",
                format!("Comparison `{id}` declares unknown kind `{other}`."),
                MANIFEST_RELATIVE_PATH,
            ))),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Kind::Revision => "revision",
            Kind::Policy => "policy",
            Kind::Supersession => "supersession",
            Kind::Amendment => "amendment",
            Kind::Envelope => "envelope",
        }
    }

    /// The directions this kind turns into findings. The engine always computes both
    /// differences; the kind decides which become findings. This table is code, not config.
    fn reports(self) -> Vec<String> {
        match self {
            Kind::Revision | Kind::Envelope => {
                vec!["widening".to_string(), "narrowing".to_string()]
            }
            Kind::Policy => vec!["widening".to_string()],
            Kind::Supersession => vec!["narrowing".to_string()],
            Kind::Amendment => vec!["widening".to_string()],
        }
    }
}

pub(crate) struct Side {
    feature: String,
    model_display: String,
    model: ModelFile,
    modelled: Vec<ModelledRequirement>,
    unmodelled: Vec<String>,
    terms: BTreeMap<String, (Term, String)>,
    /// Envelope sources: JSON-pointer paths of constructs the reader did not model.
    unmodelled_paths: Vec<String>,
    /// Non-fatal findings the side's reader produced (e.g. withheld envelope transition effects).
    pending_findings: Vec<Finding>,
}

pub struct Outcome {
    pub findings: Vec<Finding>,
    pub comparisons: Vec<ComparisonRecord>,
    pub effects_compared: usize,
    pub effects_uncompared: usize,
    pub uncompared: usize,
}

/// Normalize a project-relative path and refuse traversal outside the project.
fn normalize_relative(path: &str) -> Result<String, String> {
    let slashed = path.replace('\\', "/");
    let stripped = slashed
        .strip_prefix("./")
        .unwrap_or(&slashed)
        .trim_start_matches('/');
    if stripped.split('/').any(|segment| segment == "..") {
        return Err(format!("path escapes the project: {path}"));
    }
    Ok(stripped.to_string())
}

fn load_side(root: &Path, source: &SourceDecl, is_reference: bool) -> Result<Side, Box<[Finding]>> {
    let feature = source
        .feature
        .clone()
        .or_else(|| source.definition.clone())
        .unwrap_or_else(|| "envelope".to_string());
    let bad = |code: &str, message: String| -> Box<[Finding]> {
        vec![Finding::new(code, message, MANIFEST_RELATIVE_PATH).feature(&feature)]
            .into_boxed_slice()
    };
    let missing_code = if is_reference {
        "CONTAIN_REFERENCE_MISSING"
    } else {
        "CONTAIN_INPUT_INVALID"
    };

    // Envelope sources name a definition file instead of spec/model; the adapter registry
    // decides whether the format is readable at all.
    if let Some(definition) = &source.definition {
        let format = source
            .format
            .as_deref()
            .unwrap_or(crate::adapters::INTERCHANGE_FORMAT);
        if crate::adapters::lookup(format).is_none() {
            let finding = Finding::new(
                "CONTAIN_DEFINITION_UNREADABLE",
                format!("Envelope format `{format}` has no registered adapter."),
                MANIFEST_RELATIVE_PATH,
            )
            .feature(&feature)
            .detail("reason", format);
            return Err(vec![finding].into_boxed_slice());
        }
        let def_rel =
            normalize_relative(definition).map_err(|message| bad(missing_code, message))?;
        let def_path = root.join(&def_rel);
        if !def_path.is_file() {
            return Err(bad(
                missing_code,
                format!("Envelope definition does not exist: {definition}"),
            ));
        }
        let Some(vocab) = &source.vocabulary else {
            return Err(bad(
                "CONTAIN_INPUT_INVALID",
                "An envelope source requires an explicit `vocabulary`.".to_string(),
            ));
        };
        let vocab_rel = normalize_relative(vocab).map_err(|message| bad(missing_code, message))?;
        let vocab_path = root.join(&vocab_rel);
        if !vocab_path.is_file() {
            return Err(bad(
                "CONTAIN_INPUT_INVALID",
                format!("Envelope vocabulary does not exist: {vocab}"),
            ));
        }
        let (vocab_terms, vocab_findings) = vocabulary::load_terms_at(&vocab_path, &vocab_rel);
        if !vocab_findings.is_empty() {
            return Err(vocab_findings.into_boxed_slice());
        }
        let text = std::fs::read_to_string(&def_path).map_err(|error| {
            bad(
                "CONTAIN_DEFINITION_UNREADABLE",
                format!("Envelope definition is unreadable: {error}"),
            )
        })?;
        let side = match crate::envelope::read_definition(&text, vocab_terms, &def_rel) {
            Ok(side) => side,
            Err(errs) => return Err(errs.into_boxed_slice()),
        };
        return Ok(Side {
            feature,
            model_display: def_rel,
            model: side.model,
            modelled: side.modelled,
            unmodelled: Vec::new(),
            terms: side.terms,
            unmodelled_paths: side.unmodelled,
            pending_findings: side.findings,
        });
    }

    // Only the spec path resolves inside an externally assessed tree; model and vocabulary
    // paths always resolve under the assessing project, so an assessed tree is never written to.
    let spec_root = match &source.project {
        Some(project) => {
            let normalized =
                normalize_relative(project).map_err(|message| bad(missing_code, message))?;
            root.join(&normalized)
        }
        None => root.to_path_buf(),
    };
    let spec = source.spec.as_ref().ok_or_else(|| {
        bad(
            "CONTAIN_INPUT_INVALID",
            "Source declares neither spec nor definition.".to_string(),
        )
    })?;
    let model_decl = source.model.as_ref().ok_or_else(|| {
        bad(
            "CONTAIN_INPUT_INVALID",
            "Source declares no model.".to_string(),
        )
    })?;
    let vocab_decl = source.vocabulary.as_ref().ok_or_else(|| {
        bad(
            "CONTAIN_INPUT_INVALID",
            "Source declares no vocabulary.".to_string(),
        )
    })?;
    let spec_rel = normalize_relative(spec).map_err(|message| bad(missing_code, message))?;
    let spec_path = spec_root.join(&spec_rel);
    let model_rel = normalize_relative(model_decl).map_err(|message| bad(missing_code, message))?;
    let model_path = root.join(&model_rel);
    let vocab_rel = normalize_relative(vocab_decl).map_err(|message| bad(missing_code, message))?;
    let vocab_path = root.join(&vocab_rel);

    if !spec_path.is_file() {
        return Err(bad(
            missing_code,
            format!("Source specification does not exist: {spec}"),
        ));
    }
    if !model_path.is_file() {
        return Err(bad(
            missing_code,
            format!("Source model does not exist: {model_decl}"),
        ));
    }

    let (spec_requirements, _) = crate::requirements::parse(&spec_root, &spec_path, &feature);
    let spec_ids: BTreeSet<String> = spec_requirements
        .iter()
        .map(|requirement| requirement.identifier.clone())
        .collect();

    let model_text = std::fs::read_to_string(&model_path)
        .map_err(|error| bad(missing_code, format!("Source model is unreadable: {error}")))?;
    let model: ModelFile = toml::from_str(&model_text).map_err(|error| {
        bad(
            missing_code,
            format!(
                "Source model {} does not parse: {}",
                model_decl,
                error.message()
            ),
        )
    })?;

    if !vocab_path.is_file() {
        return Err(bad(
            "CONTAIN_INPUT_INVALID",
            format!("Source vocabulary does not exist: {vocab_decl}"),
        ));
    }
    let (terms, vocab_findings) = vocabulary::load_terms_at(&vocab_path, &vocab_rel);
    if !vocab_findings.is_empty() {
        return Err(vocab_findings.into_boxed_slice());
    }

    let mut modelled = Vec::new();
    for (identifier, entry) in &model.requirements {
        let guard = match &entry.when {
            Some(text) => guard::parse(text).map_err(|error| {
                bad(
                    "CONTAIN_INPUT_INVALID",
                    format!("Guard for {identifier} in {model_decl} does not parse: {error}",),
                )
            })?,
            None => Guard::Always,
        };
        modelled.push(ModelledRequirement {
            identifier: identifier.clone(),
            feature: feature.clone(),
            guard,
            effect: entry.then.clone(),
        });
    }
    modelled.sort_by(|a, b| a.identifier.cmp(&b.identifier));

    let unmodelled: Vec<String> = spec_ids
        .into_iter()
        .filter(|id| !model.requirements.contains_key(id))
        .collect();

    Ok(Side {
        feature,
        model_display: model_rel,
        model,
        modelled,
        unmodelled,
        terms,
        unmodelled_paths: Vec::new(),
        pending_findings: Vec::new(),
    })
}

fn domain_kind(domain: &Domain) -> &'static str {
    match domain {
        Domain::Bool | Domain::Entity => "bool",
        Domain::Enum { .. } => "enum",
        Domain::Int { .. } => "int",
    }
}

/// A term the two sides do not share an identity for makes the comparison undecidable, so the
/// verdict is withheld rather than guessed from labels.
fn check_drift(reference: &Side, candidates: &[Side], comparison_id: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let mut used: BTreeSet<String> = BTreeSet::new();
    for side in std::iter::once(reference).chain(candidates.iter()) {
        for requirement in &side.modelled {
            used.extend(requirement.guard.terms());
        }
    }
    for term in used {
        let Some((ref_decl, _)) = reference.terms.get(&term) else {
            findings.push(
                Finding::new(
                    "CONTAIN_VOCAB_DRIFT",
                    format!(
                        "Term `{term}` is used by the candidate but not declared in the reference \
                         vocabulary; the comparison `{comparison_id}` is withheld."
                    ),
                    reference.model_display.clone(),
                )
                .detail("comparison", comparison_id.to_string()),
            );
            continue;
        };
        let mut candidate_decl: Option<&Term> = None;
        for candidate in candidates {
            if let Some((decl, _)) = candidate.terms.get(&term) {
                candidate_decl = Some(decl);
                break;
            }
        }
        let Some(cand_decl) = candidate_decl else {
            findings.push(
                Finding::new(
                    "CONTAIN_VOCAB_DRIFT",
                    format!(
                        "Term `{term}` is used by the reference but not declared in any candidate \
                         vocabulary; the comparison `{comparison_id}` is withheld."
                    ),
                    reference.model_display.clone(),
                )
                .detail("comparison", comparison_id.to_string()),
            );
            continue;
        };
        if domain_kind(&ref_decl.domain) != domain_kind(&cand_decl.domain)
            || ref_decl.deprecated
            || cand_decl.deprecated
        {
            findings.push(
                Finding::new(
                    "CONTAIN_VOCAB_DRIFT",
                    format!(
                        "Term `{term}` has no compatible identity on both sides of the comparison \
                         `{comparison_id}`; the verdict is withheld."
                    ),
                    reference.model_display.clone(),
                )
                .detail("comparison", comparison_id.to_string()),
            );
        }
    }
    findings
}

/// One shared encoding per question: every term the effect's contributors mention on either
/// side, finitized over the union of both sides' comparison constants and domain boundary cuts.
/// A question ranges only over its own cone of influence, so a witness names exactly the terms
/// the effect depends on.
fn question_component(
    reference: &Side,
    candidates: &[Side],
    contributors: &[&ModelledRequirement],
) -> Component {
    let mut comparisons: Vec<(String, Op, Literal)> = Vec::new();
    for requirement in contributors {
        requirement.guard.comparisons(&mut comparisons);
    }
    let mut terms_used: BTreeSet<String> = BTreeSet::new();
    for requirement in contributors {
        terms_used.extend(requirement.guard.terms());
    }

    let mut variables: Vec<Variable> = Vec::new();
    for term in terms_used {
        let (declaration, _) = reference
            .terms
            .get(&term)
            .or_else(|| candidates.iter().find_map(|c| c.terms.get(&term)))
            .expect("drift check ran first");
        let domain = match &declaration.domain {
            Domain::Int { min, max } => {
                // Shared hull: cut at both sides' boundaries so a region outside one side's
                // declared range is representable and then excluded by that side's mask.
                let mut low = *min;
                let mut high = *max;
                for candidate in candidates {
                    if let Some((cand, _)) = candidate.terms.get(&term) {
                        if let Domain::Int {
                            min: cmin,
                            max: cmax,
                        } = &cand.domain
                        {
                            low = low.min(*cmin);
                            high = high.max(*cmax);
                        }
                    }
                }
                Domain::Int {
                    min: low,
                    max: high,
                }
            }
            Domain::Enum { values } => {
                let mut union = values.clone();
                for candidate in candidates {
                    if let Some((cand, _)) = candidate.terms.get(&term) {
                        if let Domain::Enum { values: cvals } = &cand.domain {
                            union.extend(cvals.iter().cloned());
                        }
                    }
                }
                union.sort();
                union.dedup();
                Domain::Enum { values: union }
            }
            other => other.clone(),
        };
        let relevant: Vec<(Op, Literal)> = comparisons
            .iter()
            .filter(|(name, _, _)| *name == term)
            .map(|(_, op, literal)| (*op, literal.clone()))
            .collect();
        variables.push(model::finitize(&term, &domain, &relevant));
    }
    variables.sort_by(|a, b| a.term.cmp(&b.term));

    Component {
        index: 0,
        variables,
        requirements: Vec::new(),
    }
}

/// A side can only assert something about terms it declares: its mask is the conjunction of its
/// own domain bounds over the shared variables, and false over any term it does not declare.
fn side_mask(encoding: &Encoding, side: &Side, variables: &[Variable]) -> Bdd {
    let mut mask = encoding.encode(&Guard::Always);
    for variable in variables {
        let Some((declaration, _)) = side.terms.get(&variable.term) else {
            return encoding.encode(&Guard::Always).not();
        };
        let part = match &declaration.domain {
            Domain::Int { min, max } => Guard::And(vec![
                Guard::Compare {
                    term: variable.term.clone(),
                    op: Op::GreaterOrEqual,
                    value: Literal::Int(*min),
                },
                Guard::Compare {
                    term: variable.term.clone(),
                    op: Op::LessOrEqual,
                    value: Literal::Int(*max),
                },
            ]),
            Domain::Enum { values } => Guard::Or(
                values
                    .iter()
                    .map(|value| Guard::Compare {
                        term: variable.term.clone(),
                        op: Op::Equal,
                        value: Literal::Text(value.clone()),
                    })
                    .collect(),
            ),
            Domain::Bool | Domain::Entity => Guard::Always,
        };
        mask = mask.and(&encoding.encode(&part));
    }
    mask
}

fn region(encoding: &Encoding, mask: &Bdd, domain: &Bdd, side: &Side, effect: &str) -> Bdd {
    let mut region = encoding.encode(&Guard::Always).not();
    for requirement in &side.modelled {
        if requirement.effect == effect {
            region = region.or(&encoding.encode(&requirement.guard));
        }
    }
    region.and(mask).and(domain)
}

fn witness_json(assignment: &BTreeMap<String, Value>) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for (term, value) in assignment {
        let json = match value {
            Value::Bool(b) => serde_json::json!(b),
            Value::Text(t) => serde_json::json!(t),
            Value::IntRegion {
                representative,
                low,
                high,
            } => serde_json::json!({ "representative": representative, "low": low, "high": high }),
        };
        map.insert(term.clone(), json);
    }
    serde_json::Value::Object(map)
}

/// Confirm the witness with the independent evaluator: a positive attributed guard holds and no
/// opposing contributor holds. A mismatch is an internal error and the result is withheld.
fn verify_witness(
    assignment: &BTreeMap<String, Value>,
    positive: &[&ModelledRequirement],
    negative: &[&ModelledRequirement],
) -> bool {
    positive
        .iter()
        .any(|requirement| crate::enumerate::evaluate(&requirement.guard, assignment))
        && negative
            .iter()
            .all(|requirement| !crate::enumerate::evaluate(&requirement.guard, assignment))
}

/// Whether a kind constructs conflicting-effect questions. Fixed in code: only `policy` asks,
/// because a maximum policy's whole point is that its conflicts are live.
fn asks_conflict_questions(kind: Kind) -> bool {
    kind == Kind::Policy
}

struct ComparisonRun {
    record: ComparisonRecord,
    findings: Vec<Finding>,
    effects_compared: usize,
    effects_uncompared: usize,
    uncompared_ids: Vec<String>,
}

fn run_comparison(
    root: &Path,
    config: &Config,
    decl: &ComparisonDecl,
    kind: Kind,
) -> ComparisonRun {
    let mut findings = Vec::new();
    let failure = |record: ComparisonRecord, extra: Vec<Finding>| ComparisonRun {
        record,
        findings: extra,
        effects_compared: 0,
        effects_uncompared: 0,
        uncompared_ids: Vec::new(),
    };
    let uncompared_record = |findings: Vec<Finding>| {
        failure(
            ComparisonRecord {
                id: decl.id.clone(),
                kind: kind.as_str().to_string(),
                reports: kind.reports(),
                status: "uncompared".to_string(),
                complete: false,
                effects_compared: 0,
                effects_uncompared: 0,
                requirements_uncompared: 0,
                reachability_computed: None,
                unmodelled: None,
                adapter: None,
            },
            findings,
        )
    };

    if decl.candidates.is_empty() {
        findings.push(Finding::new(
            "CONTAIN_INPUT_INVALID",
            format!("Comparison `{}` declares no candidates.", decl.id),
            MANIFEST_RELATIVE_PATH,
        ));
        return uncompared_record(findings);
    }
    if decl.candidates.len() > 1 && kind != Kind::Supersession {
        findings.push(Finding::new(
            "CONTAIN_INPUT_INVALID",
            format!(
                "Comparison `{}` declares multiple candidates, which only `supersession` allows.",
                decl.id
            ),
            MANIFEST_RELATIVE_PATH,
        ));
        return uncompared_record(findings);
    }

    // An amendment compares exactly one requirement pair; without it the comparison would
    // silently widen to whole files.
    if kind == Kind::Amendment
        && (decl.reference_requirement.is_none() || decl.candidate_requirement.is_none())
    {
        findings.push(Finding::new(
            "CONTAIN_INPUT_INVALID",
            format!(
                "Comparison `{}` of kind `amendment` needs both reference_requirement and \
                 candidate_requirement.",
                decl.id
            ),
            MANIFEST_RELATIVE_PATH,
        ));
        return uncompared_record(findings);
    }

    let reference = match load_side(root, &decl.reference, true) {
        Ok(side) => side,
        Err(errs) => return uncompared_record(errs.into_vec()),
    };
    let mut candidates = Vec::new();
    for source in &decl.candidates {
        match load_side(root, source, false) {
            Ok(side) => candidates.push(side),
            Err(errs) => return uncompared_record(errs.into_vec()),
        }
    }
    let candidate = &candidates[0];

    // Non-fatal reader findings (e.g. a withheld envelope transition effect) are part of the
    // comparison's output even though the comparison still runs.
    findings.extend(reference.pending_findings.iter().cloned());
    for cand in &candidates {
        findings.extend(cand.pending_findings.iter().cloned());
    }

    let mut uncompared_ids: Vec<String> = Vec::new();
    for side in std::iter::once(&reference).chain(candidates.iter()) {
        for id in &side.unmodelled {
            uncompared_ids.push(format!("{}:{id}", side.feature));
        }
    }
    uncompared_ids.sort();
    uncompared_ids.dedup();

    let drift = check_drift(&reference, &candidates, &decl.id);
    if !drift.is_empty() {
        findings.extend(drift);
        return uncompared_record(findings);
    }

    let budget = config.state_space_budget;

    let mut effects: BTreeSet<String> = reference.model.effects.keys().cloned().collect();
    for candidate in &candidates {
        effects.extend(candidate.model.effects.keys().cloned());
    }

    // Derogation entries are validated up front: an unknown target or a malformed entry is an
    // input error, not something a completed question can excuse. No exception can waive a
    // genuinely new effect, because there is no same-effect reference requirement to point at.
    // Only entries that pass every check may cover a widening.
    let mut valid_derogations: BTreeSet<String> = BTreeSet::new();
    for (cand_id, derogation) in &candidate.model.derogations {
        let Some(cand_entry) = candidate.model.requirements.get(cand_id) else {
            findings.push(
                Finding::new(
                    "CONTAIN_DEROGATION_INVALID",
                    format!(
                        "Derogation names unknown candidate requirement `{cand_id}` in \
                         comparison `{}`.",
                        decl.id
                    ),
                    candidate.model_display.clone(),
                )
                .feature(&candidate.feature)
                .detail("comparison", decl.id.clone()),
            );
            continue;
        };
        let Some(ref_entry) = reference.model.requirements.get(&derogation.reference) else {
            findings.push(
                Finding::new(
                    "CONTAIN_DEROGATION_UNKNOWN",
                    format!(
                        "Derogation for `{cand_id}` in comparison `{}` names unknown reference \
                         requirement `{}`.",
                        decl.id, derogation.reference
                    ),
                    candidate.model_display.clone(),
                )
                .feature(&candidate.feature)
                .detail("comparison", decl.id.clone()),
            );
            continue;
        };
        if ref_entry.then != cand_entry.then {
            findings.push(
                Finding::new(
                    "CONTAIN_DEROGATION_INVALID",
                    format!(
                        "Derogation for `{cand_id}` in comparison `{}` targets `{}`, which asserts \
                         effect `{}` rather than `{}`.",
                        decl.id, derogation.reference, ref_entry.then, cand_entry.then
                    ),
                    candidate.model_display.clone(),
                )
                .feature(&candidate.feature)
                .detail("comparison", decl.id.clone()),
            );
            continue;
        }
        if derogation.reason.trim().chars().count() < 10 {
            findings.push(
                Finding::new(
                    "CONTAIN_DEROGATION_INVALID",
                    format!(
                        "Derogation reason for `{cand_id}` in comparison `{}` is too short; a real \
                         reason is at least 10 characters.",
                        decl.id
                    ),
                    candidate.model_display.clone(),
                )
                .feature(&candidate.feature)
                .detail("comparison", decl.id.clone()),
            );
            continue;
        }
        valid_derogations.insert(cand_id.clone());
    }

    let mut effects_compared = 0usize;
    let mut effects_uncompared = 0usize;
    let mut any_widening = false;
    let mut any_narrowing = false;

    for effect in &effects {
        // An amendment compares exactly the declared requirement pair on each side.
        let cand_contributors: Vec<&ModelledRequirement> = candidate
            .modelled
            .iter()
            .filter(|r| r.effect == *effect)
            .filter(|r| {
                kind != Kind::Amendment
                    || Some(&r.identifier) == decl.candidate_requirement.as_ref()
            })
            .collect();
        let ref_contributors: Vec<&ModelledRequirement> = reference
            .modelled
            .iter()
            .filter(|r| r.effect == *effect)
            .filter(|r| {
                kind != Kind::Amendment
                    || Some(&r.identifier) == decl.reference_requirement.as_ref()
            })
            .collect();
        let mut contributors: Vec<&ModelledRequirement> = ref_contributors.clone();
        contributors.extend(cand_contributors.iter().copied());

        let component = question_component(&reference, &candidates, &contributors);
        let states = component.state_space();
        if states > budget {
            let largest: Vec<serde_json::Value> = component
                .largest_contributors(3)
                .into_iter()
                .map(|(term, size)| serde_json::json!({ "term": term, "values": size }))
                .collect();
            findings.push(
                Finding::new(
                    "CONTAIN_BUDGET_EXCEEDED",
                    format!(
                        "Comparison `{}` effect `{effect}` has {} states against a budget of {}; \
                         the question was refused before any state was evaluated.",
                        decl.id, states, budget
                    ),
                    candidate.model_display.clone(),
                )
                .feature(&candidate.feature)
                .detail("comparison", decl.id.clone())
                .detail("effect", effect.clone())
                .detail("states", states)
                .detail("budget", budget)
                .detail("largest_contributors", serde_json::Value::Array(largest)),
            );
            effects_uncompared += 1;
            continue;
        }

        let encoding = Encoding::new(&component);
        let domain = encoding.domain().clone();
        let ref_mask = side_mask(&encoding, &reference, &component.variables);
        let cand_mask = side_mask(&encoding, candidate, &component.variables);
        let ref_region = region(&encoding, &ref_mask, &domain, &reference, effect);
        // A supersession compares the union of all live successors, each masked by its own
        // domain, against the predecessor.
        let cand_region = if kind == Kind::Supersession {
            let mut union = encoding.encode(&Guard::Always).not();
            for cand in &candidates {
                let mask = side_mask(&encoding, cand, &component.variables);
                union = union.or(&region(&encoding, &mask, &domain, cand, effect));
            }
            union
        } else {
            region(&encoding, &cand_mask, &domain, candidate, effect)
        };
        let widening = cand_region.and(&ref_region.not());
        let narrowing = ref_region.and(&cand_region.not());
        let widened = !widening.is_false();
        let narrowed = !narrowing.is_false();
        any_widening |= widened;
        any_narrowing |= narrowed;
        effects_compared += 1;
        let qualified = |list: &[&ModelledRequirement]| -> Vec<String> {
            let mut v: Vec<String> = list
                .iter()
                .map(|r| format!("{}:{}", r.feature, r.identifier))
                .collect();
            v.sort();
            v
        };

        let new_effect = !reference.model.effects.contains_key(effect);
        if new_effect && !cand_contributors.is_empty() {
            findings.push(
                Finding::new(
                    "CONTAIN_EFFECT_NEW",
                    format!(
                        "Comparison `{}`: candidate declares effect `{effect}`, absent from the \
                         reference; every state asserting it is an undeclarable widening.",
                        decl.id
                    ),
                    candidate.model_display.clone(),
                )
                .feature(&candidate.feature)
                .severity(Severity::Advisory)
                .detail("comparison", decl.id.clone())
                .detail("effect", effect.clone()),
            );
        }

        if widened
            && matches!(
                kind,
                Kind::Revision | Kind::Policy | Kind::Amendment | Kind::Envelope
            )
        {
            let assignment = encoding.witness(&widening);
            let verified = assignment
                .as_ref()
                .map(|a| verify_witness(a, &cand_contributors, &ref_contributors))
                .unwrap_or(false);
            if !verified {
                findings.push(crate::enumerate::internal_error(
                    format!(
                        "Witness for effect `{effect}` in comparison `{}` failed independent \
                         verification; the result is withheld.",
                        decl.id
                    ),
                    &candidate.model_display,
                ));
                continue;
            }
            let witness = assignment
                .map(|a| witness_json(&a))
                .unwrap_or(serde_json::Value::Null);
            // A widening is declared only if every contributor with a nonempty widening region
            // has a valid derogation. Usage is evaluated over the whole region, not the witness.
            let mut any_contrib_widening = false;
            let mut all_covered = true;
            for contributor in &cand_contributors {
                let q = encoding
                    .encode(&contributor.guard)
                    .and(&cand_mask)
                    .and(&domain)
                    .and(&ref_region.not());
                if !q.is_false() {
                    any_contrib_widening = true;
                    if !valid_derogations.contains(&contributor.identifier) {
                        all_covered = false;
                    }
                }
            }
            let (code, severity) = if all_covered && any_contrib_widening {
                ("CONTAIN_WIDENED_DECLARED", Severity::Advisory)
            } else {
                ("CONTAIN_WIDENED", Severity::Error)
            };
            let mut finding = Finding::new(
                code,
                format!(
                    "Comparison `{}`: candidate permits effect `{effect}` in states the reference \
                     does not{}.",
                    decl.id,
                    if code == "CONTAIN_WIDENED_DECLARED" {
                        ", fully covered by declared derogations"
                    } else {
                        ""
                    }
                ),
                candidate.model_display.clone(),
            )
            .feature(&candidate.feature)
            .severity(severity)
            .detail("comparison", decl.id.clone())
            .detail("effect", effect.clone())
            .detail("witness", witness)
            .detail(
                "candidate_requirements",
                serde_json::json!(qualified(&cand_contributors)),
            );
            if let Some(first) = cand_contributors.first() {
                finding = finding.requirement(&first.identifier);
            }
            findings.push(finding);
        }

        if narrowed && kind == Kind::Supersession {
            let assignment = encoding.witness(&narrowing);
            let witness = assignment
                .map(|a| witness_json(&a))
                .unwrap_or(serde_json::Value::Null);
            let mut finding = Finding::new(
                "CONTAIN_SUPERSESSION_GAP",
                format!(
                    "Comparison `{}`: the successor union loses effect `{effect}` in states the \
                     predecessor obligated.",
                    decl.id
                ),
                candidate.model_display.clone(),
            )
            .feature(&candidate.feature)
            .detail("comparison", decl.id.clone())
            .detail("effect", effect.clone())
            .detail("witness", witness)
            .detail(
                "reference_requirements",
                serde_json::json!(qualified(&ref_contributors)),
            );
            if let Some(first) = ref_contributors.first() {
                finding = finding.requirement(&first.identifier);
            }
            findings.push(finding);
        }

        if narrowed && matches!(kind, Kind::Revision | Kind::Envelope) {
            let assignment = encoding.witness(&narrowing);
            let witness = assignment
                .map(|a| witness_json(&a))
                .unwrap_or(serde_json::Value::Null);
            let mut finding = Finding::new(
                "CONTAIN_NARROWED",
                format!(
                    "Comparison `{}`: reference obligated effect `{effect}` in states the \
                     candidate no longer covers.",
                    decl.id
                ),
                candidate.model_display.clone(),
            )
            .feature(&candidate.feature)
            .severity(Severity::Warning)
            .detail("comparison", decl.id.clone())
            .detail("effect", effect.clone())
            .detail("witness", witness)
            .detail(
                "reference_requirements",
                serde_json::json!(qualified(&ref_contributors)),
            );
            if let Some(first) = ref_contributors.first() {
                finding = finding.requirement(&first.identifier);
            }
            findings.push(finding);
        }

        // A valid derogation whose candidate has no widening in this completed question is
        // stale: it records a decision that no longer matches the models.
        for cand_id in &valid_derogations {
            if let Some(c) = candidate
                .modelled
                .iter()
                .find(|r| &r.identifier == cand_id && r.effect == *effect)
            {
                let q = encoding
                    .encode(&c.guard)
                    .and(&cand_mask)
                    .and(&domain)
                    .and(&ref_region.not());
                if q.is_false() {
                    findings.push(
                        Finding::new(
                            "CONTAIN_DEROGATION_STALE",
                            format!(
                                "Derogation for `{cand_id}` in comparison `{}` covers no widening \
                                 of effect `{effect}`.",
                                decl.id
                            ),
                            candidate.model_display.clone(),
                        )
                        .feature(&candidate.feature)
                        .severity(Severity::Warning)
                        .detail("comparison", decl.id.clone())
                        .detail("effect", effect.clone()),
                    );
                }
            }
        }
    }

    // Under `policy` only, ask the conflicting-effect question per cross-side pair.
    if asks_conflict_questions(kind) {
        let mut conflict_pairs: BTreeSet<(String, String)> = BTreeSet::new();
        for side in std::iter::once(&reference).chain(candidates.iter()) {
            for (name, declaration) in &side.model.effects {
                for other in &declaration.conflicts_with {
                    let (low, high) = if name <= other {
                        (name.clone(), other.clone())
                    } else {
                        (other.clone(), name.clone())
                    };
                    conflict_pairs.insert((low, high));
                }
            }
        }
        for (left, right) in &conflict_pairs {
            // One question per unordered pair: either cross direction active in the same state
            // is the conflict, so ask the union rather than reporting the pair twice.
            if !candidate.model.effects.contains_key(left)
                || !candidate.model.effects.contains_key(right)
                || !reference.model.effects.contains_key(left)
                || !reference.model.effects.contains_key(right)
            {
                continue;
            }
            {
                let cand_effect = left.as_str();
                let ref_effect = right.as_str();
                let cand_contributors: Vec<&ModelledRequirement> = candidate
                    .modelled
                    .iter()
                    .filter(|r| r.effect == cand_effect)
                    .collect();
                let ref_contributors: Vec<&ModelledRequirement> = reference
                    .modelled
                    .iter()
                    .filter(|r| r.effect == ref_effect)
                    .collect();
                let mut contributors: Vec<&ModelledRequirement> = ref_contributors.clone();
                contributors.extend(cand_contributors.iter().copied());
                let component = question_component(&reference, &candidates, &contributors);
                if component.state_space() > budget {
                    continue;
                }
                let encoding = Encoding::new(&component);
                let domain = encoding.domain().clone();
                let ref_mask = side_mask(&encoding, &reference, &component.variables);
                let cand_mask = side_mask(&encoding, candidate, &component.variables);
                let question = region(&encoding, &cand_mask, &domain, candidate, cand_effect)
                    .and(&region(
                        &encoding, &ref_mask, &domain, &reference, ref_effect,
                    ))
                    .or(
                        &region(&encoding, &cand_mask, &domain, candidate, ref_effect).and(
                            &region(&encoding, &ref_mask, &domain, &reference, cand_effect),
                        ),
                    );
                if question.is_false() {
                    continue;
                }
                let assignment = encoding.witness(&question);
                let witness = assignment
                    .map(|a| witness_json(&a))
                    .unwrap_or(serde_json::Value::Null);
                findings.push(
                    Finding::new(
                        "CONTAIN_BASELINE_CONFLICT",
                        format!(
                            "Comparison `{}`: candidate effect `{cand_effect}` and reference \
                             effect `{ref_effect}` are both active in the same state.",
                            decl.id
                        ),
                        candidate.model_display.clone(),
                    )
                    .feature(&candidate.feature)
                    .detail("comparison", decl.id.clone())
                    .detail("effect", cand_effect.to_string())
                    .detail("conflicts_with", ref_effect.to_string())
                    .detail("witness", witness),
                );
            }
        }
    }

    let status = if effects_uncompared > 0 {
        // A refused question is recorded, not guessed: the verdict is indeterminate.
        "indeterminate"
    } else if any_widening && any_narrowing {
        "changed"
    } else if any_widening {
        "widened"
    } else if any_narrowing {
        "narrowed"
    } else {
        "equal"
    };

    // Envelope comparisons disclose what they did not compute: reachability over the transition
    // relation is never inferred, and every unmodelled construct is named by JSON-pointer path.
    let (reachability_computed, unmodelled, adapter) = if kind == Kind::Envelope {
        for path in &candidate.unmodelled_paths {
            let mut finding = Finding::new(
                "CONTAIN_UNMODELLED_CONSTRUCT",
                format!(
                    "Envelope field `{path}` is not modelled; the comparison is incomplete over it."
                ),
                candidate.model_display.clone(),
            )
            .severity(Severity::Warning)
            .detail("comparison", decl.id.clone())
            .detail("path", path.clone());
            if let Some(phase) = path
                .strip_prefix("/phases/")
                .and_then(|rest| rest.split('/').next())
            {
                finding = finding.requirement(format!("PHASE-{phase}"));
            }
            findings.push(finding);
        }
        findings.push(
            Finding::new(
                "CONTAIN_REACHABILITY_OMITTED",
                format!(
                    "Comparison `{}` compared phases without computing reachability; unreachable \
                     phases are still compared.",
                    decl.id
                ),
                candidate.model_display.clone(),
            )
            .severity(Severity::Advisory)
            .detail("comparison", decl.id.clone()),
        );
        (
            Some(false),
            Some(candidate.unmodelled_paths.clone()),
            Some(crate::adapters::INTERCHANGE_FORMAT.to_string()),
        )
    } else {
        (None, None, None)
    };

    ComparisonRun {
        record: ComparisonRecord {
            id: decl.id.clone(),
            kind: kind.as_str().to_string(),
            reports: kind.reports(),
            status: status.to_string(),
            complete: effects_uncompared == 0,
            effects_compared,
            effects_uncompared,
            requirements_uncompared: uncompared_ids.len(),
            reachability_computed,
            unmodelled,
            adapter,
        },
        findings,
        effects_compared,
        effects_uncompared,
        uncompared_ids,
    }
}

pub fn validate(
    root: &Path,
    config: &Config,
    features_in_scope: &BTreeSet<String>,
    narrowed: bool,
) -> Outcome {
    let mut findings = Vec::new();
    let manifest_path: PathBuf = root.join(MANIFEST_RELATIVE_PATH);

    if !manifest_path.is_file() {
        findings.push(
            Finding::new(
                "CONTAIN_UNCOMPARED",
                "Containment is enabled but no manifest exists; no comparisons were run, and this \
                 is not a pass of zero comparisons.",
                MANIFEST_RELATIVE_PATH,
            )
            .severity(Severity::Warning),
        );
        return Outcome {
            findings,
            comparisons: Vec::new(),
            effects_compared: 0,
            effects_uncompared: 0,
            uncompared: 0,
        };
    }

    let text = match std::fs::read_to_string(&manifest_path) {
        Ok(text) => text,
        Err(error) => {
            findings.push(Finding::new(
                "CONTAIN_INPUT_INVALID",
                format!("Containment manifest is unreadable: {error}"),
                MANIFEST_RELATIVE_PATH,
            ));
            return Outcome {
                findings,
                comparisons: Vec::new(),
                effects_compared: 0,
                effects_uncompared: 0,
                uncompared: 0,
            };
        }
    };
    let manifest: Manifest = match toml::from_str(&text) {
        Ok(manifest) => manifest,
        Err(error) => {
            findings.push(Finding::new(
                "CONTAIN_INPUT_INVALID",
                format!("Containment manifest does not parse: {}", error.message()),
                MANIFEST_RELATIVE_PATH,
            ));
            return Outcome {
                findings,
                comparisons: Vec::new(),
                effects_compared: 0,
                effects_uncompared: 0,
                uncompared: 0,
            };
        }
    };
    if manifest.schema_version.as_deref() != Some("1.0") {
        findings.push(Finding::new(
            "CONTAIN_INPUT_INVALID",
            format!(
                "Containment manifest declares unsupported schema_version `{:?}`.",
                manifest.schema_version
            ),
            MANIFEST_RELATIVE_PATH,
        ));
        return Outcome {
            findings,
            comparisons: Vec::new(),
            effects_compared: 0,
            effects_uncompared: 0,
            uncompared: 0,
        };
    }

    let mut seen: BTreeSet<String> = BTreeSet::new();
    for decl in &manifest.comparisons {
        if !seen.insert(decl.id.clone()) {
            findings.push(Finding::new(
                "CONTAIN_INPUT_INVALID",
                format!("Duplicate comparison id `{}`.", decl.id),
                MANIFEST_RELATIVE_PATH,
            ));
        }
    }
    if findings.iter().any(|f| f.code == "CONTAIN_INPUT_INVALID") {
        return Outcome {
            findings,
            comparisons: Vec::new(),
            effects_compared: 0,
            effects_uncompared: 0,
            uncompared: 0,
        };
    }

    let mut decls: Vec<(&ComparisonDecl, Kind)> = Vec::new();
    for decl in &manifest.comparisons {
        match Kind::parse(&decl.kind, &decl.id) {
            Ok(kind) => decls.push((decl, kind)),
            Err(finding) => findings.push(*finding),
        }
    }
    if !findings.is_empty() {
        return Outcome {
            findings,
            comparisons: Vec::new(),
            effects_compared: 0,
            effects_uncompared: 0,
            uncompared: 0,
        };
    }

    // A narrowed scope selects the comparisons whose candidate feature is in scope.
    let selected: Vec<(&ComparisonDecl, Kind)> = if narrowed {
        decls
            .into_iter()
            .filter(|(decl, _)| {
                decl.candidates.iter().any(|c| {
                    c.feature
                        .as_ref()
                        .map(|f| features_in_scope.contains(f))
                        .unwrap_or(false)
                })
            })
            .collect()
    } else {
        decls
    };
    if selected.is_empty() {
        findings.push(
            Finding::new(
                "CONTAIN_UNCOMPARED",
                "Containment is enabled but the selected scope contains no comparisons.",
                MANIFEST_RELATIVE_PATH,
            )
            .severity(Severity::Warning),
        );
        return Outcome {
            findings,
            comparisons: Vec::new(),
            effects_compared: 0,
            effects_uncompared: 0,
            uncompared: 0,
        };
    }

    let mut comparisons = Vec::new();
    let mut effects_compared = 0usize;
    let mut effects_uncompared = 0usize;
    let mut uncompared_ids: BTreeSet<String> = BTreeSet::new();
    for (decl, kind) in selected {
        let run = run_comparison(root, config, decl, kind);
        effects_compared += run.effects_compared;
        effects_uncompared += run.effects_uncompared;
        uncompared_ids.extend(run.uncompared_ids);
        findings.extend(run.findings);
        comparisons.push(run.record);
    }
    comparisons.sort_by(|a, b| a.id.cmp(&b.id));

    Outcome {
        findings,
        comparisons,
        effects_compared,
        effects_uncompared,
        uncompared: uncompared_ids.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vocabulary::{Domain, Term};

    fn term(domain: Domain) -> Term {
        Term {
            label: "L".to_string(),
            definition: "A test term.".to_string(),
            domain,
            broader: Vec::new(),
            alt_labels: Vec::new(),
            deprecated: false,
            replaced_by: None,
        }
    }

    fn terms_of(entries: &[(&str, Domain)]) -> BTreeMap<String, (Term, String)> {
        entries
            .iter()
            .map(|(name, domain)| (name.to_string(), (term(domain.clone()), "v".to_string())))
            .collect()
    }

    fn side(feature: &str, model_toml: &str, terms: BTreeMap<String, (Term, String)>) -> Side {
        let model: ModelFile = toml::from_str(model_toml).expect("test model parses");
        let modelled = model
            .requirements
            .iter()
            .map(|(id, entry)| ModelledRequirement {
                identifier: id.clone(),
                feature: feature.to_string(),
                guard: match &entry.when {
                    Some(text) => guard::parse(text).expect("test guard parses"),
                    None => Guard::Always,
                },
                effect: entry.then.clone(),
            })
            .collect();
        Side {
            feature: feature.to_string(),
            model_display: "model.toml".to_string(),
            model,
            modelled,
            unmodelled: Vec::new(),
            terms,
            unmodelled_paths: Vec::new(),
            pending_findings: Vec::new(),
        }
    }

    #[test]
    fn comparison_inputs_are_explicit() {
        // Unknown manifest fields are rejected, so a mistyped key cannot silently widen input.
        let bogus = "schema_version = \"1.0\"\n[[comparisons]]\nid = \"x\"\nkind = \"revision\"\nbogus = 1\nreference = { feature = \"f\", spec = \"s\", model = \"m\", vocabulary = \"v\" }\ncandidates = [{ feature = \"f\", spec = \"s\", model = \"m\", vocabulary = \"v\" }]\n";
        assert!(toml::from_str::<Manifest>(bogus).is_err());

        // All five kinds parse; anything else is refused by name.
        for kind in [
            "revision",
            "policy",
            "supersession",
            "amendment",
            "envelope",
        ] {
            assert!(Kind::parse(kind, "x").is_ok());
        }
        assert!(Kind::parse("nonsense", "x").is_err());
    }

    #[test]
    fn one_shared_variable_ordering() {
        let bool = Domain::Bool;
        let reference = side(
            "r",
            "[effects]\ne = {}\n[requirements.T-001]\nwhen = \"operator-present\"\nthen = \"e\"\n",
            terms_of(&[("operator-present", bool.clone())]),
        );
        let candidate = side(
            "c",
            "[effects]\ne = {}\n[requirements.T-001]\nwhen = \"operator-present\"\nthen = \"e\"\n",
            terms_of(&[("operator-present", bool)]),
        );
        let contributors: Vec<&ModelledRequirement> = reference
            .modelled
            .iter()
            .chain(candidate.modelled.iter())
            .collect();
        let component =
            question_component(&reference, std::slice::from_ref(&candidate), &contributors);
        // Both sides mention the same one term: exactly one shared variable, canonically ordered.
        assert_eq!(component.variables.len(), 1);
        assert_eq!(component.variables[0].term, "operator-present");
    }

    #[test]
    fn shared_encoding_preserves_distinct_states() {
        let variable = model::finitize(
            "mode",
            &Domain::Enum {
                values: vec!["a".to_string(), "b".to_string()],
            },
            &[],
        );
        assert_eq!(variable.values.len(), 2);
        let component = Component {
            index: 0,
            variables: vec![variable],
            requirements: Vec::new(),
        };
        // Two declared enum members are two distinct states, not one merged region.
        assert_eq!(component.state_space(), 2);
    }

    #[test]
    fn witness_satisfies_attributed_guards() {
        let requirement = ModelledRequirement {
            identifier: "T-001".to_string(),
            feature: "f".to_string(),
            guard: guard::parse("operator-present").unwrap(),
            effect: "e".to_string(),
        };
        let variable = model::finitize("operator-present", &Domain::Bool, &[]);
        let component = Component {
            index: 0,
            variables: vec![variable],
            requirements: Vec::new(),
        };
        let encoding = Encoding::new(&component);
        let region = encoding.encode(&requirement.guard).and(encoding.domain());
        let witness = encoding.witness(&region).expect("guard is satisfiable");
        assert!(verify_witness(&witness, &[&requirement], &[]));
        // A "negative" side the witness actually satisfies fails verification: the witness must
        // be absent from the opposing region, not merely present in the positive one.
        let opposing = requirement.clone();
        assert!(!verify_witness(&witness, &[&requirement], &[&opposing]));
    }

    #[test]
    fn conflict_questions_only_under_policy() {
        assert!(asks_conflict_questions(Kind::Policy));
        assert!(!asks_conflict_questions(Kind::Revision));
        assert!(!asks_conflict_questions(Kind::Supersession));
        assert!(!asks_conflict_questions(Kind::Amendment));
    }

    /// An over-budget question is refused before any BDD is built or state evaluated: the
    /// finding says so, the effect counts as uncompared, and nothing is reported as compared.
    #[test]
    fn over_budget_does_not_evaluate() {
        let dir = std::env::temp_dir().join(format!("ears-contain-budget-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let specify = dir.join(".specify");
        let feature = dir.join("specs").join("001-alpha");
        std::fs::create_dir_all(&specify).unwrap();
        std::fs::create_dir_all(&feature).unwrap();
        std::fs::write(
            specify.join("ears-sdd.toml"),
            "state_space_budget = 3\n[checks]\nvocabulary = true\nconstraints = true\ncontainment = true\n",
        )
        .unwrap();
        std::fs::write(
            specify.join("vocabulary.toml"),
            "schema_version = \"1.0\"\n[terms.depth]\nlabel = \"Depth\"\ndefinition = \"Queue depth.\"\ndomain = { kind = \"int\", min = 0, max = 10 }\n[terms.load]\nlabel = \"Load\"\ndefinition = \"Load.\"\ndomain = { kind = \"int\", min = 0, max = 10 }\n",
        )
        .unwrap();
        std::fs::write(
            specify.join("containment.toml"),
            "schema_version = \"1.0\"\n[[comparisons]]\nid = \"c\"\nkind = \"revision\"\nreference = { feature = \"specs/001-alpha\", spec = \"specs/001-alpha/reference.md\", model = \"specs/001-alpha/reference.toml\", vocabulary = \".specify/vocabulary.toml\" }\ncandidates = [{ feature = \"specs/001-alpha\", spec = \"specs/001-alpha/spec.md\", model = \"specs/001-alpha/model.toml\", vocabulary = \".specify/vocabulary.toml\" }]\n",
        )
        .unwrap();
        let model = "schema_version = \"1.0\"\n[effects]\ne = {}\n[requirements.T-001]\nwhen = \"depth > 3 and load > 2\"\nthen = \"e\"\n";
        std::fs::write(feature.join("model.toml"), model).unwrap();
        std::fs::write(feature.join("reference.toml"), model).unwrap();
        std::fs::write(
            feature.join("spec.md"),
            "# f\n\n- T-001: The service shall act.\n",
        )
        .unwrap();
        std::fs::write(
            feature.join("reference.md"),
            "# f\n\n- T-001: The service shall act.\n",
        )
        .unwrap();

        let config = Config {
            state_space_budget: 3,
            ..Config::default()
        };
        let config = Config {
            checks: crate::config::Checks {
                containment: true,
                ..crate::config::Checks::default()
            },
            ..config
        };
        let scope: BTreeSet<String> = ["specs/001-alpha".to_string()].into_iter().collect();
        let outcome = validate(&dir, &config, &scope, false);

        assert!(outcome
            .findings
            .iter()
            .any(|f| f.code == "CONTAIN_BUDGET_EXCEEDED"));
        assert_eq!(outcome.effects_compared, 0);
        assert_eq!(outcome.effects_uncompared, 1);
        assert_eq!(outcome.comparisons[0].status, "indeterminate");

        let _ = std::fs::remove_dir_all(&dir);
    }
}
