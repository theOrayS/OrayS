# Task 2 targeted validation baseline

## Generated at
2026-05-21T18:37:50+08:00

## RV LTP summary: output_rv.md
# LTP summary: `output_rv.md`

- PASS LTP CASE: 126
- FAIL LTP CASE: 0
- Internal TFAIL/TBROK/TCONF: 4 ({'TCONF': 4})
- timeout matches: 10
- ENOSYS/not implemented matches: 0
- panic/trap matches: 0

## Suite summaries
- ltp-musl: 63 passed, 0 failed
- ltp-glibc: 63 passed, 0 failed

## Case matrix
| Case | Arch | Libc | Group | Status | Code | Runtime ms | Free frames before | Free frames after cleanup | Free frames delta | TFAIL | TBROK | TCONF | timeout | ENOSYS | panic/trap |
| --- | --- | --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| access01 | rv | glibc | ltp-glibc | PASS | 0 | 4679 | 240120 | 239399 | -721 | 0 | 0 | 0 | 0 | 0 | 0 |
| access01 | rv | musl | ltp-musl | PASS | 0 | 4111 | 250657 | 249936 | -721 | 0 | 0 | 0 | 0 | 0 | 0 |
| access03 | rv | glibc | ltp-glibc | PASS | 0 | 1203 | 189300 | 189258 | -42 | 0 | 0 | 0 | 0 | 0 | 0 |
| access03 | rv | musl | ltp-musl | PASS | 0 | 964 | 248991 | 248949 | -42 | 0 | 0 | 0 | 0 | 0 | 0 |
| alarm02 | rv | glibc | ltp-glibc | PASS | 0 | 1181 | 188880 | 188866 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| alarm02 | rv | musl | ltp-musl | PASS | 0 | 719 | 240379 | 240365 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| alarm03 | rv | glibc | ltp-glibc | PASS | 0 | 1209 | 188866 | 188845 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| alarm03 | rv | musl | ltp-musl | PASS | 0 | 872 | 240365 | 240344 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| brk01 | rv | glibc | ltp-glibc | PASS | 0 | 1147 | 239399 | 239378 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| brk01 | rv | musl | ltp-musl | PASS | 0 | 1012 | 249936 | 249915 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| chdir01 | rv | glibc | ltp-glibc | PASS | 0 | 1161 | 239378 | 239364 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| chdir01 | rv | musl | ltp-musl | PASS | 0 | 1303 | 249915 | 249901 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| chmod01 | rv | glibc | ltp-glibc | PASS | 0 | 1083 | 188985 | 188964 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| chmod01 | rv | musl | ltp-musl | PASS | 0 | 877 | 240484 | 240463 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| clock_gettime02 | rv | glibc | ltp-glibc | PASS | 0 | 1166 | 188845 | 188831 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| clock_gettime02 | rv | musl | ltp-musl | PASS | 0 | 867 | 240344 | 240330 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| clone01 | rv | glibc | ltp-glibc | PASS | 0 | 1085 | 239364 | 239341 | -23 | 0 | 0 | 0 | 0 | 0 | 0 |
| clone01 | rv | musl | ltp-musl | PASS | 0 | 877 | 249901 | 249880 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| close01 | rv | glibc | ltp-glibc | PASS | 0 | 1128 | 239341 | 239327 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| close01 | rv | musl | ltp-musl | PASS | 0 | 848 | 249880 | 249866 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| close02 | rv | glibc | ltp-glibc | PASS | 0 | 1102 | 189258 | 189244 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| close02 | rv | musl | ltp-musl | PASS | 0 | 770 | 248949 | 248935 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| creat01 | rv | glibc | ltp-glibc | PASS | 0 | 1091 | 189069 | 189055 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| creat01 | rv | musl | ltp-musl | PASS | 0 | 791 | 240568 | 240554 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| creat03 | rv | glibc | ltp-glibc | PASS | 0 | 1105 | 189055 | 189041 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| creat03 | rv | musl | ltp-musl | PASS | 0 | 787 | 240554 | 240540 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| dup01 | rv | glibc | ltp-glibc | PASS | 0 | 1122 | 239327 | 239313 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| dup01 | rv | musl | ltp-musl | PASS | 0 | 817 | 249866 | 249852 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| dup02 | rv | glibc | ltp-glibc | PASS | 0 | 1126 | 189244 | 189230 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| dup02 | rv | musl | ltp-musl | PASS | 0 | 887 | 248935 | 248921 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| exit01 | rv | glibc | ltp-glibc | PASS | 0 | 1126 | 188754 | 188740 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| exit01 | rv | musl | ltp-musl | PASS | 0 | 848 | 240253 | 240239 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| exit02 | rv | glibc | ltp-glibc | PASS | 0 | 1185 | 188740 | 188719 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| exit02 | rv | musl | ltp-musl | PASS | 0 | 782 | 240239 | 240218 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| exit_group01 | rv | glibc | ltp-glibc | PASS | 0 | 1297 | 188719 | 188696 | -23 | 0 | 0 | 0 | 0 | 0 | 0 |
| exit_group01 | rv | musl | ltp-musl | PASS | 0 | 1005 | 240218 | 240197 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| fchmod01 | rv | glibc | ltp-glibc | PASS | 0 | 1124 | 188964 | 188950 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| fchmod01 | rv | musl | ltp-musl | PASS | 0 | 819 | 240463 | 240449 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| fcntl01 | rv | glibc | ltp-glibc | PASS | 0 | 1068 | 239313 | 239306 | -7 | 0 | 0 | 0 | 0 | 0 | 0 |
| fcntl01 | rv | musl | ltp-musl | PASS | 0 | 970 | 249852 | 249845 | -7 | 0 | 0 | 0 | 0 | 0 | 0 |
| fcntl02 | rv | glibc | ltp-glibc | PASS | 0 | 1125 | 239306 | 239292 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| fcntl02 | rv | musl | ltp-musl | PASS | 0 | 936 | 249845 | 249831 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| fcntl03 | rv | glibc | ltp-glibc | PASS | 0 | 1059 | 189230 | 189216 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| fcntl03 | rv | musl | ltp-musl | PASS | 0 | 823 | 248921 | 248907 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| fork01 | rv | glibc | ltp-glibc | PASS | 0 | 1083 | 239292 | 239271 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| fork01 | rv | musl | ltp-musl | PASS | 0 | 965 | 249831 | 249810 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| ftruncate01 | rv | glibc | ltp-glibc | PASS | 0 | 1128 | 188908 | 188894 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| ftruncate01 | rv | musl | ltp-musl | PASS | 0 | 804 | 240407 | 240393 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| getcwd01 | rv | glibc | ltp-glibc | PASS | 0 | 1067 | 189216 | 189202 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| getcwd01 | rv | musl | ltp-musl | PASS | 0 | 881 | 248907 | 248893 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| getegid01 | rv | glibc | ltp-glibc | PASS | 0 | 1106 | 189125 | 189111 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| getegid01 | rv | musl | ltp-musl | PASS | 0 | 882 | 240624 | 240610 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| geteuid01 | rv | glibc | ltp-glibc | PASS | 0 | 1204 | 189153 | 189139 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| geteuid01 | rv | musl | ltp-musl | PASS | 0 | 747 | 240652 | 240638 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| getgid01 | rv | glibc | ltp-glibc | PASS | 0 | 1083 | 189139 | 189125 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| getgid01 | rv | musl | ltp-musl | PASS | 0 | 825 | 240638 | 240624 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| getpgrp01 | rv | glibc | ltp-glibc | PASS | 0 | 1092 | 188696 | 188682 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| getpgrp01 | rv | musl | ltp-musl | PASS | 0 | 853 | 240197 | 240183 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| getpid01 | rv | glibc | ltp-glibc | PASS | 0 | 4464 | 239271 | 238557 | -714 | 0 | 0 | 0 | 0 | 0 | 0 |
| getpid01 | rv | musl | ltp-musl | PASS | 0 | 3760 | 249810 | 249096 | -714 | 0 | 0 | 0 | 0 | 0 | 0 |
| getpid02 | rv | glibc | ltp-glibc | PASS | 0 | 1131 | 189202 | 189181 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| getpid02 | rv | musl | ltp-musl | PASS | 0 | 780 | 248893 | 248872 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| getppid01 | rv | glibc | ltp-glibc | PASS | 0 | 1076 | 189181 | 189167 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| getppid01 | rv | musl | ltp-musl | PASS | 0 | 843 | 248872 | 240666 | -8206 | 0 | 0 | 0 | 0 | 0 | 0 |
| getrlimit01 | rv | glibc | ltp-glibc | PASS | 0 | 1051 | 188654 | 188640 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| getrlimit01 | rv | musl | ltp-musl | PASS | 0 | 772 | 240155 | 240141 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| getrusage01 | rv | glibc | ltp-glibc | PASS | 0 | 1064 | 188640 | 188626 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| getrusage01 | rv | musl | ltp-musl | PASS | 0 | 800 | 240141 | 240127 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| gettid01 | rv | glibc | ltp-glibc | PASS | 0 | 1206 | 188682 | 188668 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| gettid01 | rv | musl | ltp-musl | PASS | 0 | 784 | 240183 | 240169 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| gettimeofday01 | rv | glibc | ltp-glibc | PASS | 0 | 1041 | 188831 | 188817 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| gettimeofday01 | rv | musl | ltp-musl | PASS | 0 | 737 | 240330 | 240316 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| getuid01 | rv | glibc | ltp-glibc | PASS | 0 | 1159 | 189167 | 189153 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| getuid01 | rv | musl | ltp-musl | PASS | 0 | 876 | 240666 | 240652 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| kill03 | rv | glibc | ltp-glibc | PASS | 0 | 1118 | 188789 | 188775 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| kill03 | rv | musl | ltp-musl | PASS | 0 | 785 | 240288 | 240274 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| lseek01 | rv | glibc | ltp-glibc | PASS | 0 | 1251 | 189111 | 189097 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| lseek01 | rv | musl | ltp-musl | PASS | 0 | 830 | 240610 | 240596 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| lstat01 | rv | glibc | ltp-glibc | PASS | 0 | 1098 | 188999 | 188985 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| lstat01 | rv | musl | ltp-musl | PASS | 0 | 872 | 240498 | 240484 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| mmap01 | rv | glibc | ltp-glibc | PASS | 0 | 1020 | 238557 | 238543 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| mmap01 | rv | musl | ltp-musl | PASS | 0 | 847 | 249096 | 249082 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| open01 | rv | glibc | ltp-glibc | PASS | 0 | 1080 | 238543 | 238529 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| open01 | rv | musl | ltp-musl | PASS | 0 | 811 | 249082 | 249068 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| open02 | rv | glibc | ltp-glibc | PASS | 0 | 1139 | 189041 | 189027 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| open02 | rv | musl | ltp-musl | PASS | 0 | 900 | 240540 | 240526 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| open03 | rv | glibc | ltp-glibc | PASS | 0 | 1150 | 189027 | 189013 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| open03 | rv | musl | ltp-musl | PASS | 0 | 908 | 240526 | 240512 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| pipe01 | rv | glibc | ltp-glibc | PASS | 0 | 1067 | 238529 | 238515 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| pipe01 | rv | musl | ltp-musl | PASS | 0 | 804 | 249068 | 249054 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| proc01 | rv | glibc | ltp-glibc | PASS | 0 | 1227 | 188761 | 188754 | -7 | 0 | 0 | 0 | 0 | 0 | 0 |
| proc01 | rv | musl | ltp-musl | PASS | 0 | 958 | 240260 | 240253 | -7 | 0 | 0 | 0 | 0 | 0 | 0 |
| read01 | rv | glibc | ltp-glibc | PASS | 0 | 1100 | 238515 | 238501 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| read01 | rv | musl | ltp-musl | PASS | 0 | 875 | 249054 | 249040 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| read02 | rv | glibc | ltp-glibc | PASS | 0 | 1123 | 189097 | 189083 | -14 | 0 | 0 | 2 | 0 | 0 | 0 |
| read02 | rv | musl | ltp-musl | PASS | 0 | 797 | 240596 | 240582 | -14 | 0 | 0 | 2 | 0 | 0 | 0 |
| readlink01 | rv | glibc | ltp-glibc | PASS | 0 | 1161 | 188929 | 188908 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| readlink01 | rv | musl | ltp-musl | PASS | 0 | 899 | 240428 | 240407 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| rmdir01 | rv | glibc | ltp-glibc | PASS | 0 | 1196 | 188950 | 188936 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| rmdir01 | rv | musl | ltp-musl | PASS | 0 | 784 | 240449 | 240435 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| rt_sigaction01 | rv | glibc | ltp-glibc | PASS | 0 | 1087 | 188775 | 188768 | -7 | 0 | 0 | 0 | 0 | 0 | 0 |
| rt_sigaction01 | rv | musl | ltp-musl | PASS | 0 | 950 | 240274 | 240267 | -7 | 0 | 0 | 0 | 0 | 0 | 0 |
| sched_yield01 | rv | glibc | ltp-glibc | PASS | 0 | 1015 | 188626 | 188619 | -7 | 0 | 0 | 0 | 0 | 0 | 0 |
| sched_yield01 | rv | musl | ltp-musl | PASS | 0 | 736 | 240127 | 240120 | -7 | 0 | 0 | 0 | 0 | 0 | 0 |
| sigaction01 | rv | glibc | ltp-glibc | PASS | 0 | 1075 | 188768 | 188761 | -7 | 0 | 0 | 0 | 0 | 0 | 0 |
| sigaction01 | rv | musl | ltp-musl | PASS | 0 | 931 | 240267 | 240260 | -7 | 0 | 0 | 0 | 0 | 0 | 0 |
| stat01 | rv | glibc | ltp-glibc | PASS | 0 | 1150 | 238501 | 238487 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| stat01 | rv | musl | ltp-musl | PASS | 0 | 725 | 249040 | 249026 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| stat02 | rv | glibc | ltp-glibc | PASS | 0 | 1150 | 189013 | 188999 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| stat02 | rv | musl | ltp-musl | PASS | 0 | 771 | 240512 | 240498 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| symlink01 | rv | glibc | ltp-glibc | PASS | 0 | 1184 | 188936 | 188929 | -7 | 0 | 0 | 0 | 0 | 0 | 0 |
| symlink01 | rv | musl | ltp-musl | PASS | 0 | 970 | 240435 | 240428 | -7 | 0 | 0 | 0 | 0 | 0 | 0 |
| time01 | rv | glibc | ltp-glibc | PASS | 0 | 1084 | 188817 | 188803 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| time01 | rv | musl | ltp-musl | PASS | 0 | 839 | 240316 | 240302 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| times01 | rv | glibc | ltp-glibc | PASS | 0 | 1113 | 188803 | 188789 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| times01 | rv | musl | ltp-musl | PASS | 0 | 835 | 240302 | 240288 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| umask01 | rv | glibc | ltp-glibc | PASS | 0 | 1298 | 188894 | 188880 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| umask01 | rv | musl | ltp-musl | PASS | 0 | 1177 | 240393 | 240379 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| uname01 | rv | glibc | ltp-glibc | PASS | 0 | 1076 | 188668 | 188654 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| uname01 | rv | musl | ltp-musl | PASS | 0 | 827 | 240169 | 240155 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| wait401 | rv | glibc | ltp-glibc | PASS | 0 | 1131 | 238487 | 238466 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| wait401 | rv | musl | ltp-musl | PASS | 0 | 880 | 249026 | 249005 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| write01 | rv | glibc | ltp-glibc | PASS | 0 | 1874 | 238466 | 189300 | -49166 | 0 | 0 | 0 | 0 | 0 | 0 |
| write01 | rv | musl | ltp-musl | PASS | 0 | 891 | 249005 | 248991 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| write02 | rv | glibc | ltp-glibc | PASS | 0 | 1099 | 189083 | 189069 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |
| write02 | rv | musl | ltp-musl | PASS | 0 | 692 | 240582 | 240568 | -14 | 0 | 0 | 0 | 0 | 0 | 0 |

