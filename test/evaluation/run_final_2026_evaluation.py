#!/usr/bin/env python3
"""Run one final-2026 group in an exact, provenance-checked guest image."""

from __future__ import annotations

import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any


EXPECTED_PROTOCOL_COMMIT = "15e0355bbee0373de4048002448cee37dbb7ca1b"
OFFICIAL_SOURCE_URL = "https://pan.baidu.com/s/1zTb2NPCp9HE_l_M-3-oczg?pwd=kyva"
PRELIMINARY_IMAGE_SHA256 = frozenset(
    {
        "4336475432728e485bc52f54f0b8ef06910e84d7c425fbba49361a4065cccb99",
        "1aa79d03cf41e2a80ae4ed43771101c1e67ec8db41c3c20b77792fe6b1b85b50",
    }
)
SUPPORTED_ARCHITECTURES = frozenset({"riscv64", "loongarch64"})
SUPPORTED_GROUPS = frozenset({"cagent", "buildstorm"})
EXPECTED_GUEST_SCRIPTS = {
    "cagent": "/glibc/cagent_testcode.sh",
    "buildstorm": "/scripts/buildstorm_testcode.sh",
}
MIN_BUILDSTORM_PHYSICAL_CORES = 8
MIN_BUILDSTORM_AVAILABLE_MEMORY_BYTES = 9 * 1024**3
MIN_BUILDSTORM_OUTPUT_FREE_BYTES = 4 * 1024**3
BUILDSTORM_GUEST_TIMEOUT_CEILING_SECONDS = 18_000
SHA256_RE = re.compile(r"^[0-9a-f]{64}$")
SAFE_FILENAME_RE = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._+-]*$")


class InfrastructureError(RuntimeError):
    """The requested run cannot produce eligible final-2026 evidence."""


@dataclass(frozen=True)
class HostResources:
    physical_cores: int
    online_cpus: int
    available_memory_bytes: int
    output_free_bytes: int


def _require_exact_keys(value: dict[str, Any], expected: set[str], where: str) -> None:
    actual = set(value)
    if actual != expected:
        raise InfrastructureError(
            f"{where} fields must be exactly {sorted(expected)}; observed {sorted(actual)}"
        )


def load_image_manifest(path: Path) -> dict[str, Any]:
    try:
        raw = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, json.JSONDecodeError) as error:
        raise InfrastructureError(f"cannot read final-2026 image manifest: {error}") from error
    if not isinstance(raw, dict):
        raise InfrastructureError("final-2026 image manifest must be an object")
    _require_exact_keys(
        raw,
        {
            "schema_version",
            "protocol_reference_commit",
            "official_source_url",
            "images",
        },
        "image manifest",
    )
    if raw["schema_version"] != 1:
        raise InfrastructureError("unsupported final-2026 image manifest schema")
    if raw["protocol_reference_commit"] != EXPECTED_PROTOCOL_COMMIT:
        raise InfrastructureError("image manifest protocol commit is not the fixed final-2026 commit")
    if raw["official_source_url"] != OFFICIAL_SOURCE_URL:
        raise InfrastructureError("image manifest source is not the official final-2026 source")
    images = raw["images"]
    if not isinstance(images, list):
        raise InfrastructureError("image manifest images must be a list")
    observed_keys: set[tuple[str, str]] = set()
    for index, entry in enumerate(images):
        where = f"image manifest entry {index}"
        if not isinstance(entry, dict):
            raise InfrastructureError(f"{where} must be an object")
        _require_exact_keys(
            entry,
            {
                "architecture",
                "group",
                "filename",
                "sha256",
                "format",
                "guest_script",
            },
            where,
        )
        architecture = entry["architecture"]
        group = entry["group"]
        if architecture not in SUPPORTED_ARCHITECTURES:
            raise InfrastructureError(f"{where} has unsupported architecture")
        if group not in SUPPORTED_GROUPS:
            raise InfrastructureError(f"{where} has unsupported group")
        key = (architecture, group)
        if key in observed_keys:
            raise InfrastructureError(f"duplicate image manifest entry for {architecture}/{group}")
        observed_keys.add(key)
        if not isinstance(entry["filename"], str) or not SAFE_FILENAME_RE.fullmatch(
            entry["filename"]
        ):
            raise InfrastructureError(f"{where} filename is unsafe")
        if not isinstance(entry["sha256"], str) or not SHA256_RE.fullmatch(entry["sha256"]):
            raise InfrastructureError(f"{where} sha256 must be lowercase hexadecimal")
        if entry["format"] != "raw":
            raise InfrastructureError(f"{where} must declare a raw read-only backing image")
        if entry["guest_script"] != EXPECTED_GUEST_SCRIPTS[group]:
            raise InfrastructureError(f"{where} guest script does not match the fixed protocol")
    return raw


