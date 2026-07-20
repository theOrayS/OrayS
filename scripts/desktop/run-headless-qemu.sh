#!/usr/bin/env bash
set -euo pipefail

repo_root=$(git -C "$(dirname "${BASH_SOURCE[0]}")" rev-parse --show-toplevel)
arch=
scenario=
run_dir=
timeout_seconds=${DESKTOP_QEMU_TIMEOUT_SECS:-90}
required_qemu_version=9.2.4

usage() {
    printf '%s\n' \
        'usage: run-headless-qemu.sh --arch {rv|la} --scenario {boot|launcher|overlap|applications|resize} [--output DIR]'
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --arch)
            arch=${2:-}
            shift 2
            ;;
        --scenario)
            scenario=${2:-}
            shift 2
            ;;
        --output)
            run_dir=${2:-}
            shift 2
            ;;
        --help)
            usage
            exit 0
            ;;
        *)
            printf 'unknown argument: %s\n' "$1" >&2
            usage >&2
            exit 2
            ;;
    esac
done

case "$arch" in
    rv|la) ;;
    *)
        printf 'unsupported desktop architecture: %s\n' "$arch" >&2
        exit 2
        ;;
esac
case "$scenario" in
    boot|launcher|overlap|applications|resize) ;;
    *)
        printf 'unsupported desktop scenario: %s\n' "$scenario" >&2
        exit 2
        ;;
esac
if [[ ! "$timeout_seconds" =~ ^[1-9][0-9]*$ ]]; then
    printf 'DESKTOP_QEMU_TIMEOUT_SECS must be a positive integer\n' >&2
    exit 2
fi

qemu_binary=qemu-system-riscv64
artifact="$repo_root/build/desktop/rv/artifacts/orays-desktop-rv.bin"
vdev_suffix=device
vnc_display=${DESKTOP_VNC_DISPLAY:-42}
if [[ "$arch" == la ]]; then
    qemu_binary=qemu-system-loongarch64
    artifact="$repo_root/build/desktop/la/artifacts/orays-desktop-la.elf"
    vdev_suffix=pci
    vnc_display=${DESKTOP_VNC_DISPLAY:-43}
fi
if [[ ! "$vnc_display" =~ ^[0-9]+$ ]]; then
    printf 'DESKTOP_VNC_DISPLAY must be a non-negative integer\n' >&2
    exit 2
fi

if [[ -z "$run_dir" ]]; then
    run_dir=$(python3 "$repo_root/scripts/desktop/create-run-dir.py" \
        --repo-root "$repo_root" --prefix "qemu-${arch}-${scenario}.")
else
    run_dir=$(python3 "$repo_root/scripts/desktop/create-run-dir.py" \
        --repo-root "$repo_root" --candidate "$run_dir")
fi

qemu_pid=
qemu_started=false
qemu_exit=
qmp_runtime_dir=
failure_stage=runtime-prerequisites
failure_reason=
cleanup() {
    if [[ -n "$qemu_pid" ]]; then
        if kill -0 "$qemu_pid" 2>/dev/null; then
            kill "$qemu_pid" 2>/dev/null || true
        fi
        local stopped_exit=0
        wait "$qemu_pid" 2>/dev/null || stopped_exit=$?
        if [[ -z "$qemu_exit" || "$qemu_exit" == 0 ]]; then
            qemu_exit=$stopped_exit
        fi
        qemu_pid=
    fi
    if [[ -n "$qmp_runtime_dir" ]]; then
        rm -f "$qmp_runtime_dir/qmp.sock"
        rmdir "$qmp_runtime_dir" 2>/dev/null || true
        qmp_runtime_dir=
    fi
}
finalize() {
    runner_exit=$?
    trap - EXIT INT TERM
    cleanup
    set +e
    finalizer_args=(
        --repo-root "$repo_root"
        --run-dir "$run_dir"
        --arch "$arch"
        --scenario "$scenario"
        --qemu-binary "$qemu_binary"
        --required-qemu-version "$required_qemu_version"
        --qemu-started "$qemu_started"
        --runner-exit "$runner_exit"
        --failure-stage "$failure_stage"
    )
    if [[ -n "$failure_reason" ]]; then
        finalizer_args+=(--failure-reason "$failure_reason")
    fi
    if [[ "$qemu_started" == true ]]; then
        finalizer_args+=(--qemu-exit "$qemu_exit")
    fi
    python3 "$repo_root/scripts/desktop/finalize-runtime-evidence.py" \
        "${finalizer_args[@]}"
    finalize_exit=$?
    if (( finalize_exit >= 70 )); then
        printf 'runtime evidence finalization failed; exit=%s evidence=%s\n' \
            "$finalize_exit" "$run_dir" >&2
        runner_exit=$finalize_exit
    elif (( runner_exit == 0 && finalize_exit != 0 )); then
        runner_exit=$finalize_exit
    fi
    exit "$runner_exit"
}
trap finalize EXIT
trap 'failure_stage=runner-signal; failure_reason=signal_interrupted; exit 130' INT
trap 'failure_stage=runner-signal; failure_reason=signal_terminated; exit 143' TERM

