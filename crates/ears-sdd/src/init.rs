//! Installing the policy into a project.
//!
//! Differences from the Python bootstrap, all of them defects it had:
//!
//! * `specify` is located by probing the interpreter's own environment without resolving symlinks,
//!   and a failure to find it is a message rather than a traceback.
//! * A second run is an upgrade rather than an abort: components already present are reinstalled
//!   with `--force` where the subcommand supports it, and reported as kept where it does not.
//! * The POSIX launcher is written with LF and marked executable, regardless of the host.
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

    install_launcher(&project.join("ears-sdd.ps1"), assets::launcher("ears-sdd.ps1"), false)?;
    install_launcher(&project.join("ears-sdd"), assets::launcher("ears-sdd"), true)?;
    warn_about_line_endings(&project);

    println!("Installed EARS/TDD policy components.");
    println!("Next: edit .specify/ears-sdd.toml, then run `ears-sdd validate --phase spec`.");
    Ok(())
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
    let name = if cfg!(windows) { "specify.exe" } else { "specify" };
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
    std::fs::write(path, contents)
        .map_err(|error| format!("Could not write {}: {error}", display(path)))?;
    println!("Created {}", display(path));
    Ok(())
}

fn install_launcher(path: &Path, contents: &[u8], posix: bool) -> Result<(), String> {
    if path.exists() {
        println!("Kept existing {}", display(path));
        return Ok(());
    }
    // Written from embedded bytes that a build-time test proves are LF, so a Windows host cannot
    // produce a launcher that fails on Linux.
    std::fs::write(path, contents)
        .map_err(|error| format!("Could not write {}: {error}", display(path)))?;
    if posix {
        set_executable(path)?;
    }
    println!("Created {}", display(path));
    Ok(())
}

#[cfg(unix)]
fn set_executable(path: &Path) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = std::fs::metadata(path)
        .map_err(|error| error.to_string())?
        .permissions();
    permissions.set_mode(permissions.mode() | 0o111);
    std::fs::set_permissions(path, permissions).map_err(|error| error.to_string())
}

/// On Windows the mode bit does not exist in the filesystem, so the only place it can survive is
/// the index. Ask git to record it rather than leaving a POSIX clone with `Permission denied`.
#[cfg(not(unix))]
fn set_executable(path: &Path) -> Result<(), String> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };
    let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
    let recorded = Command::new("git")
        .args(["update-index", "--chmod=+x", "--add", &name])
        .current_dir(parent)
        .status();
    match recorded {
        Ok(status) if status.success() => Ok(()),
        _ => {
            println!(
                "Note: could not record the execute bit for {name}. On a POSIX checkout run \
                 `chmod +x {name}` or `git update-index --chmod=+x {name}`."
            );
            Ok(())
        }
    }
}

fn warn_about_line_endings(project: &Path) {
    if project.join(".gitattributes").exists() {
        return;
    }
    println!(
        "Note: this project has no .gitattributes. On a Windows clone with core.autocrlf=true git \
         rewrites `ears-sdd` with CRLF and it stops working on Linux and macOS. Add:\n    \
         ears-sdd text eol=lf"
    );
}
