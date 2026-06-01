# RV/LA arch-specific LTP full-sweep blacklist closure report (2026-05-30)

## Verdict

Both architectures now have a parser-backed closed `LTP_CASES=blacklist` full sweep using separate architecture overlays:

- **RV closed** in `rv-arch002` with `blacklist-common.txt` + `blacklist-rv.txt`.
- **LA closed** in `la-arch012` with `blacklist-common.txt` + `blacklist-la.txt`.
- Blacklisted/skipped cases are **exclusions only**.  They are not counted as PASS and are not stable-promotion evidence.
- Ordinary `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, wrong errno, and wrapper failures remain real failures unless they were severe sweep blockers documented in a blacklist entry.

## Repo / runner context

- Branch: `exp/ltp-full-sweep-blacklist`
- HEAD used for closure logs: `cf34bc3fbf85`
- Stable list count at this HEAD (`examples/shell/src/cmd.rs::LTP_STABLE_CASES`): 460
- Mode: `LTP_CASES=blacklist`, implemented by enumerating all guest LTP binaries and subtracting default + supplemental blacklists.
- Raw logs are intentionally kept outside git under `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/`.

## Code changes for arch-specific blacklist overlays

- `examples/shell/src/cmd.rs`
  - Keeps the original common guest blacklist paths `/ltp_blacklist.txt` and `/tmp/ltp_blacklist.txt`.
  - Adds RV-only guest file paths such as `/ltp_blacklist_rv.txt`, `/tmp/ltp_blacklist_riscv64.txt`.
  - Adds LA-only guest file paths such as `/ltp_blacklist_la.txt`, `/tmp/ltp_blacklist_loongarch64.txt`.
  - Adds build-time env overlays `LTP_BLACKLIST_RV` / `LTP_BLACKLIST_RISCV64` and `LTP_BLACKLIST_LA` / `LTP_BLACKLIST_LOONGARCH64` selected by `AX_ARCH`.
- `run-eval.sh`
  - Adds optional host file overlays:
    - common: `LTP_BLACKLIST_FILE`, `LTP_BLACKLIST_COMMON_FILE`
    - RV: `LTP_BLACKLIST_RV_FILE`
    - LA: `LTP_BLACKLIST_LA_FILE`
  - These remain opt-in for local `./run-eval.sh rv|la`; if unset, local evaluator behavior is unchanged.
- `Makefile`
  - `make` / `make all` remains the explicit remote-submission entry point.
  - As of the 2026-06-01 score-safe superset follow-up, the default remote-submission mode is `REMOTE_LTP_CASES=stable-plus-blacklist`.
  - That mode runs `LTP_STABLE_CASES` first, then appends extra guest LTP binaries after removing stable duplicates and supplemental blacklist entries.
  - Pure stable remains available with `make all REMOTE_LTP_CASES=stable`; pure full-sweep blacklist remains available with `make all REMOTE_LTP_CASES=blacklist`.
  - In blacklist-consuming modes, it builds `kernel-rv` with `blacklist-common.txt` + `blacklist-rv.txt`, and `kernel-la` with `blacklist-common.txt` + `blacklist-la.txt`.
  - It still uses the remote LoongArch platform config `configs/remote-eval/axplat-loongarch64-qemu-virt.toml`; local `kernel-la`, `run-la`, and `./run-eval.sh la` remain on the local platform config path unless their own env vars are provided.


## Operator-facing blacklist contract

Authoritative local experiment entry point:

```bash
LTP_CASES=blacklist \
LTP_BLACKLIST_COMMON_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-common.txt \
LTP_BLACKLIST_RV_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-rv.txt \
./run-eval.sh rv

