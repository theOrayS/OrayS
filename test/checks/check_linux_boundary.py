#!/usr/bin/env python3
"""Static guard for the Linux ABI, user-copy, and syscall boundaries."""

from __future__ import annotations

import argparse
import re
from pathlib import Path
from typing import NamedTuple

ABI_CRATE_REL = Path("api/orays_linux_abi")
LINUX_CRATE_REL = Path("api/orays_linux")
SHELL_MANIFEST_REL = Path("user/shell/Cargo.toml")
USPACE_REL = Path("user/shell/src/uspace")
USER_MEMORY_REL = USPACE_REL / "user_memory.rs"
METADATA_REL = USPACE_REL / "syscall_metadata.rs"
DISPATCH_REL = USPACE_REL / "syscall_dispatch.rs"

LEGACY_CALLER_INVENTORY = {
    "validate_user_read": (19, 5),
    "validate_user_write": (49, 10),
    "fault_in_user_read": (5, 2),
    "fault_in_user_write": (3, 1),
    "read_user_bytes_into": (14, 4),
    "read_user_bytes": (35, 11),
    "write_user_bytes": (47, 12),
    "read_user_value": (91, 17),
    "write_user_value": (118, 18),
    "read_cstr": (46, 6),
}

RAW_BOUNDARY_INVENTORY = {
    "typed_user_range": 6,
    "validate_user_access_raw": 5,
    "read_user_bytes_validated_raw": 3,
    "read_user_bytes_into_raw": 2,
    "write_user_bytes_raw": 2,
    "ProcessUserMemory": 7,
}

UNSAFE_FINGERPRINTS = (
    "slice::from_raw_parts_mut(raw_entries.as_mut_ptr() as *mut u8, iov_bytes_len)",
    "Vec::from_raw_parts(ptr, len, cap)",
    "core::slice::from_raw_parts(value as *const T as *const u8, size_of::<T>())",
    "core::slice::from_raw_parts_mut(value.as_mut_ptr() as *mut u8, size_of::<T>())",
    "value.assume_init()",
)


class MetadataEntry(NamedTuple):
    const_name: str
    number_expr: str
    name: str
    argument_count: int
    availability: str
    handler: str
    alias_of: str | None
    audit_id: str


EXPECTED_METADATA = {
    "CLONE": (
        "numbers::__NR_clone",
        "clone",
        5,
        "SyscallAvailability::All",
        "sys_clone",
        None,
        "clone-argument-order",
    ),
    "FSYNC": (
        "numbers::__NR_fsync",
        "fsync",
        1,
        "SyscallAvailability::All",
        "sys_fsync",
        None,
        "fsync-fdatasync-handler",
    ),
    "FDATASYNC": (
        "numbers::__NR_fdatasync",
        "fdatasync",
        1,
        "SyscallAvailability::All",
        "sys_fsync",
        "fsync",
        "fsync-fdatasync-handler",
    ),
    "POLL": (
        "numbers::__NR_poll",
        "poll",
        3,
        "SyscallAvailability::Except(POLL_EXCLUDED_ARCHITECTURES)",
        "sys_poll",
        None,
        "poll-architecture-cfg",
    ),
    "RISCV64_GETRLIMIT": (
        "numbers::__NR_getrlimit",
        "getrlimit",
        2,
        "SyscallAvailability::Only(SyscallArchitecture::Riscv64)",
        "sys_getrlimit",
        None,
        "riscv64-rlimit-number",
    ),
    "RISCV64_SETRLIMIT": (
        "numbers::__NR_setrlimit",
        "setrlimit",
        2,
        "SyscallAvailability::Only(SyscallArchitecture::Riscv64)",
        "sys_setrlimit",
        None,
        "riscv64-rlimit-number",
    ),
    "LOONGARCH64_GETRLIMIT": (
        "LOONGARCH_LEGACY_GETRLIMIT",
        "getrlimit",
        2,
        "SyscallAvailability::Only(SyscallArchitecture::LoongArch64)",
        "sys_getrlimit",
        None,
        "loongarch64-legacy-rlimit-number",
    ),
    "LOONGARCH64_SETRLIMIT": (
        "LOONGARCH_LEGACY_SETRLIMIT",
        "setrlimit",
        2,
        "SyscallAvailability::Only(SyscallArchitecture::LoongArch64)",
        "sys_setrlimit",
        None,
        "loongarch64-legacy-rlimit-number",
    ),
}