sequence="$repo_root/test/desktop/fixtures/input/${scenario}.json"
cp "$sequence" "$run_dir/input-sequence.json"
: >"$run_dir/serial.log"
: >"$run_dir/qmp-input.jsonl"
: >"$run_dir/qmp-capture.jsonl"

unset DISPLAY WAYLAND_DISPLAY DBUS_SESSION_BUS_ADDRESS
if [[ -n "${DESKTOP_REQUIRED_QEMU_VERSION+x}" \
    && "$DESKTOP_REQUIRED_QEMU_VERSION" != "$required_qemu_version" ]]; then
    failure_reason=qemu_version_override_rejected
    printf 'DESKTOP_REQUIRED_QEMU_VERSION must be exactly %s\n' \
        "$required_qemu_version" >&2
    exit 3
fi
for command_name in python3 qemu-img mkfs.fat timeout sha256sum; do
    command -v "$command_name" >/dev/null 2>&1 || {
        failure_reason=missing_runtime_prerequisite
        printf 'missing required command: %s\n' "$command_name" >&2
        exit 3
    }
done
command -v "$qemu_binary" >/dev/null 2>&1 || {
    failure_reason=missing_qemu_binary
    printf 'missing required QEMU: %s\n' "$qemu_binary" >&2
    exit 3
}
qemu_version=$($qemu_binary --version | sed -n '1p')
if [[ "$qemu_version" != "QEMU emulator version ${required_qemu_version}" ]]; then
    failure_reason=qemu_version_mismatch
    printf 'required QEMU %s, got: %s\n' "$required_qemu_version" "$qemu_version" >&2
    exit 3
fi

failure_stage=runtime-metadata-initial
python3 "$repo_root/scripts/desktop/collect-runtime-metadata.py" \
    --repo-root "$repo_root" \
    --output "$run_dir/runtime-metadata.json" \
    --arch "$arch" \
    --scenario "$scenario" \
    --qemu-binary "$qemu_binary" \
    --required-qemu-version "$required_qemu_version" \
    --run-dir "$run_dir"

failure_stage=desktop-build
"$repo_root/scripts/desktop/build.sh" "$arch"
test -s "$artifact"

failure_stage=qmp-runtime-setup
qmp_runtime_dir=$(python3 "$repo_root/scripts/desktop/create-qmp-runtime-dir.py")
qmp_socket="$qmp_runtime_dir/qmp.sock"
serial_log="$run_dir/serial.log"
disk_image="$run_dir/disk.img"
frame="$run_dir/frame.ppm"

failure_stage=disk-setup
qemu-img create -q -f raw "$disk_image" 64M
mkfs.fat -F 32 "$disk_image" >"$run_dir/mkfs.log"

qemu_args=(
    -machine virt
    -m 1G
    -smp 1
    -kernel "$artifact"
    -device "virtio-blk-${vdev_suffix},drive=disk0"
    -drive "id=disk0,if=none,format=raw,file=$disk_image"
    -device "virtio-gpu-${vdev_suffix},id=gpu0"
    -device "virtio-keyboard-${vdev_suffix}"
    -device "virtio-tablet-${vdev_suffix}"
    -vga none
    -display "vnc=127.0.0.1:${vnc_display}"
    -qmp "unix:${qmp_socket},server=on,wait=off"
    -monitor none
    -serial stdio
    -no-reboot
)
if [[ "$arch" == rv ]]; then
    qemu_args=(-bios default "${qemu_args[@]}")
else
    # The LA PCI MMIO aperture is 128 KiB. QEMU's implicit, unused VirtIO NIC
    # consumes the BAR space required by the explicitly requested tablet.
    qemu_args+=(-nic none)
fi

