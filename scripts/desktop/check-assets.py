#!/usr/bin/env python3
"""Fail closed on undeclared desktop assets or missing asset licenses."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import stat
from pathlib import Path


PROJECT_LICENSE = "GPL-3.0-or-later OR Apache-2.0 OR MulanPSL-2.0"
ASSET_DIRECTORIES = ("icons", "wallpapers", "fonts", "licenses")
CONTENT_DIRECTORIES = ("icons", "wallpapers", "fonts")


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def load_manifest(path: Path) -> dict:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise ValueError("asset manifest must be a JSON object")
    return value


def validate_relative_path(relative: str, required_root: str) -> Path | None:
    path = Path(relative)
    if (
        path.is_absolute()
        or len(path.parts) < 2
        or path.parts[0] != required_root
        or any(part in {"", ".", ".."} for part in path.parts)
    ):
        return None
    return path


def real_regular_file(asset_root: Path, relative: Path) -> bool:
    try:
        resolved_root = asset_root.resolve(strict=True)
        current = resolved_root
        for index, component in enumerate(relative.parts):
            current = current / component
            metadata = current.lstat()
            if stat.S_ISLNK(metadata.st_mode):
                return False
            if index < len(relative.parts) - 1 and not stat.S_ISDIR(metadata.st_mode):
                return False
        resolved = current.resolve(strict=True)
        resolved.relative_to(resolved_root / relative.parts[0])
        return stat.S_ISREG(current.lstat().st_mode)
    except (OSError, ValueError):
        return False


def check_asset_tree(asset_root: Path, manifest: dict) -> list[str]:
    failures: list[str] = []
    if manifest.get("schema") != 1:
        failures.append("manifest schema must be 1")
    if manifest.get("project_license") != PROJECT_LICENSE:
        failures.append("manifest project_license does not match the desktop crate")
    generated = manifest.get("generated_assets")
    if not isinstance(generated, list) or not generated or not all(
        isinstance(item, str) and item.strip() for item in generated
    ):
        failures.append("generated_assets must be a non-empty string list")
    entries = manifest.get("files")
    if not isinstance(entries, list):
        failures.append("manifest files must be a list")
        entries = []

    for directory in ASSET_DIRECTORIES:
        path = asset_root / directory
        if path.is_symlink():
            failures.append(f"asset directory symlink is not allowed: {directory}")
        elif not path.is_dir():
            failures.append(f"missing asset directory: {directory}")

    declared: dict[str, dict] = {}
    for index, entry in enumerate(entries):
        if not isinstance(entry, dict):
            failures.append(f"files[{index}] is not an object")
            continue
        relative = entry.get("path")
        digest = entry.get("sha256")
        license_file = entry.get("license_file")
        if not isinstance(relative, str) or not relative:
            failures.append(f"files[{index}] has invalid path")
            continue
        path = Path(relative)
        if (
            path.is_absolute()
            or ".." in path.parts
            or not path.parts
            or path.parts[0] not in CONTENT_DIRECTORIES
        ):
            failures.append(f"files[{index}] escapes content directories: {relative}")
            continue
        if relative in declared:
            failures.append(f"duplicate asset declaration: {relative}")
            continue
        declared[relative] = entry
        if not isinstance(digest, str) or len(digest) != 64 or any(
            character not in "0123456789abcdef" for character in digest
        ):
            failures.append(f"invalid SHA-256 for {relative}")
        license_path = (
            validate_relative_path(license_file, "licenses")
            if isinstance(license_file, str)
            else None
        )
        if license_path is None:
            failures.append(f"invalid license_file for {relative}")
        elif not real_regular_file(asset_root, license_path):
            failures.append(f"missing license file for {relative}: {license_file}")

    actual: set[str] = set()
    for directory in CONTENT_DIRECTORIES:
        root = asset_root / directory
        if not root.is_dir():
            continue
        for path in root.rglob("*"):
            if path.is_symlink():
                failures.append(f"asset symlink is not allowed: {path.relative_to(asset_root)}")
                continue
            if path.is_file() and path.name != "README.md":
                actual.add(path.relative_to(asset_root).as_posix())

    license_root = asset_root / "licenses"
    if license_root.is_dir():
        for path in license_root.rglob("*"):
            if path.is_symlink():
                failures.append(
                    f"license symlink is not allowed: {path.relative_to(asset_root)}"
                )

    for relative in sorted(actual - declared.keys()):
        failures.append(f"undeclared asset file: {relative}")
    for relative in sorted(declared.keys() - actual):
        failures.append(f"declared asset file is missing: {relative}")
    for relative in sorted(actual & declared.keys()):
        expected = declared[relative].get("sha256")
        relative_path = Path(relative)
        if not real_regular_file(asset_root, relative_path):
            failures.append(f"asset path is not a real file: {relative}")
        elif isinstance(expected, str) and sha256(asset_root / relative) != expected:
            failures.append(f"asset SHA-256 mismatch: {relative}")
    return failures


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--asset-root", type=Path)
    parser.add_argument("--manifest", type=Path)
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parents[2]
    asset_root = (args.asset_root or repo_root / "user/desktop/assets").resolve()
    manifest_path = (args.manifest or asset_root / "manifest.json").resolve()
    try:
        manifest = load_manifest(manifest_path)
        failures = check_asset_tree(asset_root, manifest)
        cargo_manifest = repo_root / "user/desktop/Cargo.toml"
        cargo_text = cargo_manifest.read_text(encoding="utf-8")
        license_match = re.search(r'^license\s*=\s*"([^"]+)"\s*$', cargo_text, re.MULTILINE)
        if license_match is None or license_match.group(1) != PROJECT_LICENSE:
            failures.append("desktop Cargo.toml license does not match the asset manifest")
        reference = repo_root / "docs/references/desktop-dependencies.md"
        if not reference.is_file() or "## 资产登记" not in reference.read_text(encoding="utf-8"):
            failures.append("desktop dependency reference has no asset inventory section")
    except (OSError, ValueError, json.JSONDecodeError) as error:
        failures = [str(error)]
        manifest = {}
    if failures:
        for failure in failures:
            print(f"FAIL {failure}")
        print("ASSET_LICENSE_CHECK=FAIL")
        return 1
    print(
        "ASSET_LICENSE_CHECK=PASS "
        f"registered_files={len(manifest['files'])} generated={len(manifest['generated_assets'])}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
