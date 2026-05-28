# worker4 aborted run note

Status: ABORTED / UNTRUSTED.

Reason: leader guardrail at 2026-05-24T01:27:02Z stated parallel run-eval/QEMU evidence is invalid because shared /tmp/arceos-sdcard-*.run.qcow2 names make concurrent QEMU runs unsafe.

Action: worker-4 stopped its launched run and will not use this log for promotion. Leader must re-run any QEMU gate serially if these candidates are considered.

Cases attempted before abort: link02,rename01,unlink05,mkdir02,statfs01,statvfs01,fstatfs01,readlinkat01,readlinkat02,symlinkat01,ftruncate03,statfs02
