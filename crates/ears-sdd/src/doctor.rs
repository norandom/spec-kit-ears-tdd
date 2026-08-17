//! Reporting whether an installation is actually wired up.
//!
//! Every check here answers a question that previously had no answer short of running a gate and
//! interpreting the failure. The distinction that matters is between a project that has no policy
//! installed and one that has it installed but inert -- those produce the same passing output from
//! `validate`, and they need opposite fixes.
//!
//! Deliberately filesystem-only. Shelling out to `specify` to ask its version would make the
//! diagnosis depend on a subprocess that may itself be the thing that is broken, and `init` already
//! records what initialized the project.

use std::path::{Path, PathBuf};

use crate::config;
use crate::discovery;

/// The Spec Kit range the shipped components declare. Duplicated from `components/*/`, which have
/// no Rust-readable form; `supported_range_matches_the_components` fails if the two drift.
pub const SUPPORTED_SPECKIT: &str = ">=0.16.3,<0.17.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    /// Present and usable.
    Ok,
    /// Usable, but something the author probably intended is missing.
    Warn,
    /// The gate cannot run, or runs and means nothing.
    Fail,
}

impl Level {
    fn marker(self) -> &'static str {
        match self {
            Level::Ok => "ok  ",
            Level::Warn => "warn",
            Level::Fail => "FAIL",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Check {
    pub name: &'static str,
    pub level: Level,
    pub detail: String,
    /// The command that resolves it. Omitted when there is nothing mechanical to run.
    pub fix: Option<String>,
}

impl Check {
    fn ok(name: &'static str, detail: impl Into<String>) -> Self {
        Self {
            name,
            level: Level::Ok,
            detail: detail.into(),
            fix: None,
        }
    }

    fn warn(name: &'static str, detail: impl Into<String>, fix: impl Into<String>) -> Self {
        Self {
            name,
            level: Level::Warn,
            detail: detail.into(),
            fix: Some(fix.into()),
        }
    }

    fn fail(name: &'static str, detail: impl Into<String>, fix: impl Into<String>) -> Self {
        Self {
            name,
            level: Level::Fail,
            detail: detail.into(),
            fix: Some(fix.into()),
        }
    }
}

/// Run every check against a project.
///
/// Checks are independent and all of them always run: stopping at the first failure would report
/// one missing component at a time across as many invocations as there are problems, which is the
/// experience this command exists to remove.
pub fn inspect(project: &Path) -> Vec<Check> {
    let specify = project.join(".specify");
    if !specify.is_dir() {
        return vec![Check::fail(
            "Spec Kit project",
            format!("no .specify directory in {}", project.display()),
            "ears-sdd init",
        )];
    }

    let mut checks = vec![Check::ok("Spec Kit project", ".specify is present")];
    checks.push(speckit_version(&specify));
    checks.push(preset(&specify));
    checks.push(extension(&specify));
    checks.push(workflow(&specify));
    checks.push(configuration(&specify));
    checks.push(specifications(project));
    checks.push(enforcement(project));
    checks
}

fn speckit_version(specify: &Path) -> Check {
    let options = specify.join("init-options.json");
    let Ok(contents) = std::fs::read_to_string(&options) else {
        return Check::warn(
            "Spec Kit version",
            "init-options.json is unreadable, so the version it was initialized with is unknown",
            "ears-sdd init",
        );
    };
    let parsed: serde_json::Value = match serde_json::from_str(&contents) {
        Ok(value) => value,
        Err(error) => {
            return Check::warn(
                "Spec Kit version",
                format!("init-options.json is not valid JSON: {error}"),
                "ears-sdd init",
            )
        }
    };
    let Some(version) = parsed.get("speckit_version").and_then(|v| v.as_str()) else {
        return Check::warn(
            "Spec Kit version",
            "init-options.json records no speckit_version",
            "ears-sdd init",
        );
    };
    if supported(version) {
        Check::ok(
            "Spec Kit version",
            format!("{version}, within {SUPPORTED_SPECKIT}"),
        )
    } else {
        // A warning rather than a failure: the components may still work, and the honest statement
        // is that they are untested here, not that they are broken.
        Check::warn(
            "Spec Kit version",
            format!("{version} is outside the tested range {SUPPORTED_SPECKIT}"),
            "uv tool install specify-cli==0.16.3",
        )
    }
}

/// Whether a version satisfies `SUPPORTED_SPECKIT`, which is a fixed `>=0.16.3,<0.17.0`.
///
/// Not a general semver implementation, and not pretending to be: it compares the numeric triple
/// against the two bounds this project actually declares. A pre-release suffix is treated as the
/// release it precedes, which errs toward warning rather than silently accepting.
fn supported(version: &str) -> bool {
    let Some(parsed) = triple(version) else {
        return false;
    };
    ((0, 16, 3)..(0, 17, 0)).contains(&parsed)
}

fn triple(version: &str) -> Option<(u32, u32, u32)> {
    let core = version
        .trim()
        .trim_start_matches('v')
        .split(['-', '+'])
        .next()?;
    let mut parts = core.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next().unwrap_or("0").parse().ok()?;
    let patch = parts.next().unwrap_or("0").parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((major, minor, patch))
}

fn preset(specify: &Path) -> Check {
    if specify.join("presets").join("ears-tdd").is_dir() {
        Check::ok("Policy preset", "ears-tdd is installed")
    } else {
        // Not a hard failure: the validator does not read the preset. Without it the agent is never
        // told to write EARS in the first place, so the gate finds defects instead of preventing
        // them -- worth saying, not worth blocking on.
        Check::warn(
            "Policy preset",
            "ears-tdd is not installed, so nothing instructs the agent to write EARS requirements",
            "ears-sdd init",
        )
    }
}

fn extension(specify: &Path) -> Check {
    let installed = std::fs::read_to_string(specify.join("extensions.yml")).unwrap_or_default();
    // Substring rather than YAML: the file is Spec Kit's, the crate has no YAML parser, and the
    // question is only whether an id appears in it.
    if installed.contains("ears-validate") {
        Check::ok("Validator extension", "ears-validate is installed")
    } else {
        Check::warn(
            "Validator extension",
            "ears-validate is not installed, so agents have no command for the gate",
            "ears-sdd init",
        )
    }
}

fn workflow(specify: &Path) -> Check {
    if specify.join("workflows").join("ears-sdd").is_dir() {
        Check::ok("Workflow", "ears-sdd is installed")
    } else {
        Check::warn(
            "Workflow",
            "ears-sdd is not installed, so the gated specify/plan/tasks/implement cycle is absent",
            "ears-sdd init",
        )
    }
}

fn configuration(specify: &Path) -> Check {
    if specify.join("ears-sdd.toml").is_file() {
        Check::ok("Configuration", ".specify/ears-sdd.toml is present")
    } else {
        Check::warn(
            "Configuration",
            "no .specify/ears-sdd.toml, so every setting is at its default and the separation gate \
             has no production roots to scan",
            "ears-sdd init",
        )
    }
}

fn specifications(project: &Path) -> Check {
    // Configuration findings are discarded here: a malformed config is already reported by its own
    // check, and repeating it under "Specifications" would read as two unrelated problems.
    let (config, _) = config::load(project);
    let found = discovery::discover(project, &config, None, true);
    match found.specs.len() {
        0 => Check::warn(
            "Specifications",
            "no specifications found, so every gate passes vacuously",
            "run the specify workflow, or `ears-sdd validate --phase spec --all` once specs exist",
        ),
        count => Check::ok(
            "Specifications",
            format!("{count} found; `--all` evaluates every one"),
        ),
    }
}

fn enforcement(project: &Path) -> Check {
    // Any workflow that invokes the validator counts, not just the one `init --ci` writes. A
    // project that added the gate to an existing pipeline has solved this problem, and reporting it
    // as unenforced would train people to ignore the warning.
    if let Some(file) = workflow_invoking_validator(project) {
        Check::ok(
            "Automated enforcement",
            format!(".github/workflows/{file} runs the validator"),
        )
    } else {
        // The one check whose absence is invisible in normal use: everything else here shows up the
        // first time someone runs a command, whereas an unenforced gate looks exactly like a
        // passing one until a specification lands with no requirements in it.
        Check::warn(
            "Automated enforcement",
            "no workflow in .github/workflows runs the validator, directly or through a file it \
             names, so nothing checks the gate unless someone remembers to",
            "ears-sdd init --ci",
        )
    }
}

/// How many referenced files one workflow may pull in. Generous for real workflows and bounded, so
/// a file full of path-like strings cannot turn a diagnostic into a directory walk.
const REFERENCE_LIMIT: usize = 32;

/// Whether text invokes the gate, in any of the spellings that actually appear.
///
/// The bare binary is only one of them: this repository runs `cargo run --release -- validate
/// --project .`, and a project that has not installed the binary yet will do something similar.
fn invokes_validator(contents: &str) -> bool {
    contents.contains("ears-sdd validate")
        || contents.contains("validate --phase")
        || contents.contains("validate --project")
}

/// The GitHub Actions workflow that runs the validator, directly or through one file it names.
///
/// The indirection matters more than it looks. CI usually invokes a build tool -- a Dagger module,
/// a Makefile, a script -- and the gate lives there, so a scan of the workflow files alone reports
/// a correctly-enforced project as unenforced. That is the warning people learn to ignore.
///
/// One level only, and no attempt to interpret the workflow: it collects the path-like tokens that
/// resolve to files in the project and reads them. A gate two tools deep is not detected, which is
/// why the warning says what was searched rather than asserting nothing runs the gate.
///
/// Names are sorted before searching so the answer cannot depend on directory order, which differs
/// between filesystems and would make one project report differently on two machines.
pub(crate) fn workflow_invoking_validator(project: &Path) -> Option<String> {
    let directory = project.join(".github").join("workflows");
    let mut names: Vec<_> = std::fs::read_dir(&directory)
        .ok()?
        .filter_map(Result::ok)
        .filter(|entry| {
            matches!(
                entry.path().extension().and_then(|e| e.to_str()),
                Some("yml") | Some("yaml")
            )
        })
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();

    for name in names {
        let Ok(contents) = std::fs::read_to_string(directory.join(&name)) else {
            continue;
        };
        if invokes_validator(&contents) {
            return Some(name);
        }
        if let Some(referenced) = referenced_file_invoking_validator(project, &contents) {
            return Some(format!("{name} (via {referenced})"));
        }
    }
    None
}

fn referenced_file_invoking_validator(project: &Path, contents: &str) -> Option<String> {
    let mut seen = std::collections::BTreeSet::new();
    for token in contents.split(|c: char| c.is_whitespace() || "\"'`,;()".contains(c)) {
        let candidate = token.trim_matches(|c| c == '.' || c == ':');
        // A path, not a URL and not a flag. Anything absolute is a runner path, not a repo file.
        if !candidate.contains('/')
            || candidate.contains("://")
            || candidate.starts_with('-')
            || candidate.starts_with('/')
        {
            continue;
        }
        if !seen.insert(candidate.to_string()) {
            continue;
        }
        if seen.len() > REFERENCE_LIMIT {
            break;
        }
        let path = project.join(candidate);
        if !path.is_file() {
            continue;
        }
        if std::fs::read_to_string(&path)
            .map(|referenced| invokes_validator(&referenced))
            .unwrap_or(false)
        {
            return Some(candidate.to_string());
        }
    }
    None
}

/// Print the report and return the process exit code.
pub fn report(project: &Path) -> u8 {
    let checks = inspect(project);
    println!(
        "ears-sdd {} checking {}",
        env!("CARGO_PKG_VERSION"),
        crate::report::plain_path(project)
    );
    println!();

    // The fix line is aligned under the detail column so it reads as belonging to the check above
    // it rather than to the one below. The 9 is the width of the `  [warn] ` prefix.
    let width = checks.iter().map(|c| c.name.len()).max().unwrap_or(0);
    for check in &checks {
        println!(
            "  [{}] {:width$}  {}",
            check.level.marker(),
            check.name,
            check.detail
        );
        if let Some(fix) = &check.fix {
            println!("{:9}{:width$}  fix: {fix}", "", "");
        }
    }

    let failures = checks.iter().filter(|c| c.level == Level::Fail).count();
    let warnings = checks.iter().filter(|c| c.level == Level::Warn).count();
    println!();
    println!(
        "{} checks: {} ok, {warnings} warning(s), {failures} failure(s)",
        checks.len(),
        checks.len() - warnings - failures
    );

    // Warnings do not fail. A project on its first day legitimately has no specifications and no CI
    // yet, and a doctor that exits nonzero on that is one nobody runs twice.
    if failures > 0 {
        1
    } else {
        0
    }
}

pub struct Options {
    pub project: PathBuf,
}

pub fn run(options: &Options) -> Result<u8, String> {
    let project = options
        .project
        .canonicalize()
        .map_err(|error| format!("Project directory is unusable: {error}"))?;
    Ok(report(&project))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find<'a>(checks: &'a [Check], name: &str) -> &'a Check {
        checks
            .iter()
            .find(|check| check.name == name)
            .unwrap_or_else(|| panic!("no check named {name}"))
    }

