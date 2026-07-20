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

fn rust_lld() -> PathBuf {
    env::var_os("RUST_LLD").map_or_else(
        || {
            PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"))
                .join("../../scripts/rust-lld.sh")
        },
        PathBuf::from,
    )
}

fn target_spec(target_arch: &str) -> Option<(&'static str, &'static str)> {
    match target_arch {
        "riscv64" => Some(("riscv64gc-unknown-none-elf", "elf64lriscv")),
        "loongarch64" => Some(("loongarch64-unknown-none-softfloat", "elf64loongarch")),
        _ => None,
    }
}

fn read_u16(bytes: &[u8], offset: usize, label: &str) -> u16 {
    let raw: [u8; 2] = bytes
        .get(offset..offset + 2)
        .unwrap_or_else(|| panic!("semantic smoke ELF has truncated {label}"))
        .try_into()
        .expect("fixed-size ELF field");
    u16::from_le_bytes(raw)
}

fn read_u32(bytes: &[u8], offset: usize, label: &str) -> u32 {
    let raw: [u8; 4] = bytes
        .get(offset..offset + 4)
        .unwrap_or_else(|| panic!("semantic smoke ELF has truncated {label}"))
        .try_into()
        .expect("fixed-size ELF field");
    u32::from_le_bytes(raw)
}

fn read_u64(bytes: &[u8], offset: usize, label: &str) -> u64 {
    let raw: [u8; 8] = bytes
        .get(offset..offset + 8)
        .unwrap_or_else(|| panic!("semantic smoke ELF has truncated {label}"))
        .try_into()
        .expect("fixed-size ELF field");
    u64::from_le_bytes(raw)
}

fn validate_semantic_smoke_elf(path: &Path, expected_machine: u16, target_arch: &str) {
    const ELF_HEADER_SIZE: usize = 64;
    const PROGRAM_HEADER_SIZE: usize = 56;
    const PT_LOAD: u32 = 1;
    const PT_INTERP: u32 = 3;
    const PF_X: u32 = 1;
    const PAGE_SIZE: u64 = 0x1000;
    const USER_ASPACE_BASE: u64 = 0x1_0000;

    let bytes = fs::read(path).expect("read generated semantic smoke ELF");
    if bytes.len() < ELF_HEADER_SIZE
        || &bytes[..4] != b"\x7fELF"
        || bytes[4] != 2
        || bytes[5] != 1
        || bytes[6] != 1
    {
        panic!("semantic smoke image must be a little-endian ELF64 v1 executable");
    }
    if read_u16(&bytes, 0x10, "e_type") != 2 {
        panic!("semantic smoke ELF must be ET_EXEC");
    }
    if read_u16(&bytes, 0x12, "e_machine") != expected_machine {
        panic!("semantic smoke ELF machine does not match {target_arch}");
    }
    if target_arch == "loongarch64" && read_u32(&bytes, 0x30, "e_flags") & 0x7 != 0x1 {
        panic!("LoongArch semantic smoke ELF must retain the soft-float ABI flag");
    }

    let entry = read_u64(&bytes, 0x18, "e_entry");
    if entry < USER_ASPACE_BASE {
        panic!("semantic smoke ELF entry is below the user address-space base");
    }
    let phoff = read_u64(&bytes, 0x20, "e_phoff") as usize;
    let phentsize = read_u16(&bytes, 0x36, "e_phentsize") as usize;
    let phnum = read_u16(&bytes, 0x38, "e_phnum") as usize;
    if phentsize < PROGRAM_HEADER_SIZE || phnum == 0 {
        panic!("semantic smoke ELF has an invalid program-header table");
    }
    let table_end = phoff
        .checked_add(
            phentsize
                .checked_mul(phnum)
                .expect("program-header size overflow"),
        )
        .expect("program-header offset overflow");
    if table_end > bytes.len() {
        panic!("semantic smoke ELF program-header table exceeds the image");
    }

    let mut load_pages: Vec<(u64, u64)> = Vec::new();
    let mut entry_is_executable = false;
    for index in 0..phnum {
        let base = phoff + index * phentsize;
        let kind = read_u32(&bytes, base, "p_type");
        if kind == PT_INTERP {
            panic!("semantic smoke ELF must not contain PT_INTERP");
        }
        if kind != PT_LOAD {
            continue;
        }
        let flags = read_u32(&bytes, base + 4, "p_flags");
        let offset = read_u64(&bytes, base + 8, "p_offset");
        let vaddr = read_u64(&bytes, base + 16, "p_vaddr");
        let filesz = read_u64(&bytes, base + 32, "p_filesz");
        let memsz = read_u64(&bytes, base + 40, "p_memsz");
        let align = read_u64(&bytes, base + 48, "p_align");
        if memsz == 0 {
            continue;
        }
        if vaddr < USER_ASPACE_BASE || memsz < filesz || align < PAGE_SIZE {
            panic!("semantic smoke ELF has an invalid PT_LOAD segment");
        }
        let file_end = offset
            .checked_add(filesz)
            .expect("PT_LOAD file range overflow");
        if file_end > bytes.len() as u64 {
            panic!("semantic smoke ELF PT_LOAD data exceeds the image");
        }
        let segment_end = vaddr.checked_add(memsz).expect("PT_LOAD address overflow");
        let page_start = vaddr & !(PAGE_SIZE - 1);
        let page_end = segment_end
            .checked_add(PAGE_SIZE - 1)
            .expect("PT_LOAD page range overflow")
            & !(PAGE_SIZE - 1);
        if load_pages
            .iter()
            .any(|(start, end)| page_start < *end && *start < page_end)
        {
            panic!("semantic smoke ELF PT_LOAD segments overlap at page granularity");
        }
        load_pages.push((page_start, page_end));
        if flags & PF_X != 0 && (vaddr..segment_end).contains(&entry) {
            entry_is_executable = true;
        }
    }
    if load_pages.is_empty() || !entry_is_executable {
        panic!("semantic smoke ELF lacks an executable PT_LOAD containing its entry");
    }
}