TARGET_ENTRIES = {
    "riscv64": ("CLONE", "FSYNC", "FDATASYNC", "RISCV64_GETRLIMIT", "RISCV64_SETRLIMIT"),
    "loongarch64": (
        "CLONE",
        "FSYNC",
        "FDATASYNC",
        "LOONGARCH64_GETRLIMIT",
        "LOONGARCH64_SETRLIMIT",
    ),
}

TARGET_NUMBERS = {
    "riscv64": {
        "numbers::__NR_clone": 220,
        "numbers::__NR_fsync": 82,
        "numbers::__NR_fdatasync": 83,
        "numbers::__NR_getrlimit": 163,
        "numbers::__NR_setrlimit": 164,
    },
    "loongarch64": {
        "numbers::__NR_clone": 220,
        "numbers::__NR_fsync": 82,
        "numbers::__NR_fdatasync": 83,
        "LOONGARCH_LEGACY_GETRLIMIT": 163,
        "LOONGARCH_LEGACY_SETRLIMIT": 164,
    },
}


def read_required(root: Path, rel: Path, findings: list[str]) -> str:
    path = root / rel
    if not path.is_file():
        findings.append(f"{rel}: required Linux-boundary file is missing")
        return ""
    return path.read_text(encoding="utf-8", errors="ignore")


def dependency_section(manifest: str) -> str:
    match = re.search(r"(?ms)^\[dependencies\]\s*(.*?)(?=^\[|\Z)", manifest)
    return match.group(1) if match else ""


def all_dependency_sections(manifest: str) -> str:
    """Return normal and target-specific dependency table bodies."""
    pattern = r"(?ms)^\[(?:dependencies|target\.[^\]\r\n]+\.dependencies)\]\s*(.*?)(?=^\[|\Z)"
    return "\n".join(match.group(1) for match in re.finditer(pattern, manifest))


def scan_dependencies(root: Path) -> list[str]:
    findings: list[str] = []
    abi_manifest = read_required(root, ABI_CRATE_REL / "Cargo.toml", findings)
    linux_manifest = read_required(root, LINUX_CRATE_REL / "Cargo.toml", findings)
    shell_manifest = read_required(root, SHELL_MANIFEST_REL, findings)

    if 'name = "orays-linux-abi"' not in abi_manifest:
        findings.append(f"{ABI_CRATE_REL / 'Cargo.toml'}: package name must remain orays-linux-abi")
    if 'name = "orays-linux"' not in linux_manifest:
        findings.append(f"{LINUX_CRATE_REL / 'Cargo.toml'}: package name must remain orays-linux")

    abi_normal_deps = dependency_section(abi_manifest)
    linux_normal_deps = dependency_section(linux_manifest)
    shell_normal_deps = dependency_section(shell_manifest)
    abi_deps = all_dependency_sections(abi_manifest)
    linux_deps = all_dependency_sections(linux_manifest)
    shell_deps = all_dependency_sections(shell_manifest)
    if not re.search(r"(?m)^linux-raw-sys\s*=", abi_normal_deps):
        findings.append("orays-linux-abi: linux-raw-sys must remain its only implementation dependency")
    if len(re.findall(r"(?m)^[-A-Za-z0-9_]+\s*=", abi_deps)) != 1:
        findings.append("orays-linux-abi: unexpected dependency added")
    if not re.search(r"(?m)^orays-linux-abi\s*=\s*\{\s*workspace\s*=\s*true\s*\}", linux_normal_deps):
        findings.append("orays-linux: must depend only on workspace orays-linux-abi")
    if len(re.findall(r"(?m)^[-A-Za-z0-9_]+\s*=", linux_deps)) != 1:
        findings.append("orays-linux: implementation or external dependency added")
    if not re.search(r"(?m)^orays-linux\s*=\s*\{\s*workspace\s*=\s*true,\s*optional\s*=\s*true\s*\}", shell_normal_deps):
        findings.append("arceos-shell: missing optional orays-linux dependency")
    if re.search(r"(?m)^orays-linux-abi\s*=", shell_deps) or "dep:orays-linux-abi" in shell_manifest:
        findings.append("arceos-shell: direct ABI dependency bypasses orays-linux")
    if '"dep:orays-linux"' not in shell_manifest:
        findings.append("arceos-shell: uspace feature must propagate orays-linux")

    forbidden = (
        "arceos-shell",
        "arceos_posix_api",
        "axerrno",
        "axfs",
        "axhal",
        "axmm",
        "axtask",
    )
    for dependency in forbidden:
        if re.search(rf"(?m)^{re.escape(dependency)}\s*=", abi_deps + linux_deps):
            findings.append(f"new boundary crate reverse/implementation dependency: {dependency}")
    return findings


