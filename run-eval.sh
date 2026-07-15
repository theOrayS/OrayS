#!/usr/bin/env bash
set -euo pipefail

ARCH="${1:-rv}"
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
TESTSUITE_DIR="${TESTSUITE_DIR:-$SCRIPT_DIR/../testsuits-for-oskernel}"
RV_IMG="${RV_TESTSUITE_IMG:-$TESTSUITE_DIR/sdcard-rv.img}"
LA_IMG="${LA_TESTSUITE_IMG:-$TESTSUITE_DIR/sdcard-la.img}"
RUN_EVAL_TIMEOUT_SECS="${RUN_EVAL_TIMEOUT_SECS:-14400}"
RUN_EVAL_OUTPUT_ROOT="${RUN_EVAL_OUTPUT_ROOT:-$SCRIPT_DIR/build/eval}"
export PATH="$HOME/.cargo/bin:$PATH"
# Match the official remote `make all` timeout ceiling for full local evaluator
# runs. Some official groups have a real 900s in-kernel group budget; callers
# that want a shorter scouting run can still override this variable explicitly.
export OSCOMP_GROUP_TIMEOUT_CEILING_SECS="${OSCOMP_GROUP_TIMEOUT_CEILING_SECS:-900}"

usage() {
    printf '用法: %s [rv|la]\n' "$0" >&2
    printf '可选环境变量: RV_TESTSUITE_IMG=/path/sdcard-rv.img LA_TESTSUITE_IMG=/path/sdcard-la.img\n' >&2
    printf 'blacklist sweep 可选: LTP_BLACKLIST_FILE/LTP_BLACKLIST_COMMON_FILE + LTP_BLACKLIST_RV_FILE 或 LTP_BLACKLIST_LA_FILE\n' >&2
}

need_cmd() {
    if ! command -v "$1" >/dev/null 2>&1; then
        printf '错误: 未找到 %s。请先安装依赖后再运行评测。\n' "$1" >&2
        return 2
    fi
}

need_file() {
    if [ ! -f "$1" ]; then
        printf '错误: 未找到评测镜像: %s\n' "$1" >&2
        printf '提示: 请先生成/复制对应 sdcard 镜像，或通过 %s 指定路径。\n' "$2" >&2
        return 2
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

case "$ARCH" in
rv)
    missing=0
    need_cmd qemu-img || missing=1
    need_cmd qemu-system-riscv64 || missing=1
    need_file "$RV_IMG" RV_TESTSUITE_IMG || missing=1
    [ "$missing" -eq 0 ] || exit 2
    compose_ltp_blacklist_files rv
    make_target="run-rv"
    make_arch="riscv64"
    image_variable="RV_TESTSUITE_IMG"
    overlay_variable="RV_TESTSUITE_RUN_IMG"
    image="$RV_IMG"
    ;;
la)
    missing=0
    need_cmd qemu-img || missing=1
    need_cmd qemu-system-loongarch64 || missing=1
    need_file "$LA_IMG" LA_TESTSUITE_IMG || missing=1
    [ "$missing" -eq 0 ] || exit 2
    compose_ltp_blacklist_files la
    make_target="run-la"
    make_arch="loongarch64"
    image_variable="LA_TESTSUITE_IMG"
    overlay_variable="LA_TESTSUITE_RUN_IMG"
    image="$LA_IMG"
    ;;
*)
    usage
    exit 2
    ;;
esac

mkdir -p "$RUN_EVAL_OUTPUT_ROOT"
run_dir="$(mktemp -d "$RUN_EVAL_OUTPUT_ROOT/${ARCH}.XXXXXX")"
overlay="$run_dir/sdcard-overlay.qcow2"
raw_log="$run_dir/evaluator.log"
summary_json="$run_dir/ltp-summary.json"
summary_md="$run_dir/ltp-summary.md"
failure_report="$run_dir/failure-report.md"

cleanup() {
    rm -f -- "$overlay"
}
supervisor_pid=""
on_signal() {
    signal_number="$1"
    trap - HUP INT TERM
    if [ -n "$supervisor_pid" ] && kill -0 "$supervisor_pid" 2>/dev/null; then
        if ! kill -"$signal_number" "$supervisor_pid" 2>/dev/null; then
            :
        fi
        if ! wait "$supervisor_pid"; then
            :
        fi
    fi
    cleanup
    exit "$((128 + signal_number))"
}
trap cleanup EXIT
trap 'on_signal 1' HUP
trap 'on_signal 2' INT
trap 'on_signal 15' TERM

printf 'Evaluator evidence directory: %s\n' "$run_dir"
printf 'Base image (read-only backing file): %s\n' "$image"
printf 'Run timeout: %ss\n' "$RUN_EVAL_TIMEOUT_SECS"

command=(
    make "$make_target"
    "ARCH=$make_arch"
    SMP=1
    MEM=1G
    "$image_variable=$image"
    "$overlay_variable=$overlay"
)

set +e
python3 "$SCRIPT_DIR/scripts/semantic_evidence.py" supervise \
    --timeout "$RUN_EVAL_TIMEOUT_SECS" \
    --log "$raw_log" \
    -- "${command[@]}" &
supervisor_pid=$!
wait "$supervisor_pid"
runner_status=$?
supervisor_pid=""

python3 "$SCRIPT_DIR/scripts/ltp_summary.py" --require-clean --json "$raw_log" >"$summary_json"
parser_status=$?
python3 "$SCRIPT_DIR/scripts/ltp_summary.py" "$raw_log" >"$summary_md"
markdown_status=$?
python3 "$SCRIPT_DIR/scripts/eval_failure_report.py" --require-clean "$raw_log" -o "$failure_report"
failure_report_status=$?
set -e

printf 'Raw log: %s\n' "$raw_log"
printf 'Strict JSON summary: %s\n' "$summary_json"
printf 'Human summary: %s\n' "$summary_md"
printf 'Failure report: %s\n' "$failure_report"

if [ "$runner_status" -ne 0 ]; then
    printf 'Evaluator process failed: status=%s\n' "$runner_status" >&2
    exit "$runner_status"
fi
if [ "$parser_status" -eq 2 ] || [ "$markdown_status" -eq 2 ] || [ "$failure_report_status" -eq 2 ]; then
    printf 'Evaluator evidence is malformed or incomplete\n' >&2
    exit 2
fi
if [ "$parser_status" -ne 0 ] || [ "$failure_report_status" -ne 0 ]; then
    printf 'Evaluator completed with semantic non-pass evidence\n' >&2
    exit 1
fi
exit 0
