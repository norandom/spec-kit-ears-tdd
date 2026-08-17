//! Choosing which specifications a run evaluates.
//!
//! This is where the previous implementation was least trustworthy. `.specify/feature.json` pinned
//! one feature and every gate silently inherited it, so a project with eleven features reported on
//! one. That file is gitignored by Spec Kit, so the scope also differed between the author's machine
//! and a fresh clone — the same commit produced two verdicts with no message either way. And
//! `--feature` was joined and resolved with no containment check, so a run could return a green
//! report for a different repository entirely.

use globset::Glob;
use serde::Deserialize;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::report::{relative, Finding, ScopeSource, Severity};

pub const FEATURE_ENVIRONMENT_VARIABLE: &str = "SPECIFY_FEATURE_DIRECTORY";

#[derive(Debug, Clone)]
pub struct Discovered {
    pub specs: Vec<SpecLocation>,
    pub scope: ScopeSource,
    pub findings: Vec<Finding>,
}

#[derive(Debug, Clone)]
pub struct SpecLocation {
    pub path: PathBuf,
    /// The feature directory name, used to qualify requirement identifiers.
    pub feature: String,
}

#[derive(Deserialize)]
struct FeaturePointer {
    feature_directory: Option<String>,
}

pub fn discover(root: &Path, config: &Config, feature: Option<&str>, all: bool) -> Discovered {
    let mut findings = Vec::new();

    if !all {
        if let Some(requested) = feature {
            return explicit(root, requested, ScopeSource::Flag(requested.to_string()));
        }
        if let Ok(value) = std::env::var(FEATURE_ENVIRONMENT_VARIABLE) {
            if !value.trim().is_empty() {
                return explicit(root, &value, ScopeSource::Environment(value.clone()));
            }
        }
        let pointer = root.join(".specify").join("feature.json");
        if pointer.is_file() {
            match read_pointer(&pointer) {
                Ok(Some(directory)) => {
                    return explicit(root, &directory, ScopeSource::FeaturePointer(directory.clone()))
                }
                Ok(None) => {}
                Err(message) => findings.push(
                    Finding::new("FEATURE_POINTER_INVALID", message, ".specify/feature.json")
                        .severity(Severity::Warning),
                ),
            }
        }
    }

    let mut discovered = by_glob(root, config);
    discovered.findings.splice(0..0, findings);
    discovered
}

fn read_pointer(path: &Path) -> Result<Option<String>, String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let pointer: FeaturePointer =
        serde_json::from_str(&text).map_err(|error| error.to_string())?;
    Ok(pointer
        .feature_directory
        .filter(|directory| !directory.trim().is_empty()))
}

fn explicit(root: &Path, requested: &str, scope: ScopeSource) -> Discovered {
    let normalized = requested.replace('\\', "/");
    let candidate = root.join(&normalized);

    // Containment is checked against the lexically normalized path rather than the canonical one so
    // the answer does not depend on whether the directory happens to exist yet.
    if !is_contained(root, &candidate) {
        return Discovered {
            specs: Vec::new(),
            findings: vec![Finding::new(
                "SPEC_OUTSIDE_PROJECT",
                format!("Requested feature resolves outside the project: {normalized}"),
                normalized,
            )],
            scope,
        };
    }

    let spec_path = if candidate.is_dir() {
        candidate.join("spec.md")
    } else {
        candidate.clone()
    };
    if !spec_path.is_file() {
        return Discovered {
            specs: Vec::new(),
            findings: vec![Finding::new(
                "FEATURE_MISSING",
                "The selected feature names a specification that does not exist.",
                relative(&spec_path, root),
            )],
            scope,
        };
    }
    let feature = feature_name(root, &spec_path);
    Discovered {
        specs: vec![SpecLocation {
            path: spec_path,
            feature,
        }],
        findings: Vec::new(),
        scope,
    }
}

/// Lexical containment: no component of the relative path may escape upward.
fn is_contained(root: &Path, candidate: &Path) -> bool {
    let Ok(suffix) = candidate.strip_prefix(root) else {
        return false;
    };
    !suffix
        .components()
        .any(|component| matches!(component, std::path::Component::ParentDir))
}

fn by_glob(root: &Path, config: &Config) -> Discovered {
    let scope = ScopeSource::Glob(config.spec_glob.clone());
    let pattern = config.spec_glob.replace('\\', "/");
    let matcher = match Glob::new(&pattern) {
        Ok(glob) => glob.compile_matcher(),
        Err(error) => {
            return Discovered {
                specs: Vec::new(),
                findings: vec![Finding::new(
                    "CONFIG_INVALID",
                    format!("`spec_glob` is not a valid pattern: {error}"),
                    crate::config::CONFIG_RELATIVE_PATH,
                )],
                scope,
            }
        }
    };

    let mut specs: Vec<SpecLocation> = Vec::new();
    for entry in ignore::WalkBuilder::new(root)
        .hidden(false)
        .git_ignore(false)
        .build()
        .flatten()
    {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let candidate = relative(path, root);
        if matcher.is_match(&candidate) {
            specs.push(SpecLocation {
                feature: feature_name(root, path),
                path: path.to_path_buf(),
            });
        }
    }
    specs.sort_by(|a, b| a.path.cmp(&b.path));

    let findings = if specs.is_empty() {
        vec![Finding::new(
            "SPEC_NONE",
            "No specification matched the configured feature or glob.",
            ".",
        )]
    } else {
        Vec::new()
    };
    Discovered {
        specs,
        findings,
        scope,
    }
}

/// The feature directory name — the last path component above `spec.md`.
fn feature_name(root: &Path, spec_path: &Path) -> String {
    spec_path
        .parent()
        .map(|parent| relative(parent, root))
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "<root>".to_string())
}
