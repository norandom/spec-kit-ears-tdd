from __future__ import annotations

import importlib.util
import json
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "components" / "extension" / "ears-validate" / "scripts" / "ears_sdd.py"


def load_validator():
    spec = importlib.util.spec_from_file_location("test_ears_sdd_validator", SCRIPT)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


@pytest.fixture
def validator():
    return load_validator()


def write_project(tmp_path: Path, requirement: str | None = None) -> Path:
    (tmp_path / ".specify").mkdir()
    (tmp_path / ".specify" / "ears-sdd.toml").write_text(
        """
spec_glob = "specs/*/spec.md"
traceability_file = "traceability.toml"
require_test_files = true
test_command = "uv run pytest"
production_roots = ["src"]
test_roots = ["tests"]
source_extensions = [".py"]
""".strip()
        + "\n",
        encoding="utf-8",
    )
    feature = tmp_path / "specs" / "example"
    feature.mkdir(parents=True)
    requirement = requirement or (
        "REQ-001: When the user submits valid input, the service shall persist the record."
    )
    (feature / "spec.md").write_text(f"# Example\n\n- {requirement}\n", encoding="utf-8")
    (feature / "traceability.toml").write_text(
        """
schema_version = "1.0"

[requirements.REQ-001]
verification = "automated"
tests = ["tests/test_records.py::test_valid_record_is_persisted"]
""".strip()
        + "\n",
        encoding="utf-8",
    )
    (tmp_path / "tests").mkdir()
    (tmp_path / "tests" / "test_records.py").write_text(
        "def test_valid_record_is_persisted(): pass\n"
    )
    (tmp_path / "src").mkdir()
    (tmp_path / "src" / "records.py").write_text("def persist(record): return record\n")
    return tmp_path


def test_valid_project_passes_final_gate(tmp_path: Path, validator):
    project = write_project(tmp_path)

    result = validator.validate_project(project, "final", None)

    assert result["ok"] is True
    assert result["summary"] == {
        "features": 1,
        "requirements": 1,
        "errors": 0,
        "warnings": 0,
    }


def test_spec_gate_rejects_competing_modal(tmp_path: Path, validator):
    project = write_project(
        tmp_path,
        "REQ-001: When input arrives, the service should validate and shall persist it.",
    )

    result = validator.validate_project(project, "spec", None)

    assert result["ok"] is False
    assert "EARS_MODAL" in {finding["code"] for finding in result["findings"]}


def test_plan_gate_requires_mapping_for_every_requirement(tmp_path: Path, validator):
    project = write_project(tmp_path)
    spec = project / "specs" / "example" / "spec.md"
    spec.write_text(
        spec.read_text(encoding="utf-8")
        + "- REQ-002: While maintenance mode is active, the service shall reject writes.\n",
        encoding="utf-8",
    )

    result = validator.validate_project(project, "plan", None)

    assert result["ok"] is False
    assert any(
        finding["code"] == "TRACE_MISSING_REQ" and finding["requirement"] == "REQ-002"
        for finding in result["findings"]
    )


def test_final_gate_rejects_requirement_id_in_production_code(tmp_path: Path, validator):
    project = write_project(tmp_path)
    (project / "src" / "records.py").write_text(
        "# REQ-001\ndef persist(record): return record\n", encoding="utf-8"
    )

    result = validator.validate_project(project, "final", None)

    assert result["ok"] is False
    assert "CODE_REQ_ID" in {finding["code"] for finding in result["findings"]}


def test_cli_json_is_machine_readable(tmp_path: Path, validator, capsys):
    project = write_project(tmp_path)

    exit_code = validator.main(
        ["validate", "--project", str(project), "--phase", "final", "--json"]
    )

    payload = json.loads(capsys.readouterr().out)
    assert exit_code == 0
    assert payload["ok"] is True
