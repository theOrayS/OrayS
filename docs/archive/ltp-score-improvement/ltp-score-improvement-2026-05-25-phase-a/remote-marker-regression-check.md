# Remote Marker Regression Check

Date: 2026-05-25
Status: **PASS**

Remote scorer compatibility requires wrapper marker lines to start at column 0 without ANSI reset/color prefixes. Final stable350 logs were scanned after RV and LA final gates.

| Log | Marker scan | Result |
| --- | --- | --- |
| `raw/stable350-rv-final-002.log` | `raw/stable350-rv-final-002-marker-prefix.txt` | `TOTAL markers=700 bad=0` |
| `raw/stable350-la-final-002.log` | `raw/stable350-la-final-002-marker-prefix.txt` | `TOTAL markers=700 bad=0` |

Combined final evidence: 1400 marker lines scanned, 0 bad marker lines.

The earlier pre-replacement LA final log also had marker-prefix health, but failed semantically on `kill02`; that failure was documented and demoted rather than hidden.