def scan_boundary_source(root: Path) -> list[str]:
    findings: list[str] = []
    for crate_rel in (ABI_CRATE_REL, LINUX_CRATE_REL):
        lib = read_required(root, crate_rel / "src/lib.rs", findings)
        if "#![no_std]" not in lib or "#![forbid(unsafe_code)]" not in lib:
            findings.append(f"{crate_rel}: no_std/forbid(unsafe_code) contract changed")
        src = root / crate_rel / "src"
        if not src.is_dir():
            continue
        for path in sorted(src.rglob("*.rs")):
            text = path.read_text(encoding="utf-8", errors="ignore")
            if re.search(r"\bunsafe\s*(?:\{|fn\b|extern\b|impl\b|trait\b)", text):
                findings.append(f"{path.relative_to(root)}: unsafe is forbidden in boundary crates")

    model = read_required(root, LINUX_CRATE_REL / "src/syscall.rs", findings)
    for token in (
        "pub struct SyscallNumber",
        "pub struct SyscallArgs",
        "pub enum SyscallArchitecture",
        "pub enum SyscallAvailability",
        "pub struct SyscallMeta",
        "MAX_SYSCALL_ARGUMENTS: usize = 6",
        "argument_count as usize <= MAX_SYSCALL_ARGUMENTS",
        "This type does not invoke a handler",
    ):
        if token not in model:
            findings.append(f"orays-linux syscall model: missing contract token {token!r}")

    abi_numbers = read_required(root, ABI_CRATE_REL / "src/syscall.rs", findings)
    for token in (
        "pub const LOONGARCH_LEGACY_GETRLIMIT: u32 = 163",
        "pub const LOONGARCH_LEGACY_SETRLIMIT: u32 = 164",
        "assert!(numbers::__NR_read == 63)",
        "assert!(numbers::__NR_write == 64)",
        "assert!(numbers::__NR_ppoll == 73)",
        "assert!(numbers::__NR_clone == 220)",
        "assert!(numbers::__NR_clone3 == 435)",
        "assert!(numbers::__NR_getrlimit == 163)",
        "assert!(numbers::__NR_setrlimit == 164)",
    ):
        if token not in abi_numbers:
            findings.append(f"orays-linux-abi syscall number guard missing or changed: {token}")

    abi_time = read_required(root, ABI_CRATE_REL / "src/time.rs", findings)
    for token in (
        "#[repr(C)]",
        "assert!(size_of::<Tms>() == 4 * size_of::<c_long>())",
        "assert!(align_of::<Tms>() == align_of::<c_long>())",
        "assert!(offset_of!(Tms, tms_cstime) == 3 * size_of::<c_long>())",
        "assert!(size_of::<RtcTime>() == 9 * size_of::<i32>())",
        "assert!(align_of::<RtcTime>() == align_of::<i32>())",
        "assert!(offset_of!(RtcTime, tm_isdst) == size_of::<[i32; 8]>())",
    ):
        if token not in abi_time:
            findings.append(f"orays-linux-abi time layout guard missing or changed: {token}")
    return findings


