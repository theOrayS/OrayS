#!/usr/bin/env bash
set -euo pipefail

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

usage() {
    printf '用法: %s [rv|la]\n' "$0" >&2
    printf '可选环境变量: RV_TESTSUITE_IMG=/path/sdcard-rv.img LA_TESTSUITE_IMG=/path/sdcard-la.img\n' >&2
    printf 'blacklist sweep 可选: LTP_BLACKLIST_FILE/LTP_BLACKLIST_COMMON_FILE + LTP_BLACKLIST_RV_FILE 或 LTP_BLACKLIST_LA_FILE\n' >&2
}

need_cmd() {
    if ! command -v "$1" >/dev/null 2>&1; then
        printf '错误: 未找到 %s。请先安装依赖后再运行评测。\n' "$1" >&2
        return 1
    fi
}

need_file() {
    if [ ! -f "$1" ]; then
        printf '错误: 未找到评测镜像: %s\n' "$1" >&2
        printf '提示: 请先生成/复制对应 sdcard 镜像，或通过 %s 指定路径。\n' "$2" >&2
        return 1
    fi
}

# Make the Rust tools installed by rustup visible when this script is launched
# from a non-login shell whose PATH does not include ~/.cargo/bin.
if ! command -v cargo >/dev/null 2>&1; then
    if [ -f "${CARGO_HOME:-$HOME/.cargo}/env" ]; then
        # shellcheck disable=SC1091
        . "${CARGO_HOME:-$HOME/.cargo}/env"
    fi
fi

if ! command -v cargo >/dev/null 2>&1; then
    printf '错误: 未找到 cargo。请先安装 Rust 工具链，或执行: source "%s/env"\n' "${CARGO_HOME:-$HOME/.cargo}" >&2
    exit 1
fi


append_ltp_blacklist_files() {
    local var_name="$1"
    local value="${!var_name:-}"
    local file
    [ -n "$value" ] || return 0
    for file in $value; do
        if [ ! -f "$file" ]; then
            printf '错误: %s 指定的 blacklist 文件不存在: %s\n' "$var_name" "$file" >&2
            exit 1
        fi
        LTP_BLACKLIST="${LTP_BLACKLIST:+$LTP_BLACKLIST$'\n'}$(cat "$file")"
    done
    export LTP_BLACKLIST
}

compose_ltp_blacklist_files() {
    local arch="$1"
    append_ltp_blacklist_files LTP_BLACKLIST_FILE
    append_ltp_blacklist_files LTP_BLACKLIST_COMMON_FILE
    if [ "$arch" = "rv" ]; then
        append_ltp_blacklist_files LTP_BLACKLIST_RV_FILE
    elif [ "$arch" = "la" ]; then
        append_ltp_blacklist_files LTP_BLACKLIST_LA_FILE
    fi
}

if [ "$ARCH" = "rv" ]; then
    RV_IMG="${RV_TESTSUITE_IMG:-$SCRIPT_DIR/sdcard-rv.img}"
    missing=0
    need_cmd qemu-img || missing=1
    need_cmd qemu-system-riscv64 || missing=1
    need_file "$RV_IMG" RV_TESTSUITE_IMG || missing=1
    [ "$missing" -eq 0 ] || exit 1
    compose_ltp_blacklist_files rv
    make run-rv ARCH=riscv64 SMP=1 MEM=1G RV_TESTSUITE_IMG="$RV_IMG"
elif [ "$ARCH" = "la" ]; then
    LA_IMG="${LA_TESTSUITE_IMG:-$SCRIPT_DIR/sdcard-la.img}"
    missing=0
    need_cmd qemu-img || missing=1
    need_cmd qemu-system-loongarch64 || missing=1
    need_file "$LA_IMG" LA_TESTSUITE_IMG || missing=1
    [ "$missing" -eq 0 ] || exit 1
    compose_ltp_blacklist_files la
    make run-la ARCH=loongarch64 SMP=1 MEM=1G LA_TESTSUITE_IMG="$LA_IMG"
else
    usage
    exit 1
fi
