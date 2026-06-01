# milestone-02-stable606 blacklist change report

No blacklist changes were made in this preflight.

- No case was moved to blacklist.
- No SKIP/status0/full-sweep local TPASS evidence was counted as promotion evidence.
- The RV 80-case scout failures remain visible in `validation.md` and `promotion-candidates.md`.

## mknod mode errno follow-up

No blacklist changes were made for the `mknod03,mknod04,mknod09` follow-up. The cases were validated through targeted RV/LA x musl/glibc parser-clean runs and adjacent regression subsets; no SKIP/status0/blacklist evidence was counted.

## fchownat symlink nofollow follow-up

No blacklist changes were made for the `fchownat02` follow-up. The case was validated through targeted RV/LA x musl/glibc parser-clean runs and adjacent symlink/chown regression subsets; no SKIP/status0/blacklist evidence was counted.
