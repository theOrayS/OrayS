#!/usr/bin/env python3
"""Check canonical test registration, naming, compatibility, and output hygiene."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any

REPO_ROOT = Path(__file__).resolve().parents[2]
HISTORICAL_ID_RE = re.compile(r"(?i)(?:^|[^a-z0-9])g0\d{2}(?:$|[^a-z0-9])")
CANONICAL_REQUIRED = (
    Path("test/README.md"),
    Path("test/run_suite.py"),
    Path("test/run_unittest_suite.py"),
    Path("test/run_official_suite.sh"),
    Path("test/suite_manifest.json"),
    Path("test/evaluation/run_official_evaluation.sh"),
    Path("test/evaluation/official_case_plan.json"),
    Path("test/evaluation/parse_official_results.py"),
    Path("test/evaluation/summarize_ltp_results.py"),
    Path("test/evaluation/report_evaluation_failures.py"),
    Path("test/evidence/aggregate_semantic_evidence.sh"),
    Path("test/evidence/evaluator_protocol.py"),
    Path("test/evidence/render_semantic_evidence.py"),
    Path("test/evidence/semantic_evidence.py"),
    Path("test/evidence/semantic_evidence_manifest.json"),
    Path("test/evidence/semantic_evidence_schema.v1.json"),
    Path("test/evidence/setup_qemu.sh"),
    Path("test/fixtures/semantic_evidence/guard-ambiguous.txt"),
    Path("test/fixtures/semantic_evidence/guard-pass.txt"),
    Path("test/fixtures/semantic_evidence/smoke-rv64-duplicate.txt"),
    Path("test/fixtures/semantic_evidence/smoke-rv64-panic-after-pass.txt"),
    Path("test/fixtures/semantic_evidence/smoke-rv64-pass.txt"),
    Path("test/fixtures/semantic_evidence/smoke-rv64-truncated.txt"),
    Path("test/evaluation/config/loongarch64_submission.toml"),
    Path("test/checks/source_scan.py"),
    Path("test/fixtures/runtime_binary_patch_prohibition/musl_patch_retirement_contract.md"),
    Path("test/fixtures/runtime_binary_patch_prohibition/stable_promotion_reproof_contract.md"),
    Path("test/docs/migration_map.md"),
    Path("test/docs/baseline_validation.md"),
)
CANONICAL_CHECK_PATHS = {
    Path("test/checks/check_compliance_regressions.py"),
    Path("test/checks/check_competition_semantic_evidence.py"),
    Path("test/checks/check_evaluation_runner_and_parser_integrity.py"),
    Path("test/checks/check_file_object_event_core.py"),
    Path("test/checks/check_kernel_state_backed_semantics.py"),
    Path("test/checks/check_libc_stateful_semantics.py"),
    Path("test/checks/check_linux_boundary.py"),
    Path("test/checks/check_memory_policy_semantics.py"),
    Path("test/checks/check_no_fake_success.py"),
    Path("test/checks/check_posix_state_integrity.py"),
    Path("test/checks/check_rlimit_and_fd_semantics.py"),
    Path("test/checks/check_runtime_binary_patch_prohibition.py"),
    Path("test/checks/check_socket_message_and_buffer_semantics.py"),
    Path("test/checks/check_stat_metadata_semantics.py"),
    Path("test/checks/check_synthetic_capability_integrity.py"),
    Path("test/checks/check_syscall_boundary_regressions.py"),
    Path("test/checks/check_test_asset_integrity.py"),
    Path("test/checks/check_timer_semantics.py"),
    Path("test/checks/check_user_memory_copy_boundaries.py"),
}
CANONICAL_UNIT_PATHS = {
    Path("test/unit/test_compliance_regressions.py"),
    Path("test/unit/test_competition_semantic_evidence.py"),
    Path("test/unit/test_evaluation_failure_report.py"),
    Path("test/unit/test_evaluation_runner_and_parser_integrity.py"),
    Path("test/unit/test_evaluator_protocol.py"),
    Path("test/unit/test_file_object_event_core.py"),
    Path("test/unit/test_kernel_state_backed_semantics.py"),
    Path("test/unit/test_libc_stateful_semantics.py"),
    Path("test/unit/test_linux_boundary.py"),
    Path("test/unit/test_ltp_result_summary.py"),
    Path("test/unit/test_memory_policy_semantics.py"),
    Path("test/unit/test_no_fake_success.py"),
    Path("test/unit/test_official_result_validation.py"),
    Path("test/unit/test_posix_state_integrity.py"),
    Path("test/unit/test_qemu_setup.py"),
    Path("test/unit/test_rlimit_and_fd_semantics.py"),
    Path("test/unit/test_runtime_binary_patch_prohibition.py"),
    Path("test/unit/test_socket_message_and_buffer_semantics.py"),
    Path("test/unit/test_stat_metadata_semantics.py"),
    Path("test/unit/test_semantic_evidence.py"),
    Path("test/unit/test_suite_runner.py"),
    Path("test/unit/test_synthetic_capability_integrity.py"),
    Path("test/unit/test_syscall_boundary_regressions.py"),
    Path("test/unit/test_test_asset_integrity.py"),
    Path("test/unit/test_timer_semantics.py"),
    Path("test/unit/test_user_memory_copy_boundaries.py"),
}
RETIRED_SCRIPT_ASSETS = (
    Path("configs/remote-eval/axplat-loongarch64-qemu-virt.toml"),
    Path("scripts/check_selfcheck_compliance_fixes.py"),
    Path("scripts/test_selfcheck_compliance_fixes.py"),
    Path("scripts/check_pr1_linux_boundary.py"),
    Path("scripts/test_pr1_linux_boundary.py"),
    Path("scripts/check_pr2_file_event_core.py"),
    Path("scripts/test_pr2_file_event_core.py"),
    Path("scripts/ltp_summary.py"),
    Path("scripts/test_ltp_summary.py"),
    Path("scripts/eval_failure_report.py"),
    Path("scripts") / ("check_" + "g" + "014_pr3_workflow.py"),
    Path("scripts/evaluator_protocol.py"),
    Path("scripts/render_semantic_evidence.py"),
    Path("scripts/semantic_evidence.py"),
    Path("scripts/semantic_evidence_manifest.json"),
    Path("scripts/semantic_evidence_schema.v1.json"),
    Path("scripts/setup_qemu_pr3.sh"),
    Path("scripts/test_eval_failure_report.py"),
    Path("scripts/test_evaluator_protocol.py"),
    Path("scripts") / ("test_" + "g" + "014_pr3_workflow.py"),
    Path("scripts/test_run_eval.py"),
    Path("scripts/test_semantic_evidence.py"),
    Path("scripts/test_setup_qemu_pr3.py"),
    Path("scripts/fixtures") / ("g" + "008-musl-patch-stable"),
)


def load_manifest(path: Path) -> tuple[dict[str, Any] | None, list[str]]:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, json.JSONDecodeError) as error:
        return None, [f"cannot read canonical manifest: {error}"]
    if not isinstance(data, dict):
        return None, ["canonical manifest must be a JSON object"]
    return data, []


def registered_python_paths(manifest: dict[str, Any], profile: str, root: Path) -> tuple[list[Path], list[str]]:
    findings: list[str] = []
    profiles = manifest.get("profiles")
    cases = manifest.get("cases")
    if not isinstance(profiles, dict) or profile not in profiles or not isinstance(cases, list):
        return [], [f"manifest does not define a usable {profile} profile"]
    profile_data = profiles[profile]
    if not isinstance(profile_data, dict) or not isinstance(profile_data.get("cases"), list):
        return [], [f"manifest {profile} profile has no explicit case list"]
    by_id = {
        case.get("id"): case
        for case in cases
        if isinstance(case, dict) and isinstance(case.get("id"), str)
    }
    paths: list[Path] = []
    for case_id in profile_data["cases"]:
        case = by_id.get(case_id)
        if not isinstance(case, dict):
            findings.append(f"{profile} profile references unknown case {case_id!r}")
            continue
        expected_contract = {"checks": "check", "unit": "unittest"}[profile]
        contract = case.get("result_contract")
        if not isinstance(contract, dict) or contract.get("type") != expected_contract:
            findings.append(
                f"case {case_id} in {profile} profile must use {expected_contract} result contract"
            )
        command = case.get("command")
        if not isinstance(command, list):
            findings.append(f"case {case_id} has no argv command")
            continue
        candidates = [
            value
            for value in command
            if isinstance(value, str)
            and value.endswith(".py")
            and value != "{repo}/test/run_unittest_suite.py"
        ]
        if len(candidates) != 1:
            findings.append(f"case {case_id} must name exactly one Python implementation path")
            continue
        if profile == "unit" and command[:-1] != [
            "{python}",
            "-I",
            "-S",
            "-B",
            "-X",
            "pycache_prefix=/dev/null",
            "{repo}/test/run_unittest_suite.py",
        ]:
            findings.append(
                f"case {case_id} must use the isolated identity-binding unittest harness"
            )
        if profile == "checks" and command[:-1] != [
            "{python}",
            "-I",
            "-S",
            "-B",
            "-X",
            "pycache_prefix=/dev/null",
        ]:
            findings.append(f"case {case_id} must use isolated safe-path Python execution")
        raw = candidates[0].replace("{repo}", str(root))
        path = Path(raw)
        if not path.is_absolute():
            path = root / path
        try:
            paths.append(path.resolve().relative_to(root.resolve()))
        except ValueError:
            findings.append(f"case {case_id} implementation escapes repository: {raw}")
    return paths, findings


def scan_registration(root: Path, manifest: dict[str, Any]) -> list[str]:
    findings: list[str] = []
    actual_checks = {
        path.relative_to(root)
        for path in (root / "test/checks").glob("check_*.py")
        if path.is_file()
    }
    actual_units = {
        path.relative_to(root)
        for path in (root / "test/unit").glob("test_*.py")
        if path.is_file()
    }
    for label, expected, actual in (
        ("check", CANONICAL_CHECK_PATHS, actual_checks),
        ("unit", CANONICAL_UNIT_PATHS, actual_units),
    ):
        missing_expected = sorted(expected - actual)
        unexpected = sorted(actual - expected)
        if missing_expected:
            findings.append(
                f"missing canonical {label} implementations: "
                + ", ".join(map(str, missing_expected))
            )
        if unexpected:
            findings.append(
                f"unexpected canonical {label} implementations absent from fixed inventory: "
                + ", ".join(map(str, unexpected))
            )
    registered_checks, check_findings = registered_python_paths(manifest, "checks", root)
    registered_units, unit_findings = registered_python_paths(manifest, "unit", root)
    findings.extend(check_findings)
    findings.extend(unit_findings)
    for label, actual, registered_list in (
        ("check", actual_checks, registered_checks),
        ("unit", actual_units, registered_units),
    ):
        registered = set(registered_list)
        missing = sorted(actual - registered)
        stale = sorted(registered - actual)
        if missing:
            findings.append(f"unregistered {label} implementations: {', '.join(map(str, missing))}")
        if stale:
            findings.append(f"manifest registers missing {label} implementations: {', '.join(map(str, stale))}")
    all_paths = list(registered_checks) + list(registered_units)
    duplicates = sorted({path for path in all_paths if all_paths.count(path) > 1})
    if duplicates:
        findings.append("implementation path registered more than once: " + ", ".join(map(str, duplicates)))
    return findings


def scan_result_contracts(manifest: dict[str, Any]) -> list[str]:
    profiles = manifest.get("profiles")
    cases = manifest.get("cases")
    if not isinstance(profiles, dict) or not isinstance(cases, list):
        return ["manifest does not define usable profiles and cases for result contracts"]
    required_profiles = {"checks", "unit", "quick", "baseline", "official", "full"}
    missing_profiles = sorted(required_profiles - set(profiles))
    findings: list[str] = []
    if missing_profiles:
        findings.append(f"canonical manifest is missing required profiles: {missing_profiles}")
    by_id = {
        case.get("id"): case
        for case in cases
        if isinstance(case, dict) and isinstance(case.get("id"), str)
    }
    required_by_prefix = {
        "check.": "check",
        "unit.": "unittest",
        "official.": "official",
    }
    for case_id, case in by_id.items():
        contract = case.get("result_contract")
        result_type = contract.get("type") if isinstance(contract, dict) else None
        for prefix, expected in required_by_prefix.items():
            if case_id.startswith(prefix) and result_type != expected:
                findings.append(f"case {case_id} must use {expected} result contract")

    exact_case_contracts = {"baseline.workspace_unit_tests": "cargo_test"}
    for case_id, expected in exact_case_contracts.items():
        case = by_id.get(case_id)
        contract = case.get("result_contract") if isinstance(case, dict) else None
        result_type = contract.get("type") if isinstance(contract, dict) else None
        if result_type != expected:
            findings.append(f"case {case_id} must use {expected} result contract")

    expected_by_profile = {"checks": "check", "unit": "unittest", "official": "official"}
    for profile_name, expected in expected_by_profile.items():
        profile = profiles.get(profile_name)
        if not isinstance(profile, dict):
            findings.append(f"manifest does not define a usable {profile_name} profile")
            continue
        selected = profile.get("cases", [])
        arch_cases = profile.get("arch_cases", {})
        if not isinstance(selected, list) or not isinstance(arch_cases, dict):
            findings.append(f"manifest {profile_name} profile has malformed case registration")
            continue
        selected_ids = [*selected]
        for values in arch_cases.values():
            if isinstance(values, list):
                selected_ids.extend(values)
            else:
                findings.append(f"manifest {profile_name} profile has malformed architecture cases")
        for case_id in selected_ids:
            case = by_id.get(case_id)
            contract = case.get("result_contract") if isinstance(case, dict) else None
            result_type = contract.get("type") if isinstance(contract, dict) else None
            if result_type != expected:
                findings.append(
                    f"case {case_id} in {profile_name} profile must use {expected} result contract"
                )
            if (
                profile_name == "unit"
                and isinstance(contract, dict)
                and contract.get("identity_binding") is not True
            ):
                findings.append(
                    f"case {case_id} in unit profile must require exact runtime identity binding"
                )
    return findings


def scan_legacy_paths(root: Path) -> list[str]:
    findings: list[str] = []
    scripts = root / "scripts"
    legacy = sorted([*scripts.glob("check_g*.py"), *scripts.glob("test_g*.py")])
    if legacy:
        findings.append("legacy sequence-named test implementations remain: " + ", ".join(str(path.relative_to(root)) for path in legacy))
    retired: list[Path] = []
    for path in RETIRED_SCRIPT_ASSETS:
        candidate = root / path
        if candidate.is_file() or (
            candidate.is_dir()
            and any(descendant.is_file() for descendant in candidate.rglob("*"))
        ):
            retired.append(path)
    if retired:
        findings.append(
            "retired test/evaluation assets remain outside test/: "
            + ", ".join(map(str, retired))
        )
    return findings


def scan_canonical_names_and_text(root: Path) -> list[str]:
    findings: list[str] = []
    test_root = root / "test"
    if not test_root.is_dir():
        return ["canonical test directory is missing"]
    for path in sorted(test_root.rglob("*")):
        if "output" in path.relative_to(test_root).parts or "__pycache__" in path.parts:
            continue
        rel = path.relative_to(root)
        if HISTORICAL_ID_RE.search(path.name):
            findings.append(f"historical sequence ID in canonical path: {rel}")
        if path.is_file():
            try:
                text = path.read_text(encoding="utf-8")
            except UnicodeDecodeError as error:
                findings.append(
                    f"canonical test asset is not valid UTF-8: {rel}: {error}"
                )
                continue
            if path != test_root / "docs/migration_map.md" and HISTORICAL_ID_RE.search(text):
                findings.append(f"historical sequence ID in canonical content: {rel}")
    return findings


def scan_required_files(root: Path) -> list[str]:
    findings = [f"required canonical test asset is missing: {path}" for path in CANONICAL_REQUIRED if not (root / path).is_file()]
    output_ignore = root / "test/output/.gitignore"
    if not output_ignore.is_file():
        findings.append("test/output/.gitignore is missing")
    else:
        lines = {line.strip() for line in output_ignore.read_text(encoding="utf-8").splitlines() if line.strip()}
        if "*" not in lines or "!.gitignore" not in lines:
            findings.append("test/output/.gitignore must ignore all generated output except itself")
    wrapper = root / "run-eval.sh"
    if not wrapper.is_file():
        findings.append("root official compatibility wrapper is missing")
    else:
        if wrapper.stat().st_mode & 0o111 == 0:
            findings.append("root official compatibility wrapper is not executable")
        text = wrapper.read_text(encoding="utf-8")
        expected_wrapper = (
            "#!/bin/bash -p\n"
            "set -euo pipefail\n\n"
            'SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"\n'
            'exec "$SCRIPT_DIR/test/run_official_suite.sh" "$@"\n'
        )
        if text != expected_wrapper:
            findings.append(
                "root official compatibility wrapper does not only exec the canonical /test launcher"
            )
        if any(
            token in text
            for token in (
                "python",
                "run_suite.py",
                "--profile",
                "--arch",
                "run_official_evaluation.sh",
                "make",
                "qemu-system",
                "qemu-img",
            )
        ):
            findings.append("root official compatibility wrapper contains duplicated evaluation logic")
    canonical_launcher = root / "test/run_official_suite.sh"
    if canonical_launcher.is_file():
        if canonical_launcher.stat().st_mode & 0o111 == 0:
            findings.append("canonical official suite launcher is not executable")
        text = canonical_launcher.read_text(encoding="utf-8")
        expected_launcher = (
            "#!/bin/bash -p\n"
            "set -euo pipefail\n\n"
            'SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"\n'
            "\n"
            "usage() {\n"
            "    printf 'Usage: %s [rv|la] [--output-dir PATH] [--fail-fast]\\n' \"$0\" >&2\n"
            "}\n\n"
            'architecture="${1:-rv}"\n'
            "if (( $# > 0 )); then\n"
            "    shift\n"
            "fi\n"
            'case "$architecture" in\n'
            "    rv|la) ;;\n"
            "    *)\n"
            "        printf 'infrastructure error: invalid official architecture: %s\\n' \"$architecture\" >&2\n"
            "        usage\n"
            "        exit 2\n"
            "        ;;\n"
            "esac\n\n"
            "runner_args=()\n"
            "output_dir_seen=0\n"
            "fail_fast_seen=0\n"
            "while (( $# > 0 )); do\n"
            '    case "$1" in\n'
            "        --output-dir)\n"
            "            if (( output_dir_seen || $# < 2 )); then\n"
            "                printf 'infrastructure error: --output-dir requires exactly one value\\n' >&2\n"
            "                usage\n"
            "                exit 2\n"
            "            fi\n"
            "            output_dir_seen=1\n"
            '            runner_args+=("$1" "$2")\n'
            "            shift 2\n"
            "            ;;\n"
            "        --fail-fast)\n"
            "            if (( fail_fast_seen )); then\n"
            "                printf 'infrastructure error: duplicate --fail-fast\\n' >&2\n"
            "                usage\n"
            "                exit 2\n"
            "            fi\n"
            "            fail_fast_seen=1\n"
            '            runner_args+=("$1")\n'
            "            shift\n"
            "            ;;\n"
            "        *)\n"
            "            printf 'infrastructure error: unsupported official entry argument: %s\\n' \"$1\" >&2\n"
            "            usage\n"
            "            exit 2\n"
            "            ;;\n"
            "    esac\n"
            "done\n\n"
            'exec python3 -I -S -B -X pycache_prefix=/dev/null "$SCRIPT_DIR/run_suite.py" \\\n'
            '    --profile official --arch "$architecture" "${runner_args[@]}"\n'
        )
        if text != expected_launcher:
            findings.append(
                "canonical official suite launcher does not invoke the strict official profile"
            )
        if re.search(r"\b(?:make|qemu-system|qemu-img)\b", text):
            findings.append("canonical official suite launcher contains duplicated evaluation logic")
    evaluation_runner = root / "test/evaluation/run_official_evaluation.sh"
    if evaluation_runner.is_file() and evaluation_runner.stat().st_mode & 0o111 == 0:
        findings.append("canonical official evaluation runner is not executable")
    local_runner = root / "test/run_suite.py"
    if local_runner.is_file() and local_runner.stat().st_mode & 0o111 == 0:
        findings.append("canonical local suite runner is not executable")
    unittest_runner = root / "test/run_unittest_suite.py"
    if unittest_runner.is_file() and unittest_runner.stat().st_mode & 0o111 == 0:
        findings.append("canonical identity-binding unittest runner is not executable")
    return findings


def scan_repo(root: Path = REPO_ROOT) -> list[str]:
    root = root.resolve()
    manifest, findings = load_manifest(root / "test/suite_manifest.json")
    if manifest is not None:
        findings.extend(scan_registration(root, manifest))
        findings.extend(scan_result_contracts(manifest))
    findings.extend(scan_legacy_paths(root))
    findings.extend(scan_canonical_names_and_text(root))
    findings.extend(scan_required_files(root))
    return sorted(set(findings))


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=REPO_ROOT, help="repository root to audit")
    parser.add_argument("--json", action="store_true", help="emit machine-readable findings")
    args = parser.parse_args(argv)
    findings = scan_repo(args.root)
    if args.json:
        print(json.dumps({"finding_count": len(findings), "findings": findings}, indent=2))
    elif findings:
        print(f"test asset integrity check: FAIL ({len(findings)} findings)")
        for finding in findings:
            print(f"- {finding}")
    else:
        print("test asset integrity check: PASS (0 findings)")
    return 1 if findings else 0


if __name__ == "__main__":
    raise SystemExit(main())
