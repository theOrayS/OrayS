#!/usr/bin/env bash
set -euo pipefail

ARCH="${1:-rv}"
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../.." && pwd)"
CALLER_DIR="$PWD"

absolute_path() {
    case "$1" in
        /*) printf '%s\n' "$1" ;;
        *) printf '%s/%s\n' "$CALLER_DIR" "$1" ;;
    esac
}

TESTSUITE_DIR="$(absolute_path "${TESTSUITE_DIR:-$REPO_ROOT/../testsuits-for-oskernel}")"
RV_IMG="$(absolute_path "${RV_TESTSUITE_IMG:-$TESTSUITE_DIR/sdcard-rv.img}")"
LA_IMG="$(absolute_path "${LA_TESTSUITE_IMG:-$TESTSUITE_DIR/sdcard-la.img}")"
OUTPUT_DIR="$(absolute_path "${ORAYS_TEST_OUTPUT_DIR:-$REPO_ROOT/test/output/official}")"

export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_NET_OFFLINE=true
export OSCOMP_GROUP_TIMEOUT_CEILING_SECS="${OSCOMP_GROUP_TIMEOUT_CEILING_SECS:-900}"
unset MAKEFLAGS MFLAGS GNUMAKEFLAGS
unset RUSTFLAGS RUSTDOCFLAGS CARGO_ENCODED_RUSTFLAGS
unset CARGO_BUILD_RUSTC_WRAPPER RUSTC_WRAPPER RUSTC_WORKSPACE_WRAPPER
unset BASH_ENV ENV
for function_name in make cargo qemu-img qemu-system-riscv64 qemu-system-loongarch64; do
    if declare -F "$function_name" >/dev/null; then
        unset -f "$function_name"
    fi
done
unset KERNEL_APP KERNEL_FEATURES KERNEL_RV_FEATURES KERNEL_LA_FEATURES
unset KERNEL_APP_FEATURES KERNEL_RV_APP_FEATURES KERNEL_LA_APP_FEATURES
unset KERNEL_MODE KERNEL_LOG PLAT_CONFIG
unset KERNEL_BUILD_DIR KERNEL_TARGET_DIR KERNEL_RV_OUT_DIR KERNEL_LA_OUT_DIR
unset KERNEL_RV_CONFIG KERNEL_LA_CONFIG KERNEL_RV_TARGET_DIR KERNEL_LA_TARGET_DIR
unset KERNEL_RV_AXCONFIG_WRITES KERNEL_LA_AXCONFIG_WRITES KERNEL_RV KERNEL_LA
unset RV_AUX_DISK LA_AUX_DISK RV_NETDEV_ARGS LA_NETDEV_ARGS LA_HOSTFWD_ARGS
unset CARGO_HOME CARGO_TARGET_DIR CARGO_BUILD_TARGET RUSTUP_TOOLCHAIN
unset RUSTC RUSTDOC CARGO RUSTC_BOOTSTRAP

infrastructure_error() {
    printf 'infrastructure error: %s\n' "$1" >&2
    exit 125
}

usage() {
    printf 'Usage: %s [rv|la]\n' "$0" >&2
    printf 'Image overrides: RV_TESTSUITE_IMG=/path/sdcard-rv.img or LA_TESTSUITE_IMG=/path/sdcard-la.img\n' >&2
    printf 'Shared image directory: TESTSUITE_DIR=/path/to/testsuits-for-oskernel\n' >&2
    printf 'Output directory: ORAYS_TEST_OUTPUT_DIR=/path/to/output\n' >&2
    printf 'Optional blacklist files: LTP_BLACKLIST_FILE/LTP_BLACKLIST_COMMON_FILE plus the architecture-specific file\n' >&2
}

need_cmd() {
    if ! command -v "$1" >/dev/null 2>&1; then
        printf 'infrastructure error: required command not found: %s\n' "$1" >&2
        return 1
    fi
}

need_file() {
    if [ ! -f "$1" ] || [ ! -r "$1" ]; then
        printf 'infrastructure error: official evaluation image not found: %s\n' "$1" >&2
        printf 'set %s or TESTSUITE_DIR to a readable local image; no download is attempted\n' "$2" >&2
        return 1
    fi
}

append_ltp_blacklist_files() {
    local var_name="$1"
    local value="${!var_name:-}"
    local file
    [ -n "$value" ] || return 0
    for file in $value; do
        if [ ! -f "$file" ] || [ ! -r "$file" ]; then
            infrastructure_error "$var_name names a missing or unreadable blacklist file: $file"
        fi
        if ! blacklist_content="$(cat -- "$file")"; then
            infrastructure_error "cannot read blacklist file named by $var_name: $file"
        fi
        LTP_BLACKLIST="${LTP_BLACKLIST:+$LTP_BLACKLIST$'\n'}$blacklist_content"
    done
    export LTP_BLACKLIST
}

compose_ltp_blacklist_files() {
    local arch="$1"
    append_ltp_blacklist_files LTP_BLACKLIST_FILE
    append_ltp_blacklist_files LTP_BLACKLIST_COMMON_FILE
    if [ "$arch" = "rv" ]; then
        append_ltp_blacklist_files LTP_BLACKLIST_RV_FILE
    else
        append_ltp_blacklist_files LTP_BLACKLIST_LA_FILE
    fi
}

if [ "$#" -gt 1 ] || { [ "$ARCH" != "rv" ] && [ "$ARCH" != "la" ]; }; then
    usage
    exit 125
fi

if ! command -v cargo >/dev/null 2>&1; then
    if [ -f "${CARGO_HOME:-$HOME/.cargo}/env" ]; then
        # shellcheck disable=SC1091
        . "${CARGO_HOME:-$HOME/.cargo}/env"
    fi
fi

missing=0
for command in make cargo qemu-img; do
    if ! need_cmd "$command"; then
        missing=1
    fi
done

if [ "$ARCH" = "rv" ]; then
    if ! need_cmd qemu-system-riscv64; then
        missing=1
    fi
    if ! need_file "$RV_IMG" RV_TESTSUITE_IMG; then
        missing=1
    fi
else
    if ! need_cmd qemu-system-loongarch64; then
        missing=1
    fi
    if ! need_file "$LA_IMG" LA_TESTSUITE_IMG; then
        missing=1
    fi
fi
[ "$missing" -eq 0 ] || exit 125

if ! mkdir -p "$OUTPUT_DIR"; then
    printf 'infrastructure error: cannot create official output directory: %s\n' "$OUTPUT_DIR" >&2
    exit 125
fi

compose_ltp_blacklist_files "$ARCH"
run_image="$OUTPUT_DIR/sdcard-${ARCH}.$$.run.qcow2"
cleanup() {
    rm -f -- "$run_image"
}
trap cleanup EXIT

if [ "$ARCH" = "rv" ]; then
    if command make -C "$REPO_ROOT" run-rv ARCH=riscv64 KERNEL_SMP=1 RV_MEM=1G \
        RV_TESTSUITE_IMG="$RV_IMG" RV_TESTSUITE_RUN_IMG="$run_image"; then
        status=0
    else
        status=$?
    fi
else
    if command make -C "$REPO_ROOT" run-la ARCH=loongarch64 KERNEL_SMP=1 LA_MEM=1G \
        LA_TESTSUITE_IMG="$LA_IMG" LA_TESTSUITE_RUN_IMG="$run_image"; then
        status=0
    else
        status=$?
    fi
fi

exit "$status"