## Categories
- pass_clean: 124 (rv:glibc:access01, rv:musl:access01, rv:glibc:access03, rv:musl:access03, rv:glibc:alarm02, rv:musl:alarm02, rv:glibc:alarm03, rv:musl:alarm03, rv:glibc:brk01, rv:musl:brk01, rv:glibc:chdir01, rv:musl:chdir01, rv:glibc:chmod01, rv:musl:chmod01, rv:glibc:clock_gettime02, rv:musl:clock_gettime02, rv:glibc:clone01, rv:musl:clone01, rv:glibc:close01, rv:musl:close01, rv:glibc:close02, rv:musl:close02, rv:glibc:creat01, rv:musl:creat01, rv:glibc:creat03, rv:musl:creat03, rv:glibc:dup01, rv:musl:dup01, rv:glibc:dup02, rv:musl:dup02, rv:glibc:exit01, rv:musl:exit01, rv:glibc:exit02, rv:musl:exit02, rv:glibc:exit_group01, rv:musl:exit_group01, rv:glibc:fchmod01, rv:musl:fchmod01, rv:glibc:fcntl01, rv:musl:fcntl01, rv:glibc:fcntl02, rv:musl:fcntl02, rv:glibc:fcntl03, rv:musl:fcntl03, rv:glibc:fork01, rv:musl:fork01, rv:glibc:ftruncate01, rv:musl:ftruncate01, rv:glibc:getcwd01, rv:musl:getcwd01, rv:glibc:getegid01, rv:musl:getegid01, rv:glibc:geteuid01, rv:musl:geteuid01, rv:glibc:getgid01, rv:musl:getgid01, rv:glibc:getpgrp01, rv:musl:getpgrp01, rv:glibc:getpid01, rv:musl:getpid01, rv:glibc:getpid02, rv:musl:getpid02, rv:glibc:getppid01, rv:musl:getppid01, rv:glibc:getrlimit01, rv:musl:getrlimit01, rv:glibc:getrusage01, rv:musl:getrusage01, rv:glibc:gettid01, rv:musl:gettid01, rv:glibc:gettimeofday01, rv:musl:gettimeofday01, rv:glibc:getuid01, rv:musl:getuid01, rv:glibc:kill03, rv:musl:kill03, rv:glibc:lseek01, rv:musl:lseek01, rv:glibc:lstat01, rv:musl:lstat01, rv:glibc:mmap01, rv:musl:mmap01, rv:glibc:open01, rv:musl:open01, rv:glibc:open02, rv:musl:open02, rv:glibc:open03, rv:musl:open03, rv:glibc:pipe01, rv:musl:pipe01, rv:glibc:proc01, rv:musl:proc01, rv:glibc:read01, rv:musl:read01, rv:glibc:readlink01, rv:musl:readlink01, rv:glibc:rmdir01, rv:musl:rmdir01, rv:glibc:rt_sigaction01, rv:musl:rt_sigaction01, rv:glibc:sched_yield01, rv:musl:sched_yield01, rv:glibc:sigaction01, rv:musl:sigaction01, rv:glibc:stat01, rv:musl:stat01, rv:glibc:stat02, rv:musl:stat02, rv:glibc:symlink01, rv:musl:symlink01, rv:glibc:time01, rv:musl:time01, rv:glibc:times01, rv:musl:times01, rv:glibc:umask01, rv:musl:umask01, rv:glibc:uname01, rv:musl:uname01, rv:glibc:wait401, rv:musl:wait401, rv:glibc:write01, rv:musl:write01, rv:glibc:write02, rv:musl:write02)
- pass_with_tconf: 2 (rv:glibc:read02, rv:musl:read02)
- fail_wrapper: 0
- internal_tfail: 0
- internal_tbrok: 0
- timeout: 0
- enosys: 0
- panic_trap: 0
- unknown: 0

