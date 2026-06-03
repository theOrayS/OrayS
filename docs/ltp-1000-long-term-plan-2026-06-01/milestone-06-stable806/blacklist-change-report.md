# milestone-06 blacklist change report

No blacklist was added, removed, or reinterpreted in this checkpoint.

The timerslack repair was validated with targeted inline `LTP_CASES='prctl08,prctl09'` RV/LA runs. This is not blacklist, SKIP, status0, or full-sweep evidence. `prctl08` and `prctl09` count only as candidate-pool evidence until the full next 50-case milestone gate is assembled.

Additional note: the `symlink03` repair also made no blacklist changes. The failed scratch-permission and tmp-mode-only logs are recorded as diagnostic/blocker evidence only; only the later RV/LA parser-clean parent-permission repair is counted as candidate-pool evidence.

Additional note: the `unlink09` FS_IOC inode-flag repair made no blacklist changes. The earlier `ENOTTY`/`TBROK` log is retained as diagnostic evidence only; only the later RV/LA parser-clean targeted and adjacent-regression summaries are counted as candidate-pool evidence.

Additional note: the `mkdir09` futex bitset repair made no blacklist changes. The earlier glibc futex abort log is retained as diagnostic evidence only; only the later RV/LA parser-clean targeted and futex/clone adjacent-regression summaries are counted as candidate-pool evidence.

## gettid02 futex/glibc follow-up

No blacklist was added, removed, or used for `gettid02`. The candidate evidence comes only from RV + LA × musl + glibc targeted parser-clean logs and the existing futex/clone adjacent regression boundary. No blacklist/SKIP/status0/full-sweep local TPASS evidence is counted.


## futex_wait_bitset01 and blocker scouts

No blacklist was added, removed, or used for `futex_wait_bitset01`, the RV futex wake/requeue scout, the RV clone scout, or the RV FD/vector-IO scout. The only new candidate evidence comes from RV + LA × musl + glibc parser-clean `futex_wait_bitset01`; wake/requeue, clone, and vector-IO rows with visible parser markers remain blocker-only. No blacklist/SKIP/status0/full-sweep local TPASS evidence is counted.


## fstat02/fstat02_64 and late blocker scouts

No blacklist was added, removed, or used for `fstat02`, `fstat02_64`, the RV VFS/MM scout, the LA `mmap05` follow-up, the RV process/exec/signal scout, the RV exec-only scout, or the RV FD/path small scout. The only new candidate evidence comes from RV + LA × musl + glibc parser-clean `fstat02` and `fstat02_64`; all rows with visible `TCONF`, `TFAIL`, `TBROK`, `ENOSYS`, panic, timeout, or partial state remain blocker-only. No blacklist/SKIP/status0/full-sweep local TPASS evidence is counted.


## sync/fd/io and xattr blocker scouts

No blacklist was added, removed, or used for the RV sync/fd/io or xattr scouts. Both runs are parser-visible blocker evidence only and contribute zero promotion candidates. No blacklist/SKIP/status0/full-sweep local TPASS evidence is counted.
