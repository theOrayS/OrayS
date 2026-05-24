# Remote marker regression check

Date: 2026-05-25
Status: **partial pass / final gate not run**

## Rule

Remote scorer markers must start at column 0. ANSI reset/color prefixes before `PASS LTP CASE` or `FAIL LTP CASE` are not allowed.

## Current raw-log marker scan

```text
checked_logs=9
bad_marker_lines=0
```

## Final decision

No marker regression was accepted, but final RV/LA stable350 marker-prefix checks were not run because stable350 was not reached. Aborted/untrusted `post-team-candidate*` logs remain excluded from promotion evidence regardless of marker shape.
