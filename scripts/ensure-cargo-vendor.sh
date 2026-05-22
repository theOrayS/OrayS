#!/bin/sh
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
VENDOR_DIR="$ROOT/vendor/cargo"
ARCHIVE="$ROOT/vendor/cargo-vendor.tar.gz"

if [ -d "$VENDOR_DIR" ]; then
    exit 0
fi

if [ ! -f "$ARCHIVE" ]; then
    echo "missing $ARCHIVE" >&2
    exit 1
fi

TMP_DIR="$ROOT/vendor/.cargo.unpack.$$"
rm -rf "$TMP_DIR"
mkdir -p "$TMP_DIR"
trap 'rm -rf "$TMP_DIR"' EXIT INT TERM

tar -xzf "$ARCHIVE" -C "$TMP_DIR"

if [ ! -d "$TMP_DIR/cargo" ]; then
    echo "$ARCHIVE does not contain cargo/" >&2
    exit 1
fi

mv "$TMP_DIR/cargo" "$VENDOR_DIR"
