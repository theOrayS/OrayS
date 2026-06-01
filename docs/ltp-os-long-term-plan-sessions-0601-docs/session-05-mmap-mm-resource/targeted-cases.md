# Session 5 targeted cases

单位：LTP case。列表中的 case 名来自本 session 实际 scout、postfix、final combined gate 或明确未推广分类。

## Initial RV mmap/mm scout (14 cases)

```text
diotest1
diotest2
diotest3
diotest4
diotest5
diotest6
mprotect05
mmap001
mmap15
mmap17
mmap19
mincore01
mprotect01
mprotect02
```

## RV mincore postfix (1 case)

```text
mincore01
```

## Final combined RV/LA promotion + adjacent regression gate (22 cases)

新增候选 11 cases + 相邻 stable 回归 11 cases：

```text
diotest1
diotest2
diotest3
diotest5
diotest6
mprotect05
mmap001
mmap15
mmap17
mmap19
mincore01
mmap01
mmap02
mmap03
mmap06
mmap09
mmap10
mmap11
mlock01
mlock03
mlock04
munlock01
```

## Explicitly classified but not promoted

```text
diotest4
mprotect01
mprotect02
```
