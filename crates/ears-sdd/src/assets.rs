//! The policy components, embedded in the binary.
//!
//! The Python implementation resolved these from either a packaged `assets/` directory or the
//! repository root depending on how it had been installed, and the wheel had to force-include three
//! directory trees to make that work. Embedding removes the guess entirely: the binary always has
//! its components, and `init` materializes them to a temporary directory only because `specify`
//! takes a path.

use include_dir::{include_dir, Dir};
use std::path::Path;

pub static COMPONENTS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../components");
pub static CONFIG: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../config");

pub fn config_sample() -> &'static str {
    CONFIG
        .get_file("ears-sdd.toml.sample")
        .and_then(|file| file.contents_utf8())
        .expect("the configuration sample is embedded at build time")
}

pub fn traceability_sample() -> &'static str {
    CONFIG
        .get_file("traceability.toml.sample")
        .and_then(|file| file.contents_utf8())
        .expect("the traceability sample is embedded at build time")
}

/// The consuming project's CI workflow, with the validator version pinned to this binary.
///
/// Pinning to `CARGO_PKG_VERSION` rather than to a floating `latest` keeps a verdict reproducible:
/// the tree that passes on a developer machine is checked by the same validator in CI, and a new
/// release cannot turn a green branch red without someone choosing to upgrade.
pub fn ci_workflow() -> String {
    CONFIG
        .get_file("github-actions.yml.sample")
        .and_then(|file| file.contents_utf8())
        .expect("the CI sample is embedded at build time")
        .replace("{VERSION}", env!("CARGO_PKG_VERSION"))
}

/// Write an embedded tree to disk so `specify ... --dev <path>` has something to read.
pub fn materialize(directory: &Dir<'_>, destination: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(destination)?;
    for file in directory.files() {
        let target = destination.join(file.path().file_name().unwrap_or_default());
        std::fs::write(target, file.contents())?;
    }
    for child in directory.dirs() {
        let name = child.path().file_name().unwrap_or_default();
        materialize(child, &destination.join(name))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_component_is_embedded() {
        for expected in [
            "preset/ears-tdd/preset.yml",
            "extension/ears-validate/extension.yml",
            "workflow/ears-sdd/workflow.yml",
        ] {
            assert!(
                COMPONENTS.get_file(expected).is_some(),
                "component missing from the binary: {expected}"
            );
        }
        assert!(!config_sample().is_empty());
        assert!(!traceability_sample().is_empty());
    }

    /// The pin is the whole point of generating this file rather than shipping a static one, and a
    /// leftover placeholder would silently produce a 404 at install time inside someone else's CI.
    #[test]
    fn the_ci_workflow_pins_this_version() {
        let workflow = ci_workflow();
        assert!(
            !workflow.contains("{VERSION}"),
            "the version placeholder survived substitution"
        );
        assert!(
            workflow.contains(&format!(
                "v{}/ears-sdd-installer.sh",
                env!("CARGO_PKG_VERSION")
            )),
            "the installer URL does not pin this binary's version"
        );
        // Without --all the gate evaluates one feature and reports a pass for the project.
        assert!(workflow.contains("--phase final --all"));
    }

    fn walk(directory: &Dir<'_>, found: &mut Vec<String>) {
        for file in directory.files() {
            found.push(file.path().to_string_lossy().replace('\\', "/"));
        }
        for child in directory.dirs() {
            walk(child, found);
        }
    }

    /// Nothing shipped in the binary may reference an interpreter. The launchers and shims existed
    /// only to locate Python; if one comes back, the single-binary property has quietly been lost.
    #[test]
    fn no_component_shells_out_to_an_interpreter() {
        let mut found = Vec::new();
        walk(&COMPONENTS, &mut found);
        assert!(!found.is_empty(), "no components were embedded at all");
        for path in found {
            assert!(
                !path.ends_with(".py") && !path.ends_with(".sh") && !path.ends_with(".ps1"),
                "an interpreter script is embedded in the binary: {path}"
            );
        }
    }
}