LTP_CASES=blacklist \
LTP_BLACKLIST_COMMON_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-common.txt \
LTP_BLACKLIST_LA_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-la.txt \
./run-eval.sh la
```

Authoritative online-score submission entry point for this branch:

```bash
make all
```

Remote `make all` now defaults to a stable-first score-safe superset:

```text
REMOTE_LTP_CASES=stable-plus-blacklist
```

This is intentionally online-friendly: it preserves the known whitelist ordering/coverage before trying additional full-sweep cases.  The resulting case order is:

```text
LTP_STABLE_CASES + (all guest LTP binaries - LTP_STABLE_CASES - blacklist)
```

To run the old stable-only submission explicitly:

```bash
make all REMOTE_LTP_CASES=stable
```

To run the closed pure blacklist full-sweep experiment explicitly:

```bash
make all REMOTE_LTP_CASES=blacklist
```

When `REMOTE_LTP_CASES` is a blacklist-consuming mode (`stable-plus-blacklist`, `score-blacklist`, `stable-plus-all-minus-blacklist`, `blacklist`, `all-minus-blacklist`, or `sweep:blacklist`), `make all` compiles the supplemental blacklist overlays through these overridable make variables:

```text
REMOTE_LTP_BLACKLIST_COMMON_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-common.txt
REMOTE_LTP_BLACKLIST_RV_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-rv.txt
REMOTE_LTP_BLACKLIST_LA_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-la.txt
```

This means submitting this `exp/ltp-full-sweep-blacklist` branch to the online evaluator should exercise the stable whitelist first after the evaluator runs its normal `make` / `make all` build, then continue into non-stable extras that are not in the active blacklist.  Blacklisted/skipped extras still do not count as PASS or promotion proof.
The blacklist is intentionally applied only to the extra suffix in the default mode: stable whitelist cases still run first even if an arch-specific experimental blacklist contains their names.

Precedence / merge order inside the guest runner is:

1. built-in `LTP_SWEEP_DEFAULT_BLACKLIST_CASES`;
2. `LTP_BLACKLIST` build-time text, which `run-eval.sh` composes from `LTP_BLACKLIST_FILE`, `LTP_BLACKLIST_COMMON_FILE`, and the selected arch file variable for local runs, and which `make all` composes from `REMOTE_LTP_BLACKLIST_COMMON_FILE` only for explicit blacklist-mode remote-submission builds;
3. arch-specific build-time text (`LTP_BLACKLIST_RV` / `LTP_BLACKLIST_RISCV64` or `LTP_BLACKLIST_LA` / `LTP_BLACKLIST_LOONGARCH64`);
4. optional guest common files (`/ltp_blacklist.txt`, `/tmp/ltp_blacklist.txt`);
5. optional guest arch files (`/ltp_blacklist_rv.txt`, `/tmp/ltp_blacklist_riscv64.txt`, `/ltp_blacklist_la.txt`, `/tmp/ltp_blacklist_loongarch64.txt`, and dash-name aliases).

The host-side file variables above are the recommended local operator interface for blacklist experiments.  `make all` is the recommended online-submission interface and defaults to `stable-plus-blacklist`; blacklist file variables are consumed there for blacklist-consuming modes.  The arch-specific build-time env knobs and guest filesystem paths are advanced compatibility hooks for prepared images or external runners; missing optional guest files intentionally no-op and must not be treated as evidence that a blacklist was applied.  Every closure report must record the concrete command and skipped count, as this report does.

## Blacklist sources and counts

| Source | Count | Scope | Notes |
| --- | ---: | --- | --- |
| built-in `LTP_SWEEP_DEFAULT_BLACKLIST_CASES` | default runner list | common | unchanged default experimental sweep guardrail list in source |
| `blacklist-common.txt` | 5 | common | prior severe RV convergence blockers from 2026-05-29 (`pthserv`, `oom01`, `shmat1`, `accept02`, `mincore03`) |
| `blacklist-rv.txt` | 1 | RV-only | `kill10`, RV allocator panic evidence from `rv-arch001` |
| `blacklist-la.txt` | 376 | LA-only | `creat07`, LA network stress family, and later LA-only panic/hang/resource blockers |

The LA list is intentionally not copied into RV.  LA-only blockers such as the network stress family, `fsync02`, `pth_str01`, `write01`, futex/nice hangs, etc. remain arch-scoped unless RV evidence appears.

## Final closed sweep evidence

### RV: `rv-arch002`

- Command:
  ```bash
  LTP_CASES=blacklist \
  LTP_BLACKLIST_COMMON_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-common.txt \
  LTP_BLACKLIST_RV_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-rv.txt \
  ./run-eval.sh rv
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/rv-arch002.log`
- Raw log sha256: `70a3f9cab0c5c7a9a2743f168cabfd2eaafb7c01565b1630a02f9604aca5f096`
- Parser summary: `summaries/rv-arch002-summary.md` / `summaries/rv-arch002-summary.json`
- Marker audit: `summaries/rv-arch002-marker-audit.json`
- Selection:
  - musl: `skipped=41 (2327 cases, timeout 15s)`
  - glibc: `skipped=44 (2331 cases, timeout 15s)`
- Closure: `run-eval status=0`, raw RUN markers=4658, terminal markers=4658, incomplete=0.
- Wrapper/parser counts: PASS=1204, FAIL=3453, TIMEOUT=55, SKIP=0.
- Internal LTP signals: TBROK=1043, TCONF=2663, TFAIL=4058, ENOSYS/not-implemented matches=1280.
- Severe errors: panic/trap=0, strict resource failures=0.
- Suite summaries:
  - `ltp-musl`: passed=598 failed=1729 timed_out=27
  - `ltp-glibc`: passed=606 failed=1725 timed_out=28

### LA: `la-arch012`

- Command:
  ```bash
  LTP_CASES=blacklist \
  LTP_BLACKLIST_COMMON_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-common.txt \
  LTP_BLACKLIST_LA_FILE=docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-la.txt \
  ./run-eval.sh la
  ```
- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch012.log`
- Raw log sha256: `41a5fdbba4a56a4ea76a168d2c9c6aa1e86a572d09de734a43a5365ec52c84df`
- Parser summary: `summaries/la-arch012-summary.md` / `summaries/la-arch012-summary.json`
- Marker audit: `summaries/la-arch012-marker-audit.json`
- Selection:
  - musl: `skipped=416 (1952 cases, timeout 15s)`
  - glibc: `skipped=419 (1956 cases, timeout 15s)`