def _sha256(path: Path) -> str:
    digest = hashlib.sha256()
    try:
        with path.open("rb") as stream:
            while chunk := stream.read(1024 * 1024):
                digest.update(chunk)
    except OSError as error:
        raise InfrastructureError(f"cannot hash final-2026 image {path}: {error}") from error
    return digest.hexdigest()


def validate_image_provenance(
    image: Path,
    expected_sha256: str,
    *,
    architecture: str,
    group: str,
    manifest: dict[str, Any],
    observed_sha256: str | None = None,
) -> dict[str, str]:
    if architecture not in SUPPORTED_ARCHITECTURES or group not in SUPPORTED_GROUPS:
        raise InfrastructureError(f"unsupported final-2026 image selection: {architecture}/{group}")
    if not isinstance(expected_sha256, str) or not SHA256_RE.fullmatch(expected_sha256):
        raise InfrastructureError("caller image SHA-256 must be 64 lowercase hexadecimal digits")
    entries = [
        entry
        for entry in manifest["images"]
        if entry["architecture"] == architecture and entry["group"] == group
    ]
    if len(entries) != 1:
        raise InfrastructureError(
            f"no unique committed official image provenance for {architecture}/{group}"
        )
    entry = entries[0]
    if image.name != entry["filename"]:
        raise InfrastructureError(
            f"image basename mismatch: expected {entry['filename']!r}, observed {image.name!r}"
        )
    if expected_sha256 != entry["sha256"]:
        raise InfrastructureError("caller image SHA-256 does not match committed provenance")
    if expected_sha256 in PRELIMINARY_IMAGE_SHA256:
        raise InfrastructureError("known preliminary image cannot be used for final-2026")
    if not image.is_file() or not os.access(image, os.R_OK):
        raise InfrastructureError(f"final-2026 image is missing or unreadable: {image}")
    actual_sha256 = observed_sha256 or _sha256(image)
    if actual_sha256 != expected_sha256:
        raise InfrastructureError(
            f"final-2026 image SHA-256 mismatch: expected {expected_sha256}, observed {actual_sha256}"
        )
    return {
        "architecture": architecture,
        "group": group,
        "filename": entry["filename"],
        "sha256": actual_sha256,
        "format": entry["format"],
        "guest_script": entry["guest_script"],
    }


def validate_protocol_state(commit: str, status: str) -> dict[str, str]:
    if commit != EXPECTED_PROTOCOL_COMMIT:
        raise InfrastructureError(
            f"protocol checkout commit mismatch: expected {EXPECTED_PROTOCOL_COMMIT}, observed {commit}"
        )
    if status:
        raise InfrastructureError("protocol checkout has tracked or untracked changes")
    return {"commit": commit, "status": "clean"}


