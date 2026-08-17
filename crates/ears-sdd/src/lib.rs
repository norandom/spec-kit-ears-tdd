pub mod assets;
pub mod config;
pub mod discovery;
pub mod ears;
pub mod init;
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
    let mut mappings: Vec<vocabulary::Mapping> = Vec::new();
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
    }

    let mut production_files_scanned = 0usize;
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
    vocabulary::scaffold(&requirements)
}
