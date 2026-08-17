pub mod adjudicate;
pub mod analysis;
pub mod assets;
pub mod bdd;
pub mod config;
pub mod discovery;
pub mod doctor;
pub mod ears;
pub mod enumerate;
pub mod exemptions;
pub mod guard;
pub mod init;
pub mod model;
pub mod report;
pub mod requirements;
pub mod separation;
pub mod tasks;
pub mod traceability;
pub mod vocabulary;

use std::path::Path;

use crate::report::{
    now_iso8601, FeatureResult, Finding, Phase, Provenance, Report, ScopeSource, Severity, Summary,
    SCHEMA_VERSION,
};
use crate::requirements::Requirement;

pub const VALIDATOR_NAME: &str = "ears-sdd";
pub const VALIDATOR_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Request<'a> {
    pub root: &'a Path,
    pub phase: Phase,
    pub feature: Option<&'a str>,
    pub all: bool,
}

pub fn validate(request: Request<'_>) -> Report {
    let root = request.root;
    let (config, mut findings) = config::load(root);
    let discovered = discovery::discover(root, &config, request.feature, request.all);
    findings.extend(discovered.findings.clone());

    // A run that silently evaluates less than the project contains is the failure mode that made
    // the previous gate untrustworthy. Say so, every time, in the report itself.
    if discovered.scope.is_narrowed() {
        findings.push(
            Finding::new(
                "SPEC_SCOPE",
                "This run evaluated a single feature. Pass --all to evaluate every specification.",
                ".",
            )
            .severity(Severity::Warning),
        );
    }

    let mut features: Vec<FeatureResult> = Vec::new();
    let mut all_requirements: Vec<Requirement> = Vec::new();
    let mut tasks_covered = 0usize;
    let mut modelled = 0usize;
    let mut components = 0usize;
    let mut merged_components: Option<usize> = None;
    let mut mappings: Vec<vocabulary::Mapping> = Vec::new();
    let mut declared_by_feature: std::collections::BTreeMap<
        String,
        std::collections::BTreeSet<String>,
    > = std::collections::BTreeMap::new();
    let mut feature_dirs: Vec<(String, std::path::PathBuf)> = Vec::new();

    for location in &discovered.specs {
        let (requirements, spec_findings) =
            requirements::parse(root, &location.path, &location.feature);
        findings.extend(spec_findings);
        if request.phase.checks_traceability() {
            let outcome = traceability::validate(
                root,
                &location.path,
                &location.feature,
                &requirements,
                &config,
            );
            findings.extend(outcome.findings);
            mappings.extend(outcome.mappings);
        }
        if let Some(parent) = location.path.parent() {
            feature_dirs.push((location.feature.clone(), parent.to_path_buf()));
        }
        declared_by_feature.insert(
            location.feature.clone(),
            requirements
                .iter()
                .map(|requirement| requirement.identifier.clone())
                .collect(),
        );
        // The tasks gate is the only phase that opens tasks.md, which is what finally makes it
        // distinct from the plan gate rather than a second copy of it.
        if request.phase == Phase::Tasks {
            let outcome = tasks::validate(root, &location.path, &location.feature, &requirements);
            findings.extend(outcome.findings);
            tasks_covered += outcome.covered;
        }
        features.push(FeatureResult {
            feature: location.feature.clone(),
            spec: report::relative(&location.path, root),
            requirements: requirements.len(),
        });
        all_requirements.extend(requirements);
    }

    if request.phase.checks_traceability() {
        let borrowed: Vec<(String, &std::path::Path)> = feature_dirs
            .iter()
            .map(|(feature, path)| (feature.clone(), path.as_path()))
            .collect();
        findings.extend(vocabulary::validate(root, &mappings, &borrowed));

        // The constraint model needs the same term map the vocabulary gate used, and the intention
        // layer it classifies conflicts by. Loaded once for the whole run rather than per feature,
        // because a term collision spans features and so does a precedence declaration.
        let (terms, _) = vocabulary::load_terms(root, &borrowed);
        let precedence = adjudicate::Precedence::new(&vocabulary::load_precedence(root));
        let intents: adjudicate::Intents = mappings
            .iter()
            .filter_map(|mapping| {
                mapping.intent.as_ref().map(|intent| {
                    (
                        (mapping.feature.clone(), mapping.requirement.clone()),
                        intent.clone(),
                    )
                })
            })
            .collect();
        let context = analysis::ModelContext {
            terms: &terms,
            budget: config.state_space_budget,
            intents: &intents,
            precedence: &precedence,
        };
        let mut feature_models = Vec::new();
        for (feature, directory) in &feature_dirs {
            let declared = declared_by_feature
                .get(feature)
                .cloned()
                .unwrap_or_default();
            let outcome = analysis::validate(root, feature, directory, &declared, &context);
            findings.extend(outcome.findings);
            modelled += outcome.modelled;
            components += outcome.components;
            feature_models.push(analysis::FeatureModel {
                feature: feature.clone(),
                directory: directory.clone(),
                declared,
            });
        }

        // The merge only means anything across more than one specification. Asking for all-features
        // scope is what asks for it: constraints never in the same room cannot contradict.
        if feature_models.len() > 1 {
            let merged = analysis::validate_merged(&feature_models, &context);
            findings.extend(merged.findings);
            merged_components = Some(merged.components);
        }
    }

    let mut production_files_scanned = 0usize;
    let mut separation_exempted: Option<usize> = None;
    if request.phase == Phase::Final {
        if config.test_command.trim().is_empty() {
            findings.push(Finding::new(
                "TEST_COMMAND",
                "Set the consuming project's real `test_command` before the final gate.",
                config::CONFIG_RELATIVE_PATH,
            ));
        }
        let outcome = separation::validate(root, &all_requirements, &config);
        production_files_scanned = outcome.files_scanned;
        separation_exempted = Some(outcome.exempted);
        findings.extend(outcome.findings);
    }

    let errors = findings
        .iter()
        .filter(|finding| finding.severity == Severity::Error)
        .count();
    let warnings = findings
        .iter()
        .filter(|finding| finding.severity == Severity::Warning)
        .count();
    let advisories = findings
        .iter()
        .filter(|finding| finding.severity == Severity::Advisory)
        .count();

    Report {
        schema_version: SCHEMA_VERSION.to_string(),
        ok: errors == 0,
        phase: request.phase,
        project: root.display().to_string(),
        provenance: Provenance {
            validator: VALIDATOR_NAME.to_string(),
            validator_version: VALIDATOR_VERSION.to_string(),
            generated_at: now_iso8601(),
            scope: discovered.scope,
            specs_examined: discovered.specs.len(),
            production_files_scanned,
        },
        summary: Summary {
            features: features.len(),
            requirements: all_requirements.len(),
            specs_examined: discovered.specs.len(),
            errors,
            warnings,
            advisories,
            tasks_covered: (request.phase == Phase::Tasks).then_some(tasks_covered),
            separation_exempted,
            modelled: request.phase.checks_traceability().then_some(modelled),
            components: request.phase.checks_traceability().then_some(components),
            merged_components,
        },
        features,
        findings,
    }
}

