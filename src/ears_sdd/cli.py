from __future__ import annotations

import argparse
import importlib.util
import os
import shutil
import subprocess
import sys
from pathlib import Path

REPOSITORY_ROOT = Path(__file__).resolve().parents[2]
PACKAGED_ASSETS = Path(__file__).resolve().parent / "assets"
ASSETS_ROOT = PACKAGED_ASSETS if PACKAGED_ASSETS.is_dir() else REPOSITORY_ROOT
COMPONENTS_ROOT = ASSETS_ROOT / "components"
VALIDATOR_SCRIPT = COMPONENTS_ROOT / "extension" / "ears-validate" / "scripts" / "ears_sdd.py"


def _display_command(arguments: list[str]) -> str:
    return subprocess.list2cmdline(arguments)


def _run(arguments: list[str], project: Path) -> None:
    # Flushed so the mutation trace stays ordered when stdout is a pipe. Without it the echoes
    # are buffered while the subprocess writes straight through, and CI logs -- the one place
    # the trace matters -- show every command after the output it was meant to announce.
    print(f"> {_display_command(arguments)}", flush=True)
    subprocess.run(arguments, cwd=project, check=True)


def _specify_executable() -> str:
    executable = "specify.exe" if sys.platform == "win32" else "specify"
    # Deliberately not resolved: on POSIX every install method (uv tool, pipx, venv) makes
    # <venv>/bin/python a symlink to the base interpreter, so resolving leaves the environment
    # that holds the console script. Windows copies python.exe instead, which is why the old
    # resolved lookup only ever worked there.
    interpreter_directory = Path(sys.executable).parent
    candidates = (
        interpreter_directory / executable,
        interpreter_directory.parent / "bin" / executable,
        interpreter_directory.parent / "Scripts" / executable,
        Path(sys.prefix) / "bin" / executable,
        Path(sys.prefix) / "Scripts" / executable,
    )
    for candidate in candidates:
        if candidate.is_file():
            return str(candidate)
    # A tool environment exposes only the requested package's entry points, so a dependency's
    # console script is usually absent from PATH. Fail with an instruction, never a bare name.
    discovered = shutil.which("specify")
    if discovered:
        return discovered
    raise SystemExit(
        "specify-cli was not found next to this interpreter or on PATH. Install it alongside "
        "this tool, for example:\n"
        "  uv tool install spec-kit-ears-tdd --with specify-cli==0.16.3"
    )


def _init_project(args: argparse.Namespace) -> int:
    project = Path(args.project).resolve()
    if not project.is_dir():
        print(f"Project directory does not exist: {project}", file=sys.stderr)
        return 2

    specify = _specify_executable()
    if not (project / ".specify").is_dir():
        _run(
            [
                specify,
                "init",
                ".",
                "--integration",
                args.integration,
                "--script",
                "py",
                "--ignore-agent-tools",
                "--force",
            ],
            project,
        )
    else:
        print(f"Spec Kit already initialized: {project}")

    _run(
        [
            specify,
            "preset",
            "add",
            "--dev",
            str(COMPONENTS_ROOT / "preset" / "ears-tdd"),
            "--priority",
            str(args.priority),
        ],
        project,
    )
    _run(
        [
            specify,
            "extension",
            "add",
            str(COMPONENTS_ROOT / "extension" / "ears-validate"),
            "--dev",
            "--priority",
            str(args.priority),
        ],
        project,
    )
    _run(
        [
            specify,
            "workflow",
            "add",
            str(COMPONENTS_ROOT / "workflow" / "ears-sdd"),
            "--dev",
        ],
        project,
    )

    config = project / ".specify" / "ears-sdd.toml"
    if not config.exists():
        config.write_text(
            (ASSETS_ROOT / "config" / "ears-sdd.toml.sample").read_text(encoding="utf-8"),
            encoding="utf-8",
        )
        print(f"Created {config}")
    else:
        print(f"Kept existing {config}")

    launchers = (
        (ASSETS_ROOT / "launchers" / "ears-sdd.ps1", project / "ears-sdd.ps1"),
        (ASSETS_ROOT / "launchers" / "ears-sdd", project / "ears-sdd"),
    )
    for source, destination in launchers:
        if destination.exists():
            print(f"Kept existing {destination}")
            continue
        shutil.copyfile(source, destination)
        if destination.name == "ears-sdd":
            os.chmod(destination, destination.stat().st_mode | 0o111)
        print(f"Created {destination}")

    print("Installed EARS/TDD policy components.")
    print("Next: edit .specify/ears-sdd.toml, then run `./ears-sdd validate --phase spec`.")
    return 0


def _run_validator(arguments: list[str]) -> int:
    spec = importlib.util.spec_from_file_location("ears_sdd_validator", VALIDATOR_SCRIPT)
    if spec is None or spec.loader is None:
        print(f"Unable to load validator: {VALIDATOR_SCRIPT}", file=sys.stderr)
        return 2
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return int(module.main(arguments))


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="ears-sdd",
        description="Install and validate the reusable EARS/TDD Spec Kit policy.",
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    init_parser = subparsers.add_parser("init", help="Install the policy into a Spec Kit project")
    init_parser.add_argument("--project", default=".", help="Target project directory")
    init_parser.add_argument("--integration", default="codex", help="Spec Kit integration")
    init_parser.add_argument(
        "--priority", type=int, default=5, help="Preset/extension priority (lower wins)"
    )

    for name in ("validate", "status"):
        child = subparsers.add_parser(name, help=f"{name.title()} EARS/TDD artifacts")
        child.add_argument("--project", default=".", help="Target project directory")
        child.add_argument("--feature", help="Specific feature directory")
        child.add_argument(
            "--phase",
            choices=("spec", "plan", "tasks", "final"),
            default="final",
            help="Validation gate",
        )
        child.add_argument("--json", action="store_true", help="Emit machine-readable JSON")

    return parser


def main(arguments: list[str] | None = None) -> int:
    args = build_parser().parse_args(arguments)
    if args.command == "init":
        try:
            return _init_project(args)
        except subprocess.CalledProcessError as error:
            return error.returncode or 1
        except OSError as error:
            print(f"Failed to run specify: {error}", file=sys.stderr)
            return 2

    validator_args = [
        args.command,
        "--project",
        args.project,
        "--phase",
        args.phase,
    ]
    if args.feature:
        validator_args.extend(["--feature", args.feature])
    if args.json:
        validator_args.append("--json")
    return _run_validator(validator_args)


if __name__ == "__main__":
    raise SystemExit(main())
