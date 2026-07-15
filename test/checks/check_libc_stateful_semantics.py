#!/usr/bin/env python3
"""Static guard for libc-stateful-semantics empty-success / half-stub libc surfaces."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


def read(root: Path, rel: str) -> str:
    return (root / rel).read_text(encoding="utf-8", errors="ignore")


def c_function_block(text: str, name: str) -> str:
    match = re.search(rf"(?:^|\n)[A-Za-z_][\w\t \*]*\s+\*?\s*{re.escape(name)}\s*\([^;]*\)\s*\{{", text)
    if not match:
        return ""
    brace = text.find("{", match.start())
    depth = 0
    for idx in range(brace, len(text)):
        if text[idx] == "{":
            depth += 1
        elif text[idx] == "}":
            depth -= 1
            if depth == 0:
                return text[match.start() : idx + 1]
    return text[match.start() :]


def require_tokens(findings: list[str], block: str, desc: str, tokens: tuple[str, ...]) -> None:
    if not block:
        findings.append(f"{desc}: function block missing")
        return
    for token in tokens:
        if token not in block:
            findings.append(f"{desc}: missing {token!r}")


def scan_signal(root: Path) -> list[str]:
    findings: list[str] = []
    text = read(root, "ulib/axlibc/c/signal.c")
    sigaction = c_function_block(text, "sigaction_helper")
    raise_block = c_function_block(text, "raise")
    kill_block = c_function_block(text, "kill")
    mask_block = c_function_block(text, "pthread_sigmask")
    pthread_kill = c_function_block(text, "pthread_kill")
    deliver = c_function_block(text, "deliver_signal_now")

    require_tokens(
        findings,
        text,
        "axlibc signal registry must keep observable state instead of returning success without storing act",
        (
            "static struct sigaction signal_actions[_NSIG]",
            "static sigset_t signal_mask",
            "static sigset_t pending_signals",
            "signal_actions[signum] = next",
            "*oldact = signal_actions[signum]",
            "remove_unmaskable_signals(&next)",
        ),
    )
    if re.search(r"int\s+sigaction_helper[\s\S]*?\*oldact\s*=\s*\(struct sigaction\)\{0\};[\s\S]*?return\s+0\s*;", sigaction):
        findings.append("sigaction_helper must not report success while only zeroing oldact")
    require_tokens(
        findings,
        deliver,
        "raise/signal delivery must invoke registered handlers or honest default actions",
        ("action.sa_handler", "default_action(signum)", "action.sa_sigaction", "handler(signum)"),
    )
    require_tokens(
        findings,
        raise_block,
        "raise must validate, queue blocked signals, and dispatch current-thread handlers",
        ("valid_signal(__sig)", "signal_set_contains(&signal_mask, __sig)", "signal_set_add(&pending_signals, __sig)", "deliver_signal_now(__sig)"),
    )
    if "unimplemented" in raise_block or "errno = ENOSYS" in raise_block:
        findings.append("raise must no longer be an ENOSYS shell after sigaction state exists")
    require_tokens(
        findings,
        kill_block,
        "kill must at least implement current-process signal semantics instead of unconditional ENOSYS",
        ("__pid == getpid()", "raise(__sig)", "errno = ESRCH", "errno = ENOSYS"),
    )
    require_tokens(
        findings,
        mask_block,
        "pthread_sigmask must maintain an observable mask instead of unconditional ENOSYS",
        ("*__oldmask = signal_mask", "SIG_BLOCK", "SIG_UNBLOCK", "SIG_SETMASK", "deliver_unblocked_pending()"),
    )
    require_tokens(
        findings,
        pthread_kill,
        "pthread_kill must implement self-thread delivery or return a real error for unsupported targets",
        ("t != pthread_self()", "return ENOSYS", "raise(sig)"),
    )
    return findings


def scan_libc_helpers(root: Path) -> list[str]:
    findings: list[str] = []
    unistd = read(root, "ulib/axlibc/c/unistd.c")
    dirent = read(root, "ulib/axlibc/c/dirent.c")
    dlfcn = read(root, "ulib/axlibc/c/dlfcn.c")
    stdlib = read(root, "ulib/axlibc/c/stdlib.c")
    time = read(root, "ulib/axlibc/c/time.c")
    pthread = read(root, "ulib/axlibc/c/pthread.c")

    isatty = c_function_block(unistd, "isatty")
    require_tokens(
        findings,
        isatty,
        "isatty must distinguish invalid fd from valid non-tty descriptors",
        ("fcntl(fd, F_GETFD)", "errno = ENOTTY"),
    )

    opendir = c_function_block(dirent, "opendir")
    readdir = c_function_block(dirent, "readdir")
    require_tokens(
        findings,
        opendir,
        "opendir must open and wrap directory fds instead of unconditional NULL",
        ("open(__name", "O_DIRECTORY", "fdopendir(fd)", "close(fd)"),
    )
    if "unimplemented" in opendir:
        findings.append("opendir must not be an unconditional NULL stub")
    require_tokens(
        findings,
        readdir,
        "readdir must report unsupported getdents honestly until implemented",
        ("errno = ENOSYS", "return NULL"),
    )

    dladdr = c_function_block(dlfcn, "dladdr")
    require_tokens(
        findings,
        dladdr,
        "dladdr must report dynamic-loader absence as lookup failure, not fake success",
        ("(void)__address", "(void)__info", "errno = ENOSYS", "return 0"),
    )
    if "return 1" in dladdr:
        findings.append("dladdr must not claim a successful lookup without a dynamic loader")

    qsort = c_function_block(stdlib, "qsort")
    require_tokens(
        findings,
        qsort,
        "qsort must reorder data through the caller comparator instead of no-op success",
        ("cmp(array + j * width", "lhs[byte]", "rhs[byte]", "tmp"),
    )
    if "unimplemented" in qsort or re.search(r"void\s+qsort[\s\S]*?unimplemented\(\)[\s\S]*?return\s*;", qsort):
        findings.append("qsort must not be a void no-op shell")

    tzset = c_function_block(time, "tzset")
    require_tokens(
        findings,
        tzset,
        "tzset must make the UTC-only libc timezone model explicit instead of being an empty TODO",
        ("timezone = 0",),
    )
    if "unimplemented" in tzset:
        findings.append("tzset must not be an unimplemented void shell")

    testcancel = c_function_block(pthread, "pthread_testcancel")
    if "unimplemented" in testcancel:
        findings.append("pthread_testcancel must not log a fake unimplemented action when cancellation state is unsupported")
    return findings


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[2])
    args = parser.parse_args()
    root = args.root.resolve()
    findings = scan_signal(root) + scan_libc_helpers(root)
    if findings:
        print("FAIL: libc-stateful-semantics empty-shell guard found issues:")
        for finding in findings:
            print(f"- {finding}")
        return 1
    print("libc stateful semantics check: PASS (0 findings)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