printf 'QEMU_RUN_DIR=%s\n' "$run_dir"
failure_stage=qemu-boot
timeout --signal=TERM --kill-after=5 "$timeout_seconds" \
    "$qemu_binary" "${qemu_args[@]}" >"$serial_log" 2>&1 &
qemu_pid=$!
qemu_started=true

boot_deadline=$((SECONDS + 35))
while ! grep -q 'ORAYS_DESKTOP_FRAME boot ' "$serial_log" 2>/dev/null; do
    if ! kill -0 "$qemu_pid" 2>/dev/null; then
        wait "$qemu_pid" || qemu_exit=$?
        printf 'QEMU exited before desktop boot marker; exit=%s evidence=%s\n' "${qemu_exit:-0}" "$run_dir" >&2
        exit 1
    fi
    if (( SECONDS >= boot_deadline )); then
        printf 'desktop boot marker timed out; evidence=%s\n' "$run_dir" >&2
        exit 1
    fi
    sleep 0.2
done

display_line=$(grep -aF 'ORAYS_DESKTOP_DISPLAY width=' "$serial_log" | tail -n 1 || true)
display_width=$(sed -n 's/.*ORAYS_DESKTOP_DISPLAY width=\([0-9][0-9]*\) height=\([0-9][0-9]*\).*/\1/p' <<<"$display_line")
display_height=$(sed -n 's/.*ORAYS_DESKTOP_DISPLAY width=\([0-9][0-9]*\) height=\([0-9][0-9]*\).*/\2/p' <<<"$display_line")
if [[ -z "$display_width" || -z "$display_height" ]]; then
    printf 'desktop display geometry marker missing or invalid; evidence=%s\n' "$run_dir" >&2
    exit 1
fi
printf 'DISPLAY_GEOMETRY=%sx%s\n' "$display_width" "$display_height" >"$run_dir/display-geometry.txt"
initial_display_marker="ORAYS_DESKTOP_DISPLAY width=${display_width} height=${display_height}"

input_deadline=$((SECONDS + 10))
while ! grep -aFq 'ORAYS_DESKTOP_INPUT_READY devices=2' "$serial_log" 2>/dev/null; do
    if ! kill -0 "$qemu_pid" 2>/dev/null; then
        wait "$qemu_pid" || qemu_exit=$?
        printf 'QEMU exited before desktop input readiness; exit=%s evidence=%s\n' "${qemu_exit:-0}" "$run_dir" >&2
        exit 1
    fi
    if (( SECONDS >= input_deadline )); then
        printf 'desktop input readiness timed out; expected devices=2 evidence=%s\n' "$run_dir" >&2
        exit 1
    fi
    sleep 0.2
done

failure_stage=input-injection
if [[ "$scenario" != resize ]]; then
    python3 "$repo_root/scripts/desktop/inject-input.py" \
        --sequence "$run_dir/input-sequence.json" \
        --socket "$qmp_socket" \
        --transcript "$run_dir/qmp-input.jsonl" \
        --display-width "$display_width" \
        --display-height "$display_height" \
        --timeout 15
fi

if [[ "$scenario" == resize ]]; then
    failure_stage=runtime-resize
    resize_width=900
    resize_height=650
    python3 "$repo_root/scripts/desktop/vnc-resize.py" \
        --host 127.0.0.1 \
        --display "$vnc_display" \
        --evidence "$run_dir/vnc-resize.json" \
        --width "$resize_width" \
        --height "$resize_height" \
        --timeout 15
    resize_deadline=$((SECONDS + 20))
    resize_marker="ORAYS_DESKTOP_DISPLAY_CHANGED width=${resize_width} height=${resize_height}"
    while ! grep -aFq "$resize_marker" "$serial_log" 2>/dev/null; do
        if ! kill -0 "$qemu_pid" 2>/dev/null; then
            wait "$qemu_pid" || qemu_exit=$?
            printf 'QEMU exited before runtime resize marker; exit=%s evidence=%s\n' "${qemu_exit:-0}" "$run_dir" >&2
            exit 1
        fi
        if (( SECONDS >= resize_deadline )); then
            printf 'guest runtime resize timed out: %s evidence=%s\n' "$resize_marker" "$run_dir" >&2
            exit 1
        fi
        sleep 0.2
    done
    display_width=$resize_width
    display_height=$resize_height
    printf 'DISPLAY_GEOMETRY=%sx%s\n' "$display_width" "$display_height" >"$run_dir/display-geometry.txt"
    python3 "$repo_root/scripts/desktop/inject-input.py" \
        --sequence "$run_dir/input-sequence.json" \
        --socket "$qmp_socket" \
        --transcript "$run_dir/qmp-input.jsonl" \
        --display-width "$display_width" \
        --display-height "$display_height" \
        --timeout 15
    resize_input_marker='position: Point { x: 450, y: 325 }'
    input_after_resize_deadline=$((SECONDS + 10))
    while ! grep -aFq "$resize_input_marker" "$serial_log" 2>/dev/null; do
        if ! kill -0 "$qemu_pid" 2>/dev/null; then
            wait "$qemu_pid" || qemu_exit=$?
            printf 'QEMU exited before post-resize input marker; exit=%s evidence=%s\n' "${qemu_exit:-0}" "$run_dir" >&2
            exit 1
        fi
        if (( SECONDS >= input_after_resize_deadline )); then
            printf 'post-resize input marker timed out: %s evidence=%s\n' "$resize_input_marker" "$run_dir" >&2
            exit 1
        fi
        sleep 0.2
    done
