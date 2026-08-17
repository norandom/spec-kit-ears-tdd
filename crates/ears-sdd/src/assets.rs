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
pub static LAUNCHERS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../launchers");

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

pub fn launcher(name: &str) -> &'static [u8] {
    LAUNCHERS
        .get_file(name)
        .map(|file| file.contents())
        .expect("both launchers are embedded at build time")
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

    /// The POSIX launcher is copied into consuming projects verbatim. If a build ever happens on a
    /// worktree with CRLF, that corruption would be baked into every binary we ship.
    #[test]
    fn the_posix_launcher_is_embedded_with_unix_line_endings() {
        let bytes = launcher("ears-sdd");
        assert!(
            !bytes.windows(2).any(|pair| pair == b"\r\n"),
            "the embedded POSIX launcher contains CRLF"
        );
    }
}
