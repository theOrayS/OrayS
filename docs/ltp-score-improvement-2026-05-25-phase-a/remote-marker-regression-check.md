# Remote marker regression check

Date: 2026-05-25
Status: **partial pass / final gate not run**

## Rule

Remote scorer markers must start at column 0. ANSI reset/color prefixes before `PASS LTP CASE` or `FAIL LTP CASE` are not allowed.

## Fresh follow-up marker scan

```text
followup-rv-targeted-001.log: markers=16 bad=0
followup-la-targeted-004.log: markers=12 bad=0
TOTAL markers=28 bad=0
```

Evidence: `raw/followup-marker-prefix-check.txt`.

## Final decision

No marker regression was observed in the fresh follow-up targeted gates, but final RV/LA stable350 marker-prefix checks were not run because stable350 was not reached. Aborted/untrusted `post-team-candidate*` and `followup-la-targeted-001/002/003` outputs remain excluded from promotion evidence regardless of marker shape.
