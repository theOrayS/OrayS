# Baseline stable250 RV aggregate attempt note

A leader RV `LTP_CASES=stable` aggregate was started for baseline refresh, but it was terminated before completion after multiple worker lanes also launched `run-eval.sh rv` concurrently. The concurrent runs share `/tmp/arceos-sdcard-rv.run.qcow2`, so any overlapping QEMU evidence is treated as untrusted.

Observed before termination: early stable cases such as `access01`, `brk01`, `chdir01`, `clone01`, `close01`, `dup01`, `fcntl01`, `fcntl02`, `fork01`, `getpid01`, `mmap01`, `open01`, `pipe01`, `read01`, and `stat01` showed wrapper status `: 0` with no visible internal failures in the captured partial log. This is smoke-only, not a promotion gate.

Follow-up: leader must run QEMU/evaluator gates serially.
