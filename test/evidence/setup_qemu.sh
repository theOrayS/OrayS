#!/usr/bin/env bash
set -euo pipefail

# Reproducible QEMU source input for the required PR3 runtime smoke lanes.
QEMU_VERSION=9.2.4
QEMU_ARCHIVE="qemu-${QEMU_VERSION}.tar.xz"
QEMU_URL="https://download.qemu.org/${QEMU_ARCHIVE}"
QEMU_SHA256="f3cc1c4eabfdb288218ac3e33763dbe9e276d8bc890b867a2335d58de2ddd39a"
QEMU_SIZE_BYTES=134782772
QEMU_TARGET_LIST="riscv64-softmmu,loongarch64-softmmu"
QEMU_CONFIGURE_PROFILE="--disable-docs --disable-werror --disable-download --disable-slirp"
STAMP_NAME=".orays-pr3-qemu-${QEMU_VERSION}"

usage() {
    printf 'usage: %s [--verify-only] PREFIX CACHE_DIR\n' "$0" >&2
}

verify_install() {
    local prefix="$1"
    local stamp="$prefix/$STAMP_NAME"
    local emulator
    local actual_sha256
    local expected_sha256
    test -f "$stamp" || return 1
    test "$(wc -l <"$stamp" | tr -d '[:space:]')" = 8 || return 1
    cmp -s <(sed -n '1,6p' "$stamp") - <<EOF || return 1
version=$QEMU_VERSION
source_url=$QEMU_URL
source_sha256=$QEMU_SHA256
source_size_bytes=$QEMU_SIZE_BYTES
target_list=$QEMU_TARGET_LIST
configure_profile=$QEMU_CONFIGURE_PROFILE
EOF
    for emulator in qemu-system-riscv64 qemu-system-loongarch64; do
        test -x "$prefix/bin/$emulator" || return 1
        "$prefix/bin/$emulator" --version \
            | head -n 1 \
            | grep -F "QEMU emulator version $QEMU_VERSION" >/dev/null
    done
    for emulator in qemu-system-riscv64 qemu-system-loongarch64; do
        actual_sha256="$(sha256sum "$prefix/bin/$emulator" | awk '{print $1}')"
        expected_sha256="$(sed -n "s/^${emulator}_sha256=//p" "$stamp")"
        test -n "$expected_sha256" || return 1
        test "$actual_sha256" = "$expected_sha256" || return 1
    done
}

verify_only=0
if [ "${1:-}" = "--verify-only" ]; then
    verify_only=1
    shift
fi
if [ "$#" -ne 2 ]; then
    usage
    exit 2
fi

prefix="$1"
cache_dir="$2"
case "$prefix:$cache_dir" in
    :*|*:|/:*|*:/)
        printf 'refusing unsafe empty/root path\n' >&2
        exit 2
        ;;
esac

command -v realpath >/dev/null 2>&1 || {
    printf 'missing path validation tool: realpath\n' >&2
    exit 2
}
for candidate in "$prefix" "$cache_dir"; do
    canonical_path="$(realpath -m -- "$candidate")"
    lexical_path="$(realpath -ms -- "$candidate")"
    if [ "$canonical_path" = / ] || [ "$canonical_path" != "$lexical_path" ]; then
        printf 'refusing root or symlinked QEMU path: %s\n' "$candidate" >&2
        exit 2
    fi
done

if [ "$verify_only" -eq 1 ]; then
    if verify_install "$prefix"; then
        printf 'verified transferred QEMU %s at %s\n' "$QEMU_VERSION" "$prefix"
        exit 0
    fi
    printf 'QEMU %s transferred-artifact verification failed at %s\n' \
        "$QEMU_VERSION" "$prefix" >&2
    exit 1
fi
# Normal build mode is the provenance boundary: it must build from the pinned
# source archive and therefore never accepts a pre-populated binary prefix.
if [ -e "$prefix" ] && [ -n "$(find "$prefix" -mindepth 1 -maxdepth 1 -print -quit)" ]; then
    printf 'refusing to reuse or overwrite non-empty QEMU prefix: %s\n' "$prefix" >&2
    exit 2
fi

for command in curl sha256sum tar make; do
    command -v "$command" >/dev/null 2>&1 || {
        printf 'missing build tool: %s\n' "$command" >&2
        exit 2
    }
done

mkdir -p "$prefix" "$cache_dir"
archive="$cache_dir/$QEMU_ARCHIVE"
verify_archive() {
    local candidate="$1"
    local actual_size
    local actual_sha256
    actual_size="$(wc -c <"$candidate" | tr -d '[:space:]')"
    actual_sha256="$(sha256sum "$candidate" | awk '{print $1}')"
    test "$actual_size" = "$QEMU_SIZE_BYTES" && test "$actual_sha256" = "$QEMU_SHA256"
}
if [ -f "$archive" ] && ! verify_archive "$archive"; then
    printf 'discarding cached QEMU source that does not match pinned size/hash\n' >&2
    rm -f -- "$archive"
fi
if [ ! -f "$archive" ]; then
    temporary_archive="$archive.partial.$$"
    trap 'rm -f -- "$temporary_archive"' EXIT
    curl --fail --location --retry 3 --retry-all-errors \
        --output "$temporary_archive" "$QEMU_URL"
    mv "$temporary_archive" "$archive"
    trap - EXIT
fi

if ! verify_archive "$archive"; then
    actual_size="$(wc -c <"$archive" | tr -d '[:space:]')"
    actual_sha256="$(sha256sum "$archive" | awk '{print $1}')"
    printf 'QEMU source verification failed: size=%s sha256=%s\n' \
        "$actual_size" "$actual_sha256" >&2
    rm -f -- "$archive"
    exit 2
fi

work_dir="$(mktemp -d "$cache_dir/build.XXXXXX")"
cleanup() {
    rm -rf -- "$work_dir"
}
trap cleanup EXIT
# Archive ownership is provenance-irrelevant and may be unmappable inside a
# user namespace.  Keep the extracted tree owned by the invoking runner.
tar --no-same-owner -xf "$archive" -C "$work_dir"
source_dir="$work_dir/qemu-$QEMU_VERSION"
build_dir="$source_dir/build"
mkdir "$build_dir"
(
    cd "$build_dir"
    ../configure \
        --prefix="$prefix" \
        --target-list="$QEMU_TARGET_LIST" \
        --disable-docs \
        --disable-werror \
        --disable-download \
        --disable-slirp
)
make -C "$build_dir" -j"${QEMU_BUILD_JOBS:-2}"
make -C "$build_dir" install

printf '%s\n' \
    "version=$QEMU_VERSION" \
    "source_url=$QEMU_URL" \
    "source_sha256=$QEMU_SHA256" \
    "source_size_bytes=$QEMU_SIZE_BYTES" \
    "target_list=$QEMU_TARGET_LIST" \
    "configure_profile=$QEMU_CONFIGURE_PROFILE" \
    "qemu-system-riscv64_sha256=$(sha256sum "$prefix/bin/qemu-system-riscv64" | awk '{print $1}')" \
    "qemu-system-loongarch64_sha256=$(sha256sum "$prefix/bin/qemu-system-loongarch64" | awk '{print $1}')" \
    >"$prefix/$STAMP_NAME"
verify_install "$prefix"
printf 'built and verified QEMU %s at %s\n' "$QEMU_VERSION" "$prefix"
