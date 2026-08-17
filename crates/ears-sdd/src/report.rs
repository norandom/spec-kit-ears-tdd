//! The machine-readable result. This is the contract two implementations have to agree on, so it
//! carries a schema version and enough provenance for a reader to tell two runs apart.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

/// Bumped whenever the shape below changes in a way a consumer could notice.
pub const SCHEMA_VERSION: &str = "1.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Advisory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
    Spec,
    Plan,
    Tasks,
    Final,
}

impl Phase {
    pub fn checks_traceability(self) -> bool {
        matches!(self, Phase::Plan | Phase::Tasks | Phase::Final)
    }
}

/// How the set of specifications under evaluation was chosen. Recorded in the report because the
/// same commit validated with a different scope is a different claim, and the previous
/// implementation left that invisible.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "source", content = "value", rename_all = "snake_case")]
pub enum ScopeSource {
    Flag(String),
    Environment(String),
    FeaturePointer(String),
    Glob(String),
}

impl ScopeSource {
    pub fn is_narrowed(&self) -> bool {
        !matches!(self, ScopeSource::Glob(_))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub code: String,
    pub message: String,
    /// Always POSIX-separated and project-relative, so consumers never see two path formats.
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,
    pub severity: Severity,
    /// Structured particulars for findings a reader has to act on rather than merely locate.
    ///
    /// A budget failure that says only "budget exceeded" is a shrug. The consumer here is usually
    /// an agent, so the numbers it needs to choose a next step -- which component, how large, which
    /// terms contribute most -- belong in fields rather than buried in prose it has to parse back
    /// out of a sentence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<BTreeMap<String, serde_json::Value>>,
}

impl Finding {
    pub fn new(code: &str, message: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            code: code.to_string(),
            message: message.into(),
            path: path.into(),
            feature: None,
            requirement: None,
            line: None,
            severity: Severity::Error,
            detail: None,
        }
    }

    /// Attach one structured particular. Findings that carry these are the ones whose next action
    /// depends on a number rather than on a location.
    pub fn detail(mut self, key: &str, value: impl Into<serde_json::Value>) -> Self {
        self.detail
            .get_or_insert_with(BTreeMap::new)
            .insert(key.to_string(), value.into());
        self
    }

    pub fn feature(mut self, feature: impl Into<String>) -> Self {
        self.feature = Some(feature.into());
        self
    }

    pub fn requirement(mut self, requirement: impl Into<String>) -> Self {
        self.requirement = Some(requirement.into());
        self
    }

    pub fn line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    pub fn severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    /// The identity used to suppress duplicates. Requirement identifiers restart at `REQ-NNN` numbering in
    /// every feature, so the feature has to participate or one leaked identifier is reported once
    /// per feature that happens to define it.
    pub fn dedupe_key(&self) -> (String, String, Option<usize>, String, String) {
        (
            self.code.clone(),
            self.path.clone(),
            self.line,
            self.feature.clone().unwrap_or_default(),
            self.requirement.clone().unwrap_or_default(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    pub validator: String,
    pub validator_version: String,
    pub generated_at: String,
    pub scope: ScopeSource,
    pub specs_examined: usize,
    pub production_files_scanned: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureResult {
    pub feature: String,
    pub spec: String,
    pub requirements: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub features: usize,
    pub requirements: usize,
    pub specs_examined: usize,
    pub errors: usize,
    pub warnings: usize,
    pub advisories: usize,
    /// Declared requirements a task list references. Only the tasks gate opens `tasks.md`, so this
    /// is absent elsewhere rather than reported as a misleading zero.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasks_covered: Option<usize>,
    /// Separation findings an exemption removed. Present only on the phase that scans production
    /// code, and present even when zero, so a reader can tell "none were exempted" from "the scan
    /// did not run".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub separation_exempted: Option<usize>,
    /// Requirements carrying a constraint model, and the independent components they decomposed
    /// into. Present wherever the model layer runs, so "nothing modelled" is distinguishable from
    /// "the layer did not run".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modelled: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub schema_version: String,
    pub ok: bool,
    pub phase: Phase,
    pub project: String,
    pub provenance: Provenance,
    pub features: Vec<FeatureResult>,
    pub summary: Summary,
    pub findings: Vec<Finding>,
}

/// Project-relative, POSIX-separated. Falls back to the file name rather than an absolute path so a
/// report can never leak a machine-specific location into what is meant to be portable evidence.
pub fn relative(path: &Path, root: &Path) -> String {
    let normalized = path.strip_prefix(root).unwrap_or(path);
    normalized
        .components()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/")
}

pub fn now_iso8601() -> String {
    use time::format_description::well_known::Rfc3339;
    time::OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| "unknown".to_string())
}
