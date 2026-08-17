//! Validation must not touch the project it validates.
//!
//! The extension manifest declares `effect: "read-only"` and the architecture document lists this
//! as a safety property, so it deserves a test rather than a claim. The Python implementation
//! violated it without anyone noticing: loading the validator wrote `__pycache__` beside the source,
//! inside the committed extension tree of every consuming project.
//!
//! This is also the safety net for any future reimplementation. Whatever the internals become, a
//! gate run may not leave a mark.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Every file under `root`, mapped to its length and contents.
fn snapshot(root: &Path) -> BTreeMap<PathBuf, (u64, Vec<u8>)> {
    let mut state = BTreeMap::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(directory) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&directory) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if let Ok(bytes) = std::fs::read(&path) {
                let relative = path.strip_prefix(root).unwrap_or(&path).to_path_buf();
                state.insert(relative, (bytes.len() as u64, bytes));
            }
        }
    }
    state
}

fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("create destination");
    for entry in std::fs::read_dir(from).expect("read source").flatten() {
        let target = to.join(entry.file_name());
        if entry.path().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), &target).expect("copy file");
        }
    }
}

#[test]
fn no_gate_modifies_the_project() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("conformance")
        .join("cases")
        .join("feature-qualified-ids")
        .join("project");
    assert!(fixture.is_dir(), "fixture missing at {}", fixture.display());

    // Copied rather than used in place, so a regression cannot damage the corpus itself.
    let scratch = tempfile::tempdir().expect("temp dir");
    let project = scratch.path().join("project");
    copy_tree(&fixture, &project);

    let before = snapshot(&project);
    assert!(!before.is_empty(), "fixture copy is empty");

    for phase in ["spec", "plan", "tasks", "final"] {
        for all in [false, true] {
            let mut command = Command::new(env!("CARGO_BIN_EXE_ears-sdd"));
            command
                .arg("validate")
                .arg("--project")
                .arg(&project)
                .arg("--phase")
                .arg(phase);
            if all {
                command.arg("--all");
            }
            command.output().expect("run the validator");
        }
    }

    let after = snapshot(&project);

    let added: Vec<_> = after.keys().filter(|k| !before.contains_key(*k)).collect();
    let removed: Vec<_> = before.keys().filter(|k| !after.contains_key(*k)).collect();
    let changed: Vec<_> = before
        .iter()
        .filter(|(path, value)| after.get(*path).is_some_and(|other| other != *value))
        .map(|(path, _)| path)
        .collect();

    assert!(
        added.is_empty() && removed.is_empty() && changed.is_empty(),
        "validation modified the project\n  added: {added:?}\n  removed: {removed:?}\n  changed: {changed:?}"
    );
}
