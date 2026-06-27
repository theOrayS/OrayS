Architectural Status: CLEAR

This supersedes the earlier WATCH. The previous concern was that the regression barrier was only text/regex-based. The updated guard extracts the maybe_run_official_tests body, checks libctest dispatch calls run_libctest_suite(&suite_dir, &cwd), and rejects nearby structural variants that would reintroduce hidden skips: conditional continue/skip, fixed /musl vs /glibc suite-dir policy, and score-only/musl-only policy.

Regression coverage: scripts/test_g005_runner_parser.py adds a negative nested libctest suite-dir skip fixture and asserts the guard fails.

Conclusion: no remaining architectural blocker in the current diff/evidence.