pub fn render_human(report: &Report, status_only: bool) -> String {
    let mut out = String::new();
    let state = if report.ok { "PASS" } else { "FAIL" };
    let phase = serde_json::to_string(&report.phase)
        .unwrap_or_default()
        .trim_matches('"')
        .to_string();
    out.push_str(&format!("EARS/TDD {phase} gate: {state}\n"));

    let scope = match &report.provenance.scope {
        ScopeSource::Flag(value) => format!("{value} (from --feature)"),
        ScopeSource::Environment(value) => {
            format!("{value} (from {})", discovery::FEATURE_ENVIRONMENT_VARIABLE)
        }
        ScopeSource::FeaturePointer(value) => format!("{value} (from .specify/feature.json)"),
        ScopeSource::Glob(value) => format!("{value} (all matching specifications)"),
    };
    out.push_str(&format!("Scope: {scope}\n"));
    let summary = &report.summary;
    out.push_str(&format!(
        "Features: {}  Requirements: {}  Errors: {}  Warnings: {}\n",
        summary.features, summary.requirements, summary.errors, summary.warnings
    ));
    if status_only && report.findings.is_empty() {
        return out;
    }
    for finding in &report.findings {
        let location = match finding.line {
            Some(line) => format!("{}:{}", finding.path, line),
            None => finding.path.clone(),
        };
        let requirement = match (&finding.feature, &finding.requirement) {
            (Some(feature), Some(identifier)) => format!(" [{feature}:{identifier}]"),
            (None, Some(identifier)) => format!(" [{identifier}]"),
            _ => String::new(),
        };
        out.push_str(&format!(
            "- {}{} {}: {}\n",
            finding.code, requirement, location, finding.message
        ));
    }
    out
}

/// Gather every requirement in scope and propose vocabulary stubs for them.
pub fn scaffold_vocabulary(root: &Path, feature: Option<&str>, all: bool) -> String {
    let (config, _) = config::load(root);
    let discovered = discovery::discover(root, &config, feature, all);
    let mut requirements = Vec::new();
    for location in &discovered.specs {
        let (found, _) = requirements::parse(root, &location.path, &location.feature);
        requirements.extend(found);
    }
    // The existing vocabulary is loaded so the scaffold can leave it out. Without this a second run
    // reproposes every term the author has already defined, and every term they deliberately
    // deleted, which makes the command useful exactly once.
    let feature_dirs: Vec<(String, &Path)> = discovered
        .specs
        .iter()
        .filter_map(|location| {
            location
                .path
                .parent()
                .map(|directory| (location.feature.clone(), directory))
        })
        .collect();
    let (existing, _) = vocabulary::load_terms(root, &feature_dirs);
    vocabulary::scaffold(&requirements, &existing)
}
