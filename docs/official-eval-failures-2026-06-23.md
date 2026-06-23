# OSKernel 官方评测未通过用例记录

生成时间：2026-06-23T06:37:19+00:00

说明：本报告由 evaluator 原始日志只读生成；非零退出、TIMEOUT、libctest/busybox fail、官方组 FAIL、LTP TFAIL/TBROK 均列为未通过；LTP TCONF 单独列为配置性未通过/跳过。
若传入官方 judge 目录，libctest 会额外按官方 `judge_libctest-*.py` 的 section-scoped `START ...` + literal `Pass!` 规则列出缺失/未 Pass 项；这会暴露 wrapper 未运行但官方 baseline 计分的 case。
注：官方组通用超时上限只用于把仍未结束的长跑组闭合并记录为失败/超时；不会跳过用例，也不会把失败或超时改写为 PASS。

## 汇总

| 日志 | LTP失败 | LTP TCONF-only | libctest失败(日志) | libctest失败(官方judge) | busybox失败 | 官方组非零退出 | autorun故障 | 官方组超时 | panic/trap | ENOSYS/not implemented |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| .omx/ultragoal/evidence/final-run-eval-rv-stable-fastfail-20260623.log | 0 | 0 | 38 | 44 | 0 | 2 | 2 | 2 | 0 | 0 |
| .omx/ultragoal/evidence/final-run-eval-la-stable-fastfail-20260623.log | 0 | 0 | 85 | 91 | 0 | 2 | 2 | 2 | 0 | 0 |

## .omx/ultragoal/evidence/final-run-eval-rv-stable-fastfail-20260623.log

### LTP case list
- ltp case list: stable (1000 cases, timeout 180s)
- ltp case list: stable (1000 cases, timeout 180s)

### LTP suite summaries
- ltp cases: 1000 passed, 0 failed, 0 timed out
- ltp cases: 1000 passed, 0 failed, 0 timed out

### libctest summaries
- libctest-musl: libctest cases: 217 passed, 0 failed, 0 timed out
- libctest-glibc: libctest cases: 179 passed, 38 failed, 2 timed out

### 官方 libctest judge 汇总
| group | passed | total | failed |
| --- | ---: | ---: | ---: |
| libctest-musl | 217 | 220 | 3 |
| libctest-glibc | 179 | 220 | 41 |

### LTP 失败/TBROK/TFAIL/超时
- 无

### LTP TCONF-only（配置性未通过/跳过，未计为 PASS）
- 无

### libctest 失败
- libctest-glibc: entry-static.exe clocale_mbfuncs => 1
- libctest-glibc: entry-static.exe fnmatch => 1
- libctest-glibc: entry-static.exe fscanf => 1
- libctest-glibc: entry-static.exe fwscanf => 1
- libctest-glibc: entry-static.exe mbc => 83
- libctest-glibc: entry-static.exe pthread_cancel_points => 1
- libctest-glibc: entry-static.exe sscanf => 1
- libctest-glibc: entry-static.exe strftime => 1
- libctest-glibc: entry-static.exe strtol => 1
- libctest-glibc: entry-static.exe swprintf => 134
- libctest-glibc: entry-static.exe wcstol => 1
- libctest-glibc: entry-static.exe daemon_failure => 1
- libctest-glibc: entry-static.exe dn_expand_empty => 1
- libctest-glibc: entry-static.exe dn_expand_ptr_0 => 1
- libctest-glibc: entry-static.exe fgetwc_buffering => 1
- libctest-glibc: entry-static.exe regex_ere_backref => 1
- libctest-glibc: entry-static.exe regex_escaped_high_byte => 1
- libctest-glibc: entry-static.exe setvbuf_unget => timeout
- libctest-glibc: entry-dynamic.exe clocale_mbfuncs => 1
- libctest-glibc: entry-dynamic.exe fnmatch => 1
- libctest-glibc: entry-dynamic.exe fscanf => 1
- libctest-glibc: entry-dynamic.exe fwscanf => 1
- libctest-glibc: entry-dynamic.exe mbc => 83
- libctest-glibc: entry-dynamic.exe pthread_cancel_points => 134
- libctest-glibc: entry-dynamic.exe pthread_cancel => 134
- libctest-glibc: entry-dynamic.exe sscanf => 1
- libctest-glibc: entry-dynamic.exe strftime => 1
- libctest-glibc: entry-dynamic.exe strtol => 1
- libctest-glibc: entry-dynamic.exe swprintf => 134
- libctest-glibc: entry-dynamic.exe wcstol => 1
- libctest-glibc: entry-dynamic.exe daemon_failure => 1
- libctest-glibc: entry-dynamic.exe dn_expand_empty => 1
- libctest-glibc: entry-dynamic.exe dn_expand_ptr_0 => 1
- libctest-glibc: entry-dynamic.exe fgetwc_buffering => 1
- libctest-glibc: entry-dynamic.exe pthread_exit_cancel => 134
- libctest-glibc: entry-dynamic.exe regex_ere_backref => 1
- libctest-glibc: entry-dynamic.exe regex_escaped_high_byte => 1
- libctest-glibc: entry-dynamic.exe setvbuf_unget => timeout

