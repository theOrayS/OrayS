# AI Slop Cleaner final gate scan

## Scope
- examples/shell/src/cmd.rs
- examples/shell/src/uspace/fd_table.rs
- examples/shell/src/uspace/linux_abi.rs
- examples/shell/src/uspace/memory_map.rs
- examples/shell/src/uspace/metadata.rs
- examples/shell/src/uspace/mod.rs
- examples/shell/src/uspace/process_lifecycle.rs
- examples/shell/src/uspace/synthetic_fs.rs
- examples/shell/src/uspace/syscall_dispatch.rs
- scripts/ltp_summary.py

## Fallback-like / fake-result keyword scan
examples/shell/src/uspace/memory_map.rs:328:    for page in PageIter4K::new(VirtAddr::from(addr), VirtAddr::from(end)).unwrap() {
examples/shell/src/uspace/memory_map.rs:369:                    PageIter4K::new(VirtAddr::from(prefault_start), VirtAddr::from(end)).unwrap()
scripts/ltp_summary.py:312:        f"- PASS LTP CASE: {data['pass_count']}",
examples/shell/src/uspace/process_lifecycle.rs:204:        .expect("user task must have a kernel stack");
examples/shell/src/uspace/process_lifecycle.rs:650:        .expect("user task must have a kernel stack");
examples/shell/src/cmd.rs:259:        let rwx = str::from_utf8(&rwx).unwrap();
examples/shell/src/cmd.rs:775:    // LTP cases create large temporary files and directories in ramfs-backed
examples/shell/src/cmd.rs:1429:                println!("PASS LTP CASE {case} : 0");
examples/shell/src/cmd.rs:1609:    std::io::stdout().flush().unwrap();

## LTP case-name branch scan
scripts/ltp_summary.py:23:TIMEOUT_CASE_RE = re.compile(r"\bTIMEOUT LTP CASE\s+(\S+)(?:\s*:\s*(-?\d+))?", re.IGNORECASE)
scripts/ltp_summary.py:30:    r"\b(TIMEOUT LTP CASE|timed out|timeout reached|timeout expired|killed after timeout)\b",
scripts/ltp_summary.py:312:        f"- PASS LTP CASE: {data['pass_count']}",
scripts/ltp_summary.py:313:        f"- FAIL LTP CASE: {data['fail_count']}",
scripts/ltp_summary.py:372:        lines.append("## FAIL LTP CASE")
examples/shell/src/cmd.rs:45:    "access01", "brk01", "chdir01", "clone01", "close01", "dup01", "fcntl02", "fork01", "getpid01",
examples/shell/src/cmd.rs:46:    "mmap01", "open01", "pipe01", "read01", "stat01", "wait401", "write01",
examples/shell/src/cmd.rs:52:    "chdir01",
examples/shell/src/cmd.rs:60:    "mmap01",
examples/shell/src/cmd.rs:163:const LTP_CASE_BATCHES: &[(&str, &[&str])] = &[
examples/shell/src/cmd.rs:172:const LTP_CASE_TIMEOUT_ENV: &str = "LTP_CASE_TIMEOUT_SECS";
examples/shell/src/cmd.rs:487:const LTP_CASE_TIMEOUT_SECS: u64 = 10;
examples/shell/src/cmd.rs:537:    LTP_CASE_BATCHES
examples/shell/src/cmd.rs:551:        .or(option_env!("LTP_CASES"))
examples/shell/src/cmd.rs:571:        let known = LTP_CASE_BATCHES
examples/shell/src/cmd.rs:580:            .map_err(|err| format!("failed to read LTP_CASES file '{path}': {err}"))?;
examples/shell/src/cmd.rs:583:            return Err(format!("LTP_CASES file '{path}' did not contain any cases"));
examples/shell/src/cmd.rs:600:        .or_else(|| option_env!("LTP_CASE_TIMEOUT_SECS").map(str::to_string))
examples/shell/src/cmd.rs:603:        .unwrap_or(LTP_CASE_TIMEOUT_SECS)
examples/shell/src/cmd.rs:961:            "{indent}({busybox_path} setsid {busybox_path} sh -c 'tools_dir=\"${{TESTSUITE_TOOLS_DIR:-${{0%/*}}}}\"; PATH=\"$tools_dir:${{0%/*}}:$PATH\"; ({busybox_path} sleep \"${{LTP_CASE_TIMEOUT_SECS:-10}}\"; {busybox_path} echo \"TIMEOUT LTP SCRIPT $0\"; {busybox_path} kill -KILL 0 >/dev/null 2>&1) & ltp_timer=$!; \"$0\"; ltp_status=$?; {busybox_path} kill -KILL $ltp_timer >/dev/null 2>&1; exit $ltp_status' \"$file\")"
examples/shell/src/cmd.rs:1141:        format!("{LTP_CASE_TIMEOUT_ENV}={}", ltp_case_timeout_secs()),
examples/shell/src/cmd.rs:1143:    if case == "chdir01" {
examples/shell/src/cmd.rs:1144:        // chdir01 needs an LTP test device only to mount a scratch filesystem.
examples/shell/src/cmd.rs:1399:            println!("FAIL LTP CASE {case} : -1");
examples/shell/src/cmd.rs:1429:                println!("PASS LTP CASE {case} : 0");
examples/shell/src/cmd.rs:1434:                println!("FAIL LTP CASE {case} : {status}");
examples/shell/src/cmd.rs:1435:                println!("TIMEOUT LTP CASE {case} after {timeout_secs}s");
examples/shell/src/cmd.rs:1440:                println!("FAIL LTP CASE {case} : {status}");
examples/shell/src/cmd.rs:1444:                println!("FAIL LTP CASE {case} : -1");
examples/shell/src/cmd.rs:1446:                    println!("TIMEOUT LTP CASE {case} after {timeout_secs}s");
