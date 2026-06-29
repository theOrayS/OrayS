use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn run(command: &mut Command) {
    let display = format!("{:?}", command);
    let status = command
        .status()
        .unwrap_or_else(|err| panic!("failed to run {display}: {err}"));
    if !status.success() {
        panic!("command failed ({status}): {display}");
    }
}

fn rust_lld() -> String {
    env::var("RUST_LLD").unwrap_or_else(|_| "rust-lld".to_string())
}

fn target_spec(target_arch: &str) -> Option<(&'static str, &'static str)> {
    match target_arch {
        "riscv64" => Some(("riscv64gc-unknown-none-elf", "elf64lriscv")),
        "loongarch64" => Some(("loongarch64-unknown-none-softfloat", "elf64loongarch")),
        _ => None,
    }
}

fn set_loongarch_lp64d_flags(path: &Path) {
    let mut bytes = fs::read(path).expect("read generated LoongArch compatibility library");
    if bytes.len() < 0x34 || &bytes[..4] != b"\x7fELF" {
        panic!("generated LoongArch compatibility library is not an ELF64 file");
    }
    // The integer-only compatibility layer is built from the always-installed
    // unknown-none soft-float target, then marked with the LP64D e_flags used by
    // the official musl LoongArch runtime so the dynamic loader accepts it.
    bytes[0x30..0x34].copy_from_slice(&0x43u32.to_le_bytes());
    fs::write(path, bytes).expect("write generated LoongArch compatibility library");
}

fn main() {
    println!("cargo:rerun-if-changed=runtime_compat/musl_oscompat.rs");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let so = out_dir.join("liboscompat.so");
    let Some((compat_target, emulation)) = target_spec(
        env::var("CARGO_CFG_TARGET_ARCH")
            .expect("CARGO_CFG_TARGET_ARCH")
            .as_str(),
    ) else {
        fs::write(&so, []).expect("write empty compatibility library placeholder");
        return;
    };

    let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let obj = out_dir.join("liboscompat.o");
    run(Command::new(&rustc)
        .arg("--target")
        .arg(compat_target)
        .arg("-O")
        .arg("-C")
        .arg("panic=abort")
        .arg("--crate-type=lib")
        .arg("--emit=obj")
        .arg("-o")
        .arg(&obj)
        .arg("runtime_compat/musl_oscompat.rs"));
    run(Command::new(rust_lld())
        .arg("-flavor")
        .arg("gnu")
        .arg("-m")
        .arg(emulation)
        .arg("-shared")
        .arg("-soname")
        .arg("liboscompat.so")
        .arg("-o")
        .arg(&so)
        .arg(&obj));
    if env::var("CARGO_CFG_TARGET_ARCH").as_deref() == Ok("loongarch64") {
        set_loongarch_lp64d_flags(&so);
    }
}