### 官方 libctest judge 未通过（libctest-musl，含缺失/未运行/无 Pass!）
- libctest dynamic crypt
- libctest static crypt
- libctest static pleval

### 官方 libctest judge 未通过（libctest-glibc，含缺失/未运行/无 Pass!）
- libctest dynamic clocale_mbfuncs
- libctest dynamic crypt
- libctest dynamic daemon_failure
- libctest dynamic dn_expand_empty
- libctest dynamic dn_expand_ptr_0
- libctest dynamic fgetwc_buffering
- libctest dynamic fnmatch
- libctest dynamic fscanf
- libctest dynamic fwscanf
- libctest dynamic mbc
- libctest dynamic pthread_cancel
- libctest dynamic pthread_cancel_points
- libctest dynamic pthread_exit_cancel
- libctest dynamic regex_ere_backref
- libctest dynamic regex_escaped_high_byte
- libctest dynamic setvbuf_unget
- libctest dynamic sscanf
- libctest dynamic strftime
- libctest dynamic strtol
- libctest dynamic swprintf
- libctest dynamic wcstol
- libctest static clocale_mbfuncs
- libctest static crypt
- libctest static daemon_failure
- libctest static dn_expand_empty
- libctest static dn_expand_ptr_0
- libctest static fgetwc_buffering
- libctest static fnmatch
- libctest static fscanf
- libctest static fwscanf
- libctest static mbc
- libctest static pleval
- libctest static pthread_cancel_points
- libctest static regex_ere_backref
- libctest static regex_escaped_high_byte
- libctest static setvbuf_unget
- libctest static sscanf
- libctest static strftime
- libctest static strtol
- libctest static swprintf
- libctest static wcstol

### busybox 失败
- 无

### 官方组非零退出
- unixbench-musl => status 137
- unixbench-glibc => status 137

### autorun 故障细节
- unixbench-musl: /tmp/t/m/unixbench/unixbench_testcode.sh timed out after 300s
- unixbench-glibc: /tmp/t/g/unixbench/unixbench_testcode.sh timed out after 300s

### 官方组超时
- unixbench-musl => timeout after 300s
- unixbench-glibc => timeout after 300s

### panic/trap
- 无

### ENOSYS/not implemented
- 无

## .omx/ultragoal/evidence/final-run-eval-la-stable-fastfail-20260623.log

### LTP case list
- ltp case list: stable (1000 cases, timeout 180s)
- ltp case list: stable (1000 cases, timeout 180s)

### LTP suite summaries
- ltp cases: 1000 passed, 0 failed, 0 timed out
- ltp cases: 1000 passed, 0 failed, 0 timed out

### libctest summaries
- libctest-musl: libctest cases: 217 passed, 0 failed, 0 timed out
- libctest-glibc: libctest cases: 132 passed, 85 failed, 58 timed out

### 官方 libctest judge 汇总
| group | passed | total | failed |
| --- | ---: | ---: | ---: |
| libctest-musl | 217 | 220 | 3 |
| libctest-glibc | 132 | 220 | 88 |

### LTP 失败/TBROK/TFAIL/超时
- 无

### LTP TCONF-only（配置性未通过/跳过，未计为 PASS）
- 无

