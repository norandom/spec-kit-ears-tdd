//! Installing the policy into a project.
//!
//! Differences from the Python bootstrap, all of them defects it had:
//!
//! * `specify` is located by probing the interpreter's own environment without resolving symlinks,
//!   and a failure to find it is a message rather than a traceback.
//! * A second run is an upgrade rather than an abort: components already present are reinstalled
//!   with `--force` where the subcommand supports it, and reported as kept where it does not.
//! * No launcher scripts are installed. The Python bootstrap copied a `.sh` and a `.ps1` into every
//!   project whose only job was to locate an interpreter; a single binary on PATH makes both
//!   redundant, and with them go the CRLF hazard and the lost execute bit they carried.
//! * The traceability sample is written into the project, so the file the documentation tells
//!   authors to copy actually exists locally.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::assets;

pub struct Options {
    pub project: PathBuf,
    pub integration: String,
    pub priority: u32,
    /// Install the GitHub Actions gate. Opt-in: writing into `.github/` is a larger and more
    /// opinionated footprint than `.specify/`, and it is wrong outright for a project that does not
    /// use GitHub Actions. A project that skips it is told what it is skipping.
    pub ci: bool,
}

pub fn run(options: &Options) -> Result<(), String> {
    let project = options
        .project
        .canonicalize()
        .map_err(|error| format!("Project directory is unusable: {error}"))?;
    if !project.is_dir() {
        return Err(format!(
            "Project directory does not exist: {}",
            project.display()
        ));
    }

    let specify = locate_specify()?;
    let staging = tempfile::tempdir()
        .map_err(|error| format!("Could not create a staging directory: {error}"))?;
    let components = staging.path().join("components");
    assets::materialize(&assets::COMPONENTS, &components)
        .map_err(|error| format!("Could not stage the policy components: {error}"))?;

    let already_initialized = project.join(".specify").is_dir();
    if already_initialized {
        println!("Spec Kit already initialized: {}", display(&project));
    } else {
        run_specify(
            &specify,
            &project,
            &[
                "init",
                ".",
                "--integration",
                &options.integration,
                "--script",
                "py",
                "--ignore-agent-tools",
                "--force",
            ],
        )?;
    }

    let priority = options.priority.to_string();
    let preset = components.join("preset").join("ears-tdd");
    let extension = components.join("extension").join("ears-validate");
    let workflow = components.join("workflow").join("ears-sdd");

    // `preset add` and `workflow add` have no --force, so a reinstall is reported rather than
    // treated as a failure. `extension add` does, so it is always brought up to date.
    reinstallable(
        &specify,
        &project,
        &[
            "preset",
            "add",
            "--dev",
            &preset.to_string_lossy(),
            "--priority",
            &priority,
        ],
        "preset",
    )?;
    run_specify(
        &specify,
        &project,
        &[
            "extension",
            "add",
            &extension.to_string_lossy(),
            "--dev",
            "--force",
            "--priority",
            &priority,
        ],
    )?;
    reinstallable(
        &specify,
        &project,
        &["workflow", "add", &workflow.to_string_lossy(), "--dev"],
        "workflow",
    )?;

    write_if_absent(
        &project.join(".specify").join("ears-sdd.toml"),
        assets::config_sample().as_bytes(),
    )?;
    write_if_absent(
        &project.join(".specify").join("traceability.toml.sample"),
        assets::traceability_sample().as_bytes(),
    )?;

    if options.ci {
        write_ci_workflow(&project)?;
    }

    println!("Installed EARS/TDD policy components.");
    println!("Next: edit .specify/ears-sdd.toml, then run `ears-sdd validate --phase spec --all`.");
    if !options.ci {
        // Said plainly because the alternative is a project that believes it has a gate. The
        // preset and the skill describe the policy to an agent; neither can make the check run,
        // and an agent that decides it has finished simply does not run it.
        println!(
            "Nothing runs the gate automatically. `ears-sdd init --ci` adds a GitHub Actions \
             workflow that does."
        );
    }
    Ok(())
}

/// Install the GitHub Actions gate into a project.
///
/// Separate from `run` so it can be exercised without a Spec Kit installation and a network: the
/// interesting part is that it creates `.github/workflows/` when absent and never overwrites a
/// workflow someone has since edited.
pub fn write_ci_workflow(project: &Path) -> Result<(), String> {
    write_if_absent(
        &project
            .join(".github")
            .join("workflows")
            .join("ears-sdd.yml"),
        assets::ci_workflow().as_bytes(),
    )
}