    #[test]
    fn a_directory_that_is_not_a_spec_kit_project_fails_with_one_check() {
        let project = tempfile::tempdir().expect("a temporary directory");
        let checks = inspect(project.path());

        assert_eq!(checks.len(), 1, "the remaining checks cannot be meaningful");
        assert_eq!(checks[0].level, Level::Fail);
        assert_eq!(checks[0].fix.as_deref(), Some("ears-sdd init"));
    }

    /// The case the command exists for: everything reports fine except the part that makes the gate
    /// actually run, which is invisible from a passing `validate`.
    #[test]
    fn an_installed_project_without_ci_warns_rather_than_fails() {
        let project = tempfile::tempdir().expect("a temporary directory");
        let specify = project.path().join(".specify");
        std::fs::create_dir_all(specify.join("presets/ears-tdd")).expect("the preset directory");
        std::fs::create_dir_all(specify.join("workflows/ears-sdd")).expect("the workflow dir");
        std::fs::write(
            specify.join("extensions.yml"),
            "installed:\n- ears-validate\n",
        )
        .expect("the extension registry");
        std::fs::write(specify.join("ears-sdd.toml"), "").expect("the configuration");
        std::fs::write(
            specify.join("init-options.json"),
            r#"{"speckit_version": "0.16.3"}"#,
        )
        .expect("the init options");

        let checks = inspect(project.path());

        for name in [
            "Spec Kit project",
            "Spec Kit version",
            "Policy preset",
            "Validator extension",
            "Workflow",
            "Configuration",
        ] {
            assert_eq!(find(&checks, name).level, Level::Ok, "{name} should pass");
        }
        let ci = find(&checks, "Automated enforcement");
        assert_eq!(ci.level, Level::Warn);
        assert_eq!(ci.fix.as_deref(), Some("ears-sdd init --ci"));
        assert_eq!(
            checks.iter().filter(|c| c.level == Level::Fail).count(),
            0,
            "a fresh install must not be reported as broken"
        );
    }

