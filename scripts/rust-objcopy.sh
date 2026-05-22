#!/usr/bin/env sh
set -eu
if command -v rustup >/dev/null 2>&1; then
  tool=$(rustup which llvm-objcopy 2>/dev/null || true)
  if [ -n "$tool" ] && [ -x "$tool" ]; then
    exec "$tool" "$@"
  fi
fi
if command -v llvm-objcopy >/dev/null 2>&1; then
  exec llvm-objcopy "$@"
fi
echo "rust-objcopy offline shim: llvm-objcopy not found; install rust llvm-tools component" >&2
exit 127
