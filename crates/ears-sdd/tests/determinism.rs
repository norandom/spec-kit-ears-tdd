//! Repeat-run stability of containment verdicts.
//!
//! A witness is only evidence if a second run produces the same one. The corpus pins the JSON
//! shape; this test pins the *repeatability* property, which a fixture alone cannot establish.

use std::collections::BTreeSet;
use std::path::Path;

use ears_sdd::config::{Checks, Config};

fn fixture(dir: &Path) {
    let specify = dir.join(".specify");
    let feature = dir.join("specs").join("001-alpha");
    std::fs::create_dir_all(&specify).unwrap();
    std::fs::create_dir_all(&feature).unwrap();
    std::fs::write(
        specify.join("ears-sdd.toml"),
        "[checks]\nvocabulary = true\nconstraints = true\ncontainment = true\n",
    )
    .unwrap();
    std::fs::write(
        specify.join("vocabulary.toml"),
        "schema_version = \"1.0\"\n[terms.maintenance-window]\nlabel = \"Maintenance window\"\ndefinition = \"Whether the workstation is inside a scheduled maintenance window.\"\ndomain = { kind = \"bool\" }\n[terms.operator-present]\nlabel = \"Operator present\"\ndefinition = \"Whether a named operator is signed in.\"\ndomain = { kind = \"bool\" }\n",
    )
    .unwrap();
    std::fs::write(
        specify.join("containment.toml"),
        "schema_version = \"1.0\"\n[[comparisons]]\nid = \"c\"\nkind = \"revision\"\nreference = { feature = \"specs/001-alpha\", spec = \"specs/001-alpha/reference.md\", model = \"specs/001-alpha/reference.toml\", vocabulary = \".specify/vocabulary.toml\" }\ncandidates = [{ feature = \"specs/001-alpha\", spec = \"specs/001-alpha/spec.md\", model = \"specs/001-alpha/model.toml\", vocabulary = \".specify/vocabulary.toml\" }]\n",
    )
    .unwrap();
    let model = "schema_version = \"1.0\"\n[effects]\nallow_write = {}\n";
    let candidate = format!(
        "{model}[requirements.T-001]\nwhen = \"maintenance-window\"\nthen = \"allow_write\"\n"
    );
    let reference = format!(
        "{model}[requirements.T-001]\nwhen = \"maintenance-window and operator-present\"\nthen = \"allow_write\"\n"
    );
    std::fs::write(feature.join("model.toml"), candidate).unwrap();
    std::fs::write(feature.join("reference.toml"), reference).unwrap();
    std::fs::write(
        feature.join("spec.md"),
        "# f\n\n- T-001: The service shall act.\n",
    )
    .unwrap();
    std::fs::write(
        feature.join("reference.md"),
        "# f\n\n- T-001: The service shall act.\n",
    )
    .unwrap();
}

#[test]
fn containment_witness_is_stable() {
    let dir = std::env::temp_dir().join(format!("ears-contain-determinism-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    fixture(&dir);

    let config = Config {
        checks: Checks {
            containment: true,
            ..Checks::default()
        },
        ..Config::default()
    };
    let scope: BTreeSet<String> = ["specs/001-alpha".to_string()].into_iter().collect();
    let first = ears_sdd::contain::validate(&dir, &config, &scope, false);
    let second = ears_sdd::contain::validate(&dir, &config, &scope, false);

    let witness_of = |outcome: &ears_sdd::contain::Outcome| {
        outcome
            .findings
            .iter()
            .find(|f| f.code == "CONTAIN_WIDENED")
            .and_then(|f| f.detail.as_ref().and_then(|d| d.get("witness").cloned()))
            .expect("a widening with a witness")
    };
    assert_eq!(witness_of(&first), witness_of(&second));
    assert_eq!(
        serde_json::to_string(&first.comparisons).unwrap(),
        serde_json::to_string(&second.comparisons).unwrap()
    );

    let _ = std::fs::remove_dir_all(&dir);
}
