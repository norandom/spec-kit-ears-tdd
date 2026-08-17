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

/// Drop everything that legitimately varies between runs — the timestamp, the absolute project
/// path, the validator version — and impose a total order on findings so comparison is stable.
fn normalize(report: &Value) -> Value {
    let mut findings: Vec<Value> = report["findings"]
        .as_array()
        .cloned()
        .unwrap_or_default()
        .iter()
        .map(|finding| {
            json!({
                "code": finding.get("code").cloned().unwrap_or(Value::Null),
                "path": finding.get("path").cloned().unwrap_or(Value::Null),
                "feature": finding.get("feature").cloned().unwrap_or(Value::Null),
                "requirement": finding.get("requirement").cloned().unwrap_or(Value::Null),
                "line": finding.get("line").cloned().unwrap_or(Value::Null),
                "severity": finding.get("severity").cloned().unwrap_or(Value::Null),
            })
        })
        .collect();
    findings.sort_by_key(|finding| {
        (
            finding["code"].as_str().unwrap_or("").to_string(),
            finding["path"].as_str().unwrap_or("").to_string(),
            finding["line"].as_u64().unwrap_or(0),
            finding["requirement"].as_str().unwrap_or("").to_string(),
            finding["feature"].as_str().unwrap_or("").to_string(),
        )
    });

    json!({
        "schema_version": report["schema_version"],
        "ok": report["ok"],
        "phase": report["phase"],
        "scope": report["provenance"]["scope"],
        "summary": report["summary"],
        "findings": findings,
    })
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

        let actual = normalize(&report);
        if actual != expected {
            failures.push(format!(
                "{name}:\n  expected {}\n  actual   {}",
                serde_json::to_string(&expected).unwrap(),
                serde_json::to_string(&actual).unwrap()
            ));
        }

        // A failing gate has to be observable to a shell, not only to a JSON consumer.
        let expected_failure = expected["ok"] == json!(false);
        if expected_failure && output.status.success() {
            failures.push(format!("{name}: report is not ok but the process exited zero"));
        }
        if !expected_failure && !output.status.success() {
            failures.push(format!("{name}: report is ok but the process exited non-zero"));
        }
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
