#!/usr/bin/env python3
"""Read-only verification gates for the OSKernel workflow-skill migration."""

from __future__ import annotations

import argparse
import datetime as dt
import fnmatch
import hashlib
import json
import os
import pathlib
import re
import subprocess
import sys
from typing import Any, Iterable


EVIDENCE_PARTS = (".omx", "evidence", "finals-kernel-workflow-skills")
KNOWN_SKILLS = {
    "oskernel-kernel-engineering",
    "oskernel-validation",
    "oskernel-cross-arch-delivery",
    "oskernel-compatibility-evaluation",
    "oskernel-experimental-features",
    "oskernel-collaboration-delivery",
    "oskernel-repo-hygiene",
}
ALLOWED_RESOURCES = {"SKILL.md", "agents", "references", "scripts"}
DISPOSITIONS = {"preserved", "merged", "replaced", "retired"}
LINK_RE = re.compile(r"\[[^\]]+\]\(([^)]+)\)")
EXPLICIT_ID_ANCHOR_RE = re.compile(r"<a\s+id=[\"']([^\"']+)[\"']\s*>\s*</a>", re.I)


def now() -> str:
    return dt.datetime.now(dt.timezone.utc).isoformat()


def run(argv: list[str], cwd: pathlib.Path, check: bool = True) -> subprocess.CompletedProcess[str]:
    return subprocess.run(argv, cwd=cwd, text=True, capture_output=True, check=check)


def git(repo: pathlib.Path, *args: str, check: bool = True) -> str:
    return run(["git", *args], repo, check=check).stdout


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def file_digest(path: pathlib.Path) -> str:
    if path.is_symlink():
        return sha256(os.readlink(path).encode())
    return sha256(path.read_bytes())


