#!/usr/bin/env sh
set -eu
if [ "${1:-}" = "--version" ] || [ "${1:-}" = "-V" ]; then
  echo "cargo-axplat 0.3.0"
  exit 0
fi
if [ "${1:-}" = "axplat" ]; then
  shift
fi
if [ "${1:-}" = "--version" ] || [ "${1:-}" = "-V" ]; then
  echo "cargo-axplat 0.3.0"
  exit 0
fi
if [ "${1:-}" = "info" ]; then
  shift
  pkg=""
  while [ "$#" -gt 0 ]; do
    case "$1" in
      -c|--config-path)
        pkg="$2"; shift 2 ;;
      -C)
        shift 2 ;;
      *)
        shift ;;
    esac
  done
  if [ -n "$pkg" ]; then
    for dir in "$PWD/configs/platforms" "$PWD/configs/remote-eval"; do
      if [ -f "$dir/$pkg.toml" ]; then
        printf '%s\n' "$dir/$pkg.toml"
        exit 0
      fi
    done
  fi
fi
echo "cargo-axplat offline shim: unsupported arguments: $*" >&2
exit 2