## Groups
### libctest-musl
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 1
- ENOSYS/not implemented: 0
- panic/trap: 0

### libctest-glibc
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 1
- ENOSYS/not implemented: 0
- panic/trap: 0

### basic-musl
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### basic-glibc
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### busybox-musl
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### busybox-glibc
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### lua-musl
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### lua-glibc
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### ltp-musl
- PASS: 63
- FAIL: 0
- Internal: {'TCONF': 2}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### ltp-glibc
- PASS: 63
- FAIL: 0
- Internal: {'TCONF': 2}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### libcbench-musl
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### libcbench-glibc
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### iperf-musl
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### iperf-glibc
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### lmbench-musl
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 1
- ENOSYS/not implemented: 0
- panic/trap: 0

### lmbench-glibc
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 1
- ENOSYS/not implemented: 0
- panic/trap: 0

### netperf-musl
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### netperf-glibc
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### cyclictest-musl
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 1
- ENOSYS/not implemented: 0
- panic/trap: 0

### cyclictest-glibc
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 1
- ENOSYS/not implemented: 0
- panic/trap: 0

### iozone-musl
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 1
- ENOSYS/not implemented: 0
- panic/trap: 0

### iozone-glibc
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 1
- ENOSYS/not implemented: 0
- panic/trap: 0