- Closure: `run-eval status=0`, raw RUN markers=3908, terminal markers=3908, incomplete=0.
- Wrapper/parser counts: PASS=1207, FAIL=2698, TIMEOUT=53, SKIP=0.
- Internal LTP signals: TBROK=1031, TCONF=1936, TFAIL=4041, ENOSYS/not-implemented matches=1279.
- Severe errors: panic/trap=0, strict resource failures=0.
- Suite summaries:
  - `ltp-musl`: passed=602 failed=1350 timed_out=25
  - `ltp-glibc`: passed=605 failed=1351 timed_out=28

## RV/LA differences

| Metric | RV closed run | LA closed run | Interpretation |
| --- | ---: | ---: | --- |
| RUN markers | 4658 | 3908 | LA skips many more arch-only severe blockers, especially network stress cases. |
| Parser PASS | 1204 | 1207 | Similar high-level PASS count despite different skip surface. |
| Parser FAIL | 3453 | 2698 | RV runs more network/stress cases, so it exposes more ordinary failures. |
| TIMEOUT | 55 | 53 | Similar timeout scale; these are normal terminal markers, not blacklist reasons. |
| Incomplete | 0 | 0 | Both closed. |
| Panic/trap | 0 | 0 | Both final runs are clean of severe kernel stop conditions. |
| Strict resource failures | 0 | 0 | No fork/resource-polluted final closure. |

Important: LA's lower FAIL count is mostly because its LA-only blacklist excludes severe network/resource blockers.  It is not evidence that LA is semantically more correct.

## High-yield targeted fix candidates

These are candidates for later real fixes.  They are not promotion claims.

1. **High TPASS density but final wrapper failure on both RV and LA**
   - `getitimer01`: TPASS=32 on each arch but still bad terminal outcome on both closed runs; likely high-yield timer/itimer semantics work.
   - `ppoll01`: TPASS=32 on each arch with remaining failures; poll/signal timeout semantics can fan out to other poll/select cases.
   - `diotest4`: TPASS=28 on each arch with remaining failures/TCONF; useful FS/direct-IO behavior candidate.
   - `select02`: TPASS=23 RV / 21 LA with wrapper failure; select/readiness semantics likely reusable.
   - `execve05`: TPASS=16 on each arch but TBROK/failure remains; process/exec boundary fix candidate.
