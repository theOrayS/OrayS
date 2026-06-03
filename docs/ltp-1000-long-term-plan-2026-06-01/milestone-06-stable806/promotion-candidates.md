# milestone-06 promotion candidates so far

These cases are candidate-pool evidence only. They are not yet promoted into `LTP_STABLE_CASES` because milestone-06 still needs the full next 50-case cohort plus adjacent stable regression evidence.

| Case | Evidence | Status |
| --- | --- | --- |
| `prctl08` | RV + LA × musl + glibc targeted parser-clean after timerslack repair | candidate pool |
| `prctl09` | RV + LA × musl + glibc targeted parser-clean after timerslack repair | candidate pool |
| `utsname02` | RV + LA × musl + glibc targeted parser-clean after shared default UTS hostname repair | candidate pool |

Evidence artifacts:

- RV final log: `target/ltp-1000-milestone-06-stable806/rv-prctl08-09-after-timerslack-default-inherit-20260603T183244+0800.log`
- RV summary: `target/ltp-1000-milestone-06-stable806/rv-prctl08-09-after-timerslack-default-inherit-20260603T183244+0800.summary.txt`
- RV candidate report: `target/ltp-1000-milestone-06-stable806/rv-prctl08-09-after-timerslack-default-inherit-20260603T183244+0800.promotion-candidates.txt`
- LA final log: `target/ltp-1000-milestone-06-stable806/la-prctl08-09-after-timerslack-default-inherit-20260603T183438+0800.log`
- LA summary: `target/ltp-1000-milestone-06-stable806/la-prctl08-09-after-timerslack-default-inherit-20260603T183438+0800.summary.txt`
- LA candidate report: `target/ltp-1000-milestone-06-stable806/la-prctl08-09-after-timerslack-default-inherit-20260603T183438+0800.promotion-candidates.txt`


Additional UTS evidence artifacts:

- RV UTS targeted log: `target/ltp-1000-milestone-06-stable806/rv-utsname-shared-hostname-20260603T190100+0800.log`
- RV UTS summary: `target/ltp-1000-milestone-06-stable806/rv-utsname-shared-hostname-20260603T190100+0800.summary.txt`
- LA UTS targeted log: `target/ltp-1000-milestone-06-stable806/la-utsname-shared-hostname-20260603T190234+0800.log`
- LA UTS summary: `target/ltp-1000-milestone-06-stable806/la-utsname-shared-hostname-20260603T190234+0800.summary.txt`
- Combined RV+LA UTS candidate report: `target/ltp-1000-milestone-06-stable806/rv-la-utsname-shared-hostname-20260603T190408+0800.promotion-candidates.txt`
- RV UTS adjacent regression summary: `target/ltp-1000-milestone-06-stable806/rv-utsname-adjacent-regression-20260603T190435+0800.summary.txt`
- LA UTS adjacent regression summary: `target/ltp-1000-milestone-06-stable806/la-utsname-adjacent-regression-20260603T190701+0800.summary.txt`

Note: `utsname01` is four-combo clean in the targeted UTS run but is already present in `LTP_STABLE_CASES`, so it is counted as adjacent regression evidence, not as a new unique candidate.
