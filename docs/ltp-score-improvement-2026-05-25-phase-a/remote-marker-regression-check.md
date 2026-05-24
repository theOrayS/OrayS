# Remote marker regression check

Date: 2026-05-25
Status: **partial pass / final gate not run**

## Rule

Remote scorer markers must start at column 0. ANSI reset/color prefixes before `PASS LTP CASE` or `FAIL LTP CASE` are not allowed.

## Fresh follow-up marker scans

Original follow-up scan:

```text
followup-rv-targeted-001.log: markers=16 bad=0
followup-la-targeted-004.log: markers=12 bad=0
followup-la-sched_getscheduler02-afterfix-001.log: markers=2 bad=0
TOTAL markers=30 bad=0
```

Waitpid/pipe follow-up scan:

```text
followup-rv-waitpid01-maskrestore-001.log: markers=2 bad=0
followup-la-waitpid01-maskrestore-001.log: markers=2 bad=0
followup-rv-waitpid-signal-guard-001.log: markers=16 bad=0
followup-la-waitpid-signal-guard-001.log: markers=16 bad=0
followup-rv-pipe2_02-resource-prestage-001.log: markers=2 bad=0
followup-rv-pipe2_02-resource-prestage-002.log: markers=2 bad=0
followup-rv-pipe2_02-resource-prestage-003.log: markers=2 bad=0
TOTAL markers=42 bad=0
```

Evidence: `raw/followup-marker-prefix-check.txt` and `raw/followup-waitpid-marker-prefix-check.txt`.

## Final decision

No marker-prefix regression was observed in the fresh follow-up targeted/guard gates, but final RV/LA stable350 marker-prefix checks were not run because stable350 was not reached. Aborted/untrusted `post-team-candidate*` and `followup-la-targeted-001/002/003` outputs remain excluded from promotion evidence regardless of marker shape.