def count_symbol(text: str, symbol: str) -> int:
    return len(re.findall(rf"\b{re.escape(symbol)}\b", text))


def scan_user_copy_boundary(root: Path) -> list[str]:
    findings: list[str] = []
    uspace = root / USPACE_REL
    if not uspace.is_dir():
        return [f"{USPACE_REL}: missing uspace source tree"]
    rust_files = sorted(uspace.rglob("*.rs"))
    contents = {path: path.read_text(encoding="utf-8", errors="ignore") for path in rust_files}

    for symbol, (expected_occurrences, expected_files) in LEGACY_CALLER_INVENTORY.items():
        occurrences = sum(count_symbol(text, symbol) for text in contents.values())
        file_count = sum(count_symbol(text, symbol) > 0 for text in contents.values())
        if (occurrences, file_count) != (expected_occurrences, expected_files):
            findings.append(
                f"legacy user-copy caller inventory changed for {symbol}: "
                f"expected {expected_occurrences}/{expected_files}, got {occurrences}/{file_count}"
            )

    user_memory = read_required(root, USER_MEMORY_REL, findings)
    for symbol, expected_occurrences in RAW_BOUNDARY_INVENTORY.items():
        occurrences = count_symbol(user_memory, symbol)
        if occurrences != expected_occurrences:
            findings.append(
                f"{USER_MEMORY_REL}: raw boundary symbol {symbol} expected "
                f"{expected_occurrences} occurrences, got {occurrences}"
            )
    for function in (
        "typed_user_range",
        "validate_user_access_raw",
        "read_user_bytes_validated_raw",
        "read_user_bytes_into_raw",
        "write_user_bytes_raw",
    ):
        if not re.search(rf"(?m)^fn\s+{re.escape(function)}\b", user_memory):
            findings.append(f"{USER_MEMORY_REL}: {function} must remain a module-private low-level function")
    if not re.search(r"(?m)^struct\s+ProcessUserMemory\b", user_memory):
        findings.append(f"{USER_MEMORY_REL}: ProcessUserMemory must remain a private shell adapter")

    unsafe_count = len(re.findall(r"\bunsafe\s*\{", user_memory))
    if unsafe_count != 5:
        findings.append(f"{USER_MEMORY_REL}: expected 5 audited unsafe blocks, got {unsafe_count}")
    for fingerprint in UNSAFE_FINGERPRINTS:
        if fingerprint not in user_memory:
            findings.append(f"{USER_MEMORY_REL}: audited unsafe expression changed or moved: {fingerprint}")
    return findings


def split_top_level_arguments(arguments: str) -> list[str]:
    result: list[str] = []
    start = 0
    depths = {"(": 0, "[": 0, "{": 0}
    closing = {")": "(", "]": "[", "}": "{"}
    in_string = False
    escaped = False
    for index, char in enumerate(arguments):
        if in_string:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == '"':
                in_string = False
            continue
        if char == '"':
            in_string = True
        elif char in depths:
            depths[char] += 1
        elif char in closing:
            depths[closing[char]] -= 1
        elif char == "," and all(depth == 0 for depth in depths.values()):
            result.append(arguments[start:index].strip())
            start = index + 1
    tail = arguments[start:].strip()
    if tail:
        result.append(tail)
    return result


def syscall_meta_arguments(text: str, const_name: str) -> list[str] | None:
    match = re.search(
        rf"\bconst\s+{re.escape(const_name)}\s*:\s*SyscallMeta\s*=\s*SyscallMeta::new\s*\(",
        text,
    )
    if not match:
        return None
    start = match.end()
    depth = 1
    in_string = False
    escaped = False
    for index in range(start, len(text)):
        char = text[index]
        if in_string:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == '"':
                in_string = False
            continue
        if char == '"':
            in_string = True
        elif char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
            if depth == 0:
                return split_top_level_arguments(text[start:index])
    return None


def rust_string(expression: str) -> str | None:
    expression = expression.strip()
    if len(expression) >= 2 and expression[0] == expression[-1] == '"':
        return expression[1:-1]
    return None