### unixbench-musl
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 1
- ENOSYS/not implemented: 0
- panic/trap: 0

### unixbench-glibc
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 1
- ENOSYS/not implemented: 0
- panic/trap: 0

## LA LTP summary: output_la.md
# LTP summary: `output_la.md`

- PASS LTP CASE: 126
- FAIL LTP CASE: 0
- Internal TFAIL/TBROK/TCONF: 4 ({'TCONF': 4})
- timeout matches: 10
- ENOSYS/not implemented matches: 0
- panic/trap matches: 0

## Suite summaries
- ltp-musl: 63 passed, 0 failed
- ltp-glibc: 63 passed, 0 failed

## Case matrix
| Case | Arch | Libc | Group | Status | Code | Runtime ms | Free frames before | Free frames after cleanup | Free frames delta | TFAIL | TBROK | TCONF | timeout | ENOSYS | panic/trap |
| --- | --- | --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| access01 | la | glibc | ltp-glibc | PASS | 0 | 7539 | 155004 | 154074 | -930 | 0 | 0 | 0 | 0 | 0 | 0 |
| access01 | la | musl | ltp-musl | PASS | 0 | 4985 | 157684 | 156860 | -824 | 0 | 0 | 0 | 0 | 0 | 0 |
| access03 | la | glibc | ltp-glibc | PASS | 0 | 3175 | 120043 | 119986 | -57 | 0 | 0 | 0 | 0 | 0 | 0 |
| access03 | la | musl | ltp-musl | PASS | 0 | 1705 | 155780 | 155732 | -48 | 0 | 0 | 0 | 0 | 0 | 0 |
| alarm02 | la | glibc | ltp-glibc | PASS | 0 | 2845 | 119422 | 119401 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| alarm02 | la | musl | ltp-musl | PASS | 0 | 1486 | 155300 | 155284 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| alarm03 | la | glibc | ltp-glibc | PASS | 0 | 2904 | 119401 | 119371 | -30 | 0 | 0 | 0 | 0 | 0 | 0 |
| alarm03 | la | musl | ltp-musl | PASS | 0 | 1505 | 155284 | 155260 | -24 | 0 | 0 | 0 | 0 | 0 | 0 |
| brk01 | la | glibc | ltp-glibc | PASS | 0 | 2754 | 154074 | 154044 | -30 | 0 | 0 | 0 | 0 | 0 | 0 |
| brk01 | la | musl | ltp-musl | PASS | 0 | 1483 | 156860 | 156836 | -24 | 0 | 0 | 0 | 0 | 0 | 0 |
| chdir01 | la | glibc | ltp-glibc | PASS | 0 | 2939 | 154044 | 154023 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| chdir01 | la | musl | ltp-musl | PASS | 0 | 1733 | 156836 | 156820 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| chmod01 | la | glibc | ltp-glibc | PASS | 0 | 2930 | 119578 | 119548 | -30 | 0 | 0 | 0 | 0 | 0 | 0 |
| chmod01 | la | musl | ltp-musl | PASS | 0 | 1537 | 155420 | 155396 | -24 | 0 | 0 | 0 | 0 | 0 | 0 |
| clock_gettime02 | la | glibc | ltp-glibc | PASS | 0 | 2822 | 119371 | 119350 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| clock_gettime02 | la | musl | ltp-musl | PASS | 0 | 1615 | 155260 | 155244 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| clone01 | la | glibc | ltp-glibc | PASS | 0 | 2928 | 154023 | 153993 | -30 | 0 | 0 | 0 | 0 | 0 | 0 |
| clone01 | la | musl | ltp-musl | PASS | 0 | 1448 | 156820 | 156796 | -24 | 0 | 0 | 0 | 0 | 0 | 0 |
| close01 | la | glibc | ltp-glibc | PASS | 0 | 2771 | 153993 | 153972 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| close01 | la | musl | ltp-musl | PASS | 0 | 1504 | 156796 | 156780 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| close02 | la | glibc | ltp-glibc | PASS | 0 | 2968 | 119986 | 119965 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| close02 | la | musl | ltp-musl | PASS | 0 | 1432 | 155732 | 155716 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| creat01 | la | glibc | ltp-glibc | PASS | 0 | 2834 | 119704 | 119683 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| creat01 | la | musl | ltp-musl | PASS | 0 | 1498 | 155516 | 155500 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| creat03 | la | glibc | ltp-glibc | PASS | 0 | 2828 | 119683 | 119662 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| creat03 | la | musl | ltp-musl | PASS | 0 | 1465 | 155500 | 155484 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| dup01 | la | glibc | ltp-glibc | PASS | 0 | 2842 | 153972 | 153951 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| dup01 | la | musl | ltp-musl | PASS | 0 | 1550 | 156780 | 156764 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| dup02 | la | glibc | ltp-glibc | PASS | 0 | 2831 | 119965 | 119944 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| dup02 | la | musl | ltp-musl | PASS | 0 | 1448 | 155716 | 155700 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| exit01 | la | glibc | ltp-glibc | PASS | 0 | 2848 | 119230 | 119209 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| exit01 | la | musl | ltp-musl | PASS | 0 | 1579 | 155156 | 155140 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| exit02 | la | glibc | ltp-glibc | PASS | 0 | 2772 | 119209 | 119179 | -30 | 0 | 0 | 0 | 0 | 0 | 0 |
| exit02 | la | musl | ltp-musl | PASS | 0 | 1489 | 155140 | 155116 | -24 | 0 | 0 | 0 | 0 | 0 | 0 |
| exit_group01 | la | glibc | ltp-glibc | PASS | 0 | 3021 | 119179 | 119147 | -32 | 0 | 0 | 0 | 0 | 0 | 0 |
| exit_group01 | la | musl | ltp-musl | PASS | 0 | 1748 | 155116 | 155092 | -24 | 0 | 0 | 0 | 0 | 0 | 0 |
| fchmod01 | la | glibc | ltp-glibc | PASS | 0 | 2893 | 119548 | 119527 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| fchmod01 | la | musl | ltp-musl | PASS | 0 | 1496 | 155396 | 155380 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| fcntl01 | la | glibc | ltp-glibc | PASS | 0 | 2907 | 153951 | 153939 | -12 | 0 | 0 | 0 | 0 | 0 | 0 |
| fcntl01 | la | musl | ltp-musl | PASS | 0 | 1443 | 156764 | 156756 | -8 | 0 | 0 | 0 | 0 | 0 | 0 |
| fcntl02 | la | glibc | ltp-glibc | PASS | 0 | 2583 | 153939 | 153918 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| fcntl02 | la | musl | ltp-musl | PASS | 0 | 1504 | 156756 | 156740 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| fcntl03 | la | glibc | ltp-glibc | PASS | 0 | 2781 | 119944 | 119923 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| fcntl03 | la | musl | ltp-musl | PASS | 0 | 1585 | 155700 | 155684 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| fork01 | la | glibc | ltp-glibc | PASS | 0 | 2816 | 153918 | 153888 | -30 | 0 | 0 | 0 | 0 | 0 | 0 |
| fork01 | la | musl | ltp-musl | PASS | 0 | 1513 | 156740 | 156716 | -24 | 0 | 0 | 0 | 0 | 0 | 0 |
| ftruncate01 | la | glibc | ltp-glibc | PASS | 0 | 2983 | 119464 | 119443 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| ftruncate01 | la | musl | ltp-musl | PASS | 0 | 1427 | 155332 | 155316 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| getcwd01 | la | glibc | ltp-glibc | PASS | 0 | 2963 | 119923 | 119902 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| getcwd01 | la | musl | ltp-musl | PASS | 0 | 1550 | 155684 | 155668 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| getegid01 | la | glibc | ltp-glibc | PASS | 0 | 2810 | 119788 | 119767 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| getegid01 | la | musl | ltp-musl | PASS | 0 | 1480 | 155580 | 155564 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| geteuid01 | la | glibc | ltp-glibc | PASS | 0 | 2720 | 119830 | 119809 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| geteuid01 | la | musl | ltp-musl | PASS | 0 | 1466 | 155612 | 155596 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| getgid01 | la | glibc | ltp-glibc | PASS | 0 | 2869 | 119809 | 119788 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| getgid01 | la | musl | ltp-musl | PASS | 0 | 1590 | 155596 | 155580 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| getpgrp01 | la | glibc | ltp-glibc | PASS | 0 | 2944 | 119147 | 119126 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| getpgrp01 | la | musl | ltp-musl | PASS | 0 | 1611 | 155092 | 155076 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| getpid01 | la | glibc | ltp-glibc | PASS | 0 | 6601 | 153888 | 152967 | -921 | 0 | 0 | 0 | 0 | 0 | 0 |
| getpid01 | la | musl | ltp-musl | PASS | 0 | 4401 | 156716 | 155900 | -816 | 0 | 0 | 0 | 0 | 0 | 0 |
| getpid02 | la | glibc | ltp-glibc | PASS | 0 | 2771 | 119902 | 119872 | -30 | 0 | 0 | 0 | 0 | 0 | 0 |
| getpid02 | la | musl | ltp-musl | PASS | 0 | 1517 | 155668 | 155644 | -24 | 0 | 0 | 0 | 0 | 0 | 0 |
| getppid01 | la | glibc | ltp-glibc | PASS | 0 | 2780 | 119872 | 119851 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| getppid01 | la | musl | ltp-musl | PASS | 0 | 1611 | 155644 | 155628 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| getrlimit01 | la | glibc | ltp-glibc | PASS | 0 | 2800 | 119084 | 119063 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| getrlimit01 | la | musl | ltp-musl | PASS | 0 | 1501 | 155044 | 155028 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| getrusage01 | la | glibc | ltp-glibc | PASS | 0 | 2870 | 119063 | 119042 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| getrusage01 | la | musl | ltp-musl | PASS | 0 | 1511 | 155028 | 155012 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| gettid01 | la | glibc | ltp-glibc | PASS | 0 | 2727 | 119126 | 119105 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| gettid01 | la | musl | ltp-musl | PASS | 0 | 1386 | 155076 | 155060 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| gettimeofday01 | la | glibc | ltp-glibc | PASS | 0 | 2747 | 119350 | 119329 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| gettimeofday01 | la | musl | ltp-musl | PASS | 0 | 1641 | 155244 | 155228 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| getuid01 | la | glibc | ltp-glibc | PASS | 0 | 2892 | 119851 | 119830 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| getuid01 | la | musl | ltp-musl | PASS | 0 | 1560 | 155628 | 155612 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| kill03 | la | glibc | ltp-glibc | PASS | 0 | 2802 | 119287 | 119266 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| kill03 | la | musl | ltp-musl | PASS | 0 | 1479 | 155196 | 155180 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| lseek01 | la | glibc | ltp-glibc | PASS | 0 | 2914 | 119767 | 119746 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| lseek01 | la | musl | ltp-musl | PASS | 0 | 1458 | 155564 | 155548 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| lstat01 | la | glibc | ltp-glibc | PASS | 0 | 2655 | 119599 | 119578 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| lstat01 | la | musl | ltp-musl | PASS | 0 | 1525 | 155436 | 155420 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| mmap01 | la | glibc | ltp-glibc | PASS | 0 | 2829 | 152967 | 152946 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| mmap01 | la | musl | ltp-musl | PASS | 0 | 1474 | 155900 | 155884 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| open01 | la | glibc | ltp-glibc | PASS | 0 | 2796 | 152946 | 152925 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| open01 | la | musl | ltp-musl | PASS | 0 | 1556 | 155884 | 155868 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| open02 | la | glibc | ltp-glibc | PASS | 0 | 2977 | 119662 | 119641 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| open02 | la | musl | ltp-musl | PASS | 0 | 1597 | 155484 | 155468 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| open03 | la | glibc | ltp-glibc | PASS | 0 | 4011 | 119641 | 119620 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| open03 | la | musl | ltp-musl | PASS | 0 | 1493 | 155468 | 155452 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| pipe01 | la | glibc | ltp-glibc | PASS | 0 | 2791 | 152925 | 152904 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| pipe01 | la | musl | ltp-musl | PASS | 0 | 1520 | 155868 | 155852 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| proc01 | la | glibc | ltp-glibc | PASS | 0 | 2980 | 119242 | 119230 | -12 | 0 | 0 | 0 | 0 | 0 | 0 |
| proc01 | la | musl | ltp-musl | PASS | 0 | 1608 | 155164 | 155156 | -8 | 0 | 0 | 0 | 0 | 0 | 0 |
| read01 | la | glibc | ltp-glibc | PASS | 0 | 2769 | 152904 | 152883 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| read01 | la | musl | ltp-musl | PASS | 0 | 1440 | 155852 | 155836 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| read02 | la | glibc | ltp-glibc | PASS | 0 | 2741 | 119746 | 119725 | -21 | 0 | 0 | 2 | 0 | 0 | 0 |
| read02 | la | musl | ltp-musl | PASS | 0 | 1527 | 155548 | 155532 | -16 | 0 | 0 | 2 | 0 | 0 | 0 |
| readlink01 | la | glibc | ltp-glibc | PASS | 0 | 2825 | 119494 | 119464 | -30 | 0 | 0 | 0 | 0 | 0 | 0 |
| readlink01 | la | musl | ltp-musl | PASS | 0 | 1473 | 155356 | 155332 | -24 | 0 | 0 | 0 | 0 | 0 | 0 |
| rmdir01 | la | glibc | ltp-glibc | PASS | 0 | 2699 | 119527 | 119506 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| rmdir01 | la | musl | ltp-musl | PASS | 0 | 1490 | 155380 | 155364 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| rt_sigaction01 | la | glibc | ltp-glibc | PASS | 0 | 3114 | 119266 | 119254 | -12 | 0 | 0 | 0 | 0 | 0 | 0 |
| rt_sigaction01 | la | musl | ltp-musl | PASS | 0 | 1769 | 155180 | 155172 | -8 | 0 | 0 | 0 | 0 | 0 | 0 |
| sched_yield01 | la | glibc | ltp-glibc | PASS | 0 | 2806 | 119042 | 119030 | -12 | 0 | 0 | 0 | 0 | 0 | 0 |
| sched_yield01 | la | musl | ltp-musl | PASS | 0 | 1439 | 155012 | 155004 | -8 | 0 | 0 | 0 | 0 | 0 | 0 |
| sigaction01 | la | glibc | ltp-glibc | PASS | 0 | 2838 | 119254 | 119242 | -12 | 0 | 0 | 0 | 0 | 0 | 0 |
| sigaction01 | la | musl | ltp-musl | PASS | 0 | 1548 | 155172 | 155164 | -8 | 0 | 0 | 0 | 0 | 0 | 0 |
| stat01 | la | glibc | ltp-glibc | PASS | 0 | 2885 | 152883 | 152862 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| stat01 | la | musl | ltp-musl | PASS | 0 | 1571 | 155836 | 155820 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| stat02 | la | glibc | ltp-glibc | PASS | 0 | 2920 | 119620 | 119599 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| stat02 | la | musl | ltp-musl | PASS | 0 | 1434 | 155452 | 155436 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| symlink01 | la | glibc | ltp-glibc | PASS | 0 | 2871 | 119506 | 119494 | -12 | 0 | 0 | 0 | 0 | 0 | 0 |
| symlink01 | la | musl | ltp-musl | PASS | 0 | 1527 | 155364 | 155356 | -8 | 0 | 0 | 0 | 0 | 0 | 0 |
| time01 | la | glibc | ltp-glibc | PASS | 0 | 2856 | 119329 | 119308 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| time01 | la | musl | ltp-musl | PASS | 0 | 1486 | 155228 | 155212 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| times01 | la | glibc | ltp-glibc | PASS | 0 | 2881 | 119308 | 119287 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| times01 | la | musl | ltp-musl | PASS | 0 | 1497 | 155212 | 155196 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| umask01 | la | glibc | ltp-glibc | PASS | 0 | 2964 | 119443 | 119422 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| umask01 | la | musl | ltp-musl | PASS | 0 | 1741 | 155316 | 155300 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| uname01 | la | glibc | ltp-glibc | PASS | 0 | 2917 | 119105 | 119084 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| uname01 | la | musl | ltp-musl | PASS | 0 | 1523 | 155060 | 155044 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| wait401 | la | glibc | ltp-glibc | PASS | 0 | 2907 | 152862 | 152832 | -30 | 0 | 0 | 0 | 0 | 0 | 0 |
| wait401 | la | musl | ltp-musl | PASS | 0 | 1502 | 155820 | 155796 | -24 | 0 | 0 | 0 | 0 | 0 | 0 |
| write01 | la | glibc | ltp-glibc | PASS | 0 | 3751 | 152832 | 120043 | -32789 | 0 | 0 | 0 | 0 | 0 | 0 |
| write01 | la | musl | ltp-musl | PASS | 0 | 1478 | 155796 | 155780 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |
| write02 | la | glibc | ltp-glibc | PASS | 0 | 2763 | 119725 | 119704 | -21 | 0 | 0 | 0 | 0 | 0 | 0 |
| write02 | la | musl | ltp-musl | PASS | 0 | 1524 | 155532 | 155516 | -16 | 0 | 0 | 0 | 0 | 0 | 0 |

