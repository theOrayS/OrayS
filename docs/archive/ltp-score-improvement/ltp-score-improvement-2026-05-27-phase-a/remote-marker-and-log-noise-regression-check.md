# Remote marker and log-noise regression check

Date: 2026-05-27
Scope: stable460 final RV/LA aggregate gates.

## Verdict

**Pass.** LTP marker lines remain parser-safe: final RV and LA logs both have zero bad marker-prefix lines.

## Evidence

| Log | Marker evidence | Result |
| --- | --- | --- |
| `raw/stable460-rv-final-gate-002.log` | `raw/stable460-rv-final-gate-002-marker-prefix.txt` | `bad_marker_prefix_lines=0` |
| `raw/stable460-la-final-gate-002.log` | `raw/stable460-la-final-gate-002-marker-prefix.txt` | `bad_marker_prefix_lines=0` |

## Noise counts

| Log | `AxError::NotADirectory` | `AxError::IsADirectory` | `AxError::AlreadyExists` | `axfs::fops:297 [AxError::NotADirectory]` | `axfs_ramfs::file:69` | Impact |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| RV final 002 | 12 | 0 | 0 | 0 | 12 | disclosed; no parser impact |
| LA final 002 | 12 | 0 | 0 | 0 | 12 | disclosed; no parser impact |
| LA failed first attempt | 10 | 0 | 0 | 0 | 10 | negative evidence only; failure was `kill02` TBROK, not marker/noise |

Compared with the stable413 handoff note (`axfs_ramfs::file:69` NotADirectory: 22 per arch), final stable460 aggregate noise is lower and still does not pollute LTP wrapper marker prefixes.

## Remote-output guardrail

No code path changed the current remote-compatible marker wire. ANSI/color prefixes remain outside wrapper marker starts, and the final parser evidence is from `scripts/ltp_summary.py` summaries rather than manual log inspection.