    /// The case that made this check worth writing: this repository's own CI runs the gate through
    /// a Dagger module, so scanning only the workflow files reports it as unenforced.
    #[test]
    fn a_gate_reached_through_a_referenced_file_counts_as_enforcement() {
        let project = tempfile::tempdir().expect("a temporary directory");
        let workflows = project.path().join(".github/workflows");
        std::fs::create_dir_all(&workflows).expect("the workflows directory");
        std::fs::create_dir_all(project.path().join("ci")).expect("the ci directory");
        std::fs::write(
            workflows.join("ci.yml"),
            "jobs:\n  gate:\n    steps:\n      - run: dagger --progress plain -M ci/rust.dag\n",
        )
        .expect("the workflow");
        std::fs::write(
            project.path().join("ci/rust.dag"),
            "with-exec -- cargo run --release -- validate --project . --phase final --all\n",
        )
        .expect("the dagger module");

        let check = enforcement(project.path());

        assert_eq!(check.level, Level::Ok);
        assert!(
            check.detail.contains("ci/rust.dag"),
            "the report must name where the gate actually is: {}",
            check.detail
        );
    }

    /// A workflow naming a file that does not run the gate must not be read as enforcement.
    #[test]
    fn an_unrelated_referenced_file_is_not_enforcement() {
        let project = tempfile::tempdir().expect("a temporary directory");
        let workflows = project.path().join(".github/workflows");
        std::fs::create_dir_all(&workflows).expect("the workflows directory");
        std::fs::create_dir_all(project.path().join("ci")).expect("the ci directory");
        std::fs::write(workflows.join("ci.yml"), "- run: bash ci/build.sh\n").expect("workflow");
        std::fs::write(project.path().join("ci/build.sh"), "cargo build\n").expect("the script");

        assert_eq!(enforcement(project.path()).level, Level::Warn);
    }

    #[test]
    fn the_supported_range_is_exactly_the_one_the_components_declare() {
        assert!(supported("0.16.3"));
        assert!(supported("0.16.99"));
        assert!(!supported("0.16.2"));
        assert!(!supported("0.17.0"));
        assert!(!supported("1.0.0"));
        // Two-part and prefixed versions appear in the wild; neither may be read as satisfying.
        assert!(!supported("0.16"));
        assert!(supported("v0.16.4"));
        assert!(!supported("not-a-version"));
    }

    /// The range lives in three component manifests and once here. This is what keeps the fourth
    /// copy honest, since a stale one would report a supported project as untested or the reverse.
    #[test]
    fn supported_range_matches_the_components() {
        for manifest in [
            "extension/ears-validate/extension.yml",
            "preset/ears-tdd/preset.yml",
            "workflow/ears-sdd/workflow.yml",
        ] {
            let contents = crate::assets::COMPONENTS
                .get_file(manifest)
                .and_then(|file| file.contents_utf8())
                .unwrap_or_else(|| panic!("{manifest} is embedded"));
            assert!(
                contents.contains(SUPPORTED_SPECKIT),
                "{manifest} declares a different Spec Kit range than doctor reports"
            );
        }
    }
}