def _run_text(argv: list[str], *, cwd: Path | None = None) -> str:
    try:
        result = subprocess.run(
            argv,
            cwd=cwd,
            stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            timeout=30,
            check=False,
        )
    except (OSError, subprocess.TimeoutExpired) as error:
        raise InfrastructureError(f"cannot run provenance probe {argv[0]!r}: {error}") from error
    if result.returncode != 0:
        detail = result.stderr.strip() or result.stdout.strip() or f"exit {result.returncode}"
        raise InfrastructureError(f"provenance probe failed: {detail}")
    return result.stdout


def validate_protocol_checkout(root: Path) -> dict[str, str]:
    if not root.is_dir():
        raise InfrastructureError(f"final-2026 protocol checkout is missing: {root}")
    git = shutil.which("git")
    if git is None:
        raise InfrastructureError("required command not found: git")
    commit = _run_text([git, "rev-parse", "HEAD"], cwd=root).strip()
    status = _run_text(
        [git, "status", "--porcelain=v1", "--untracked-files=all"], cwd=root
    )
    evidence = validate_protocol_state(commit, status)
    for relative in ("scripts/cagent_testcode.sh", "scripts/buildstorm_testcode.sh"):
        if not (root / relative).is_file():
            raise InfrastructureError(f"protocol checkout is missing {relative}")
    return evidence


def inspect_host_resources(output_dir: Path) -> HostResources:
    topology: set[tuple[str, str]] = set()
    online_cpus = 0
    for cpu_dir in sorted(Path("/sys/devices/system/cpu").glob("cpu[0-9]*")):
        online_path = cpu_dir / "online"
        try:
            online = online_path.read_text(encoding="ascii").strip() != "0"
        except FileNotFoundError:
            online = True
        except OSError as error:
            raise InfrastructureError(f"cannot inspect CPU topology: {error}") from error
        if not online:
            continue
        try:
            package = (cpu_dir / "topology/physical_package_id").read_text(
                encoding="ascii"
            ).strip()
            core = (cpu_dir / "topology/core_id").read_text(encoding="ascii").strip()
        except OSError as error:
            raise InfrastructureError(f"cannot inspect physical CPU topology: {error}") from error
        online_cpus += 1
        topology.add((package, core))
    if online_cpus <= 0 or not topology:
        raise InfrastructureError("host CPU topology is unavailable")
    available_kib: int | None = None
    try:
        for line in Path("/proc/meminfo").read_text(encoding="ascii").splitlines():
            if line.startswith("MemAvailable:"):
                fields = line.split()
                if len(fields) == 3 and fields[2] == "kB":
                    available_kib = int(fields[1])
                break
    except (OSError, ValueError) as error:
        raise InfrastructureError(f"cannot inspect available host memory: {error}") from error
    if available_kib is None:
        raise InfrastructureError("host MemAvailable is unavailable")
    try:
        free_bytes = shutil.disk_usage(output_dir).free
    except OSError as error:
        raise InfrastructureError(f"cannot inspect output filesystem capacity: {error}") from error
    return HostResources(len(topology), online_cpus, available_kib * 1024, free_bytes)


def require_buildstorm_host(resources: HostResources) -> None:
    if resources.physical_cores < MIN_BUILDSTORM_PHYSICAL_CORES:
        raise InfrastructureError(
            f"BuildStorm requires at least 8 physical cores; observed {resources.physical_cores}"
        )
    if resources.online_cpus < 8:
        raise InfrastructureError(
            f"BuildStorm requires at least 8 online CPUs; observed {resources.online_cpus}"
        )
    if resources.available_memory_bytes < MIN_BUILDSTORM_AVAILABLE_MEMORY_BYTES:
        raise InfrastructureError(
            "BuildStorm requires at least 9 GiB MemAvailable to launch an 8 GiB guest"
        )
    if resources.output_free_bytes < MIN_BUILDSTORM_OUTPUT_FREE_BYTES:
        raise InfrastructureError(
            "BuildStorm requires at least 4 GiB free in the evidence filesystem"
        )