### libctest 失败
- libctest-glibc: entry-static.exe clocale_mbfuncs => 1
- libctest-glibc: entry-static.exe fnmatch => 1
- libctest-glibc: entry-static.exe fscanf => 1
- libctest-glibc: entry-static.exe fwscanf => 139
- libctest-glibc: entry-static.exe mbc => 83
- libctest-glibc: entry-static.exe pthread_cancel_points => 1
- libctest-glibc: entry-static.exe sscanf => 1
- libctest-glibc: entry-static.exe strftime => 1
- libctest-glibc: entry-static.exe strtol => 1
- libctest-glibc: entry-static.exe swprintf => 1
- libctest-glibc: entry-static.exe wcstol => 1
- libctest-glibc: entry-static.exe daemon_failure => 1
- libctest-glibc: entry-static.exe dn_expand_empty => 1
- libctest-glibc: entry-static.exe dn_expand_ptr_0 => 1
- libctest-glibc: entry-static.exe fgetwc_buffering => 139
- libctest-glibc: entry-static.exe regex_ere_backref => 1
- libctest-glibc: entry-static.exe regex_escaped_high_byte => 1
- libctest-glibc: entry-static.exe setvbuf_unget => timeout
- libctest-glibc: entry-dynamic.exe clocale_mbfuncs => 1
- libctest-glibc: entry-dynamic.exe fnmatch => 1
- libctest-glibc: entry-dynamic.exe fscanf => 1
- libctest-glibc: entry-dynamic.exe fwscanf => 1
- libctest-glibc: entry-dynamic.exe mbc => 83
- libctest-glibc: entry-dynamic.exe pthread_cancel_points => 134
- libctest-glibc: entry-dynamic.exe pthread_cancel => 134
- libctest-glibc: entry-dynamic.exe socket => timeout
- libctest-glibc: entry-dynamic.exe sscanf => timeout
- libctest-glibc: entry-dynamic.exe sscanf_long => timeout
- libctest-glibc: entry-dynamic.exe stat => timeout
- libctest-glibc: entry-dynamic.exe strftime => timeout
- libctest-glibc: entry-dynamic.exe string => timeout
- libctest-glibc: entry-dynamic.exe string_memcpy => timeout
- libctest-glibc: entry-dynamic.exe string_memmem => timeout
- libctest-glibc: entry-dynamic.exe string_memset => timeout
- libctest-glibc: entry-dynamic.exe string_strchr => timeout
- libctest-glibc: entry-dynamic.exe string_strcspn => timeout
- libctest-glibc: entry-dynamic.exe string_strstr => timeout
- libctest-glibc: entry-dynamic.exe strptime => timeout
- libctest-glibc: entry-dynamic.exe strtod => timeout
- libctest-glibc: entry-dynamic.exe strtod_simple => timeout
- libctest-glibc: entry-dynamic.exe strtof => timeout
- libctest-glibc: entry-dynamic.exe strtol => timeout
- libctest-glibc: entry-dynamic.exe strtold => timeout
- libctest-glibc: entry-dynamic.exe swprintf => timeout
- libctest-glibc: entry-dynamic.exe tgmath => timeout
- libctest-glibc: entry-dynamic.exe time => timeout
- libctest-glibc: entry-dynamic.exe tls_init => timeout
- libctest-glibc: entry-dynamic.exe tls_local_exec => timeout
- libctest-glibc: entry-dynamic.exe udiv => timeout
- libctest-glibc: entry-dynamic.exe ungetc => timeout
- libctest-glibc: entry-dynamic.exe utime => timeout
- libctest-glibc: entry-dynamic.exe wcsstr => timeout
- libctest-glibc: entry-dynamic.exe wcstol => timeout
- libctest-glibc: entry-dynamic.exe daemon_failure => timeout
- libctest-glibc: entry-dynamic.exe dn_expand_empty => timeout
- libctest-glibc: entry-dynamic.exe dn_expand_ptr_0 => timeout
- libctest-glibc: entry-dynamic.exe fflush_exit => timeout
- libctest-glibc: entry-dynamic.exe fgets_eof => timeout
- libctest-glibc: entry-dynamic.exe fgetwc_buffering => timeout
- libctest-glibc: entry-dynamic.exe fpclassify_invalid_ld80 => timeout
- libctest-glibc: entry-dynamic.exe ftello_unflushed_append => timeout
- libctest-glibc: entry-dynamic.exe getpwnam_r_crash => timeout
- libctest-glibc: entry-dynamic.exe getpwnam_r_errno => timeout
- libctest-glibc: entry-dynamic.exe iconv_roundtrips => timeout
- libctest-glibc: entry-dynamic.exe inet_ntop_v4mapped => timeout
- libctest-glibc: entry-dynamic.exe inet_pton_empty_last_field => timeout
- libctest-glibc: entry-dynamic.exe iswspace_null => timeout
- libctest-glibc: entry-dynamic.exe lrand48_signextend => timeout
- libctest-glibc: entry-dynamic.exe lseek_large => timeout
- libctest-glibc: entry-dynamic.exe malloc_0 => timeout
- libctest-glibc: entry-dynamic.exe mbsrtowcs_overflow => timeout
- libctest-glibc: entry-dynamic.exe memmem_oob_read => timeout
- libctest-glibc: entry-dynamic.exe memmem_oob => timeout
- libctest-glibc: entry-dynamic.exe mkdtemp_failure => timeout
- libctest-glibc: entry-dynamic.exe mkstemp_failure => timeout
- libctest-glibc: entry-dynamic.exe printf_1e9_oob => timeout
- libctest-glibc: entry-dynamic.exe printf_fmt_g_round => timeout
- libctest-glibc: entry-dynamic.exe printf_fmt_g_zeros => timeout
- libctest-glibc: entry-dynamic.exe printf_fmt_n => timeout
- libctest-glibc: entry-dynamic.exe pthread_robust_detach => timeout
- libctest-glibc: entry-dynamic.exe pthread_cond_smasher => timeout
- libctest-glibc: entry-dynamic.exe pthread_exit_cancel => 134
- libctest-glibc: entry-dynamic.exe regex_ere_backref => 1
- libctest-glibc: entry-dynamic.exe regex_escaped_high_byte => 1
- libctest-glibc: entry-dynamic.exe setvbuf_unget => timeout

