//! Envelope interchange-format adapters.
//!
//! The registry is a fixed table in code: there is no plugin path and no configuration that adds
//! to it. A format with no registered adapter is refused by name (`CONTAIN_DEFINITION_UNREADABLE`),
//! never interpreted from memory of documentation. A native adapter is written only from a
//! checked-in sample of the runtime's real file (feature 010 task T009, blocked on that input).

/// The built-in interchange format's name.
pub const INTERCHANGE_FORMAT: &str = "ears-sdd-envelope/1.0";

/// Look up a format name in the fixed registry. Returns the adapter id and version.
pub fn lookup(format: &str) -> Option<(&'static str, &'static str)> {
    match format {
        INTERCHANGE_FORMAT => Some(("ears-sdd-envelope", "1.0")),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_is_fixed() {
        assert_eq!(
            lookup(INTERCHANGE_FORMAT),
            Some(("ears-sdd-envelope", "1.0"))
        );
        assert!(lookup("openshell-native/1.0").is_none());
        assert!(lookup("").is_none());
    }
}
