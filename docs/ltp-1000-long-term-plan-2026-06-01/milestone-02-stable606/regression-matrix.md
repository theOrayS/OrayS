# milestone-02-stable606 regression matrix preflight

## Stable baseline

- Current stable count: 556 total / 556 unique / 0 duplicate.
- Stable list changed in this preflight: no.

## Socket errno fix regression set

Rationale: the code change affects `sys_socket_bridge` errno behavior. The protected adjacent subset combines the newly fixed `socket01` with existing stable socket-adjacent cases.

Cases:

- `socket01` (new candidate)
- `socket02` (existing stable)
- `socketpair02` (existing stable)
- `accept01` (existing stable)
- `listen01` (existing stable)

Evidence:

- RV: `target/ltp-1000-milestone-02-stable606/rv-socket-adjacent-postfix-20260601T160853Z.summary.txt`
  - 10 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA: `target/ltp-1000-milestone-02-stable606/la-socket-adjacent-postfix-20260601T160953Z.summary.txt`
  - 10 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Combined report: `target/ltp-1000-milestone-02-stable606/socket-adjacent-rv-la-postfix.promotion-candidates.txt`
  - All five cases clean across RV + LA x musl + glibc.

## Time lane rescout

Case: `nanosleep01`.

Evidence:

- RV: `target/ltp-1000-milestone-02-stable606/rv-nanosleep01-rescout-20260601T160605Z.summary.txt`
  - 2 PASS / 0 FAIL, no parser caveats.
- LA: `target/ltp-1000-milestone-02-stable606/la-nanosleep01-rescout-20260601T160721Z.summary.txt`
  - 2 PASS / 0 FAIL, no parser caveats.
- Combined report: `target/ltp-1000-milestone-02-stable606/nanosleep01-rv-la-rescout.promotion-candidates.txt`
  - `nanosleep01` clean across RV + LA x musl + glibc.

Caveat: the earlier grouped RV scout had one musl timing TFAIL, so this row needs grouped revalidation before final promotion.

## Not run yet

- Full stable606 RV + LA x musl + glibc gate.
- Broad stable regression beyond the socket-adjacent subset.
- LA full-sweep shard from G010.
