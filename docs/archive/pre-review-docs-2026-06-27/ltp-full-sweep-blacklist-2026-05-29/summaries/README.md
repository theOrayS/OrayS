# Per-round summaries

Raw logs live under `target/ltp-full-sweep-blacklist-2026-05-29/raw/` and are intentionally not committed by default.  Each committed summary file here should list:

- exact command and environment
- raw log path
- parser command and exit status
- marker counts: RUN/PASS/FAIL/TIMEOUT/SKIP/incomplete
- inner markers: TPASS/TFAIL/TBROK/TCONF/ENOSYS plus panic/trap/OOM/hang notes
- blacklist additions and evidence


## Artifact index

- `manifest.json` is the canonical per-iteration artifact map: raw log path, raw hash when recorded, summary, compact JSON, marker audit, closure verdict, and blacklist delta.
- `*-summary.txt` is the parser-backed human-readable summary from `python3 scripts/ltp_summary.py`.
- `*-compact.json` is the durable compact gate summary used by `iterations.md` and `final-report.md`.
- `*-marker-audit.json` / `*-inline-marker-audit.json` are auxiliary wrapper-marker audits for duplicate case names and glued stdout+wrapper marker lines.
- `*.log.sha256` files verify retained local raw logs only; raw logs under `target/` are intentionally not committed and may be cleaned.
