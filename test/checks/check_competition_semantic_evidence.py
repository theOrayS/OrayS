#!/usr/bin/env python3
"""Static fail-closed guard for PR3 required/observational workflow policy."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path


ACTION_REF_RE = re.compile(r"^\s*-?\s*uses:\s*([^\s]+)\s*$", re.MULTILINE)
PINNED_ACTION_RE = re.compile(r"^[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+@[0-9a-f]{40}$")
JOB_RE = re.compile(r"^  ([a-z0-9][a-z0-9-]*):\s*$", re.MULTILINE)
QEMU_SOURCE_SHA256 = "f3cc1c4eabfdb288218ac3e33763dbe9e276d8bc890b867a2335d58de2ddd39a"
QEMU_REQUIRED_VERSION = "QEMU emulator version 9.2.4"
SUPPORTED_TARGET_ARCHES = ("riscv64", "loongarch64")
UNSUPPORTED_TARGET_ARCHES = ("x86_64", "aarch64")
DOCUMENTED_REQUIRED_CHECKS = (
    "Unit tests (required)",
    "PR3 infrastructure + host evidence (required)",
    "PR3 QEMU 9.2.4 source baseline (required)",
    "PR3 RV64 fixed build + runtime smoke (required)",
    "PR3 LA64 fixed build + runtime smoke (required)",
    "PR3 required aggregate",
    *(f"Clippy ({arch}, fixed-required)" for arch in SUPPORTED_TARGET_ARCHES),
    *(f"Build ({arch}, fixed-required)" for arch in SUPPORTED_TARGET_ARCHES),
    *(f"Application tests ({arch}, fixed-required)" for arch in SUPPORTED_TARGET_ARCHES),
    "Docs (ubuntu-24.04)",
)
DOCUMENTED_OBSERVATIONAL_CHECKS = (
    *(f"Clippy ({arch}, moving-nightly-observational)" for arch in SUPPORTED_TARGET_ARCHES),
    *(f"Build ({arch}, moving-nightly-observational)" for arch in SUPPORTED_TARGET_ARCHES),
)


def job_blocks(text: str) -> dict[str, str]:
    jobs_match = re.search(r"^jobs:\s*$", text, re.MULTILINE)
    if jobs_match is None:
        return {}
    body = text[jobs_match.end() :]
    matches = list(JOB_RE.finditer(body))
    return {
        match.group(1): body[
            match.start() : matches[index + 1].start() if index + 1 < len(matches) else len(body)
        ]
        for index, match in enumerate(matches)
    }


def scan_actions(path: Path, text: str) -> list[str]:
    findings = []
    for reference in ACTION_REF_RE.findall(text):
        if reference.startswith("./"):
            continue
        if not PINNED_ACTION_RE.fullmatch(reference):
            findings.append(f"{path}: mutable or non-SHA action reference: {reference}")
    return findings


def scan_common(path: Path, text: str) -> list[str]:
    findings = scan_actions(path, text)
    if not re.search(r"^permissions:\s*\n\s+contents:\s+read\s*$", text, re.MULTILINE):
        findings.append(f"{path}: missing top-level least-privilege contents: read")
    if "ubuntu-latest" in text or "macos-latest" in text:
        findings.append(f"{path}: moving latest runner label is forbidden")
    if "|| true" in text:
        findings.append(f"{path}: unconditional success token '|| true' is forbidden")
    checkout_count = len(re.findall(r"uses:\s*actions/checkout@", text))
    persist_count = len(re.findall(r"persist-credentials:\s*false", text))
    if checkout_count != persist_count:
        findings.append(
            f"{path}: every checkout must set persist-credentials: false "
            f"({checkout_count} checkout, {persist_count} hardened)"
        )
    for name, block in job_blocks(text).items():
        if "timeout-minutes:" not in block:
            findings.append(f"{path}: job {name} has no timeout-minutes")
    return findings


def require_tokens(path: Path, text: str, tokens: tuple[str, ...]) -> list[str]:
    return [f"{path}: missing required token: {token}" for token in tokens if token not in text]


def scan_test_workflow(path: Path, text: str) -> list[str]:
    findings = scan_common(path, text)
    blocks = job_blocks(text)
    jobs_match = re.search(r"^jobs:\s*$", text, re.MULTILINE)
    workflow_header = text[: jobs_match.start()] if jobs_match is not None else text
    if "runner." in workflow_header:
        findings.append(f"{path}: workflow-level keys cannot use the runner context")
    for job, block in blocks.items():
        lines = block.splitlines()
        for index, line in enumerate(lines):
            if line != "    env:":
                continue
            values: list[str] = []
            for value in lines[index + 1 :]:
                if value.strip() and len(value) - len(value.lstrip()) <= 4:
                    break
                values.append(value)
            if any("runner." in value for value in values):
                findings.append(
                    f"{path}: job-level env in {job} cannot use the runner context"
                )
    required_jobs = {
        "unit-test": "Unit tests (required)",
        "pr3-infrastructure": "PR3 infrastructure + host evidence (required)",
        "pr3-qemu-baseline": "PR3 QEMU 9.2.4 source baseline (required)",
        "pr3-runtime": "PR3 RV64 fixed build + runtime smoke (required)",
        "pr3-required-aggregate": "PR3 required aggregate",
    }
    for job, name_token in required_jobs.items():
        block = blocks.get(job)
        if block is None:
            findings.append(f"{path}: missing required job {job}")
            continue
        if name_token not in block:
            findings.append(f"{path}: required job {job} is missing stable name {name_token}")
        if "continue-on-error:" in block:
            findings.append(f"{path}: required job {job} uses continue-on-error")
    findings.extend(
        require_tokens(
            path,
            text,
            (
                QEMU_SOURCE_SHA256,
                "bash test/evidence/setup_qemu.sh",
                "--profile evidence-host",
                "--profile evidence-runtime",
                "--arch ${{ matrix.runner_arch }}",
                "--profile evidence-aggregate",
                "path: build/pr3-evidence/host",
                "path: build/pr3-evidence/rv64",
                "path: build/pr3-evidence/la64",
                "${{ runner.temp }}/orays-pr3-qemu-9.2.4",
                "$GITHUB_STEP_SUMMARY",
                "pr3-required-semantic-evidence",
                "if-no-files-found: error",
                "Application tests (${{ matrix.arch }}, fixed-required)",
            ),
        )
    )
    app = blocks.get("app-test", "")
    if "continue-on-error:" in app:
        findings.append(f"{path}: fixed application tests must remain hard-fail coverage")
    if (
        "arch: [riscv64, loongarch64]" not in app
        or "arch_list: riscv64,loongarch64" not in app
    ):
        findings.append(
            f"{path}: application tests must gate exactly the supported targets "
            "(riscv64, loongarch64)"
        )
    runtime = blocks.get("pr3-runtime", "")
    if "needs: pr3-qemu-baseline\n    if: ${{ always() }}" not in runtime:
        findings.append(
            f"{path}: pr3-runtime needs job-level always() so an upstream failure cannot "
            "become a skipped-success required check"
        )
    producer_conclusion = 'test "${{ needs.pr3-qemu-baseline.result }}" = success'
    if producer_conclusion not in runtime:
        findings.append(
            f"{path}: pr3-runtime must preserve the QEMU producer conclusion"
        )
    for check_name in (
        "PR3 RV64 fixed build + runtime smoke (required)",
        "PR3 LA64 fixed build + runtime smoke (required)",
    ):
        if check_name not in runtime:
            findings.append(f"{path}: missing architecture check name {check_name}")
    qemu_prefix = "QEMU_PREFIX: ${{ runner.temp }}/orays-pr3-qemu-9.2.4"
    if text.count(qemu_prefix) != 3:
        findings.append(
            f"{path}: pinned QEMU prefix must be defined in exactly three step-level env blocks"
        )
    qemu_prefix_value = "${{ runner.temp }}/orays-pr3-qemu-9.2.4"
    if text.count(qemu_prefix_value) != 3:
        findings.append(f"{path}: required QEMU binary prefix must never be cached")
    baseline = blocks.get("pr3-qemu-baseline", "")
    if "path: build/qemu-source/qemu-9.2.4.tar.xz" not in baseline:
        findings.append(f"{path}: required QEMU cache must contain only the pinned source archive")
    for token in (
        "libfdt-dev",
        "python3 -I -S -B -X pycache_prefix=/dev/null test/evidence/semantic_evidence.py supervise",
        "--timeout 4800",
        "--log build/pr3-qemu-artifact/setup.log",
        "name: Upload verified QEMU\n        if: ${{ always() }}",
    ):
        if token not in baseline:
            findings.append(f"{path}: QEMU baseline lacks bounded/log-preserving contract: {token}")
    if "cache-targets: false" not in runtime:
        findings.append(f"{path}: runtime Rust cache must not restore opaque target outputs")
    for token in (
        "Install QEMU runtime prerequisites",
        "libfdt1",
        "libglib2.0-0t64",
        "libpixman-1-0",
        "zlib1g",
    ):
        if token not in runtime:
            findings.append(
                f"{path}: QEMU consumer runtime dependency contract missing: {token}"
            )
    safe_consumer_extractor = (
        'tar --no-same-owner -C "$QEMU_PREFIX" -xzf '
        "build/pr3-qemu-artifact/qemu-9.2.4.tar.gz"
    )
    active_consumer_extractors = []
    for line in runtime.splitlines():
        stripped = line.strip()
        if (
            not stripped
            or stripped.startswith("#")
            or "build/pr3-qemu-artifact/qemu-9.2.4.tar.gz" not in stripped
        ):
            continue
        if re.search(r"(^|[^A-Za-z0-9_])tar(?:\s|$)", stripped):
            active_consumer_extractors.append(stripped)
    if active_consumer_extractors != [safe_consumer_extractor]:
        findings.append(
            f"{path}: exactly one active QEMU consumer artifact extraction must "
            "use --no-same-owner"
        )
    return findings


def target_block(text: str, target: str) -> str:
    match = re.search(
        rf"^{re.escape(target)}:[^\n]*\n(?P<body>(?:\t.*\n|\n)*)",
        text,
        re.MULTILINE,
    )
    return match.group("body") if match is not None else ""


def timeout_minutes(block: str) -> int | None:
    match = re.search(r"^    timeout-minutes:\s*(\d+)\s*$", block, re.MULTILINE)
    return int(match.group(1)) if match is not None else None


def scan_repository_contracts(root: Path, test_text: str) -> list[str]:
    findings: list[str] = []
    try:
        makefile = (root / "Makefile").read_text(encoding="utf-8")
        setup = (root / "test" / "evidence" / "setup_qemu.sh").read_text(encoding="utf-8")
        evidence_docs = (root / "docs" / "pr3-semantic-evidence.md").read_text(
            encoding="utf-8"
        )
        manifest = json.loads(
            (root / "test" / "evidence" / "semantic_evidence_manifest.json").read_text(
                encoding="utf-8"
            )
        )
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        return [f"PR3 repository contract input cannot be read: {exc}"]

    for check_name in (*DOCUMENTED_REQUIRED_CHECKS, *DOCUMENTED_OBSERVATIONAL_CHECKS):
        if f"`{check_name}`" not in evidence_docs:
            findings.append(
                "docs/pr3-semantic-evidence.md: missing exact CI check classification: "
                f"{check_name}"
            )
    if "Application tests (<arch>, fixed-required)" in evidence_docs:
        findings.append(
            "docs/pr3-semantic-evidence.md: branch-protection names must not use "
            "an <arch> placeholder"
        )

    capabilities = {
        capability.get("id"): capability
        for capability in manifest.get("capabilities", [])
        if isinstance(capability, dict)
    }
    for capability_id, executable in (
        ("qemu-rv64", "qemu-system-riscv64"),
        ("qemu-la64", "qemu-system-loongarch64"),
    ):
        capability = capabilities.get(capability_id)
        expected = {
            "id": capability_id,
            "kind": "tool",
            "value": executable,
            "external": False,
            "required_version": QEMU_REQUIRED_VERSION,
        }
        if capability != expected:
            findings.append(
                "test/evidence/semantic_evidence_manifest.json: "
                f"{capability_id} must bind exact required QEMU version "
                f"{QEMU_REQUIRED_VERSION!r}"
            )

    cases = {
        case.get("id"): case
        for case in manifest.get("cases", [])
        if isinstance(case, dict)
    }
    for case_id, capability_id in (
        ("smoke.rv64.abi", "qemu-rv64"),
        ("smoke.la64.abi", "qemu-la64"),
    ):
        if capability_id not in cases.get(case_id, {}).get("requires", []):
            findings.append(
                "test/evidence/semantic_evidence_manifest.json: "
                f"{case_id} must require {capability_id}"
            )

    for target, binary_variable, nic in (
        ("pr3-smoke-run-rv-raw", "PR3_QEMU_RV_BIN", "virtio-net-device"),
        ("pr3-smoke-run-la-raw", "PR3_QEMU_LA_BIN", "virtio-net-pci"),
    ):
        block = target_block(makefile, target)
        for token in (
            f'"$${binary_variable}" -machine virt',
            f"-device {nic},netdev=net",
            "-netdev hubport,id=net,hubid=0",
        ):
            if token not in block:
                findings.append(f"Makefile: {target} missing exact runtime token: {token}")
        if "-netdev user" in block or "-nic none" in block:
            findings.append(f"Makefile: {target} must use only the internal hub backend")
    for token in ("--disable-download", "--disable-slirp"):
        if setup.count(token) < 2:
            findings.append(
                f"test/evidence/setup_qemu.sh: missing offline configure token {token}"
            )
    safe_archive_extractor = 'tar --no-same-owner -xf "$archive" -C "$work_dir"'
    active_archive_extractors = []
    for line in setup.splitlines():
        stripped = line.strip()
        if not stripped or stripped.startswith("#") or "$archive" not in stripped:
            continue
        if re.search(r"(^|[^A-Za-z0-9_])tar(?:\s|$)", stripped):
            active_archive_extractors.append(stripped)
    if active_archive_extractors != [safe_archive_extractor]:
        findings.append(
            "test/evidence/setup_qemu.sh: exactly one active archive extraction must "
            "use --no-same-owner"
        )

    blocks = job_blocks(test_text)
    direct_cases = manifest.get("cases", [])
    host_budget = sum(
        case["timeout_seconds"]
        for case in direct_cases
        if "host" in case.get("architectures", [])
    ) + sum(
        inventory["timeout_seconds"] * 2 * len(inventory["expected_ids"])
        for inventory in manifest.get("inventories", [])
    )
    architecture_budgets = {
        arch: sum(
            case["timeout_seconds"]
            for case in direct_cases
            if arch in case.get("architectures", [])
        )
        for arch in ("riscv64", "loongarch64")
    }
    required_outer = {
        "pr3-infrastructure": host_budget + 600,
        "pr3-runtime": max(architecture_budgets.values()) + 600,
        "pr3-qemu-baseline": 4800 + 600,
    }
    for job, required_seconds in required_outer.items():
        observed = timeout_minutes(blocks.get(job, ""))
        if observed is None or observed * 60 < required_seconds:
            findings.append(
                f".github/workflows/test.yml: {job} timeout must cover inner budgets "
                f"plus cleanup/upload margin ({required_seconds}s required)"
            )
    required_local = target_block(makefile, "pr3-evidence-required")
    for token in ("test/run_suite.py", "--profile evidence-required"):
        if token not in required_local:
            findings.append(f"Makefile: pr3-evidence-required must delegate to the canonical profile: {token}")
    return findings


def scan_build_workflow(path: Path, text: str) -> list[str]:
    findings = scan_common(path, text)
    findings.extend(
        require_tokens(
            path,
            text,
            (
                "fixed-required",
                "moving-nightly-observational",
                "continue-on-error: ${{ matrix.lane == 'moving-nightly-observational' }}",
            ),
        )
    )
    # Supported scope: required target gates cover RISC-V64 and LoongArch64
    # only. x86_64/aarch64/macOS jobs must not linger as required or
    # observational gates, and no unsupported-target job may remain.
    if text.count("arch: [riscv64, loongarch64]") != 2:
        findings.append(
            f"{path}: clippy and build matrices must both be exactly "
            "arch: [riscv64, loongarch64]"
        )
    for job in ("build-for-other-platforms", "build-for-macos"):
        if f"{job}:" in text:
            findings.append(f"{path}: unsupported-target job must be removed: {job}")
    for token in ("macos-14", "Other platforms (", "macOS ("):
        if token in text:
            findings.append(f"{path}: unsupported platform gate remains: {token}")
    blocks = job_blocks(text)
    for job in ("clippy", "build"):
        block = blocks.get(job, "")
        if "lane: [fixed-required, moving-nightly-observational]" not in block:
            findings.append(f"{path}: {job} must keep fixed and moving lanes explicit")
        if (
            "continue-on-error: ${{ matrix.lane == 'moving-nightly-observational' }}"
            not in block
        ):
            findings.append(f"{path}: {job} must keep only moving nightly observational")
    for line_number, line in enumerate(text.splitlines(), 1):
        if "run: make " in line and "A=user/shell" in line and not line.rstrip().endswith(" build"):
            findings.append(
                f"{path}:{line_number}: shell build must name the explicit build target"
            )
    return findings


def scan_docs_workflow(path: Path, text: str) -> list[str]:
    findings = scan_common(path, text)
    blocks = job_blocks(text)
    if text.count("contents: write") != 1:
        findings.append(f"{path}: contents: write must occur exactly once")
    if "contents: write" not in blocks.get("deploy", ""):
        findings.append(f"{path}: only the deploy job may receive contents: write")
    if "pull_request:" not in text or "github.event_name == 'push'" not in blocks.get("deploy", ""):
        findings.append(f"{path}: docs deployment is not restricted to default-branch push")
    if "macos" in text:
        findings.append(f"{path}: macOS is outside the supported docs scope")
    if "make doc_check_missing ARCH=riscv64" not in text:
        findings.append(
            f"{path}: Linux docs validation must build the primary supported "
            "target with make doc_check_missing ARCH=riscv64"
        )
    return findings


def scan(root: Path) -> list[str]:
    workflow_dir = root / ".github" / "workflows"
    paths = {
        "build": workflow_dir / "build.yml",
        "test": workflow_dir / "test.yml",
        "docs": workflow_dir / "docs.yml",
    }
    findings = []
    texts = {}
    for name, path in paths.items():
        try:
            texts[name] = path.read_text(encoding="utf-8")
        except (OSError, UnicodeError) as exc:
            findings.append(f"{path}: cannot read workflow: {exc}")
    if "build" in texts:
        findings.extend(scan_build_workflow(paths["build"], texts["build"]))
    if "test" in texts:
        findings.extend(scan_test_workflow(paths["test"], texts["test"]))
    if "docs" in texts:
        findings.extend(scan_docs_workflow(paths["docs"], texts["docs"]))
    if "test" in texts:
        findings.extend(scan_repository_contracts(root, texts["test"]))
    return findings


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[2])
    args = parser.parse_args()
    findings = scan(args.root.resolve())
    if findings:
        print("competition semantic-evidence workflow check: FAIL")
        for finding in findings:
            print(f"- {finding}")
        return 1
    print("competition semantic-evidence workflow check: PASS (0 findings)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