/// `canonicalize` returns a verbatim path on Windows (`\\?\C:\...`). It is correct and unreadable,
/// and it is not what a user would type back, so strip it for anything printed.
fn display(path: &Path) -> String {
    let rendered = path.display().to_string();
    rendered
        .strip_prefix(r"\\?\")
        .map(|stripped| stripped.to_string())
        .unwrap_or(rendered)
}

fn locate_specify() -> Result<PathBuf, String> {
    let name = if cfg!(windows) {
        "specify.exe"
    } else {
        "specify"
    };
    let interpreter = std::env::current_exe().ok();
    let mut roots: Vec<PathBuf> = Vec::new();
    if let Some(directory) = interpreter.as_ref().and_then(|path| path.parent()) {
        roots.push(directory.to_path_buf());
        if let Some(parent) = directory.parent() {
            roots.push(parent.join("bin"));
            roots.push(parent.join("Scripts"));
        }
    }
    for root in roots {
        let candidate = root.join(name);
        if candidate.is_file() {
            return Ok(candidate);
        }
    }
    which(name).ok_or_else(|| {
        format!(
            "`{name}` was not found next to this binary or on PATH. Spec Kit provides it; install \
             it with `uv tool install specify-cli==0.16.3`."
        )
    })
}

fn which(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|directory| directory.join(name))
        .find(|candidate| candidate.is_file())
}

fn run_specify(specify: &Path, project: &Path, arguments: &[&str]) -> Result<(), String> {
    announce(specify, arguments);
    let status = Command::new(specify)
        .args(arguments)
        .current_dir(project)
        .status()
        .map_err(|error| format!("Failed to run specify: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "`specify {}` exited with {}",
            arguments.join(" "),
            status.code().unwrap_or(-1)
        ))
    }
}

/// A component whose `add` subcommand cannot overwrite. A second run reports rather than aborts.
fn reinstallable(
    specify: &Path,
    project: &Path,
    arguments: &[&str],
    kind: &str,
) -> Result<(), String> {
    match run_specify(specify, project, arguments) {
        Ok(()) => Ok(()),
        Err(message) => {
            println!("Kept the installed {kind} ({message}).");
            Ok(())
        }
    }
}

/// Printed before execution and flushed, so the mutation trace stays ordered even when stdout is a
/// pipe -- which is where an audit of what the tool changed actually happens.
fn announce(specify: &Path, arguments: &[&str]) {
    println!("> {} {}", specify.display(), arguments.join(" "));
    let _ = std::io::stdout().flush();
}

fn write_if_absent(path: &Path, contents: &[u8]) -> Result<(), String> {
    if path.exists() {
        println!("Kept existing {}", display(path));
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("Could not create {}: {error}", display(parent)))?;
    }
    std::fs::write(path, contents)
        .map_err(|error| format!("Could not write {}: {error}", display(path)))?;
    println!("Created {}", display(path));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_ci_workflow_lands_in_a_project_without_a_github_directory() {
        let project = tempfile::tempdir().expect("a temporary directory");
        write_ci_workflow(project.path()).expect("the workflow is written");

        let written = project.path().join(".github/workflows/ears-sdd.yml");
        let contents = std::fs::read_to_string(&written).expect("the workflow exists");
        assert!(contents.contains("ears-sdd validate --project . --phase final --all"));
    }

    /// A project that has tightened the gate -- pinned a newer validator, added a matrix, narrowed
    /// the triggers -- must not silently lose that on the next `init`. Reporting the file as kept is
    /// the only honest outcome, since the alternative is a gate that quietly reverts.
    #[test]
    fn an_edited_ci_workflow_is_never_overwritten() {
        let project = tempfile::tempdir().expect("a temporary directory");
        let written = project.path().join(".github/workflows/ears-sdd.yml");
        std::fs::create_dir_all(written.parent().expect("a parent")).expect("the directory");
        std::fs::write(&written, "# edited by hand\n").expect("the seed file");

        write_ci_workflow(project.path()).expect("the call succeeds");

        assert_eq!(
            std::fs::read_to_string(&written).expect("still readable"),
            "# edited by hand\n"
        );
    }
}
