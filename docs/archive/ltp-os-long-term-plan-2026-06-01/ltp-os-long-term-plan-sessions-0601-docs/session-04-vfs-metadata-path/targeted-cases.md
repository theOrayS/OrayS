# Session 4 targeted cases

单位说明：下列条目全部是 LTP case 名，不是“组”或 wrapper 维度；每个 case 在一次 RV/LA gate 中通常覆盖 musl 与 glibc 两个 wrapper 条目。

## Initial RV scout cases (15 cases)

```text
fpathconf01
pathconf01
rename14
mknod08
mknodat01
readlinkat02
statx01
statx03
statx04
getxattr01
listxattr01
statfs01
getdents01
getdents02
readlink03
```

结论：`fpathconf01/pathconf01/rename14/mknod08/mknodat01/readlinkat02` RV clean；`getxattr01/listxattr01/statx03` 有可修复真实语义缺口；`getdents01/readlink03/statx01/statx04/statfs01/getdents02` 暂不推广。

## RV postfix cases after xattr/statx/getdents/readlink repairs (11 cases)

```text
getxattr01
listxattr01
statx03
getdents01
readlink03
fpathconf01
pathconf01
rename14
mknod08
mknodat01
readlinkat02
```

结论：`getxattr01/listxattr01/statx03` 修复后 RV clean；`getdents01/readlink03` 仍失败；`readlinkat02` RV clean 但需 LA 复核。

## RV final combined promotion + adjacent regression gate (35 cases)

```text
fpathconf01
pathconf01
rename14
mknod08
mknodat01
readlinkat02
getxattr01
listxattr01
statx03
stat01
stat02
lstat01
symlink01
readlink01
fstatat01
statx02
lstat01_64
stat01_64
stat02_64
statvfs02
symlink02
symlink04
fstat03
fstat03_64
statfs02
fstatfs02
fstatfs02_64
readlinkat01
symlinkat01
statfs02_64
statfs03
statfs03_64
mknod06
mknod02
mknod05
```

结论：RV musl+glibc 全部 parser-clean。

## LA final combined gate with readlinkat02 included (35 cases)

同 RV final combined 的 35 cases。结论：仅 `ltp-musl:readlinkat02` 失败，故 `readlinkat02` 不进入 stable promotion。

## LA final promotion-clean gate (34 cases)

```text
fpathconf01
pathconf01
rename14
mknod08
mknodat01
getxattr01
listxattr01
statx03
stat01
stat02
lstat01
symlink01
readlink01
fstatat01
statx02
lstat01_64
stat01_64
stat02_64
statvfs02
symlink02
symlink04
fstat03
fstat03_64
statfs02
fstatfs02
fstatfs02_64
readlinkat01
symlinkat01
statfs02_64
statfs03
statfs03_64
mknod06
mknod02
mknod05
```

结论：LA musl+glibc 全部 parser-clean；其中 8 个新增 case 可推广。