def parse_metadata(text: str, findings: list[str]) -> dict[str, MetadataEntry]:
    entries: dict[str, MetadataEntry] = {}
    for const_name, expected in EXPECTED_METADATA.items():
        arguments = syscall_meta_arguments(text, const_name)
        if arguments is None:
            findings.append(f"{METADATA_REL}: missing metadata constant {const_name}")
            continue
        if len(arguments) != 7:
            findings.append(f"{METADATA_REL}: {const_name} must have seven metadata fields")
            continue
        number_match = re.fullmatch(r"SyscallNumber::new\((.*)\)", re.sub(r"\s+", "", arguments[0]))
        number_expr = number_match.group(1) if number_match else ""
        name = rust_string(arguments[1])
        handler = rust_string(arguments[4])
        alias_match = re.fullmatch(r"Some\((\".*\")\)", re.sub(r"\s+", "", arguments[5]))
        alias_of = rust_string(alias_match.group(1)) if alias_match else None
        if re.sub(r"\s+", "", arguments[5]) == "None":
            alias_of = None
        audit_id = rust_string(arguments[6])
        try:
            argument_count = int(arguments[2])
        except ValueError:
            argument_count = -1
        entry = MetadataEntry(
            const_name,
            number_expr,
            name or "",
            argument_count,
            re.sub(r"\s+", "", arguments[3]),
            handler or "",
            alias_of,
            audit_id or "",
        )
        entries[const_name] = entry
        actual = entry[1:]
        if actual != expected:
            findings.append(f"{METADATA_REL}: {const_name} declaration drifted: {actual!r}")
    return entries


def validate_registration_set(target: str, registrations: list[tuple[str, int, str, str | None]]) -> list[str]:
    findings: list[str] = []
    by_name = {name: (number, handler) for name, number, handler, _ in registrations}
    for name, _, handler, alias_of in registrations:
        if alias_of is None:
            continue
        canonical = by_name.get(alias_of)
        if canonical is None:
            findings.append(f"{target}: syscall alias {name} refers to missing {alias_of}")
        elif canonical[1] != handler:
            findings.append(f"{target}: syscall alias {name} and {alias_of} have different handlers")

    by_number: dict[int, list[tuple[str, str, str | None]]] = {}
    for name, number, handler, alias_of in registrations:
        by_number.setdefault(number, []).append((name, handler, alias_of))
    for number, duplicates in by_number.items():
        if len(duplicates) < 2:
            continue
        canonicals = [entry for entry in duplicates if entry[2] is None]
        if len(canonicals) != 1:
            findings.append(f"{target}: duplicate syscall number {number} lacks one explicit canonical entry")
            continue
        canonical_name, canonical_handler, _ = canonicals[0]
        for name, handler, alias_of in duplicates:
            if name == canonical_name:
                continue
            if alias_of != canonical_name or handler != canonical_handler:
                findings.append(
                    f"{target}: duplicate syscall number {number} for {name} is not an explicit handler alias of {canonical_name}"
                )
    return findings


def compact(text: str) -> str:
    return re.sub(r"\s+", "", text)


