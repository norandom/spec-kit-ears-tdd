//! Runs the conformance corpus through the command-line interface.
//!
//! The corpus is data, not test code, so a second implementation can be held to exactly the same
//! cases by writing a runner of this size. That is the whole point: when this crate replaces the
//! Python validator, the evidence that it behaves the same — or knowingly differently — is the
//! corpus, not a promise.

use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::process::Command;

fn corpus_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("conformance")
        .join("cases")
}

/// Finding codes whose structured `detail` is part of the claim, not decoration.
///
/// For most findings the code, location and severity are the evidence and `detail` is a courtesy
/// to the reader. For a containment finding the detail is the finding: which comparison, which
/// effect, which witness state, which requirements. A runner that dropped it would accept a report
/// naming the wrong witness, or a witness from the wrong reference, as a match.
const EVIDENCE_BEARING_PREFIXES: &[&str] = &["CONTAIN_"];

fn detail_is_evidence(code: &str) -> bool {
    EVIDENCE_BEARING_PREFIXES
        .iter()
        .any(|prefix| code.starts_with(prefix))
}

/// Drop everything that legitimately varies between runs — the timestamp, the absolute project
/// path, the validator version — and impose a total order on findings so comparison is stable.
///
/// Containment evidence is kept: `detail` on evidence-bearing findings and the top-level
/// `comparisons` array. Both are added only when present, so a report from a run without the
/// containment layer normalizes to exactly the shape it always had and the existing corpus needs
/// no change.
fn normalize(report: &Value) -> Value {
    let mut findings: Vec<Value> = report["findings"]
        .as_array()
        .cloned()
        .unwrap_or_default()
        .iter()
        .map(|finding| {
            let code = finding.get("code").cloned().unwrap_or(Value::Null);
            let mut normalized = json!({
                "code": code,
                "path": finding.get("path").cloned().unwrap_or(Value::Null),
                "feature": finding.get("feature").cloned().unwrap_or(Value::Null),
                "requirement": finding.get("requirement").cloned().unwrap_or(Value::Null),
                "line": finding.get("line").cloned().unwrap_or(Value::Null),
                "severity": finding.get("severity").cloned().unwrap_or(Value::Null),
            });
            if detail_is_evidence(code.as_str().unwrap_or("")) {
                if let Some(detail) = finding.get("detail") {
                    normalized["detail"] = detail.clone();
                }
            }
            normalized
        })
        .collect();
    // The comparison and effect are part of a containment finding's identity. Without them two
    // widenings on one requirement, from different comparisons or on different effects, would sort
    // by accident of emission order and the corpus could not state them as data.
    findings.sort_by_key(|finding| {
        (
            finding["code"].as_str().unwrap_or("").to_string(),
            finding["path"].as_str().unwrap_or("").to_string(),
            finding["line"].as_u64().unwrap_or(0),
            finding["requirement"].as_str().unwrap_or("").to_string(),
            finding["feature"].as_str().unwrap_or("").to_string(),
            finding["detail"]["comparison"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            finding["detail"]["effect"]
                .as_str()
                .unwrap_or("")
                .to_string(),
        )
    });

    let mut normalized = json!({
        "schema_version": report["schema_version"],
        "ok": report["ok"],
        "phase": report["phase"],
        "scope": report["provenance"]["scope"],
        "summary": report["summary"],
        "findings": findings,
    });
    if let Some(comparisons) = report.get("comparisons").and_then(Value::as_array) {
        let mut comparisons = comparisons.clone();
        comparisons.sort_by_key(|comparison| comparison["id"].as_str().unwrap_or("").to_string());
        normalized["comparisons"] = Value::Array(comparisons);
    }
    normalized
}

/// Every way one case's run disagrees with its expected result. Empty means the case holds.
///
/// Separated from the corpus loop so that the runner itself can be tested: a runner that silently
/// accepts a changed witness is a corpus that proves nothing, and the only way to know it does not
/// is to hand it one.
fn divergences(name: &str, expected: &Value, report: &Value, exited_zero: bool) -> Vec<String> {
    let mut found = Vec::new();

    let actual = normalize(report);
    if &actual != expected {
        found.push(format!(
            "{name}:\n  expected {}\n  actual   {}",
            serde_json::to_string(expected).unwrap(),
            serde_json::to_string(&actual).unwrap()
        ));
    }

    // A failing gate has to be observable to a shell, not only to a JSON consumer.
    let expected_failure = expected["ok"] == json!(false);
    if expected_failure && exited_zero {
        found.push(format!(
            "{name}: report is not ok but the process exited zero"
        ));
    }
    if !expected_failure && !exited_zero {
        found.push(format!(
            "{name}: report is ok but the process exited non-zero"
        ));
    }
    found
}

#[test]
fn corpus_matches_expected_results() {
    let root = corpus_root();
    let mut cases: Vec<PathBuf> = std::fs::read_dir(&root)
        .unwrap_or_else(|error| panic!("corpus not readable at {}: {error}", root.display()))
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect();
    cases.sort();
    assert!(!cases.is_empty(), "corpus is empty at {}", root.display());

    let mut failures: Vec<String> = Vec::new();

    for case in &cases {
        let name = case.file_name().unwrap().to_string_lossy().to_string();
        let invocation: Value =
            serde_json::from_str(&std::fs::read_to_string(case.join("cmd.json")).unwrap())
                .unwrap_or_else(|error| panic!("{name}: cmd.json is not valid JSON: {error}"));
        let expected: Value =
            serde_json::from_str(&std::fs::read_to_string(case.join("expected.json")).unwrap())
                .unwrap_or_else(|error| panic!("{name}: expected.json is not valid JSON: {error}"));

        let mut command = Command::new(env!("CARGO_BIN_EXE_ears-sdd"));
        for argument in invocation["args"].as_array().unwrap() {
            command.arg(argument.as_str().unwrap());
        }
        command
            .arg("--project")
            .arg(case.join("project"))
            .arg("--json");

        let output = command
            .output()
            .unwrap_or_else(|error| panic!("{name}: could not run the validator: {error}"));
        let stdout = String::from_utf8_lossy(&output.stdout);
        let report: Value = match serde_json::from_str(&stdout) {
            Ok(report) => report,
            Err(error) => {
                failures.push(format!(
                    "{name}: output was not JSON ({error}); stderr: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
                continue;
            }
        };

        failures.extend(divergences(
            &name,
            &expected,
            &report,
            output.status.success(),
        ));
    }

    assert!(
        failures.is_empty(),
        "{} of {} conformance cases diverged:\n{}",
        failures.len(),
        cases.len(),
        failures.join("\n")
    );
}

#[test]
fn every_case_has_a_fixture_and_an_expected_result() {
    for case in std::fs::read_dir(corpus_root())
        .unwrap()
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
    {
        let name = case.file_name().unwrap().to_string_lossy().to_string();
        assert!(case.join("project").is_dir(), "{name}: missing project/");
        assert!(case.join("cmd.json").is_file(), "{name}: missing cmd.json");
        assert!(
            case.join("expected.json").is_file(),
            "{name}: missing expected.json"
        );
    }
}

/// A containment report of the shape the 009 contract specifies, with its normalized expectation.
fn containment_pair() -> (Value, Value) {
    let report = json!({
        "schema_version": "1.0",
        "ok": false,
        "phase": "plan",
        "project": "/somewhere/on/this/machine",
        "provenance": { "scope": { "source": "glob", "value": "specs/*/spec.md" }, "generated_at": "now" },
        "features": [],
        "summary": { "errors": 1, "effects_compared": 1 },
        "comparisons": [{
            "id": "alpha-revision",
            "kind": "revision",
            "reports": ["widening", "narrowing"],
            "reference": { "model": "specs/001-alpha/reference.toml" },
            "status": "widened",
            "complete": true
        }],
        "findings": [{
            "code": "CONTAIN_WIDENED",
            "message": "prose that is free to change",
            "path": "specs/001-alpha/model.toml",
            "feature": "specs/001-alpha",
            "requirement": "W-1",
            "severity": "error",
            "detail": {
                "comparison": "alpha-revision",
                "effect": "allow_write",
                "witness": { "maintenance-window": true, "operator-present": false },
                "candidate_requirements": ["specs/001-alpha:W-1"]
            }
        }]
    });
    let expected = normalize(&report);
    (report, expected)
}

/// One edit to a report, named for the evidence it changes.
type Mutation = fn(&mut Value);

#[test]
fn containment_evidence_is_not_discarded() {
    let (report, expected) = containment_pair();
    assert!(
        divergences("baseline", &expected, &report, false).is_empty(),
        "an unchanged containment report must match its own normalization"
    );

    let mutations: Vec<(&str, Mutation)> = vec![
        ("witness", |r| {
            r["findings"][0]["detail"]["witness"]["operator-present"] = json!(true)
        }),
        ("effect", |r| {
            r["findings"][0]["detail"]["effect"] = json!("block_write")
        }),
        ("comparison id on the finding", |r| {
            r["findings"][0]["detail"]["comparison"] = json!("other")
        }),
        ("attributed requirements", |r| {
            r["findings"][0]["detail"]["candidate_requirements"] = json!([])
        }),
        ("reference source", |r| {
            r["comparisons"][0]["reference"]["model"] = json!("specs/001-alpha/model.toml")
        }),
        ("comparison kind", |r| {
            r["comparisons"][0]["kind"] = json!("policy")
        }),
        ("comparison status", |r| {
            r["comparisons"][0]["status"] = json!("equal")
        }),
        ("comparison record removed", |r| {
            r.as_object_mut().unwrap().remove("comparisons");
        }),
    ];
    for (what, mutate) in mutations {
        let mut changed = report.clone();
        mutate(&mut changed);
        assert!(
            !divergences("mutated", &expected, &changed, false).is_empty(),
            "changing the {what} was not detected"
        );
    }

    assert!(
        !divergences("exit", &expected, &report, true).is_empty(),
        "a failing report whose process exited zero was not detected"
    );

    // Things that legitimately vary must not cause a divergence.
    let mut noise = report.clone();
    noise["project"] = json!("/elsewhere");
    noise["provenance"]["generated_at"] = json!("later");
    noise["findings"][0]["message"] = json!("reworded");
    assert!(divergences("noise", &expected, &noise, false).is_empty());
}

#[test]
fn reports_without_containment_keep_their_normalized_shape() {
    let report = json!({
        "schema_version": "1.0",
        "ok": true,
        "phase": "spec",
        "provenance": { "scope": { "source": "glob", "value": "specs/*/spec.md" } },
        "summary": { "errors": 0 },
        "findings": [{
            "code": "MODEL_CONFLICT",
            "path": "specs/001-alpha/model.toml",
            "severity": "error",
            "detail": { "states": 4 }
        }]
    });
    let normalized = normalize(&report);
    let mut keys: Vec<&str> = normalized
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        [
            "findings",
            "ok",
            "phase",
            "schema_version",
            "scope",
            "summary"
        ]
    );
    let mut finding_keys: Vec<&str> = normalized["findings"][0]
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    finding_keys.sort_unstable();
    assert_eq!(
        finding_keys,
        ["code", "feature", "line", "path", "requirement", "severity"],
        "detail on a non-containment finding must stay out of the comparison"
    );
}
