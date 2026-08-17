"""Requirement discovery: the gate has to see everything it claims to have checked.

These tests specify `specs/001-complete-requirement-discovery`. Several of them fail against the
current validator; that is the point. Each test names the requirement it verifies in its docstring
so the mapping in traceability.toml stays reviewable.
"""

from __future__ import annotations

import json

from conftest import automated_trace, one_requirement

SPEC_FORMS = """\
# Requirement forms

- REQ-001: The service shall persist the record.

1. REQ-002: The service shall emit an audit entry.

### REQ-003: The service shall reject an unknown caller.

| ID | Requirement |
| --- | --- |
| REQ-004 | The service shall retry a failed write. |

> REQ-005: The service shall close the connection.
"""

SPEC_FENCED = """\
# Fenced examples

- REQ-001: The service shall persist the record.

The block below illustrates a requirement that would fail validation. It is documentation, not a
requirement of this feature:

```text
- REQ-900: the service persists things
```
"""


def test_all_markdown_forms_are_discovered(project, validator):
    """REQ-001: bullet, numbered, heading, table row, and block quote are all requirements."""
    directory = project.feature("001-forms", SPEC_FORMS)

    requirements, _ = validator.parse_requirements(project.root, directory / "spec.md")

    assert {requirement.identifier for requirement in requirements} == {
        "REQ-001",
        "REQ-002",
        "REQ-003",
        "REQ-004",
        "REQ-005",
    }


def test_fenced_code_block_is_not_a_requirement(project, validator):
    """REQ-002: an illustrative requirement inside a fence is not discovered."""
    directory = project.feature("001-fenced", SPEC_FENCED)

    requirements, _ = validator.parse_requirements(project.root, directory / "spec.md")

    identifiers = {requirement.identifier for requirement in requirements}
    assert "REQ-001" in identifiers
    assert "REQ-900" not in identifiers


def test_duplicate_identifier_is_reported(project, validator):
    """REQ-003: the same identifier twice in one specification is an error."""
    body = (
        "# Duplicates\n\n"
        "- REQ-001: The service shall persist the record.\n"
        "- REQ-001: The service shall discard the record.\n"
    )
    project.feature("001-duplicate", body)

    result = validator.validate_project(project.root, "spec", None)

    assert "REQ_DUPLICATE" in project.codes(result)


def test_identifier_accepts_three_or_more_digits(project, validator):
    """REQ-004: identifiers are not limited to exactly three digits."""
    body = (
        "# Wide identifiers\n\n"
        "- REQ-001: The service shall persist the record.\n"
        "- REQ-0042: The service shall emit an audit entry.\n"
    )
    directory = project.feature("001-wide", body)

    requirements, _ = validator.parse_requirements(project.root, directory / "spec.md")

    assert {requirement.identifier for requirement in requirements} == {"REQ-001", "REQ-0042"}


def test_all_features_mode_evaluates_every_spec(project, validator, capsys):
    """REQ-005: all-features mode ignores the active-feature pointer."""
    for name in ("001-alpha", "002-bravo", "003-charlie"):
        project.feature(name, one_requirement())
    project.active_feature("specs/001-alpha")

    exit_code = validator.main(
        ["validate", "--project", str(project.root), "--phase", "spec", "--all", "--json"]
    )

    payload = json.loads(capsys.readouterr().out)
    assert exit_code == 0
    assert payload["summary"]["features"] == 3


def test_missing_active_feature_reports_finding(project, validator):
    """REQ-006: a stale active-feature pointer is an error, not a silent fallback."""
    project.feature("001-alpha", one_requirement())
    project.active_feature("specs/999-deleted")

    result = validator.validate_project(project.root, "spec", None)

    assert "FEATURE_MISSING" in project.codes(result)


def test_selector_outside_test_roots_is_reported(project, validator):
    """REQ-009: a traversal selector does not get normalized into the test roots."""
    project.feature(
        "001-alpha",
        one_requirement(),
        automated_trace("REQ-001", selector="../tests/test_records.py::test_case"),
    )
    project.test_file("tests/test_records.py")

    result = validator.validate_project(project.root, "plan", None)

    assert "TRACE_TEST_ROOT" in project.codes(result)


def test_undecodable_spec_reports_finding(project, validator):
    """REQ-010: a specification that is not valid UTF-8 is reported, not skipped."""
    project.raw_spec("001-broken", b"# Broken\n\n- REQ-001: The service shall \xff\xfe persist.\n")

    result = validator.validate_project(project.root, "spec", None)

    assert "SPEC_UNREADABLE" in project.codes(result)


def test_modal_inside_quoted_literal_is_non_normative(project, validator):
    """REQ-012: a competing modal inside a quoted literal is not a policy violation."""
    directory = project.feature(
        "001-quoted",
        one_requirement(text='The service shall log the message "operation may fail".'),
    )

    requirements, _ = validator.parse_requirements(project.root, directory / "spec.md")
    findings = validator.validate_ears(project.root, requirements[0])

    assert "EARS_MODAL" not in {finding.code for finding in findings}
