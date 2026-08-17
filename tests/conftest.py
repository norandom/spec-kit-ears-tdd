from __future__ import annotations

import importlib.util
import json
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "components" / "extension" / "ears-validate" / "scripts" / "ears_sdd.py"

DEFAULT_CONFIG = """\
spec_glob = "specs/*/spec.md"
traceability_file = "traceability.toml"
require_test_files = true
test_command = "uv run pytest"
production_roots = ["src"]
test_roots = ["tests"]
source_extensions = [".py"]
"""


def load_validator():
    spec = importlib.util.spec_from_file_location("ears_sdd_under_test", SCRIPT)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


@pytest.fixture
def validator():
    return load_validator()


class Project:
    """Builder for a throwaway project tree the discovery tests operate on."""

    def __init__(self, root: Path) -> None:
        self.root = root
        (root / ".specify").mkdir(parents=True, exist_ok=True)
        (root / ".specify" / "ears-sdd.toml").write_text(DEFAULT_CONFIG, encoding="utf-8")
        (root / "tests").mkdir(exist_ok=True)
        (root / "src").mkdir(exist_ok=True)

    def feature(self, name: str, spec_body: str, traceability: str | None = None) -> Path:
        directory = self.root / "specs" / name
        directory.mkdir(parents=True, exist_ok=True)
        (directory / "spec.md").write_text(spec_body, encoding="utf-8")
        if traceability is not None:
            (directory / "traceability.toml").write_text(traceability, encoding="utf-8")
        return directory

    def raw_spec(self, name: str, payload: bytes) -> Path:
        directory = self.root / "specs" / name
        directory.mkdir(parents=True, exist_ok=True)
        target = directory / "spec.md"
        target.write_bytes(payload)
        return target

    def active_feature(self, relative: str) -> None:
        (self.root / ".specify" / "feature.json").write_text(
            json.dumps({"feature_directory": relative}), encoding="utf-8"
        )

    def test_file(self, relative: str, body: str = "def test_placeholder(): pass\n") -> Path:
        target = self.root / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(body, encoding="utf-8")
        return target

    def codes(self, result: dict) -> set[str]:
        return {finding["code"] for finding in result["findings"]}


@pytest.fixture
def project(tmp_path: Path) -> Project:
    return Project(tmp_path)


def one_requirement(identifier: str = "REQ-001", text: str | None = None) -> str:
    text = text or "The service shall persist the record."
    return f"# Example\n\n- {identifier}: {text}\n"


def automated_trace(*identifiers: str, selector: str = "tests/test_records.py::test_case") -> str:
    lines = ['schema_version = "1.0"', ""]
    for identifier in identifiers:
        lines.append(f"[requirements.{identifier}]")
        lines.append('verification = "automated"')
        lines.append(f'tests = ["{selector}"]')
        lines.append("")
    return "\n".join(lines)