## Categories
- pass_clean: 124 (la:glibc:access01, la:musl:access01, la:glibc:access03, la:musl:access03, la:glibc:alarm02, la:musl:alarm02, la:glibc:alarm03, la:musl:alarm03, la:glibc:brk01, la:musl:brk01, la:glibc:chdir01, la:musl:chdir01, la:glibc:chmod01, la:musl:chmod01, la:glibc:clock_gettime02, la:musl:clock_gettime02, la:glibc:clone01, la:musl:clone01, la:glibc:close01, la:musl:close01, la:glibc:close02, la:musl:close02, la:glibc:creat01, la:musl:creat01, la:glibc:creat03, la:musl:creat03, la:glibc:dup01, la:musl:dup01, la:glibc:dup02, la:musl:dup02, la:glibc:exit01, la:musl:exit01, la:glibc:exit02, la:musl:exit02, la:glibc:exit_group01, la:musl:exit_group01, la:glibc:fchmod01, la:musl:fchmod01, la:glibc:fcntl01, la:musl:fcntl01, la:glibc:fcntl02, la:musl:fcntl02, la:glibc:fcntl03, la:musl:fcntl03, la:glibc:fork01, la:musl:fork01, la:glibc:ftruncate01, la:musl:ftruncate01, la:glibc:getcwd01, la:musl:getcwd01, la:glibc:getegid01, la:musl:getegid01, la:glibc:geteuid01, la:musl:geteuid01, la:glibc:getgid01, la:musl:getgid01, la:glibc:getpgrp01, la:musl:getpgrp01, la:glibc:getpid01, la:musl:getpid01, la:glibc:getpid02, la:musl:getpid02, la:glibc:getppid01, la:musl:getppid01, la:glibc:getrlimit01, la:musl:getrlimit01, la:glibc:getrusage01, la:musl:getrusage01, la:glibc:gettid01, la:musl:gettid01, la:glibc:gettimeofday01, la:musl:gettimeofday01, la:glibc:getuid01, la:musl:getuid01, la:glibc:kill03, la:musl:kill03, la:glibc:lseek01, la:musl:lseek01, la:glibc:lstat01, la:musl:lstat01, la:glibc:mmap01, la:musl:mmap01, la:glibc:open01, la:musl:open01, la:glibc:open02, la:musl:open02, la:glibc:open03, la:musl:open03, la:glibc:pipe01, la:musl:pipe01, la:glibc:proc01, la:musl:proc01, la:glibc:read01, la:musl:read01, la:glibc:readlink01, la:musl:readlink01, la:glibc:rmdir01, la:musl:rmdir01, la:glibc:rt_sigaction01, la:musl:rt_sigaction01, la:glibc:sched_yield01, la:musl:sched_yield01, la:glibc:sigaction01, la:musl:sigaction01, la:glibc:stat01, la:musl:stat01, la:glibc:stat02, la:musl:stat02, la:glibc:symlink01, la:musl:symlink01, la:glibc:time01, la:musl:time01, la:glibc:times01, la:musl:times01, la:glibc:umask01, la:musl:umask01, la:glibc:uname01, la:musl:uname01, la:glibc:wait401, la:musl:wait401, la:glibc:write01, la:musl:write01, la:glibc:write02, la:musl:write02)
- pass_with_tconf: 2 (la:glibc:read02, la:musl:read02)
- fail_wrapper: 0
- internal_tfail: 0
- internal_tbrok: 0
- timeout: 0
- enosys: 0
- panic_trap: 0
- unknown: 0

