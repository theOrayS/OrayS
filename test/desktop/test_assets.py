from __future__ import annotations

import hashlib
import importlib.util
import json
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "scripts/desktop/check-assets.py"
SPEC = importlib.util.spec_from_file_location("desktop_check_assets", SCRIPT)
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class AssetLicenseTests(unittest.TestCase):
    def manifest(self) -> dict:
        return {
            "schema": 1,
            "project_license": MODULE.PROJECT_LICENSE,
            "generated_assets": ["test primitive"],
            "files": [],
        }

    def create_tree(self, root: Path) -> None:
        for name in MODULE.ASSET_DIRECTORIES:
            (root / name).mkdir(parents=True)

    def test_repository_assets_and_license_inventory_pass(self) -> None:
        asset_root = ROOT / "user/desktop/assets"
        manifest = MODULE.load_manifest(asset_root / "manifest.json")
        self.assertEqual(MODULE.check_asset_tree(asset_root, manifest), [])

    def test_undeclared_asset_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            self.create_tree(root)
            (root / "icons/unknown.ppm").write_bytes(b"P6\n1 1\n255\n\0\0\0")
            failures = MODULE.check_asset_tree(root, self.manifest())
            self.assertTrue(any("undeclared asset file" in item for item in failures))

    def test_registered_asset_requires_hash_and_license(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            self.create_tree(root)
            payload = b"original test asset"
            (root / "icons/known.bin").write_bytes(payload)
            manifest = self.manifest()
            manifest["files"] = [
                {
                    "path": "icons/known.bin",
                    "sha256": hashlib.sha256(payload).hexdigest(),
                    "license_file": "licenses/known.txt",
                }
            ]
            failures = MODULE.check_asset_tree(root, manifest)
            self.assertTrue(any("missing license file" in item for item in failures))
            (root / "licenses/known.txt").write_text("test license\n", encoding="utf-8")
            self.assertEqual(MODULE.check_asset_tree(root, manifest), [])

    def test_license_traversal_is_rejected_even_when_target_exists(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            self.create_tree(root)
            payload = b"asset"
            (root / "icons/known.bin").write_bytes(payload)
            (root / "outside-license.txt").write_text("not a license\n", encoding="utf-8")
            manifest = self.manifest()
            manifest["files"] = [
                {
                    "path": "icons/known.bin",
                    "sha256": hashlib.sha256(payload).hexdigest(),
                    "license_file": "licenses/../outside-license.txt",
                }
            ]
            failures = MODULE.check_asset_tree(root, manifest)
            self.assertTrue(any("invalid license_file" in item for item in failures))

    def test_license_symlink_is_rejected_even_when_target_is_a_file(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            self.create_tree(root)
            payload = b"asset"
            (root / "icons/known.bin").write_bytes(payload)
            (root / "outside-license.txt").write_text("not a license\n", encoding="utf-8")
            (root / "licenses/known.txt").symlink_to("../outside-license.txt")
            manifest = self.manifest()
            manifest["files"] = [
                {
                    "path": "icons/known.bin",
                    "sha256": hashlib.sha256(payload).hexdigest(),
                    "license_file": "licenses/known.txt",
                }
            ]
            failures = MODULE.check_asset_tree(root, manifest)
            self.assertTrue(any("license symlink is not allowed" in item for item in failures))


if __name__ == "__main__":
    unittest.main()
