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

/// Which optional layers run.
///
/// Adoption happens in two phases. The first checks artifacts a Spec Kit project already has and is
/// on by default. The second reads a vocabulary and constraint models, which do not exist until
/// someone writes them, so it waits to be asked for.
///
/// Switching one on or off never makes the run quieter about it. The disabled set is printed on every run
/// and recorded in the machine-readable report, because a gate that can be silently narrowed is the
/// failure this project exists to prevent: a passing result that looks identical to a checked one.
#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Checks {
    /// Requirement-to-verification mapping: traceability files, test selectors, manual rationales.
    pub traceability: bool,
    /// Declared terms, the tags requirements carry, and the intentions they serve.
    pub vocabulary: bool,
    /// Constraint models, within a specification and merged across all of them.
    pub constraints: bool,
    /// Every requirement covered by a task before implementation.
    pub tasks: bool,
    /// Requirement prose and identifiers kept out of production code.
    pub separation: bool,
}

impl Default for Checks {
    /// EARS and test-first discipline are on; the grounding and constraint layers are not.
    ///
    /// The default is what a project gets before it has read anything, so it should be the part
    /// that pays for itself immediately. Requirement form, verification mapping, task coverage and
    /// separation need no vocabulary, no constraint models, and no new files: they check artifacts
    /// a Spec Kit project already has.
    ///
    /// The other two need work before they say anything. A vocabulary has to be written or imported
    /// before a tag can resolve, and a constraint model has to be authored before a contradiction
    /// can be found. Defaulting them on means a project either does that work immediately or reads
    /// advisories about not having done it, and the second is how a gate teaches people to ignore
    /// it. They are one line to enable once there is something for them to check.
    fn default() -> Self {
        Self {
            traceability: true,
            vocabulary: false,
            constraints: false,
            tasks: true,
            separation: true,
        }
    }
}

impl Checks {
    /// The layers switched off, in a stable order, for reporting.
    pub fn disabled(&self) -> Vec<&'static str> {
        [
            (self.traceability, "traceability"),
            (self.vocabulary, "vocabulary"),
            (self.constraints, "constraints"),
            (self.tasks, "tasks"),
            (self.separation, "separation"),
        ]
        .into_iter()
        .filter_map(|(enabled, name)| (!enabled).then_some(name))
        .collect()
    }
}

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
    /// Which optional layers run. The second phase is off until asked for.
    pub checks: Checks,
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
            checks: Checks::default(),
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

/// Whether the enabled checks can actually operate.
///
/// The gate refuses a configuration that cannot work. Whether a workable one has anything to read
/// yet is a different question, and `ears-sdd doctor` answers that one.
///
/// The constraint layer reads guards written over vocabulary terms and needs each term's declared
/// domain to know what values it ranges over. Enabling it without the vocabulary asks it to
/// type-check against declarations nobody is reading, so the combination is refused rather than run
/// at half strength. That is the whole of what enabling a phase can get wrong on its own.
pub fn readiness(config: &Config) -> Vec<Finding> {
    let mut findings = Vec::new();

    if config.checks.constraints && !config.checks.vocabulary {
        findings.push(Finding::new(
            "CHECK_DEPENDENCY",
            "The constraint check reads guards written over vocabulary terms and needs their \
             declared domains. Enable `checks.vocabulary` alongside it, or disable both.",
            CONFIG_RELATIVE_PATH,
        ));
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Phase one is what a Spec Kit project can check on the day it installs this: the artifacts it
    /// already has. Phase two needs files that do not exist yet, so it waits to be asked.
    #[test]
    fn the_advanced_layers_are_off_until_a_project_asks_for_them() {
        let checks = Checks::default();

        assert!(checks.traceability && checks.tasks && checks.separation);
        assert_eq!(checks.disabled(), vec!["vocabulary", "constraints"]);
    }

    #[test]
    fn a_project_can_switch_off_a_layer_it_has_not_adopted() {
        let parsed: Config =
            toml::from_str("[checks]\ntraceability = false\n").expect("the table parses");

        // Reported in a fixed order, alongside the two that were already off by default.
        assert_eq!(
            parsed.checks.disabled(),
            vec!["traceability", "vocabulary", "constraints"]
        );
        // Switching one off drags nothing else with it.
        assert!(parsed.checks.tasks);
        assert!(parsed.checks.separation);
    }

    /// The second phase is one line, and turning it on must not disturb the first.
    #[test]
    fn a_project_can_turn_the_advanced_layers_on() {
        let parsed: Config = toml::from_str("[checks]\nvocabulary = true\nconstraints = true\n")
            .expect("the table parses");

        assert!(parsed.checks.disabled().is_empty());
        assert!(parsed.checks.traceability && parsed.checks.tasks && parsed.checks.separation);
    }

    /// A mistyped switch that silently leaves a layer on is the better failure of the two, but it
    /// still means the author believes something is off that is not. `deny_unknown_fields` on the
    /// table turns that into a parse error.
    #[test]
    fn a_misspelt_check_is_refused_rather_than_ignored() {
        let parsed = toml::from_str::<Config>("[checks]\ntracability = false\n");
        assert!(parsed.is_err(), "{parsed:?}");
    }
}

#[cfg(test)]
mod readiness_tests {
    use super::*;

    /// A guard is written over declared terms and type-checked against their domains. Running the
    /// constraint layer while the declarations go unread is not a narrower check, it is a different
    /// and unsound one, so the combination is refused rather than reported as partial.
    #[test]
    fn the_constraint_check_is_refused_without_the_vocabulary_it_reads() {
        let config = Config {
            checks: Checks {
                vocabulary: false,
                constraints: true,
                ..Checks::default()
            },
            ..Config::default()
        };

        let findings = readiness(&config);

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].code, "CHECK_DEPENDENCY");
        assert_eq!(findings[0].severity, Severity::Error);
    }

    #[test]
    fn the_two_coherent_configurations_are_accepted() {
        // Phase one, and phase two with both layers on.
        assert!(readiness(&Config::default()).is_empty());
        let both = Config {
            checks: Checks {
                vocabulary: true,
                constraints: true,
                ..Checks::default()
            },
            ..Config::default()
        };
        assert!(readiness(&both).is_empty());
    }
}
