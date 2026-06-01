# milestone-02-stable606 blacklist change report

No blacklist changes were made in this preflight.

- No case was moved to blacklist.
- No SKIP/status0/full-sweep local TPASS evidence was counted as promotion evidence.
- The RV 80-case scout failures remain visible in `validation.md` and `promotion-candidates.md`.

## mknod mode errno follow-up

No blacklist changes were made for the `mknod03,mknod04,mknod09` follow-up. The cases were validated through targeted RV/LA x musl/glibc parser-clean runs and adjacent regression subsets; no SKIP/status0/blacklist evidence was counted.

## fchownat symlink nofollow follow-up

No blacklist changes were made for the `fchownat02` follow-up. The case was validated through targeted RV/LA x musl/glibc parser-clean runs and adjacent symlink/chown regression subsets; no SKIP/status0/blacklist evidence was counted.

## setrlimit04 busybox applet exec follow-up

No blacklist changes were made for the `setrlimit04` follow-up. The case was validated through targeted RV/LA x musl/glibc parser-clean runs and adjacent rlimit/exec/wait regression subsets; no SKIP/status0/blacklist evidence was counted.


## clock_gettime04 evidence-only follow-up

No blacklist changes were made for the `clock_gettime04` follow-up. The case was validated through targeted RV/LA x musl/glibc parser-clean runs and adjacent time regression subsets; no SKIP/status0/blacklist evidence was counted. The unrelated failed rows in the earlier mixed RV mm/time scout remain visible and non-countable.


## legacy clean-tail evidence-only follow-up

No blacklist changes were made for the `locktests`, `ltpServer`, and `stress` follow-up. The cases were validated through targeted RV/LA x musl/glibc parser-clean runs; no SKIP/status0/blacklist/full-sweep partial evidence was counted.

## Non-countable post-clock scouts

No failed row from `rv-light-process-scout-20260601T193756Z.log`, `rv-vfs-fd-remainder-scout-20260601T194216Z.log`, or `la-readlinkat02-rescout-20260601T194310Z.log` was moved to blacklist. The timeout, panic/trap, TFAIL/TBROK/TCONF, and LA-musl `readlinkat02` failure remain visible blockers and non-countable evidence.