fn build_semantic_smoke_program(
    out_dir: &Path,
    target_arch: &str,
    rust_target: &str,
    emulation: &str,
    source_name: &str,
    output_name: &str,
) {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let source = manifest_dir.join("runtime_smoke").join(source_name);
    let linker_script = manifest_dir.join("runtime_smoke/semantic_smoke.ld");
    let object = out_dir.join(format!("{output_name}.o"));
    let executable = out_dir.join(output_name);
    let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());

    run(Command::new(&rustc)
        .arg("--target")
        .arg(rust_target)
        .arg("--edition=2024")
        .arg("-O")
        .arg("-C")
        .arg("panic=abort")
        .arg("-C")
        .arg("relocation-model=static")
        .arg("--crate-type=bin")
        .arg("--emit=obj")
        .arg("-o")
        .arg(&object)
        .arg(&source));
    run(Command::new(rust_lld())
        .arg("-flavor")
        .arg("gnu")
        .arg("-m")
        .arg(emulation)
        .arg("-static")
        .arg("--gc-sections")
        .arg("-T")
        .arg(&linker_script)
        .arg("-o")
        .arg(&executable)
        .arg(&object));

    let expected_machine = match target_arch {
        "riscv64" => 243,
        "loongarch64" => 258,
        _ => unreachable!("semantic smoke is only built for competition architectures"),
    };
    validate_semantic_smoke_elf(&executable, expected_machine, target_arch);
}

fn build_semantic_smoke(out_dir: &Path, target_arch: &str, rust_target: &str, emulation: &str) {
    build_semantic_smoke_program(
        out_dir,
        target_arch,
        rust_target,
        emulation,
        "semantic_smoke.rs",
        "pr3-semantic-smoke",
    );
    build_semantic_smoke_program(
        out_dir,
        target_arch,
        rust_target,
        emulation,
        "semantic_exec_helper.rs",
        "pr3-semantic-exec-helper",
    );
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
    println!("cargo:rerun-if-changed=../../scripts/rust-lld.sh");
    println!("cargo:rerun-if-changed=runtime_compat/musl_oscompat.rs");
    println!("cargo:rerun-if-changed=runtime_smoke/semantic_exec_helper.rs");
    println!("cargo:rerun-if-changed=runtime_smoke/semantic_smoke.rs");
    println!("cargo:rerun-if-changed=runtime_smoke/semantic_smoke.ld");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let so = out_dir.join("liboscompat.so");
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH");
    let Some((compat_target, emulation)) = target_spec(target_arch.as_str()) else {
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
    if target_arch == "loongarch64" {
        set_loongarch_lp64d_flags(&so);
    }
    if env::var_os("CARGO_FEATURE_SEMANTIC_SMOKE").is_some() {
        build_semantic_smoke(
            out_dir.as_path(),
            target_arch.as_str(),
            compat_target,
            emulation,
        );
    }
}