2. **Fan-out syscall/mm/fs families**
   - `fcntl*`/`fcntl*_64`, `select`/`ppoll`/`poll`, `futex_*`, `mmap`/`mincore`/`move_pages`, `statx`/`getxattr`, and `shm*`/`sem*`/`msg*` families dominate ordinary failures and timeouts.
   - `write01`, `fsync02`, `lftest`, `mmstress` are LA blacklist entries because they currently panic/hang in full sweep; future real fixes should target allocator pressure and late-order memory retention before removing them.
3. **RV/LA divergence cases from final closed logs**
   - RV clean / LA bad candidate: `readlinkat02` (RV pass_code0=2, LA pass_code0=1 + 1 bad terminal outcome, TPASS RV/LA=12/11).
   - LA clean / RV bad candidates: `atof01`, `epoll_create02`, `nice04`, `fptest01`, `fptest02`, `clone04` each have one RV-bad vs LA-clean aggregate outcome in the final logs.
4. **Blacklist removal candidates after real fixes**
   - RV: `kill10` after RV targeted/full sweep produces a normal marker without allocator panic or resource pollution.
   - LA: network stress family after LA full/targeted network sweep closes without fork/resource pollution; `fsync02`, `lftest`, `mmstress`, `write01` after allocator panics stop; `pth_str01`, `dirtyc0w`, `futex_wait01`, `futex_wait05`, `nice05`, `creat07` after normal PASS/FAIL/TIMEOUT closure markers appear.

## Online evaluator compatibility

- The default invocation `./run-eval.sh rv` / `./run-eval.sh la` still works with no new required environment variables.
- Remote `make` / `make all` defaults to `REMOTE_LTP_CASES=stable-plus-blacklist` for online scoring: stable whitelist first, then non-stable all-minus-blacklist extras.
- New blacklist file variables are optional for local experiments and required by the remote default because it consumes the checked-in common/RV/LA blacklist overlays for the extra suffix.
- Arch-specific guest file paths are optional; absence is ignored.
- No testsuite/evaluator scripts were modified to fake PASS, suppress TCONF/TBROK/TFAIL, or hide panic/trap/timeout.

## Final local validation

- `bash -n run-eval.sh` passed.
- `rustfmt +nightly-2025-05-20 --edition 2024 --check examples/shell/src/cmd.rs` passed.
- `git diff --check -- examples/shell/src/cmd.rs run-eval.sh docs/ltp-full-sweep-blacklist-2026-05-30-arch` passed.
- `python3 -m json.tool docs/ltp-full-sweep-blacklist-2026-05-30-arch/final-quality-gate.json` passed.
- 2026-06-01 score-safe superset follow-up: `make -n all` shows default `LTP_CASES="stable-plus-blacklist"` plus RV/LA blacklist env injection; `make -n all REMOTE_LTP_CASES=stable` shows stable-only with no blacklist env injection; `make -n all REMOTE_LTP_CASES=blacklist` still shows pure blacklist env injection.
- 2026-06-01 score-safe superset follow-up: `make all` completed with default `LTP_CASES="stable-plus-blacklist"` and regenerated `kernel-rv` / `kernel-la`; disk stayed at `/` and `/root` 59G size, 23G used, 34G available, 41% used before and after.
- Closure assertion over `rv-arch002-summary.json` and `la-arch012-summary.json` passed: closed=true, run_eval_status=0, incomplete=0, panic/trap/resource=0.
- No live `qemu-system-*`, `run-eval.sh rv/la`, or `make run-rv/run-la` processes after final sweeps.
- Disk after runs: `/` and `/root` both 59G size, 23G used, 34G available, 40% used.

## Final independent review gate

- Code review lane: **APPROVE**, 0 issues in scoped files.
- AI slop / cleanup lane: **APPROVE/CLEAR**, no masking fallback slop; optional blacklist ingress paths are grounded compatibility hooks.
- Architecture lane: **CLEAR** after documenting the operator-facing blacklist contract and precedence in this report.
- Durable review artifact: `final-review-gate.md`.
