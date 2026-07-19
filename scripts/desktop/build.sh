#!/usr/bin/env bash
set -euo pipefail

repo_root=$(git -C "$(dirname "${BASH_SOURCE[0]}")" rev-parse --show-toplevel)
build_root=${DESKTOP_BUILD_ROOT:-"$repo_root/build/desktop"}
cargo_home="$build_root/cargo-home"
vendor_root="$build_root/vendor"
manifest="$repo_root/user/desktop/Cargo.toml"

usage() {
    printf '%s\n' \
        'usage: scripts/desktop/build.sh {host-test|golden|golden-check|rv|la|all}' \
        '       scripts/desktop/build.sh scene {boot|launcher|overlap|applications|light|power}' \
        'all generated files stay under build/desktop or test/output/desktop'
}

prepare_offline_cargo() {
    mkdir -p "$cargo_home" "$vendor_root"
    if [[ ! -d "$vendor_root/cargo" ]]; then
        local archive="$repo_root/vendor/cargo-vendor.tar.gz"
        [[ -f "$archive" ]] || {
            printf 'missing offline Cargo archive: %s\n' "$archive" >&2
            return 2
        }
        local unpack
        unpack=$(mktemp -d "$build_root/vendor-unpack.XXXXXX")
        trap 'rm -rf "$unpack"' RETURN
        tar -xzf "$archive" -C "$unpack"
        [[ -d "$unpack/cargo" ]] || {
            printf 'offline Cargo archive does not contain cargo/\n' >&2
            return 2
        }
        mv "$unpack/cargo" "$vendor_root/cargo"
        rm -rf "$unpack"
        trap - RETURN
    fi

    local escaped_vendor=${vendor_root//\\/\\\\}
    escaped_vendor=${escaped_vendor//\"/\\\"}
    printf '%s\n' \
        '[source.crates-io]' \
        'replace-with = "vendored-sources"' \
        '' \
        '[source.vendored-sources]' \
        "directory = \"$escaped_vendor/cargo\"" \
        '' \
        '[net]' \
        'offline = true' >"$cargo_home/config.toml"
}

run_cargo() {
    local subcommand=$1
    shift
    CARGO_HOME="$cargo_home" CARGO_NET_OFFLINE=true \
        cargo "$subcommand" --offline --locked --manifest-path "$manifest" "$@"
}

host_test() {
    prepare_offline_cargo
    run_cargo test --lib --tests --features host-tools --target-dir "$build_root/target/host"
}

golden() {
    prepare_offline_cargo
    local output="$repo_root/test/output/desktop/golden/desktop-boot.ppm"
    mkdir -p "$(dirname "$output")"
    run_cargo run --features host-tools --bin render-golden \
        --target-dir "$build_root/target/host" -- "$output" 1280 720
    sha256sum "$output"
}

scene() {
    local name=${1:-boot}
    case "$name" in
        boot|launcher|overlap|applications|light|power) ;;
        *)
            printf 'unsupported desktop scene: %s\n' "$name" >&2
            return 2
            ;;
    esac
    prepare_offline_cargo
    local output="$repo_root/test/output/desktop/scenes/$name.ppm"
    mkdir -p "$(dirname "$output")"
    run_cargo run --features host-tools --bin render-scene \
        --target-dir "$build_root/target/host" -- "$output" "$name" 1280 720
    sha256sum "$output"
}

golden_check() {
    local name
    for name in boot launcher light power applications; do
        scene "$name"
    done
    python3 "$repo_root/scripts/desktop/compare-golden.py" \
        --manifest "$repo_root/test/desktop/fixtures/golden/shell-scenes.sha256" \
        --actual-dir "$repo_root/test/output/desktop/scenes"
}

cross_build() {
    local selector=$1
    local arch target platform platform_package bus_feature
    case "$selector" in
        rv)
            arch=riscv64
            target=riscv64gc-unknown-none-elf
            platform=riscv64-qemu-virt
            platform_package=axplat-riscv64-qemu-virt
            bus_feature=bus-mmio
            ;;
        la)
            arch=loongarch64
            target=loongarch64-unknown-none-softfloat
            platform=loongarch64-qemu-virt
            platform_package=axplat-loongarch64-qemu-virt
            bus_feature=bus-pci
            ;;
        *)
            printf 'unsupported desktop architecture: %s\n' "$selector" >&2
            return 2
            ;;
    esac

    prepare_offline_cargo
    local arch_root="$build_root/$selector"
    local target_dir="$arch_root/target"
    local output_dir="$arch_root/artifacts"
    local config="$arch_root/axconfig.toml"
    local platform_config="$repo_root/configs/platforms/$platform_package.toml"
    local linker="$target_dir/$target/release/linker_$platform.lds"
    mkdir -p "$target_dir/$target/release" "$output_dir"

    python3 "$repo_root/scripts/axconfig-gen.py" \
        "$repo_root/configs/defconfig.toml" "$platform_config" \
        -w "arch=\"$arch\"" -w "platform=\"$platform\"" -o "$config"

    AX_ARCH="$arch" \
    AX_PLATFORM="$platform" \
    AX_MODE=release \
    AX_LOG=warn \
    AX_TARGET="$target" \
    AX_CONFIG_PATH="$config" \
    RUSTFLAGS="-A unsafe_op_in_unsafe_fn -C link-arg=-T$linker -C link-arg=-no-pie -C link-arg=-znostart-stop-gc" \
        run_cargo build --release --bin orays-desktop --no-default-features \
            --features "orays,$bus_feature" --target "$target" \
            --target-dir "$target_dir"

    local elf="$target_dir/$target/release/orays-desktop"
    local artifact_elf="$output_dir/orays-desktop-$selector.elf"
    local artifact_bin="$output_dir/orays-desktop-$selector.bin"
    cp "$elf" "$artifact_elf"
    "$repo_root/scripts/rust-objcopy.sh" "$artifact_elf" --strip-all -O binary "$artifact_bin"
    test -s "$artifact_elf"
    test -s "$artifact_bin"
    sha256sum "$artifact_elf" "$artifact_bin"
}

command=${1:-}
case "$command" in
    host-test) host_test ;;
    golden) golden ;;
    golden-check) golden_check ;;
    scene) scene "${2:-boot}" ;;
    rv) cross_build rv ;;
    la) cross_build la ;;
    all)
        host_test
        golden
        cross_build rv
        cross_build la
        ;;
    *)
        usage >&2
        exit 2
        ;;
esac