fi

if [[ "$scenario" != boot ]]; then
    failure_stage=guest-action
    expected_marker=
    case "$scenario" in
        launcher) expected_marker='ORAYS_DESKTOP_ACTION LAUNCHER OPEN' ;;
        overlap) expected_marker='ORAYS_DESKTOP_ACTION ALT_TAB reverse=false' ;;
        applications) expected_marker='ORAYS_DESKTOP_ACTION THEME Light' ;;
        resize) expected_marker="ORAYS_DESKTOP_DISPLAY_CHANGED width=${display_width} height=${display_height}" ;;
    esac
    action_deadline=$((SECONDS + 25))
    while ! grep -aFq "$expected_marker" "$serial_log" 2>/dev/null; do
        if ! kill -0 "$qemu_pid" 2>/dev/null; then
            wait "$qemu_pid" || qemu_exit=$?
            printf 'QEMU exited before action marker; exit=%s evidence=%s\n' "${qemu_exit:-0}" "$run_dir" >&2
            exit 1
        fi
        if (( SECONDS >= action_deadline )); then
            printf 'guest action marker timed out: %s evidence=%s\n' "$expected_marker" "$run_dir" >&2
            exit 1
        fi
        sleep 0.2
    done
fi

capture_precondition_args=()
if [[ "$scenario" == launcher ]]; then
    stable_marker='ORAYS_DESKTOP_STATE LAUNCHER OPEN_STABLE'
    stable_deadline=$((SECONDS + 25))
    # LA64 serial output uses CRLF; the fail-closed Python checks below still
    # require one exact logical line after universal-newline decoding.
    while ! grep -aFq "$stable_marker" "$serial_log" 2>/dev/null; do
        if ! kill -0 "$qemu_pid" 2>/dev/null; then
            wait "$qemu_pid" || qemu_exit=$?
            printf 'QEMU exited before launcher stable marker; exit=%s evidence=%s\n' "${qemu_exit:-0}" "$run_dir" >&2
            exit 1
        fi
        if (( SECONDS >= stable_deadline )); then
            printf 'guest launcher stable marker timed out: %s evidence=%s\n' "$stable_marker" "$run_dir" >&2
            exit 1
        fi
        sleep 0.2
    done
    capture_precondition_args=(
        --serial-log "$serial_log"
        --action-marker "$expected_marker"
        --stable-marker "$stable_marker"
        --precondition-output "$run_dir/capture-precondition.json"
    )
else
    capture_precondition_args=(
        --serial-log "$serial_log"
        --required-marker "$initial_display_marker"
        --precondition-output "$run_dir/capture-precondition.json"
    )
    if [[ "$scenario" != boot ]]; then
        capture_precondition_args+=(--required-marker "$expected_marker")
    fi
fi

failure_stage=frame-capture
python3 "$repo_root/scripts/desktop/qmp_screendump.py" \
    --socket "$qmp_socket" \
    --output "$frame" \
    --transcript "$run_dir/qmp-capture.jsonl" \
    --settle 0.5 \
    "${capture_precondition_args[@]}" \
    --quit-after

failure_stage=qemu-shutdown
qemu_exit=0
wait "$qemu_pid" || qemu_exit=$?
qemu_pid=
if (( qemu_exit != 0 )); then
    exit 1
fi
failure_stage=complete