def build_make_command(
    repo: Path,
    output_dir: Path,
    image: Path,
    architecture: str,
    group: str,
) -> list[str]:
    if architecture not in SUPPORTED_ARCHITECTURES or group not in SUPPORTED_GROUPS:
        raise InfrastructureError(f"unsupported final-2026 launch: {architecture}/{group}")
    short_arch = "RV" if architecture == "riscv64" else "LA"
    target = "run-rv" if architecture == "riscv64" else "run-la"
    lower_arch = "riscv64" if architecture == "riscv64" else "loongarch64"
    smp = 8 if group == "buildstorm" else 1
    memory = "8G" if group == "buildstorm" else "1G"
    if group == "buildstorm":
        physical_size = "0x2_0000_0000" if architecture == "riscv64" else "0x1_f000_0000"
    else:
        physical_size = "0x4000_0000" if architecture == "riscv64" else "0x3000_0000"
    build_dir = output_dir / "kernel-build"
    kernel = output_dir / f"kernel-{architecture}"
    overlay = output_dir / f"final-{architecture}-{group}.run.qcow2"
    return [
        "make",
        "-C",
        str(repo),
        target,
        f"ARCH={lower_arch}",
        f"KERNEL_SMP={smp}",
        f"{short_arch}_MEM={memory}",
        f"OSCOMP_TEST_GROUPS={group}",
        f"KERNEL_BUILD_DIR={build_dir}",
        f"KERNEL_TARGET_DIR={build_dir / 'target'}",
        f"KERNEL_{short_arch}_OUT_DIR={build_dir / lower_arch}",
        f"KERNEL_{short_arch}_CONFIG={build_dir / (lower_arch + '.axconfig.toml')}",
        f"KERNEL_{short_arch}_TARGET_DIR={build_dir / 'target' / lower_arch}",
        f"KERNEL_{short_arch}={kernel}",
        f"KERNEL_{short_arch}_AXCONFIG_WRITES=-w plat.phys-memory-size={physical_size}",
        f"{short_arch}_TESTSUITE_IMG={image}",
        f"{short_arch}_TESTSUITE_RUN_IMG={overlay}",
    ]


def _validate_raw_image(image: Path) -> None:
    qemu_img = shutil.which("qemu-img")
    if qemu_img is None:
        raise InfrastructureError("required command not found: qemu-img")
    output = _run_text([qemu_img, "info", "--output=json", str(image)])
    try:
        info = json.loads(output)
    except json.JSONDecodeError as error:
        raise InfrastructureError(f"qemu-img returned invalid JSON: {error}") from error
    if info.get("format") != "raw" or info.get("backing-filename") is not None:
        raise InfrastructureError("official final-2026 backing image must be standalone raw")


def _validate_guest_script(image: Path, guest_script: str) -> None:
    debugfs = shutil.which("debugfs")
    if debugfs is None:
        raise InfrastructureError("required command not found: debugfs")
    output = _run_text([debugfs, "-R", f"stat {guest_script}", str(image)])
    if "Inode:" not in output or "Type: regular" not in output:
        raise InfrastructureError(f"official image lacks regular guest script {guest_script}")


def _closed_make_environment(group: str) -> dict[str, str]:
    environment = {
        "PATH": os.environ.get("PATH", os.defpath),
        "HOME": os.environ.get("HOME", str(Path.home())),
        "LC_ALL": "C",
        "LANG": "C",
        "CARGO_NET_OFFLINE": "true",
        "PYTHONNOUSERSITE": "1",
        "PYTHONDONTWRITEBYTECODE": "1",
        "PYTHONPYCACHEPREFIX": "/dev/null",
        "OSCOMP_TEST_GROUPS": group,
    }
    if group == "buildstorm":
        environment["OSCOMP_EXTRA_TESTSUITE_DIRS"] = str(
            Path(EXPECTED_GUEST_SCRIPTS[group]).parent
        )
        environment["OSCOMP_GROUP_TIMEOUT_CEILING_SECS"] = str(
            BUILDSTORM_GUEST_TIMEOUT_CEILING_SECONDS
        )
    return environment


