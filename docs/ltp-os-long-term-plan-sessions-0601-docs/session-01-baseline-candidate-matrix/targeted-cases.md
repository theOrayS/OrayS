# Session 1 Targeted Cases (40 cases)

单位：LTP case。该清单是下一批 targeted run/diagnosis 的输入，不是 PASS/promotion 证据。

1. `getitimer01` — time/select/signal — 0/4 clean; blocked/failing; blockers: TFAIL×4, ENOSYS×4, event-failures×4, status×4
2. `ppoll01` — time/select/signal — 0/4 clean; blocked/failing; blockers: TFAIL×4, event-failures×4, status×4
3. `select02` — time/select/signal — 0/4 clean; blocked/failing; blockers: TCONF×4, timeout×4, event-failures×4, status×4
4. `diotest4` — mmap/mm/resource — 0/4 clean; blocked/failing; blockers: TFAIL×4, TCONF×4, event-failures×4, status×4
5. `execve05` — futex/process/IPC — 0/4 clean; blocked/failing; blockers: TBROK×4, event-failures×4, status×4
6. `readlinkat02` — VFS/metadata/path — 3/4 clean; blocked in 1/4; blockers: TFAIL×1, event-failures×1, status×1
7. `epoll_create02` — time/select/signal — 0/4 clean; blocked/failing; blockers: TCONF×4, TFAIL×1, ENOSYS×1, event-failures×1, status×1
8. `nice04` — futex/process/IPC — 3/4 clean; blocked in 1/4; blockers: TFAIL×1, event-failures×1, status×1
9. `clone04` — futex/process/IPC — 3/4 clean; blocked in 1/4; blockers: TBROK×1, event-failures×1, status×1
10. `clock_gettime04` — time/select/signal — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
11. `clock_nanosleep02` — time/select/signal — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
12. `nanosleep01` — time/select/signal — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
13. `poll02` — time/select/signal — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
14. `pselect01` — time/select/signal — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
15. `pselect01_64` — time/select/signal — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
16. `settimeofday01` — time/select/signal — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
17. `time-schedule` — time/select/signal — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
18. `fpathconf01` — VFS/metadata/path — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
19. `pathconf01` — VFS/metadata/path — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
20. `rename14` — VFS/metadata/path — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
21. `mknod08` — VFS/metadata/path — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
22. `mknodat01` — VFS/metadata/path — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
23. `diotest1` — mmap/mm/resource — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
24. `diotest2` — mmap/mm/resource — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
25. `diotest3` — mmap/mm/resource — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
26. `diotest5` — mmap/mm/resource — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
27. `diotest6` — mmap/mm/resource — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
28. `mprotect05` — mmap/mm/resource — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
29. `mmap001` — mmap/mm/resource — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
30. `mmap15` — mmap/mm/resource — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
31. `mmap17` — mmap/mm/resource — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
32. `mmap19` — mmap/mm/resource — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
33. `futex_wait02` — futex/process/IPC — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
34. `futex_wait04` — futex/process/IPC — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
35. `futex_wake01` — futex/process/IPC — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
36. `kill02` — futex/process/IPC — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
37. `tkill01` — futex/process/IPC — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
38. `tkill02` — futex/process/IPC — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
39. `vfork01` — futex/process/IPC — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
40. `vfork02` — futex/process/IPC — 4/4 clean wrapper PASS (sweep evidence only); blockers: none