## Groups
### libctest-musl
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 1
- ENOSYS/not implemented: 0
- panic/trap: 0

### libctest-glibc
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 1
- ENOSYS/not implemented: 0
- panic/trap: 0

### basic-musl
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### basic-glibc
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### busybox-musl
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### busybox-glibc
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### lua-musl
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### lua-glibc
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### ltp-musl
- PASS: 63
- FAIL: 0
- Internal: {'TCONF': 2}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### ltp-glibc
- PASS: 63
- FAIL: 0
- Internal: {'TCONF': 2}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### libcbench-musl
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### libcbench-glibc
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### iperf-musl
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### iperf-glibc
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### lmbench-musl
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 1
- ENOSYS/not implemented: 0
- panic/trap: 0

### lmbench-glibc
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 1
- ENOSYS/not implemented: 0
- panic/trap: 0

### netperf-musl
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### netperf-glibc
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### cyclictest-musl
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 1
- ENOSYS/not implemented: 0
- panic/trap: 0

### cyclictest-glibc
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 1
- ENOSYS/not implemented: 0
- panic/trap: 0

### iozone-musl
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 1
- ENOSYS/not implemented: 0
- panic/trap: 0

### iozone-glibc
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 1
- ENOSYS/not implemented: 0
- panic/trap: 0