### 官方 libctest judge 未通过（libctest-musl，含缺失/未运行/无 Pass!）
- libctest dynamic crypt
- libctest static crypt
- libctest static pleval

### 官方 libctest judge 未通过（libctest-glibc，含缺失/未运行/无 Pass!）
- libctest dynamic clocale_mbfuncs
- libctest dynamic crypt
- libctest dynamic daemon_failure
- libctest dynamic dn_expand_empty
- libctest dynamic dn_expand_ptr_0
- libctest dynamic fflush_exit
- libctest dynamic fgets_eof
- libctest dynamic fgetwc_buffering
- libctest dynamic fnmatch
- libctest dynamic fpclassify_invalid_ld80
- libctest dynamic fscanf
- libctest dynamic ftello_unflushed_append
- libctest dynamic fwscanf
- libctest dynamic getpwnam_r_crash
- libctest dynamic getpwnam_r_errno
- libctest dynamic iconv_roundtrips
- libctest dynamic inet_ntop_v4mapped
- libctest dynamic inet_pton_empty_last_field
- libctest dynamic iswspace_null
- libctest dynamic lrand48_signextend
- libctest dynamic lseek_large
- libctest dynamic malloc_0
- libctest dynamic mbc
- libctest dynamic mbsrtowcs_overflow
- libctest dynamic memmem_oob
- libctest dynamic memmem_oob_read
- libctest dynamic mkdtemp_failure
- libctest dynamic mkstemp_failure
- libctest dynamic printf_1e9_oob
- libctest dynamic printf_fmt_g_round
- libctest dynamic printf_fmt_g_zeros
- libctest dynamic printf_fmt_n
- libctest dynamic pthread_cancel
- libctest dynamic pthread_cancel_points
- libctest dynamic pthread_cond_smasher
- libctest dynamic pthread_exit_cancel
- libctest dynamic pthread_robust_detach
- libctest dynamic regex_ere_backref
- libctest dynamic regex_escaped_high_byte
- libctest dynamic setvbuf_unget
- libctest dynamic socket
- libctest dynamic sscanf
- libctest dynamic sscanf_long
- libctest dynamic stat
- libctest dynamic strftime
- libctest dynamic string
- libctest dynamic string_memcpy
- libctest dynamic string_memmem
- libctest dynamic string_memset
- libctest dynamic string_strchr
- libctest dynamic string_strcspn
- libctest dynamic string_strstr
- libctest dynamic strptime
- libctest dynamic strtod
- libctest dynamic strtod_simple
- libctest dynamic strtof
- libctest dynamic strtol
- libctest dynamic strtold
- libctest dynamic swprintf
- libctest dynamic tgmath
- libctest dynamic time
- libctest dynamic tls_init
- libctest dynamic tls_local_exec
- libctest dynamic udiv
- libctest dynamic ungetc
- libctest dynamic utime
- libctest dynamic wcsstr
- libctest dynamic wcstol
- libctest static clocale_mbfuncs
- libctest static crypt
- libctest static daemon_failure
- libctest static dn_expand_empty
- libctest static dn_expand_ptr_0
- libctest static fgetwc_buffering
- libctest static fnmatch
- libctest static fscanf
- libctest static fwscanf
- libctest static mbc
- libctest static pleval
- libctest static pthread_cancel_points
- libctest static regex_ere_backref
- libctest static regex_escaped_high_byte
- libctest static setvbuf_unget
- libctest static sscanf
- libctest static strftime
- libctest static strtol
- libctest static swprintf
- libctest static wcstol

### busybox 失败
- 无

### 官方组非零退出
- unixbench-musl => status 137
- unixbench-glibc => status 137

### autorun 故障细节
- unixbench-musl: /tmp/t/m/unixbench/unixbench_testcode.sh timed out after 300s
- unixbench-glibc: /tmp/t/g/unixbench/unixbench_testcode.sh timed out after 300s

### 官方组超时
- unixbench-musl => timeout after 300s
- unixbench-glibc => timeout after 300s

### panic/trap
- 无

### ENOSYS/not implemented
- 无
