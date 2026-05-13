#!/usr/bin/env bash
set -e

ARCH="${1:-rv}"
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"

if [ "$ARCH" = "rv" ]; then
    make run-rv ARCH=riscv64 SMP=1 MEM=1G RV_TESTSUITE_IMG="$SCRIPT_DIR/sdcard-rv.img"
elif [ "$ARCH" = "la" ]; then
    make run-la ARCH=loongarch64 SMP=1 MEM=1G LA_TESTSUITE_IMG="$SCRIPT_DIR/sdcard-la.img"
else
    echo "用法: $0 [rv|la]"
    exit 1
fi