### unixbench-musl
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 1
- ENOSYS/not implemented: 0
- panic/trap: 0

### unixbench-glibc
- PASS: 0
- FAIL: 0
- Internal: {}
- timeout: 1
- ENOSYS/not implemented: 0
- panic/trap: 0

## Promotion candidates across rv,la x musl,glibc
# LTP promotion-candidate report

- Inputs: `output_rv.md`, `output_la.md`
- Required arches: la, rv
- Required libcs: glibc, musl
- Required arch/libc combos: 4
- Promotion candidates: 62
- Blocked/incomplete cases: 1

## Candidates
| Case | Clean combos | Max runtime ms | Min free-frames delta after cleanup |
| --- | --- | ---: | ---: |
| access01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 7539 | -930 |
| access03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 3175 | -57 |
| alarm02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2845 | -21 |
| alarm03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2904 | -30 |
| brk01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2754 | -30 |
| chdir01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2939 | -21 |
| chmod01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2930 | -30 |
| clock_gettime02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2822 | -21 |
| clone01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2928 | -30 |
| close01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2771 | -21 |
| close02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2968 | -21 |
| creat01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2834 | -21 |
| creat03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2828 | -21 |
| dup01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2842 | -21 |
| dup02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2831 | -21 |
| exit01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2848 | -21 |
| exit02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2772 | -30 |
| exit_group01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 3021 | -32 |
| fchmod01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2893 | -21 |
| fcntl01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2907 | -12 |
| fcntl02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2583 | -21 |
| fcntl03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2781 | -21 |
| fork01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2816 | -30 |
| ftruncate01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2983 | -21 |
| getcwd01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2963 | -21 |
| getegid01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2810 | -21 |
| geteuid01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2720 | -21 |
| getgid01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2869 | -21 |
| getpgrp01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2944 | -21 |
| getpid01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 6601 | -921 |
| getpid02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2771 | -30 |
| getppid01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2780 | -8206 |
| getrlimit01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2800 | -21 |
| getrusage01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2870 | -21 |
| gettid01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2727 | -21 |
| gettimeofday01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2747 | -21 |
| getuid01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2892 | -21 |
| kill03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2802 | -21 |
| lseek01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2914 | -21 |
| lstat01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2655 | -21 |
| mmap01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2829 | -21 |
| open01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2796 | -21 |
| open02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2977 | -21 |
| open03 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 4011 | -21 |
| pipe01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2791 | -21 |
| proc01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2980 | -12 |
| read01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2769 | -21 |
| readlink01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2825 | -30 |
| rmdir01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2699 | -21 |
| rt_sigaction01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 3114 | -12 |
| sched_yield01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2806 | -12 |
| sigaction01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2838 | -12 |
| stat01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2885 | -21 |
| stat02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2920 | -21 |
| symlink01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2871 | -12 |
| time01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2856 | -21 |
| times01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2881 | -21 |
| umask01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2964 | -21 |
| uname01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2917 | -21 |
| wait401 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2907 | -30 |
| write01 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 3751 | -49166 |
| write02 | la:glibc:ltp-glibc, la:musl:ltp-musl, rv:glibc:ltp-glibc, rv:musl:ltp-musl | 2763 | -21 |

## Blocked or incomplete
| Case | Reason |
| --- | --- |
| read02 | la:glibc:ltp-glibc TCONF=2; la:musl:ltp-musl TCONF=2; rv:glibc:ltp-glibc TCONF=2; rv:musl:ltp-musl TCONF=2 |