def canonical_inputs(argv: list[str], paths: Iterable[pathlib.Path]) -> str:
    items = []
    for path in sorted({p.resolve() for p in paths if p.exists()}, key=str):
        items.append({"path": str(path), "sha256": file_digest(path) if path.is_file() or path.is_symlink() else "directory"})
    payload = {"argv": argv, "inputs": items}
    raw = json.dumps(payload, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()
    return sha256(raw)


def repo_baseline(repo: pathlib.Path) -> str:
    cp = run(["git", "rev-parse", "HEAD"], repo, check=False)
    return cp.stdout.strip() if cp.returncode == 0 else ""


def ensure_output(repo: pathlib.Path, out: str) -> pathlib.Path:
    path = pathlib.Path(out)
    if not path.is_absolute():
        path = repo / path
    path = path.resolve()
    evidence = repo.resolve().joinpath(*EVIDENCE_PARTS)
    try:
        path.relative_to(evidence)
    except ValueError as exc:
        raise ValueError(f"--out must resolve below {evidence}") from exc
    path.parent.mkdir(parents=True, exist_ok=True)
    return path


def emit(repo: pathlib.Path, args: argparse.Namespace, started: str, failures: list[str], inputs: Iterable[pathlib.Path], details: dict[str, Any]) -> int:
    out = ensure_output(repo, args.out)
    result = {
        "check": args.command,
        "command": sys.argv,
        "baseline_sha": repo_baseline(repo),
        "started_at": started,
        "finished_at": now(),
        "passed": not failures,
        "failures": failures,
        "inputs_sha256": canonical_inputs(sys.argv, inputs),
        **details,
    }
    out.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    if failures:
        for failure in failures:
            print(f"FAIL: {failure}", file=sys.stderr)
        return 1
    print(f"PASS: {args.command} ({out})")
    return 0


def parse_frontmatter(path: pathlib.Path) -> tuple[dict[str, str], str]:
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---\n"):
        raise ValueError("missing YAML frontmatter")
    end = text.find("\n---\n", 4)
    if end < 0:
        raise ValueError("unterminated YAML frontmatter")
    data: dict[str, str] = {}
    for line in text[4:end].splitlines():
        if not line.strip():
            continue
        if ":" not in line:
            raise ValueError(f"malformed frontmatter line: {line}")
        key, value = line.split(":", 1)
        data[key.strip()] = value.strip().strip('"\'')
    return data, text[end + 5 :]


def parse_interface(path: pathlib.Path) -> dict[str, str]:
    values: dict[str, str] = {}
    in_interface = False
    for raw in path.read_text(encoding="utf-8").splitlines():
        if raw.strip() == "interface:":
            in_interface = True
            continue
        if in_interface and raw and not raw.startswith((" ", "\t")):
            break
        if in_interface:
            match = re.match(r"\s{2}([a-z_]+):\s*[\"'](.*)[\"']\s*$", raw)
            if match:
                values[match.group(1)] = match.group(2)
    return values


def explicit_anchor_counts(path: pathlib.Path) -> dict[str, int]:
    counts: dict[str, int] = {}
    for anchor in EXPLICIT_ID_ANCHOR_RE.findall(path.read_text(encoding="utf-8")):
        counts[anchor] = counts.get(anchor, 0) + 1
    return counts


def package_check(args: argparse.Namespace) -> int:
    started = now()
    repo = pathlib.Path(args.root).resolve()
    skill_root = repo / ".codex" / "skills"
    failures: list[str] = []
    inputs: list[pathlib.Path] = []
    dirs = sorted(p.parent for p in skill_root.glob(f"{args.expected_prefix}*/SKILL.md"))
    names = [p.name for p in dirs]
    if len(dirs) != args.expected_count:
        failures.append(f"expected {args.expected_count} skills, found {len(dirs)}: {names}")
    if args.expected_count == 7 and set(names) != KNOWN_SKILLS:
        failures.append(f"seven-skill set mismatch: {sorted(set(names) ^ KNOWN_SKILLS)}")
    if len(names) != len(set(names)):
        failures.append("duplicate skill directory/name")

    quick = pathlib.Path("/root/.codex/skills/.system/skill-creator/scripts/quick_validate.py")
    for directory in dirs:
        inputs.extend(p for p in directory.rglob("*") if p.is_file() or p.is_symlink())
        extras = {p.name for p in directory.iterdir()} - ALLOWED_RESOURCES
        if extras:
            failures.append(f"{directory}: forbidden top-level resources {sorted(extras)}")
        if any(p.name.lower() == "readme.md" for p in directory.rglob("*")):
            failures.append(f"{directory}: README is forbidden")
        if any(p.is_dir() for p in (directory / "references").glob("*/*")) if (directory / "references").exists() else False:
            failures.append(f"{directory}: nested reference directory")
        scripts = sorted(p for p in (directory / "scripts").glob("*") if p.is_file()) if (directory / "scripts").exists() else []
        allowed_script = directory.name == "oskernel-validation" and [p.name for p in scripts] == ["verify_workflow_skills.py"]
        if scripts and not allowed_script:
            failures.append(f"{directory}: unplanned scripts {[p.name for p in scripts]}")
        for path in directory.rglob("*"):
            if path.is_file() and path.suffix.lower() in {".md", ".yaml", ".yml"} and re.search(r"\b(TODO|PLACEHOLDER)\b", path.read_text(encoding="utf-8", errors="ignore"), re.I):
                failures.append(f"{path}: placeholder marker")
        try:
            meta, body = parse_frontmatter(directory / "SKILL.md")
            if set(meta) != {"name", "description"}:
                failures.append(f"{directory}: frontmatter keys must be name,description")
            if meta.get("name") != directory.name:
                failures.append(f"{directory}: frontmatter name mismatch")
            desc = meta.get("description", "")
            if "Use " not in desc or not re.search(r"do not use|exclude", desc, re.I):
                failures.append(f"{directory}: description lacks positive trigger or exclusion")
            for target in LINK_RE.findall(body):
                if target.startswith(("http:", "https:", "#")):
                    continue
                rel = target.split("#", 1)[0]
                resolved = (directory / rel).resolve()
                try:
                    resolved.relative_to(directory.resolve())
                except ValueError:
                    failures.append(f"{directory}: link escapes skill: {target}")
                    continue
                if not resolved.is_file():
                    failures.append(f"{directory}: broken link: {target}")
        except Exception as exc:
            failures.append(f"{directory / 'SKILL.md'}: {exc}")
        ui = directory / "agents" / "openai.yaml"
        if not ui.is_file():
            failures.append(f"{directory}: missing agents/openai.yaml")
        else:
            text = ui.read_text(encoding="utf-8")
            if re.search(r"^dependencies:", text, re.M):
                failures.append(f"{ui}: dependencies are forbidden")
            interface = parse_interface(ui)
            if not interface.get("display_name"):
                failures.append(f"{ui}: missing display_name")
            short = interface.get("short_description", "")
            if not 25 <= len(short) <= 64:
                failures.append(f"{ui}: short_description length {len(short)} outside 25..64")
            if f"${directory.name}" not in interface.get("default_prompt", ""):
                failures.append(f"{ui}: default_prompt must invoke ${directory.name}")
        cp = run([sys.executable, str(quick), str(directory)], repo, check=False)
        if cp.returncode != 0 or "Skill is valid!" not in cp.stdout:
            failures.append(f"{directory}: quick_validate failed: {(cp.stdout + cp.stderr).strip()}")

    return emit(repo, args, started, failures, inputs, {"skills": names})


def markdown_sections(text: str) -> list[dict[str, Any]]:
    headings: list[tuple[int, int, str]] = []
    fenced = False
    for number, line in enumerate(text.splitlines(), 1):
        if re.match(r"^\s*(```|~~~)", line):
            fenced = not fenced
            continue
        if fenced:
            continue
        match = re.match(r"^(#{1,3})\s+(.+?)\s*$", line)
        if match:
            headings.append((number, len(match.group(1)), match.group(2).strip()))
    lines = text.splitlines()
    result = []
    for index, (start, level, heading) in enumerate(headings):
        end = len(lines)
        for next_start, next_level, _ in headings[index + 1 :]:
            if next_level <= level:
                end = next_start - 1
                break
        result.append({"heading": heading, "line_start": start, "line_end": end})
    return result


def hard_constraint_items(text: str) -> list[dict[str, Any]]:
    """Return every list item, including nested items, under 高频硬约束."""
    lines = text.splitlines()
    headings = [
        number
        for number, line in enumerate(lines, 1)
        if re.match(r"^##\s+高频硬约束\s*$", line)
    ]
    if len(headings) != 1:
        raise ValueError(f"expected one AGENTS 高频硬约束 heading, found {len(headings)}")
    section_start = headings[0] + 1
    section_end = len(lines) + 1
    for number in range(section_start, len(lines) + 1):
        if re.match(r"^#{1,2}\s+", lines[number - 1]):
            section_end = number
            break

    starts = [
        number
        for number in range(section_start, section_end)
        if re.match(r"^\s*[-*+]\s+\S", lines[number - 1])
    ]
    result = []
    for index, start in enumerate(starts):
        end = (starts[index + 1] - 1) if index + 1 < len(starts) else section_end - 1
        while end >= start and not lines[end - 1].strip():
            end -= 1
        result.append(
            {
                "source_line": start,
                "line_start": start,
                "line_end": end,
                "source_text": "\n".join(lines[start - 1 : end]),
            }
        )
    return result


def marker_json(text: str) -> dict[str, Any]:
    match = re.search(r"<!--\s*MIGRATION_MAP:START\s*-->(.*?)<!--\s*MIGRATION_MAP:END\s*-->", text, re.S)
    if not match:
        raise ValueError("migration markers missing")
    payload = match.group(1).strip()
    if payload.startswith("```json"):
        payload = payload[7:]
    if payload.endswith("```"):
        payload = payload[:-3]
    return json.loads(payload.strip())


def field(row: dict[str, Any], *names: str) -> Any:
    for name in names:
        if name in row:
            return row[name]
    return None


def migration_check(args: argparse.Namespace) -> int:
    started = now()
    repo = pathlib.Path.cwd().resolve()
    failures: list[str] = []
    inputs = [repo / args.audit]
    try:
        audit = marker_json((repo / args.audit).read_text(encoding="utf-8"))
    except Exception as exc:
        return emit(repo, args, started, [str(exc)], inputs, {})
    if audit.get("baseline_sha") != args.baseline:
        failures.append("audit baseline_sha mismatch")
    if audit.get("status") != "reconciled":
        failures.append("audit status must be reconciled")
    rows = audit.get("heading_rows", [])
    root_rows = audit.get("root_invariants", [])
    if not isinstance(rows, list) or not isinstance(root_rows, list):
        failures.append("heading_rows and root_invariants must be lists")
        rows, root_rows = [], []

    old_files = git(repo, "ls-tree", "-r", "--name-only", args.baseline, "--", args.old_dir).splitlines()
    expected: list[tuple[str, str, int, int]] = []
    baseline_text: dict[str, list[str]] = {}
    for old in old_files:
        if not old.endswith(".md"):
            continue
        text = git(repo, "show", f"{args.baseline}:{old}")
        baseline_text[old] = text.splitlines()
        expected.extend((old, sec["heading"], sec["line_start"], sec["line_end"]) for sec in markdown_sections(text))
    if len(baseline_text) != 9:
        failures.append(f"expected 9 baseline guidance files, found {len(baseline_text)}")
    expected_set = set(expected)
    covered_headings: set[tuple[str, str, int, int]] = set()
    heading_targets: set[tuple[tuple[str, str, int, int], str, str]] = set()
    checked_targets = 0

    def required_int(row: dict[str, Any], *names: str) -> int | None:
        value = field(row, *names)
        return value if type(value) is int else None

    def validate_target(row: dict[str, Any], label: str) -> None:
        nonlocal checked_targets
        disposition = field(row, "disposition")
        if disposition not in DISPOSITIONS:
            failures.append(f"{label}: invalid disposition {disposition!r}")
        target = field(row, "target_path", "target")
        anchor = str(field(row, "target_anchor", "anchor") or "").lstrip("#")
        if not target or not anchor:
            failures.append(f"{label}: target_path and target_anchor are required")
        else:
            target_path = (repo / str(target)).resolve()
            try:
                target_path.relative_to(repo)
            except ValueError:
                failures.append(f"{label}: target escapes repository: {target}")
            else:
                inputs.append(target_path)
                if not target_path.is_file():
                    failures.append(f"{label}: missing target {target}")
                else:
                    count = explicit_anchor_counts(target_path).get(anchor, 0)
                    if count != 1:
                        failures.append(
                            f"{label}: explicit anchor {target}#{anchor} occurs {count} times"
                        )
                    else:
                        checked_targets += 1
        rationale = field(row, "rationale")
        delta = field(row, "semantic_delta", "delta")
        if not isinstance(rationale, str) or not rationale.strip():
            failures.append(f"{label}: non-empty rationale is required")
        if not isinstance(delta, str) or not delta.strip():
            failures.append(f"{label}: non-empty semantic_delta is required")

    seen = set()
    for index, row in enumerate(rows):
        label = f"heading_rows[{index}]"
        if not isinstance(row, dict):
            failures.append(f"{label}: row must be an object")
            continue
        key = json.dumps(row, sort_keys=True, ensure_ascii=False)
        if key in seen:
            failures.append(f"{label}: identical duplicate migration row")
        seen.add(key)
        if row.get("baseline_sha") != args.baseline:
            failures.append(f"{label}: baseline_sha mismatch")
        source = field(row, "source_path", "source")
        heading = field(row, "heading", "source_heading")
        row_start = required_int(row, "line_start", "source_line_start")
        row_end = required_int(row, "line_end", "source_line_end")
        section = (source, heading, row_start, row_end)
        if section not in expected_set:
            failures.append(
                f"{label}: source is not an exact baseline section: "
                f"{source}:{row_start}-{row_end} {heading!r}"
            )
        else:
            covered_headings.add(section)
            source_text = field(row, "source_text")
            if source_text is not None:
                lines = baseline_text[str(source)]
                exact = "\n".join(lines[row_start - 1 : row_end])
                if source_text != exact:
                    failures.append(f"{label}: source_text does not match baseline section")
            target = str(field(row, "target_path", "target") or "")
            anchor = str(field(row, "target_anchor", "anchor") or "").lstrip("#")
            target_key = (section, target, anchor)
            if target_key in heading_targets:
                failures.append(f"{label}: duplicate source-to-target coverage")
            heading_targets.add(target_key)
        validate_target(row, label)

    for old, heading, start, end in expected:
        if (old, heading, start, end) not in covered_headings:
            failures.append(f"unmapped heading {old}:{start}-{end} {heading}")

    baseline_agents_text = git(repo, "show", f"{args.baseline}:{args.baseline_agents}")
    expected_root = hard_constraint_items(baseline_agents_text)
    if len(expected_root) != 23:
        failures.append(f"expected 23 baseline root invariants, parsed {len(expected_root)}")
    expected_root_by_line = {item["source_line"]: item for item in expected_root}
    root_coverage: dict[int, int] = {line: 0 for line in expected_root_by_line}
    for index, row in enumerate(root_rows):
        label = f"root_invariants[{index}]"
        if not isinstance(row, dict):
            failures.append(f"{label}: row must be an object")
            continue
        key = json.dumps(row, sort_keys=True, ensure_ascii=False)
        if key in seen:
            failures.append(f"{label}: identical duplicate migration row")
        seen.add(key)
        if row.get("baseline_sha") != args.baseline:
            failures.append(f"{label}: baseline_sha mismatch")
        if field(row, "source_path", "source") != args.baseline_agents:
            failures.append(f"{label}: source_path must be {args.baseline_agents}")
        source_line = required_int(row, "source_line", "line")
        row_start = required_int(row, "line_start", "source_line_start")
        row_end = required_int(row, "line_end", "source_line_end")
        source_text = field(row, "source_text", "text")
        expected_item = expected_root_by_line.get(source_line)
        if expected_item is None:
            failures.append(f"{label}: source_line {source_line!r} is not a baseline hard constraint")
        else:
            root_coverage[source_line] += 1
            if row_start != expected_item["line_start"] or row_end != expected_item["line_end"]:
                failures.append(
                    f"{label}: range {row_start}-{row_end} does not match baseline "
                    f"{expected_item['line_start']}-{expected_item['line_end']}"
                )
            if source_text != expected_item["source_text"]:
                failures.append(f"{label}: source_text does not exactly match baseline")
        validate_target(row, label)
    for line, count in sorted(root_coverage.items()):
        if count != 1:
            failures.append(f"baseline root invariant at line {line} has {count} rows, expected exactly 1")
    if len(root_rows) != len(expected_root):
        failures.append(
            f"root_invariants row count {len(root_rows)} does not equal expected {len(expected_root)}"
        )

    return emit(
        repo,
        args,
        started,
        failures,
        inputs,
        {
            "baseline_sha": args.baseline,
            "audit_status": audit.get("status"),
            "baseline_guidance_files": len(baseline_text),
            "source_headings": len(expected),
            "heading_rows": len(rows),
            "heading_sections_covered": len(covered_headings),
            "expected_root_invariants": len(expected_root),
            "root_invariants": len(root_rows),
            "root_invariants_covered": sum(count == 1 for count in root_coverage.values()),
            "explicit_target_anchors_checked": checked_targets,
        },
    )


def zpaths(repo: pathlib.Path, argv: list[str]) -> list[str]:
    cp = subprocess.run(argv, cwd=repo, capture_output=True, check=True)
    return [item.decode() for item in cp.stdout.split(b"\0") if item]


def allowed(path: str, patterns: list[str]) -> bool:
    return any(fnmatch.fnmatchcase(path, pattern) or (pattern.endswith("/**") and path == pattern[:-3]) for pattern in patterns)


def scope_check(args: argparse.Namespace) -> int:
    started = now()
    repo = pathlib.Path(args.repo).resolve()
    failures: list[str] = []
    inputs = [pathlib.Path(args.baseline_files)]
    patterns = [item.strip() for item in args.allow.split(",") if item.strip()]
    if run(["git", "merge-base", "--is-ancestor", args.baseline, args.head], repo, check=False).returncode:
        failures.append("baseline is not an ancestor of head")
    paths = set(zpaths(repo, ["git", "diff", "--name-only", "-z", "--no-renames", f"{args.baseline}..{args.head}", "--"]))
    paths |= set(zpaths(repo, ["git", "diff", "--name-only", "-z", "--no-renames", args.baseline, "--"]))
    paths |= set(zpaths(repo, ["git", "diff", "--cached", "--name-only", "-z", "--no-renames", "--"]))
    paths |= set(zpaths(repo, ["git", "ls-files", "--others", "--exclude-standard", "-z"]))
    for path in sorted(paths):
        if not allowed(path, patterns):
            failures.append(f"path outside allowlist: {path}")
    baseline_file = pathlib.Path(args.baseline_files)
    if not baseline_file.is_absolute():
        baseline_file = repo / baseline_file
    if baseline_file.is_file():
        snapshot = json.loads(baseline_file.read_text(encoding="utf-8"))
        for item in snapshot.get("dirty", []):
            path = item["path"]
            if allowed(path, patterns):
                continue
            current = repo / path
            digest = None if not current.exists() and not current.is_symlink() else (file_digest(current) if current.is_file() or current.is_symlink() else "directory")
            index = git(repo, "ls-files", "-s", "--", path).strip()
            if digest != item.get("worktree_sha256") or index != item.get("index", ""):
                failures.append(f"baseline-dirty path changed: {path}")
    else:
        failures.append(f"baseline-files missing: {baseline_file}")
    commits = git(repo, "rev-list", "--reverse", f"{args.baseline}..{args.head}").splitlines()
    for commit in commits:
        if len(git(repo, "show", "-s", "--format=%P", commit).split()) > 1:
            failures.append(f"merge commit forbidden: {commit}")
        for path in git(repo, "diff-tree", "--no-commit-id", "--name-only", "-r", "--no-renames", commit).splitlines():
            if not allowed(path, patterns):
                failures.append(f"commit {commit} path outside allowlist: {path}")
    return emit(repo, args, started, failures, inputs, {"paths": sorted(paths), "allowlist": patterns, "commits": commits})


def load_json(path: pathlib.Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def trace_supports(path: pathlib.Path, skill: str, references: list[str]) -> list[str]:
    failures = []
    text = path.read_text(encoding="utf-8", errors="replace") if path.is_file() else ""
    if not text:
        return [f"missing/empty trace {path}"]
    if f".codex/skills/{skill}/SKILL.md" not in text and not re.search(rf"activation.*{re.escape(skill)}", text, re.I):
        failures.append(f"trace lacks activation/read evidence for {skill}: {path}")
    for reference in references:
        if reference not in text:
            failures.append(f"trace lacks consulted reference {reference}: {path}")
    return failures


def routing_check(args: argparse.Namespace) -> int:
    started = now()
    repo = pathlib.Path.cwd().resolve()
    failures: list[str] = []
    discovery_path = pathlib.Path(args.discovery)
    explicit_dir = pathlib.Path(args.explicit_dir)
    blind_dir = pathlib.Path(args.blind_dir)
    oracle_path = pathlib.Path(args.oracle)
    trace_dir = pathlib.Path(args.trace_dir)
    inputs = [discovery_path, oracle_path]
    discovery = load_json(discovery_path)
    discovered = discovery.get("skills", discovery if isinstance(discovery, list) else [])
    discovered_names = {item.get("name") if isinstance(item, dict) else item for item in discovered}
    if discovered_names != KNOWN_SKILLS:
        failures.append(f"discovery mismatch: {sorted(discovered_names ^ KNOWN_SKILLS)}")
    oracle = load_json(oracle_path)
    cases = oracle.get("cases", oracle if isinstance(oracle, list) else [])

    def validate_result(path: pathlib.Path, expected: str, allowed_secondary: set[str]) -> None:
        inputs.append(path)
        if not path.is_file():
            failures.append(f"missing routing result {path}")
            return
        result = load_json(path)
        primary = result.get("primary_skill")
        secondary = set(result.get("secondary_skills", []))
        if primary != expected:
            failures.append(f"{path}: primary {primary!r}, expected {expected!r}")
        if not secondary <= allowed_secondary:
            failures.append(f"{path}: disallowed secondary {sorted(secondary - allowed_secondary)}")
        if result.get("would_write") is not False:
            failures.append(f"{path}: would_write must be false")
        refs = result.get("references_consulted", [])
        if not refs:
            failures.append(f"{path}: references_consulted is empty")
        trace = trace_dir / f"{path.stem}.jsonl"
        inputs.append(trace)
        failures.extend(trace_supports(trace, expected, refs))

    for skill in sorted(KNOWN_SKILLS):
        validate_result(explicit_dir / f"{skill}.json", skill, set())
    for case in cases:
        case_id = str(field(case, "id", "case_id"))
        expected = str(field(case, "primary_skill", "expected_primary"))
        secondaries = set(field(case, "allowed_secondary_skills", "allowed_secondary") or [])
        validate_result(blind_dir / f"{case_id}.json", expected, secondaries)
    return emit(repo, args, started, failures, inputs, {"discovered": sorted(discovered_names), "explicit": 7, "blind": len(cases)})


def parser() -> argparse.ArgumentParser:
    main = argparse.ArgumentParser(description=__doc__)
    sub = main.add_subparsers(dest="command", required=True)
    packages = sub.add_parser("packages")
    packages.add_argument("--root", required=True)
    packages.add_argument("--expected-prefix", required=True)
    packages.add_argument("--expected-count", required=True, type=int)
    packages.add_argument("--out", required=True)
    packages.set_defaults(func=package_check)
    migration = sub.add_parser("migration")
    migration.add_argument("--baseline", required=True)
    migration.add_argument("--audit", required=True)
    migration.add_argument("--old-dir", required=True)
    migration.add_argument("--baseline-agents", required=True)
    migration.add_argument("--out", required=True)
    migration.set_defaults(func=migration_check)
    scope = sub.add_parser("scope")
    scope.add_argument("--repo", required=True)
    scope.add_argument("--baseline", required=True)
    scope.add_argument("--head", required=True)
    scope.add_argument("--baseline-files", required=True)
    scope.add_argument("--allow", required=True)
    scope.add_argument("--out", required=True)
    scope.set_defaults(func=scope_check)
    routing = sub.add_parser("routing")
    routing.add_argument("--discovery", required=True)
    routing.add_argument("--explicit-dir", required=True)
    routing.add_argument("--blind-dir", required=True)
    routing.add_argument("--oracle", required=True)
    routing.add_argument("--trace-dir", required=True)
    routing.add_argument("--out", required=True)
    routing.set_defaults(func=routing_check)
    return main


def main() -> int:
    args = parser().parse_args()
    try:
        return args.func(args)
    except (OSError, ValueError, subprocess.CalledProcessError, json.JSONDecodeError) as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
