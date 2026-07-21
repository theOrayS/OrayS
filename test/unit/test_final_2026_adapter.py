#!/usr/bin/env python3
"""Mutation tests for the fail-closed final-2026 guest adapter."""

from __future__ import annotations

import hashlib
import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "evaluation"))

from run_final_2026_evaluation import (
    EXPECTED_PROTOCOL_COMMIT,
    OFFICIAL_SOURCE_URL,
    HostResources,
    InfrastructureError,
    _closed_make_environment,
    build_make_command,
    load_image_manifest,
    require_buildstorm_host,
    validate_image_provenance,
    validate_protocol_state,
)


class Final2026AdapterTest(unittest.TestCase):
    def manifest(
        self,
        path: Path,
        *,
        image: Path,
        digest: str,
        protocol_commit: str = EXPECTED_PROTOCOL_COMMIT,
        source_url: str = OFFICIAL_SOURCE_URL,
    ) -> None:
        path.write_text(
            json.dumps(
                {
                    "schema_version": 1,
                    "protocol_reference_commit": protocol_commit,
                    "official_source_url": source_url,
                    "images": [
                        {
                            "architecture": "riscv64",
                            "group": "cagent",
                            "filename": image.name,
                            "sha256": digest,
                            "format": "raw",
                            "guest_script": "/glibc/cagent_testcode.sh",
                        }
                    ],
                }
            )
            + "\n",
            encoding="utf-8",
        )

    def test_manifest_binds_exact_protocol_commit_and_official_source(self) -> None:
        with tempfile.TemporaryDirectory(prefix="final-manifest-") as temporary:
            root = Path(temporary)
            image = root / "official-rv-cagent.img"
            image.write_bytes(b"official fixture")
            digest = hashlib.sha256(image.read_bytes()).hexdigest()
            manifest = root / "manifest.json"
            for mutation in ("commit", "source"):
                with self.subTest(mutation=mutation):
                    self.manifest(
                        manifest,
                        image=image,
                        digest=digest,
                        protocol_commit=("0" * 40 if mutation == "commit" else EXPECTED_PROTOCOL_COMMIT),
                        source_url=("https://example.invalid/image" if mutation == "source" else OFFICIAL_SOURCE_URL),
                    )
                    with self.assertRaises(InfrastructureError):
                        load_image_manifest(manifest)

    def test_image_requires_exact_arch_group_filename_and_sha(self) -> None:
        with tempfile.TemporaryDirectory(prefix="final-image-") as temporary:
            root = Path(temporary)
            image = root / "official-rv-cagent.img"
            image.write_bytes(b"official fixture")
            digest = hashlib.sha256(image.read_bytes()).hexdigest()
            manifest_path = root / "manifest.json"
            self.manifest(manifest_path, image=image, digest=digest)
            manifest = load_image_manifest(manifest_path)
            evidence = validate_image_provenance(
                image,
                digest,
                architecture="riscv64",
                group="cagent",
                manifest=manifest,
            )
            self.assertEqual(evidence["sha256"], digest)
            self.assertEqual(evidence["guest_script"], "/glibc/cagent_testcode.sh")
            for arch, group, expected_sha in (
                ("loongarch64", "cagent", digest),
                ("riscv64", "buildstorm", digest),
                ("riscv64", "cagent", "f" * 64),
            ):
                with self.subTest(arch=arch, group=group, expected_sha=expected_sha):
                    with self.assertRaises(InfrastructureError):
                        validate_image_provenance(
                            image,
                            expected_sha,
                            architecture=arch,
                            group=group,
                            manifest=manifest,
                        )

    def test_known_preliminary_images_are_rejected_even_if_caller_claims_hash(self) -> None:
        old_rv_digest = "4336475432728e485bc52f54f0b8ef06910e84d7c425fbba49361a4065cccb99"
        with tempfile.TemporaryDirectory(prefix="final-old-image-") as temporary:
            root = Path(temporary)
            image = root / "sdcard-rv.img"
            image.write_bytes(b"placeholder")
            manifest_path = root / "manifest.json"
            self.manifest(manifest_path, image=image, digest=old_rv_digest)
            manifest = load_image_manifest(manifest_path)
            with self.assertRaisesRegex(InfrastructureError, "preliminary"):
                validate_image_provenance(
                    image,
                    old_rv_digest,
                    architecture="riscv64",
                    group="cagent",
                    manifest=manifest,
                    observed_sha256=old_rv_digest,
                )

    def test_protocol_checkout_requires_exact_clean_commit(self) -> None:
        evidence = validate_protocol_state(EXPECTED_PROTOCOL_COMMIT, "")
        self.assertEqual(evidence["commit"], EXPECTED_PROTOCOL_COMMIT)
        with self.assertRaises(InfrastructureError):
            validate_protocol_state("0" * 40, "")
        with self.assertRaises(InfrastructureError):
            validate_protocol_state(EXPECTED_PROTOCOL_COMMIT, " M scripts/cagent_testcode.sh\n")

    def test_buildstorm_requires_real_host_capacity(self) -> None:
        qualified = HostResources(
            physical_cores=8,
            online_cpus=8,
            available_memory_bytes=12 * 1024**3,
            output_free_bytes=16 * 1024**3,
        )
        require_buildstorm_host(qualified)
        mutations = (
            HostResources(7, 8, qualified.available_memory_bytes, qualified.output_free_bytes),
            HostResources(8, 8, 8 * 1024**3, qualified.output_free_bytes),
            HostResources(8, 8, qualified.available_memory_bytes, 3 * 1024**3),
        )
        for resources in mutations:
            with self.subTest(resources=resources):
                with self.assertRaises(InfrastructureError):
                    require_buildstorm_host(resources)

    def test_buildstorm_make_command_uses_exact_eight_core_memory_geometry(self) -> None:
        root = Path("/workspace/orays")
        output = Path("/evidence/case")
        image = Path("/images/final.img")
        rv = build_make_command(root, output, image, "riscv64", "buildstorm")
        la = build_make_command(root, output, image, "loongarch64", "buildstorm")
        for command in (rv, la):
            self.assertIn("KERNEL_SMP=8", command)
            self.assertTrue(any(value.endswith("_MEM=8G") for value in command))
            self.assertIn("OSCOMP_TEST_GROUPS=buildstorm", command)
        self.assertIn("KERNEL_RV_AXCONFIG_WRITES=-w plat.phys-memory-size=0x2_0000_0000", rv)
        self.assertIn("KERNEL_LA_AXCONFIG_WRITES=-w plat.phys-memory-size=0x1_f000_0000", la)

    def test_cagent_make_command_stays_small_and_group_scoped(self) -> None:
        command = build_make_command(
            Path("/workspace/orays"),
            Path("/evidence/case"),
            Path("/images/final.img"),
            "riscv64",
            "cagent",
        )
        self.assertIn("KERNEL_SMP=1", command)
        self.assertIn("RV_MEM=1G", command)
        self.assertIn("OSCOMP_TEST_GROUPS=cagent", command)
        self.assertNotIn("LA_MEM=8G", command)

    def test_buildstorm_compile_environment_discovers_script_and_preserves_timeout(self) -> None:
        environment = _closed_make_environment("buildstorm")
        self.assertEqual(environment["OSCOMP_EXTRA_TESTSUITE_DIRS"], "/scripts")
        self.assertGreaterEqual(
            int(environment["OSCOMP_GROUP_TIMEOUT_CEILING_SECS"]),
            14_400,
        )
        cagent = _closed_make_environment("cagent")
        self.assertNotIn("OSCOMP_EXTRA_TESTSUITE_DIRS", cagent)


if __name__ == "__main__":
    unittest.main()
