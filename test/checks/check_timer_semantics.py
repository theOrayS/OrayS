#!/usr/bin/env python3
"""Guard POSIX timer and interval-timer state against fake success."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from source_scan import function_block, read


def scan_timer(root: Path) -> list[str]:
    text = read(root / "user/shell/src/uspace/time_abi.rs")
    findings: list[str] = []
    parse_block = function_block(text, "parse_posix_timer_notify")
    setitimer_block = function_block(text, "sys_setitimer")
    if not parse_block:
        findings.append("user/shell/src/uspace/time_abi.rs: missing parse_posix_timer_notify")
    else:
        if "SIGEV_THREAD" not in parse_block or "Err(LinuxError::EINVAL)" not in parse_block:
            findings.append(
                "timer_create raw SIGEV_THREAD must fail with EINVAL instead of creating a silent timer"
            )
        if "Accept it as a non-delivering" in parse_block or re.search(
            r"SIGEV_THREAD[\s\S]{0,240}Ok\(PosixTimerNotify::None\)", parse_block
        ):
            findings.append("timer_create still accepts SIGEV_THREAD as non-delivering fake success")
    if not setitimer_block:
        findings.append("user/shell/src/uspace/time_abi.rs: missing sys_setitimer")
    else:
        required = [
            "itimer_clock_micros(process, which)",
            "deadline_cell.store(now_us.saturating_add(first_us)",
            "process.real_timer_generation",
        ]
        for token in required:
            if token not in setitimer_block:
                findings.append(
                    f"setitimer must arm timers against their real backing clock; missing {token}"
                )
        if (
            "which != general::ITIMER_REAL" in setitimer_block
            and "LinuxError::EOPNOTSUPP" in setitimer_block
        ):
            findings.append(
                "setitimer must not preserve capability by rejecting armed ITIMER_VIRTUAL/PROF"
            )
        if "virtual/prof" in setitimer_block and "report real state" in setitimer_block:
            findings.append("setitimer still documents fake tracked virtual/prof timer state")
    if (
        "fn itimer_clock_micros" not in text
        or "general::ITIMER_VIRTUAL" not in text
        or "general::ITIMER_PROF" not in text
    ):
        findings.append("time_abi must expose real backing clocks for ITIMER_VIRTUAL and ITIMER_PROF")
    if "consume_expired_cpu_timers" not in text:
        findings.append("time_abi must check CPU-backed interval timers on user return")
    return findings


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[2])
    args = parser.parse_args()
    findings = scan_timer(args.root.resolve())
    if findings:
        print("timer semantics check: FAIL")
        for finding in findings:
            print(f"- {finding}")
        return 1
    print("timer semantics check: PASS (0 findings)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
