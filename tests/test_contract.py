"""The machine-readable result is a contract, and validation is read-only.

These tests specify the contract requirements of `specs/001-complete-requirement-discovery`. The
mutation test is the safety net for the eventual reimplementation: whatever the internals become,
a validation run may not touch the project.
"""

from __future__ import annotations

import hashlib
from pathlib import Path

from conftest import one_requirement


def snapshot(root: Path) -> dict[str, tuple[int, str]]:
    """Map every file under *root* to its size and content digest."""
    state: dict[str, tuple[int, str]] = {}
    for path in sorted(root.rglob("*")):
        if not path.is_file():
            continue
        payload = path.read_bytes()
        state[path.relative_to(root).as_posix()] = (
            len(payload),
            hashlib.sha256(payload).hexdigest(),
        )
    return state


def test_result_carries_schema_version(project, validator):
    """REQ-008: consumers can detect a breaking change to the result shape."""
    project.feature("001-alpha", one_requirement())

    result = validator.validate_project(project.root, "spec", None)

    assert result["schema_version"] == "1.0"


def test_result_reports_examined_file_count(project, validator):
    """REQ-007: the result states how many specification files were actually read."""
    project.feature("001-alpha", one_requirement())
    project.feature("002-bravo", one_requirement())

    result = validator.validate_project(project.root, "spec", None)

    assert result["summary"]["specs_examined"] == 2


def test_validation_run_mutates_nothing(project, validator):
    """REQ-011: validation is read-only across every gate."""
    project.feature("001-alpha", one_requirement())
    project.test_file("tests/test_records.py")
    before = snapshot(project.root)

    for phase in ("spec", "plan", "tasks", "final"):
        validator.validate_project(project.root, phase, None)

    assert snapshot(project.root) == before