def _usage() -> str:
    return f"usage: {Path(sys.argv[0]).name} <riscv64|loongarch64> <cagent|buildstorm>"


def main(argv: list[str] | None = None) -> int:
    arguments = list(sys.argv[1:] if argv is None else argv)
    if len(arguments) != 2:
        print(f"infrastructure error: {_usage()}", file=sys.stderr)
        return 125
    architecture, group = arguments
    if architecture not in SUPPORTED_ARCHITECTURES or group not in SUPPORTED_GROUPS:
        print(f"infrastructure error: {_usage()}", file=sys.stderr)
        return 125

    repo = Path(__file__).resolve().parents[2]
    output_dir = Path(
        os.environ.get(
            "ORAYS_TEST_OUTPUT_DIR",
            str(repo / "test/output/final-2026" / f"{architecture}-{group}"),
        )
    ).expanduser()
    if not output_dir.is_absolute():
        output_dir = (Path.cwd() / output_dir).resolve()
    image_prefix = "RV" if architecture == "riscv64" else "LA"
    group_prefix = "CAGENT" if group == "cagent" else "BUILDSTORM"
    image_name = f"{image_prefix}_{group_prefix}_FINAL_2026_IMG"
    sha_name = f"{image_name}_SHA256"
    try:
        output_dir.mkdir(parents=True, exist_ok=True)
        protocol_raw = os.environ.get("FINAL_2026_PROTOCOL_ROOT", "")
        image_raw = os.environ.get(image_name, "")
        expected_sha256 = os.environ.get(sha_name, "")
        if not protocol_raw:
            raise InfrastructureError("FINAL_2026_PROTOCOL_ROOT is required")
        if not image_raw:
            raise InfrastructureError(f"{image_name} is required")
        protocol_root = Path(protocol_raw).expanduser().resolve()
        image = Path(image_raw).expanduser().resolve()
        validate_protocol_checkout(protocol_root)
        manifest = load_image_manifest(repo / "test/images/manifest.final-2026.json")
        provenance = validate_image_provenance(
            image,
            expected_sha256,
            architecture=architecture,
            group=group,
            manifest=manifest,
        )
        _validate_raw_image(image)
        _validate_guest_script(image, provenance["guest_script"])
        resources = inspect_host_resources(output_dir)
        if group == "buildstorm":
            require_buildstorm_host(resources)
        required_qemu = (
            "qemu-system-riscv64" if architecture == "riscv64" else "qemu-system-loongarch64"
        )
        for command in ("make", "cargo", required_qemu):
            if shutil.which(command) is None:
                raise InfrastructureError(f"required command not found: {command}")
        print(
            "FINAL_2026_PROVENANCE "
            f"commit={EXPECTED_PROTOCOL_COMMIT} arch={architecture} group={group} "
            f"image={provenance['filename']} sha256={provenance['sha256']}"
        )
        command = build_make_command(repo, output_dir, image, architecture, group)
        overlay = output_dir / f"final-{architecture}-{group}.run.qcow2"
        try:
            try:
                result = subprocess.run(
                    command,
                    cwd=repo,
                    env=_closed_make_environment(group),
                    stdin=subprocess.DEVNULL,
                    check=False,
                )
            except OSError as error:
                raise InfrastructureError(f"cannot launch final-2026 guest: {error}") from error
        finally:
            try:
                overlay.unlink()
            except FileNotFoundError:
                pass
        final_sha256 = _sha256(image)
        if final_sha256 != provenance["sha256"]:
            raise InfrastructureError(
                "official backing image changed during evaluation; evidence is invalid"
            )
        return result.returncode
    except InfrastructureError as error:
        print(f"infrastructure error: {error}", file=sys.stderr)
        return 125


if __name__ == "__main__":
    raise SystemExit(main())
