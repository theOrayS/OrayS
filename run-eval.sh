#!/usr/bin/env bash
set -e

ARCH="${1:-rv}"
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
TESTSUITE_DIR="${TESTSUITE_DIR:-$SCRIPT_DIR/../testsuits-for-oskernel}"
RV_IMG="${RV_TESTSUITE_IMG:-$TESTSUITE_DIR/sdcard-rv.img}"
LA_IMG="${LA_TESTSUITE_IMG:-$TESTSUITE_DIR/sdcard-la.img}"
export PATH="$HOME/.cargo/bin:$PATH"

require_img() {
    if [ ! -f "$1" ]; then
        echo "缺少评测镜像: $1"
        echo "可通过 TESTSUITE_DIR 或 RV_TESTSUITE_IMG/LA_TESTSUITE_IMG 指定路径"
        exit 1
    fi
}

if [ "$ARCH" = "rv" ]; then
    require_img "$RV_IMG"
    make run-rv ARCH=riscv64 SMP=1 MEM=1G RV_TESTSUITE_IMG="$RV_IMG"
elif [ "$ARCH" = "la" ]; then
    require_img "$LA_IMG"
    make run-la ARCH=loongarch64 SMP=1 MEM=1G LA_TESTSUITE_IMG="$LA_IMG"
else
    echo "用法: $0 [rv|la]"
    echo "可选环境变量: TESTSUITE_DIR RV_TESTSUITE_IMG LA_TESTSUITE_IMG"
    exit 1
fi
