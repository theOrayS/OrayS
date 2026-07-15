#!/usr/bin/env sh
set -eu

# rustup does not expose rust-lld as a proxy, but the pinned llvm-tools
# component installs it in the host toolchain's rustlib directory. Resolve that
# repository-independently so clean CI runners do not depend on cargo-binutils
# or an unpinned system linker.
resolve_tool() {
  if command -v rustc >/dev/null 2>&1; then
    sysroot=$(rustc --print sysroot 2>/dev/null || true)
    host=$(rustc -vV 2>/dev/null | sed -n 's/^host: //p')
    tool="$sysroot/lib/rustlib/$host/bin/rust-lld"
    if [ -n "$sysroot" ] && [ -n "$host" ] && [ -x "$tool" ]; then
      printf '%s\n' "$tool"
      return 0
    fi
  fi
  if command -v rust-lld >/dev/null 2>&1; then
    command -v rust-lld
    return 0
  fi
  return 1
}

tool=$(resolve_tool) || {
  echo "rust-lld offline shim: bundled rust-lld not found; install the pinned rust llvm-tools component" >&2
  exit 127
}
if [ "$#" -eq 1 ] && [ "$1" = "--pr3-print-effective-tool" ]; then
  printf '%s\n' "$tool"
  exit 0
fi
exec "$tool" "$@"
