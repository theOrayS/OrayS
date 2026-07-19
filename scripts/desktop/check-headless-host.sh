#!/usr/bin/env bash
set -u

fail=0
warn=0

check_cmd() {
  local name="$1"
  local required="$2"
  if command -v "$name" >/dev/null 2>&1; then
    printf 'PASS command %-28s %s\n' "$name" "$(command -v "$name")"
  elif [[ "$required" == "yes" ]]; then
    printf 'FAIL command %-28s missing\n' "$name"
    fail=1
  else
    printf 'WARN command %-28s missing\n' "$name"
    warn=1
  fi
}

check_cmd git yes
check_cmd python3 yes
check_cmd cargo no
check_cmd rustc no
check_cmd make yes
check_cmd sha256sum yes
check_cmd timeout yes
check_cmd qemu-system-riscv64 no
check_cmd qemu-system-loongarch64 no
check_cmd socat no
check_cmd convert no

if [[ -n "${DISPLAY:-}" ]]; then
  echo "WARN DISPLAY is set to '$DISPLAY'; start-codex.sh will unset it"
  warn=1
else
  echo "PASS DISPLAY is not set"
fi

if [[ -n "${WAYLAND_DISPLAY:-}" ]]; then
  echo "WARN WAYLAND_DISPLAY is set; start-codex.sh will unset it"
  warn=1
else
  echo "PASS WAYLAND_DISPLAY is not set"
fi

if python3 - <<'PY' >/dev/null 2>&1
import tomllib
PY
then
  echo "PASS Python tomllib available"
else
  echo "WARN Python tomllib unavailable; setup validation will be limited"
  warn=1
fi

for qemu in qemu-system-riscv64 qemu-system-loongarch64; do
  if command -v "$qemu" >/dev/null 2>&1; then
    if "$qemu" -display help 2>&1 | grep -qiE 'vnc|egl-headless|dbus'; then
      echo "PASS $qemu has at least one potential headless display backend"
    else
      echo "WARN $qemu does not advertise VNC/egl-headless/dbus"
      warn=1
    fi
  fi
done

if [[ "$fail" -ne 0 ]]; then
  echo "HEADLESS_HOST_CHECK=FAIL"
  exit 1
fi

if [[ "$warn" -ne 0 ]]; then
  echo "HEADLESS_HOST_CHECK=PASS_WITH_WARNINGS"
else
  echo "HEADLESS_HOST_CHECK=PASS"
fi
