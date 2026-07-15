#!/usr/bin/env python3
"""Render deterministic JUnit, offline HTML, and Markdown from canonical JSON."""

from __future__ import annotations

import argparse
import html
import os
import sys
import tempfile
import xml.etree.ElementTree as ET
from pathlib import Path
from typing import Any, Sequence
from urllib.parse import quote

from semantic_evidence import (
    EvidenceError,
    ensure_safe_output_directory,
    load_and_validate_result,
    load_validate_result_with_manifest,
)


JUNIT_NAME = "semantic-evidence-v1.junit.xml"
HTML_NAME = "semantic-evidence-v1.html"
MATRIX_NAME = "semantic-matrix-v1.md"


def _atomic_write(path: Path, data: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, temporary = tempfile.mkstemp(prefix=f".{path.name}.", dir=path.parent)
    try:
        with os.fdopen(fd, "wb") as stream:
            stream.write(data)
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(temporary, path)
    except BaseException:
        try:
            os.unlink(temporary)
        except FileNotFoundError:
            pass
        raise


def _junit_disposition(case: dict[str, Any]) -> str:
    state = case["state"]
    if state == "pass":
        return "pass"
    if state == "fail":
        return "failure"
    if state in {"error", "timeout"}:
        return "error"
    if state == "blocked":
        return "error" if case["policy"] == "required" else "skipped"
    if state == "skipped":
        return "error" if case["policy"] == "required" else "skipped"
    raise AssertionError(state)


def _xml_safe(value: Any) -> str:
    text = str(value)
    return "".join(
        character
        if character in "\t\n\r" or 0x20 <= ord(character) <= 0xD7FF or 0xE000 <= ord(character) <= 0xFFFD
        else "\uFFFD"
        for character in text
    )


def render_junit(document: dict[str, Any]) -> bytes:
    root = ET.Element("testsuites")
    total_counts = {"tests": 0, "failures": 0, "errors": 0, "skipped": 0}
    for policy in ("required", "observational"):
        cases = [case for case in document["cases"] if case["policy"] == policy]
        incomplete = policy == "required" and not document["run"]["selection"][
            "complete_required"
        ]
        counts = {
            "tests": len(cases) + int(incomplete),
            "failures": 0,
            "errors": int(incomplete),
            "skipped": 0,
        }
        suite = ET.SubElement(root, "testsuite", {"name": f"semantic-evidence-{policy}"})
        if incomplete:
            coverage = ET.SubElement(
                suite,
                "testcase",
                {
                    "classname": "semantic-evidence.coverage",
                    "name": "required-suite-completeness",
                    "time": "0.000000",
                },
            )
            ET.SubElement(
                coverage,
                "error",
                {
                    "type": "incomplete_required_evidence",
                    "message": "result does not cover every required manifest instance",
                },
            ).text = "partial evidence is never a successful required gate"
        for case in cases:
            test = ET.SubElement(
                suite,
                "testcase",
                {
                    "classname": _xml_safe(
                        f"{case['category']}.{case['architecture']}"
                    ),
                    "name": _xml_safe(case["case_id"]),
                    "time": f"{float(case['duration_seconds']):.6f}",
                },
            )
            properties = ET.SubElement(test, "properties")
            for name, value in (
                ("architecture", case["architecture"]),
                ("policy", case["policy"]),
                ("state", case["state"]),
                ("target_evidence", case["target_evidence"]),
                ("observed_evidence", case["observed_evidence"] or "none"),
                ("reason_code", case["reason_code"]),
                ("raw_log", case["logs"]["raw"]["path"]),
            ):
                ET.SubElement(
                    properties,
                    "property",
                    {"name": name, "value": _xml_safe(value)},
                )
            disposition = _junit_disposition(case)
            if disposition == "failure":
                counts["failures"] += 1
                node = ET.SubElement(
                    test,
                    "failure",
                    {
                        "type": _xml_safe(case["reason_code"]),
                        "message": _xml_safe(case["reason"]),
                    },
                )
                node.text = _xml_safe(case["reason"])
            elif disposition == "error":
                counts["errors"] += 1
                node = ET.SubElement(
                    test,
                    "error",
                    {
                        "type": _xml_safe(case["state"]),
                        "message": _xml_safe(
                            f"{case['reason_code']}: {case['reason']}"
                        ),
                    },
                )
                node.text = _xml_safe(case["reason"])
            elif disposition == "skipped":
                counts["skipped"] += 1
                ET.SubElement(
                    test,
                    "skipped",
                    {
                        "message": _xml_safe(
                            f"{case['reason_code']}: {case['reason']}"
                        )
                    },
                )
            log_ref = ET.SubElement(test, "system-out")
            log_ref.text = _xml_safe(f"raw log: {case['logs']['raw']['path']}")
        for name, value in counts.items():
            suite.set(name, str(value))
            total_counts[name] += value
    for name, value in total_counts.items():
        root.set(name, str(value))
    ET.indent(root, space="  ")
    return ET.tostring(root, encoding="utf-8", xml_declaration=True, short_empty_elements=True)


def _state_class(state: str) -> str:
    return "state-" + state


def render_html(document: dict[str, Any], *, log_prefix: str = ".") -> bytes:
    summary = document["summary"]
    rows: list[str] = []
    for case in document["cases"]:
        raw_path = case["logs"]["raw"]["path"]
        values = (
            case["case_id"],
            case["architecture"],
            case["policy"],
            case["target_evidence"],
            case["observed_evidence"] or "—",
            case["state"],
            case["reason_code"],
            case["reason"],
            f"{float(case['duration_seconds']):.3f}s",
        )
        cells = "".join(f"<td>{html.escape(str(value), quote=True)}</td>" for value in values)
        link = html.escape(
            quote(f"{log_prefix.rstrip('/')}/{raw_path}", safe="/._-"), quote=True
        )
        rows.append(
            f'<tr class="{_state_class(case["state"])}">{cells}'
            f'<td><a href="{link}">raw log</a></td></tr>'
        )
    state_summary = " ".join(
        f"{state}={summary['states'][state]}" for state in summary["states"]
    )
    title = "OrayS PR3 Semantic Evidence v1"
    coverage = document["run"]["selection"]
    coverage_banner = (
        '<p class="coverage-complete"><strong>Coverage:</strong> COMPLETE required suite</p>'
        if coverage["complete_required"]
        else '<p class="coverage-incomplete"><strong>Coverage: INCOMPLETE / NOT A REQUIRED GATE</strong></p>'
    )
    content = f"""<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src 'unsafe-inline'">
<title>{title}</title>
<style>
:root {{ color-scheme: light dark; font-family: system-ui, sans-serif; }}
body {{ margin: 2rem; line-height: 1.4; }}
table {{ border-collapse: collapse; width: 100%; font-size: 0.9rem; }}
th, td {{ border: 1px solid #8888; padding: 0.45rem; text-align: left; vertical-align: top; }}
th {{ position: sticky; top: 0; background: Canvas; }}
.state-pass {{ background: color-mix(in srgb, #2e7d32 15%, Canvas); }}
.state-fail, .state-error, .state-timeout {{ background: color-mix(in srgb, #c62828 15%, Canvas); }}
.state-blocked, .state-skipped {{ background: color-mix(in srgb, #ed6c02 15%, Canvas); }}
.coverage-incomplete {{ border: 3px solid #c62828; padding: 1rem; font-size: 1.2rem; }}
.coverage-complete {{ border: 2px solid #2e7d32; padding: 0.6rem; }}
code {{ overflow-wrap: anywhere; }}
</style>
</head>
<body>
<h1>{title}</h1>
{coverage_banner}
<p><strong>Suite:</strong> {html.escape(document['manifest']['suite_id'])}</p>
<p><strong>Revision:</strong> <code>{html.escape(document['repository'].get('revision', 'unknown'))}</code>
 &nbsp; <strong>Manifest SHA-256:</strong> <code>{html.escape(document['manifest']['sha256'])}</code></p>
<p><strong>Repository dirty:</strong> {str(bool(document['repository']['dirty'])).lower()}
 &nbsp; <strong>Source content SHA-256:</strong>
 <code>{html.escape(document['repository']['content_sha256'])}</code></p>
<p><strong>Total:</strong> {summary['total']} &nbsp; <strong>Required non-pass:</strong>
 {summary['required_nonpass']} &nbsp; {html.escape(state_summary)}</p>
<table>
<thead><tr><th>Case</th><th>Arch</th><th>Policy</th><th>Target evidence</th>
<th>Observed evidence</th><th>State</th><th>Reason code</th><th>Reason</th>
<th>Duration</th><th>Evidence</th></tr></thead>
<tbody>
{''.join(rows)}
</tbody>
</table>
</body>
</html>
"""
    return content.encode("utf-8")


def _markdown_escape(value: Any) -> str:
    text = str(value)
    text = text.replace("\\", "\\\\")
    text = text.replace("|", "\\|")
    text = text.replace("\r\n", "<br>").replace("\r", "<br>").replace("\n", "<br>")
    text = text.replace("<", "&lt;").replace(">", "&gt;")
    return text


def render_matrix(document: dict[str, Any], *, log_prefix: str = ".") -> bytes:
    complete_required = document["run"]["selection"]["complete_required"]
    lines = [
        "# OrayS PR3 Semantic Evidence Matrix v1",
        "",
        f"- Suite: {_markdown_escape(document['manifest']['suite_id'])}",
        f"- Revision: {_markdown_escape(document['repository'].get('revision', 'unknown'))}",
        f"- Repository dirty: {str(bool(document['repository']['dirty'])).lower()}",
        f"- Source content SHA-256: {_markdown_escape(document['repository']['content_sha256'])}",
        f"- Manifest SHA-256: {_markdown_escape(document['manifest']['sha256'])}",
        f"- Required non-pass: {document['summary']['required_nonpass']}",
        f"- Required coverage: {'COMPLETE' if complete_required else 'INCOMPLETE / NOT A REQUIRED GATE'}",
        "",
        "| Case | Arch | Policy | Target evidence | Observed evidence | State | Reason | Duration | Raw log |",
        "|---|---|---|---|---|---|---|---:|---|",
    ]
    for case in document["cases"]:
        values = (
            case["case_id"],
            case["architecture"],
            case["policy"],
            case["target_evidence"],
            case["observed_evidence"] or "—",
            case["state"],
            f"{case['reason_code']}: {case['reason']}",
            f"{float(case['duration_seconds']):.3f}s",
        )
        raw_path = case["logs"]["raw"]["path"]
        link = quote(f"{log_prefix.rstrip('/')}/{raw_path}", safe="/._-")
        cells = [_markdown_escape(value) for value in values]
        cells.append(f"[raw log]({link})")
        lines.append("| " + " | ".join(cells) + " |")
    lines.append("")
    lines.append(
        "A pass means only that the stated target evidence was established; static or build "
        "evidence is never a claim of runtime syscall compatibility."
    )
    lines.append("")
    return "\n".join(lines).encode("utf-8")


def render_all(
    input_path: Path,
    output_dir: Path,
    *,
    manifest_path: Path | None = None,
    repo_root: Path | None = None,
    allow_partial: bool = False,
) -> dict[str, Path]:
    outputs = {
        "junit": output_dir / JUNIT_NAME,
        "html": output_dir / HTML_NAME,
        "matrix": output_dir / MATRIX_NAME,
    }
    if manifest_path is not None:
        if repo_root is None:
            raise EvidenceError("manifest-aware rendering requires a repository root")
        ensure_safe_output_directory(output_dir, repo_root)
    for path in outputs.values():
        try:
            path.unlink(missing_ok=True)
        except OSError as exc:
            raise EvidenceError(f"cannot clear stale owned report {path}: {exc}") from exc
    if manifest_path is None:
        if not allow_partial:
            raise EvidenceError("rendering requires --manifest for evidence binding")
        document = load_and_validate_result(input_path)
    else:
        assert repo_root is not None
        document = load_validate_result_with_manifest(
            input_path,
            manifest_path=manifest_path,
            repo_root=repo_root,
            require_full_required=not allow_partial,
        )
    relative_bundle = os.path.relpath(input_path.parent, output_dir).replace(os.sep, "/")
    rendered = {
        "junit": render_junit(document),
        "html": render_html(document, log_prefix=relative_bundle),
        "matrix": render_matrix(document, log_prefix=relative_bundle),
    }
    for name in ("junit", "html", "matrix"):
        _atomic_write(outputs[name], rendered[name])
    return outputs


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--output", "--output-dir", dest="output", type=Path, required=True)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument(
        "--allow-partial",
        action="store_true",
        help="render an explicitly incomplete report with a non-green coverage marker",
    )
    return parser


def main(argv: Sequence[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    try:
        repo_root = Path(__file__).resolve().parent.parent
        outputs = render_all(
            args.input.resolve(),
            args.output,
            manifest_path=args.manifest.resolve(),
            repo_root=repo_root,
            allow_partial=args.allow_partial,
        )
    except (EvidenceError, OSError, ValueError) as exc:
        print(f"render-semantic-evidence: {exc}", file=sys.stderr)
        return 2
    for name in ("junit", "html", "matrix"):
        print(outputs[name])
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
