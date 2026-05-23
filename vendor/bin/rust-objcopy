#!/usr/bin/env sh
set -eu
if command -v rustup >/dev/null 2>&1; then
  tool=$(rustup which llvm-objcopy 2>/dev/null || true)
  if [ -n "$tool" ] && [ -x "$tool" ]; then
    exec "$tool" "$@"
  fi
fi
if command -v rustc >/dev/null 2>&1; then
  sysroot=$(rustc --print sysroot 2>/dev/null || true)
  host=$(rustc -vV 2>/dev/null | sed -n 's/^host: //p')
  tool="$sysroot/lib/rustlib/$host/bin/llvm-objcopy"
  if [ -n "$sysroot" ] && [ -n "$host" ] && [ -x "$tool" ]; then
    exec "$tool" "$@"
  fi
fi
if command -v llvm-objcopy >/dev/null 2>&1; then
  exec llvm-objcopy "$@"
fi
echo "rust-objcopy offline shim: llvm-objcopy not found; install rust llvm-tools component" >&2
exit 127
