//! Typed policy configuration.
//!
//! The previous implementation merged the parsed TOML into a defaults dict without inspecting it,
//! so `production_roots = "src"` silently disabled the separation gate and a mistyped key was never
//! reported at all. Deserializing into a struct makes a wrong type a parse error, and unknown keys
//! are captured rather than ignored so a typo surfaces as a finding instead of as silence.

use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

use crate::report::{Finding, Severity};

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    pub spec_glob: String,
    pub traceability_file: String,
    pub require_test_files: bool,
    pub test_command: String,
    pub production_roots: Vec<String>,
    pub test_roots: Vec<String>,
    pub source_extensions: Vec<String>,
    /// Globs whose separation findings are suppressed wholesale, for cases a per-line marker cannot
    /// reach. Kept separate from the marker so a broad exemption is visible in configuration rather
    /// than scattered through source.
    pub separation_exempt: Vec<String>,
    /// The largest state space a single component may have before the validator declines to
    /// evaluate it.
    ///
    /// A count rather than a duration, deliberately. A time limit makes the verdict depend on the
    /// machine, and a gate whose answer changes with load is not evidence. Versioned here rather
    /// than exposed as a flag, because raising it is a decision about how much the project is
    /// willing to leave unchecked, not a knob for making a red build green.
    pub state_space_budget: u64,
    #[serde(flatten)]
    pub unknown: BTreeMap<String, toml::Value>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            spec_glob: "specs/*/spec.md".to_string(),
            traceability_file: "traceability.toml".to_string(),
            require_test_files: true,
            test_command: String::new(),
            production_roots: ["src", "app", "lib"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            test_roots: vec!["tests".to_string()],
            source_extensions: [
                ".c", ".cc", ".cpp", ".cs", ".go", ".java", ".js", ".jsx", ".kt", ".php", ".ps1",
                ".py", ".rb", ".rs", ".ts", ".tsx",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            separation_exempt: Vec::new(),
            // A million states is microseconds to enumerate and far above anything a component
            // reaches once decomposition has done its work. A component that exceeds it is
            // signalling a modelling problem rather than a performance one.
            state_space_budget: 1_000_000,
            unknown: BTreeMap::new(),
        }
    }
}

impl Config {
    /// Test roots as POSIX prefixes ending in `/`, ready for a `starts_with` check.
    pub fn test_root_prefixes(&self) -> Vec<String> {
        self.test_roots
            .iter()
            .map(|root| {
                let normalized = root.replace('\\', "/");
                format!("{}/", normalized.trim_end_matches('/'))
            })
            .collect()
    }

    pub fn matches_source_extension(&self, path: &Path) -> bool {
        let Some(extension) = path.extension().and_then(|e| e.to_str()) else {
            return false;
        };
        let candidate = format!(".{}", extension.to_ascii_lowercase());
        self.source_extensions
            .iter()
            .any(|declared| declared.to_ascii_lowercase() == candidate)
    }
}

pub const CONFIG_RELATIVE_PATH: &str = ".specify/ears-sdd.toml";

pub fn load(root: &Path) -> (Config, Vec<Finding>) {
    let path = root.join(".specify").join("ears-sdd.toml");
    if !path.is_file() {
        return (
            Config::default(),
            vec![Finding::new(
                "CONFIG_MISSING",
                "Create .specify/ears-sdd.toml from the sample.",
                CONFIG_RELATIVE_PATH,
            )],
        );
    }
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) => {
            return (
                Config::default(),
                vec![Finding::new(
                    "CONFIG_UNREADABLE",
                    error.to_string(),
                    CONFIG_RELATIVE_PATH,
                )],
            )
        }
    };
    match toml::from_str::<Config>(&text) {
        Ok(config) => {
            let findings = config
                .unknown
                .keys()
                .map(|key| {
                    Finding::new(
                        "CONFIG_UNKNOWN_KEY",
                        format!("Unrecognized configuration key `{key}`; it has no effect."),
                        CONFIG_RELATIVE_PATH,
                    )
                    .severity(Severity::Warning)
                })
                .collect();
            (config, findings)
        }
        Err(error) => (
            Config::default(),
            vec![Finding::new(
                "CONFIG_INVALID",
                error.message().to_string(),
                CONFIG_RELATIVE_PATH,
            )],
        ),
    }
}