def scan_metadata_and_dispatcher(root: Path) -> list[str]:
    findings: list[str] = []
    metadata = read_required(root, METADATA_REL, findings)
    dispatcher = read_required(root, DISPATCH_REL, findings)
    entries = parse_metadata(metadata, findings)
    compact_metadata = compact(metadata)

    for token in (
        "useorays_linux::abi::syscall::numbers;",
        "#[used]staticSYSCALL_METADATA:&[SyscallMeta]=&[CLONE,FSYNC,FDATASYNC,",
        '#[cfg(target_arch="riscv64")]constRISCV64_GETRLIMIT:SyscallMeta',
        '#[cfg(target_arch="riscv64")]constRISCV64_SETRLIMIT:SyscallMeta',
        '#[cfg(target_arch="loongarch64")]constLOONGARCH64_GETRLIMIT:SyscallMeta',
        '#[cfg(target_arch="loongarch64")]constLOONGARCH64_SETRLIMIT:SyscallMeta',
        '#[cfg(target_arch="riscv64")]RISCV64_GETRLIMIT,',
        '#[cfg(target_arch="riscv64")]RISCV64_SETRLIMIT,',
        '#[cfg(target_arch="loongarch64")]LOONGARCH64_GETRLIMIT,',
        '#[cfg(target_arch="loongarch64")]LOONGARCH64_SETRLIMIT,',
    ):
        if token not in compact_metadata:
            findings.append(f"{METADATA_REL}: cfg/table registration token missing: {token}")
    poll_cfg = '#[cfg(not(any(target_arch="riscv64",target_arch="aarch64",target_arch="loongarch64")))]'
    if compact_metadata.count(poll_cfg) < 3:
        findings.append(f"{METADATA_REL}: poll exclusion cfg must cover list, entry, and table registration")

    for target, names in TARGET_ENTRIES.items():
        target_entries: list[tuple[str, int, str, str | None]] = []
        for const_name in names:
            entry = entries.get(const_name)
            if entry is None:
                continue
            number = TARGET_NUMBERS[target].get(entry.number_expr)
            if number is None:
                findings.append(f"{target}: unresolved metadata number expression {entry.number_expr}")
                continue
            target_entries.append((entry.name, number, entry.handler, entry.alias_of))
        findings.extend(validate_registration_set(target, target_entries))

    dispatch = compact(dispatcher)
    required_routes = (
        'general::__NR_fsync|general::__NR_fdatasync=>sys_fsync(process,tf.arg0()),',
        '#[cfg(target_arch="riscv64")]general::__NR_getrlimit=>sys_getrlimit(process,tf.arg0()asu32,tf.arg1()),',
        '#[cfg(target_arch="riscv64")]general::__NR_setrlimit=>sys_setrlimit(process,tf.arg0()asu32,tf.arg1()),',
        '#[cfg(target_arch="loongarch64")]LOONGARCH_LEGACY_GETRLIMIT=>sys_getrlimit(process,tf.arg0()asu32,tf.arg1()),',
        '#[cfg(target_arch="loongarch64")]LOONGARCH_LEGACY_SETRLIMIT=>sys_setrlimit(process,tf.arg0()asu32,tf.arg1()),',
        '#[cfg(not(target_arch="loongarch64"))]general::__NR_clone=>sys_clone(&ext.process,tf,tf.arg0(),tf.arg1(),tf.arg2(),tf.arg3(),tf.arg4(),),',
        '#[cfg(target_arch="loongarch64")]general::__NR_clone=>sys_clone(&ext.process,tf,tf.arg0(),tf.arg1(),tf.arg2(),tf.arg4(),tf.arg3(),),',
        'general::__NR_clone3=>sys_clone3(&ext.process,tf,tf.arg0(),tf.arg1()),',
        'constLOONGARCH_LEGACY_GETRLIMIT:u32=163;',
        'constLOONGARCH_LEGACY_SETRLIMIT:u32=164;',
    )
    for route in required_routes:
        if route not in dispatch:
            findings.append(f"{DISPATCH_REL}: architecture/handler route changed or missing: {route}")
    poll_route = (
        poll_cfg
        + 'general::__NR_poll=>sys_poll(process,tf.arg0(),tf.arg1(),tf.arg2()asi32),'
    )
    if poll_route not in dispatch:
        findings.append(f"{DISPATCH_REL}: poll architecture cfg or handler route changed")
    return findings


def scan(root: Path) -> list[str]:
    findings: list[str] = []
    findings.extend(scan_dependencies(root))
    findings.extend(scan_boundary_source(root))
    findings.extend(scan_user_copy_boundary(root))
    findings.extend(scan_metadata_and_dispatcher(root))
    return findings


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[2])
    args = parser.parse_args()
    findings = scan(args.root.resolve())
    if findings:
        print("Linux boundary check: FAIL")
        for finding in findings:
            print(f"- {finding}")
        return 1
    print("Linux boundary check: PASS (0 findings)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
