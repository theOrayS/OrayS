use core::cmp;
use core::mem::size_of;

use axhal::paging::MappingFlags;
use axhal::trap::PageFaultFlags;
use axmm::AddrSpace;
use axsync::Mutex;
use lazyinit::LazyInit;
use linux_raw_sys::auxvec;
use linux_raw_sys::general;
use memory_addr::{VirtAddr, PAGE_SIZE_4K};
use std::fs::File;
use std::io::Read;
use std::string::{String, ToString};
use std::vec::Vec;
use xmas_elf::header::{Machine, Type as ElfType};
use xmas_elf::program::{Flags as PhFlags, ProgramHeader, Type as PhType};
#[cfg(any(target_arch = "loongarch64", target_arch = "riscv64"))]
use xmas_elf::sections::SectionData;
#[cfg(any(target_arch = "loongarch64", target_arch = "riscv64"))]
use xmas_elf::symbol_table::Entry;
use xmas_elf::ElfFile;

use super::linux_abi::{
    AUX_CLOCK_TICKS, AUX_PLATFORM, MAX_SCRIPT_INTERPRETER_DEPTH, USER_BRK_GROW_SIZE,
    USER_MMAP_BASE, USER_PIE_LOAD_BASE, USER_STACK_SIZE, USER_STACK_TOP,
};
use super::memory_map::{align_down, align_up, user_mapping_flags};
use super::runtime_paths::{
    derive_exec_root_from_path, resolve_host_path, resolve_runtime_support_file,
};
use super::{str_err, BrkState, UserProcess};

pub(super) struct LoadedImage {
    pub(super) entry: usize,
    pub(super) stack_ptr: usize,
    pub(super) argc: usize,
    pub(super) brk: BrkState,
    pub(super) mappings: Vec<LoadedMapping>,
    pub(super) exec_root: String,
    pub(super) exec_path: String,
}

#[derive(Clone, Copy)]
pub(super) struct LoadedMapping {
    pub(super) start: usize,
    pub(super) size: usize,
    pub(super) prot: u32,
}

struct PreparedProgram {
    argv: Vec<String>,
    path: String,
    exec_root: String,
}

const MAX_EXEC_IMAGE_SIZE: usize = 64 * 1024 * 1024;
// Keep a bounded reusable exec buffer so long full-suite runs do not need a
// fresh multi-MiB contiguous allocation after the kernel heap is fragmented.
const RETAINED_EXEC_IMAGE_CAPACITY: usize = 4 * 1024 * 1024;

struct ElfLoadInfo {
    load_bias: usize,
    entry: usize,
    phdr: usize,
    max_segment_end: usize,
    base: usize,
    interpreter: Option<String>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct AuxEntry {
    key: usize,
    value: usize,
}

#[cfg(any(target_arch = "loongarch64", target_arch = "riscv64"))]
type MuslPatchManifestEntry = (
    &'static str,
    &'static str,
    &'static [&'static str],
    &'static str,
);

#[cfg(any(target_arch = "loongarch64", target_arch = "riscv64"))]
const MUSL_PATCH_RETIREMENT_DIRECTIVE: &str =
    "temporary; requires raw syscall + musl + glibc cross-check evidence before stable promotion";

#[cfg(target_arch = "riscv64")]
const RISCV_MUSL_PATCH_MANIFEST: &[MuslPatchManifestEntry] = &[
    (
        "main-executable",
        "brk",
        &["brk"],
        "temporary musl brk ENOSYS-stub replacement",
    ),
    (
        "interpreter",
        "brk",
        &["brk"],
        "temporary musl brk ENOSYS-stub replacement",
    ),
    (
        "interpreter",
        "sbrk",
        &["brk"],
        "temporary musl sbrk wrapper over raw brk",
    ),
    (
        "interpreter",
        "nice",
        &["getpriority", "setpriority"],
        "temporary musl nice wrapper over priority syscalls",
    ),
    (
        "interpreter",
        "gethostname",
        &["uname"],
        "temporary musl gethostname errno-normalization wrapper",
    ),
];

#[cfg(target_arch = "loongarch64")]
const LOONGARCH_MUSL_PATCH_MANIFEST: &[MuslPatchManifestEntry] = &[
    (
        "main-executable",
        "brk",
        &["brk"],
        "temporary musl brk ENOSYS-stub replacement",
    ),
    (
        "interpreter",
        "sched_setparam",
        &["sched_setparam"],
        "temporary musl sched wrapper over raw syscall",
    ),
    (
        "interpreter",
        "sched_getparam",
        &["sched_getparam"],
        "temporary musl sched wrapper over raw syscall",
    ),
    (
        "interpreter",
        "sched_setscheduler",
        &["sched_setscheduler"],
        "temporary musl sched wrapper over raw syscall",
    ),
    (
        "interpreter",
        "sched_getscheduler",
        &["sched_getscheduler"],
        "temporary musl sched wrapper over raw syscall",
    ),
    (
        "interpreter",
        "brk",
        &["brk"],
        "temporary musl brk ENOSYS-stub replacement",
    ),
    (
        "interpreter",
        "sbrk",
        &["brk"],
        "temporary musl sbrk wrapper over raw brk",
    ),
    (
        "interpreter",
        "gethostname",
        &["uname"],
        "temporary musl gethostname errno-normalization wrapper",
    ),
    (
        "interpreter",
        "readlink",
        &["readlinkat"],
        "temporary musl readlink wrapper over readlinkat",
    ),
    (
        "interpreter",
        "readlinkat",
        &["readlinkat"],
        "temporary musl readlinkat errno-normalization wrapper",
    ),
];

#[cfg(target_arch = "riscv64")]
fn validate_riscv_musl_patch_manifest() -> Result<(), String> {
    validate_musl_patch_manifest(RISCV_MUSL_PATCH_MANIFEST)
}

#[cfg(target_arch = "riscv64")]
fn ensure_riscv_musl_patch_manifest_symbol(symbol: &str) -> Result<(), String> {
    ensure_musl_patch_manifest_symbol("riscv64", RISCV_MUSL_PATCH_MANIFEST, symbol)
}

#[cfg(target_arch = "loongarch64")]
fn validate_loongarch_musl_patch_manifest() -> Result<(), String> {
    validate_musl_patch_manifest(LOONGARCH_MUSL_PATCH_MANIFEST)
}

#[cfg(target_arch = "loongarch64")]
fn ensure_loongarch_musl_patch_manifest_symbol(symbol: &str) -> Result<(), String> {
    ensure_musl_patch_manifest_symbol("loongarch64", LOONGARCH_MUSL_PATCH_MANIFEST, symbol)
}

#[cfg(any(target_arch = "loongarch64", target_arch = "riscv64"))]
fn validate_musl_patch_manifest(manifest: &[MuslPatchManifestEntry]) -> Result<(), String> {
    for (target, symbol, raw_syscalls, reason) in manifest {
        if target.is_empty()
            || symbol.is_empty()
            || raw_syscalls.is_empty()
            || reason.is_empty()
            || !reason.contains("temporary")
            || !MUSL_PATCH_RETIREMENT_DIRECTIVE.contains("raw syscall + musl + glibc")
        {
            return Err(format!("incomplete musl patch manifest entry for {symbol}"));
        }
    }
    Ok(())
}

#[cfg(any(target_arch = "loongarch64", target_arch = "riscv64"))]
fn ensure_musl_patch_manifest_symbol(
    arch: &str,
    manifest: &[MuslPatchManifestEntry],
    symbol: &str,
) -> Result<(), String> {
    validate_musl_patch_manifest(manifest)?;
    if manifest
        .iter()
        .any(|(_target, manifest_symbol, _raw_syscalls, _reason)| *manifest_symbol == symbol)
    {
        Ok(())
    } else {
        Err(format!(
            "{arch} musl patch for symbol {symbol} is missing from the manifest"
        ))
    }
}

fn default_exec_env(exec_root: &str, cwd: &str) -> Vec<String> {
    let path = if exec_root == "/glibc" {
        "PATH=/glibc:/musl"
    } else {
        "PATH=/musl:/glibc"
    };
    let pwd = if cwd.is_empty() { "/" } else { cwd };
    vec![path.into(), "HOME=/".into(), format!("PWD={pwd}")]
}

fn main_exec_image_buffer() -> &'static Mutex<Vec<u8>> {
    static BUFFER: LazyInit<Mutex<Vec<u8>>> = LazyInit::new();
    let _ = BUFFER.call_once(|| Mutex::new(Vec::new()));
    &BUFFER
}

fn interp_exec_image_buffer() -> &'static Mutex<Vec<u8>> {
    static BUFFER: LazyInit<Mutex<Vec<u8>>> = LazyInit::new();
    let _ = BUFFER.call_once(|| Mutex::new(Vec::new()));
    &BUFFER
}

fn trim_exec_image_buffer(image: &mut Vec<u8>) {
    image.clear();
    if image.capacity() > RETAINED_EXEC_IMAGE_CAPACITY {
        image.shrink_to(RETAINED_EXEC_IMAGE_CAPACITY);
    }
}

#[cfg(feature = "auto-run-tests")]
pub fn exec_image_buffer_stats() -> (usize, usize, usize, usize) {
    let main = main_exec_image_buffer().lock();
    let main_len = main.len();
    let main_cap = main.capacity();
    drop(main);
    let interp = interp_exec_image_buffer().lock();
    (main_len, main_cap, interp.len(), interp.capacity())
}

fn read_exec_image_into(
    process: Option<&UserProcess>,
    path: &str,
    label: &str,
    image: &mut Vec<u8>,
) -> Result<(), String> {
    let mut file =
        File::open(path).map_err(|err| format!("failed to open {label} {path}: {err}"))?;
    let physical_len = file
        .metadata()
        .ok()
        .and_then(|metadata| usize::try_from(metadata.len()).ok())
        .unwrap_or(0);
    let expected_len = process
        .and_then(|process| {
            process
                .path_sparse_size(path)
                .and_then(|size| usize::try_from(size).ok())
        })
        .unwrap_or(0)
        .max(physical_len);
    if expected_len > MAX_EXEC_IMAGE_SIZE {
        return Err(format!(
            "{label} {path} is too large to load ({} bytes, limit {} bytes)",
            expected_len, MAX_EXEC_IMAGE_SIZE
        ));
    }

    image.clear();
    if expected_len > image.capacity() {
        image
            .try_reserve_exact(expected_len - image.capacity())
            .map_err(|_| format!("not enough kernel memory to read {label} {path}"))?;
    }

    let mut chunk = [0u8; 4096];
    loop {
        let count = file
            .read(&mut chunk)
            .map_err(|err| format!("failed to read {label} {path}: {err}"))?;
        if count == 0 {
            break;
        }
        let next_len = image
            .len()
            .checked_add(count)
            .ok_or_else(|| format!("{label} {path} size overflow while loading"))?;
        if next_len > MAX_EXEC_IMAGE_SIZE {
            return Err(format!(
                "{label} {path} is too large to load (exceeds {} bytes)",
                MAX_EXEC_IMAGE_SIZE
            ));
        }
        image
            .try_reserve_exact(count)
            .map_err(|_| format!("not enough kernel memory to read {label} {path}"))?;
        image.extend_from_slice(&chunk[..count]);
    }

    if image.len() < expected_len {
        image
            .try_reserve_exact(expected_len - image.len())
            .map_err(|_| format!("not enough kernel memory to read sparse {label} {path}"))?;
        image.resize(expected_len, 0);
    }
    if let Some(process) = process {
        process.copy_path_sparse_data(path, 0, image.as_mut_slice());
    }
    Ok(())
}

pub(super) fn load_program_image(
    process: Option<&UserProcess>,
    aspace: &mut AddrSpace,
    cwd: &str,
    program_path: &str,
    argv: &[&str],
    env_override: Option<&[String]>,
) -> Result<LoadedImage, String> {
    let mut main_image = main_exec_image_buffer().lock();
    let prepared = prepare_program(process, cwd, program_path, argv, 0, &mut main_image)?;
    #[cfg(target_arch = "riscv64")]
    patch_riscv_musl_main_syscall_stubs(prepared.exec_root.as_str(), main_image.as_mut_slice())?;
    #[cfg(target_arch = "loongarch64")]
    patch_loongarch_musl_main_syscall_stubs(
        prepared.exec_root.as_str(),
        main_image.as_mut_slice(),
    )?;
    let elf = ElfFile::new(main_image.as_slice()).map_err(|err| format!("invalid ELF: {err}"))?;
    let main = analyze_elf(&elf, USER_PIE_LOAD_BASE)?;
    let exec_root = effective_exec_root(prepared.exec_root.as_str(), main.interpreter.as_deref());

    aspace.clear();

    let mut mappings = map_elf_image(aspace, main_image.as_slice(), &elf, &main)?;
    let mut max_mapped_end = main.max_segment_end;
    let mut runtime_entry = main.entry;
    let mut interp_base = 0usize;

    if let Some(raw_interp) = main.interpreter.as_deref() {
        let interp_path = resolve_runtime_support_file(exec_root.as_str(), raw_interp)?;
        let mut interp_image = interp_exec_image_buffer().lock();
        read_exec_image_into(
            process,
            interp_path.as_str(),
            "interpreter",
            &mut interp_image,
        )?;
        #[cfg(target_arch = "loongarch64")]
        patch_loongarch_musl_syscall_stubs(
            exec_root.as_str(),
            raw_interp,
            interp_image.as_mut_slice(),
        )?;
        #[cfg(target_arch = "riscv64")]
        patch_riscv_musl_syscall_stubs(
            exec_root.as_str(),
            raw_interp,
            interp_image.as_mut_slice(),
        )?;
        let interp_elf = ElfFile::new(interp_image.as_slice())
            .map_err(|err| format!("invalid interpreter ELF: {err}"))?;
        let interp = analyze_elf(
            &interp_elf,
            align_up(
                cmp::max(max_mapped_end + PAGE_SIZE_4K, USER_MMAP_BASE),
                PAGE_SIZE_4K,
            ),
        )?;
        mappings.extend(map_elf_image(
            aspace,
            interp_image.as_slice(),
            &interp_elf,
            &interp,
        )?);
        max_mapped_end = cmp::max(max_mapped_end, interp.max_segment_end);
        runtime_entry = interp.entry;
        interp_base = interp.base;
        drop(interp_elf);
        trim_exec_image_buffer(&mut interp_image);
    }

    let brk_start = align_up(main.max_segment_end, PAGE_SIZE_4K);
    let brk_limit = align_up(brk_start + USER_BRK_GROW_SIZE, PAGE_SIZE_4K);
    if brk_limit > USER_STACK_TOP - USER_STACK_SIZE {
        return Err("user virtual address space is too small".into());
    }

    aspace
        .map_alloc(
            VirtAddr::from(brk_start),
            brk_limit - brk_start,
            user_mapping_flags(true, true, false),
            false,
        )
        .map_err(|err| format!("failed to reserve brk area: {err}"))?;

    let stack_top = align_down(USER_STACK_TOP, PAGE_SIZE_4K);
    let stack_base = stack_top - USER_STACK_SIZE;
    aspace
        .map_alloc(
            VirtAddr::from(stack_base),
            USER_STACK_SIZE,
            user_mapping_flags(true, true, false),
            false,
        )
        .map_err(|err| format!("failed to reserve user stack: {err}"))?;

    let argv_refs = prepared.argv.iter().map(String::as_str).collect::<Vec<_>>();
    let default_env;
    let env_refs = if let Some(env) = env_override {
        env.iter().map(String::as_str).collect::<Vec<_>>()
    } else {
        default_env = default_exec_env(exec_root.as_str(), cwd);
        default_env.iter().map(String::as_str).collect::<Vec<_>>()
    };
    let ph_entry_size = elf.header.pt2.ph_entry_size() as usize;
    let ph_count = elf.header.pt2.ph_count() as usize;
    let stack_ptr = build_initial_stack(
        aspace,
        stack_base,
        stack_top,
        &argv_refs,
        &env_refs,
        prepared.path.as_str(),
        main.entry,
        interp_base,
        main.phdr,
        ph_entry_size,
        ph_count,
    )?;
    drop(elf);
    trim_exec_image_buffer(&mut main_image);

    Ok(LoadedImage {
        entry: runtime_entry,
        stack_ptr,
        argc: prepared.argv.len(),
        brk: BrkState {
            start: brk_start,
            end: brk_start,
            limit: brk_limit,
            next_mmap: align_up(
                cmp::max(
                    max_mapped_end + PAGE_SIZE_4K,
                    cmp::max(brk_limit + PAGE_SIZE_4K, USER_MMAP_BASE),
                ),
                PAGE_SIZE_4K,
            ),
        },
        mappings,
        exec_root,
        exec_path: prepared.path,
    })
}

fn effective_exec_root(path_root: &str, interpreter: Option<&str>) -> String {
    if path_root != "/" {
        return path_root.into();
    }
    let Some(interpreter) = interpreter else {
        return path_root.into();
    };
    let name = interpreter.rsplit('/').next().unwrap_or(interpreter);
    if name.starts_with("ld-musl-") {
        "/musl".into()
    } else if name.starts_with("ld-linux-") {
        "/glibc".into()
    } else {
        path_root.into()
    }
}

fn prepare_program(
    process: Option<&UserProcess>,
    cwd: &str,
    program_path: &str,
    argv: &[&str],
    depth: usize,
    image: &mut Vec<u8>,
) -> Result<PreparedProgram, String> {
    if program_path.is_empty() || argv.is_empty() {
        return Err("empty argv".into());
    }
    if depth > MAX_SCRIPT_INTERPRETER_DEPTH {
        return Err("script interpreter recursion limit exceeded".into());
    }

    let path = resolve_host_path(cwd.to_string(), program_path)?;
    read_exec_image_into(process, path.as_str(), "program", image)?;

    if let Some(next_argv) = parse_shebang_argv(path.as_str(), image.as_slice(), argv)? {
        let next_refs = next_argv.iter().map(String::as_str).collect::<Vec<_>>();
        let next_program = next_refs.first().copied().unwrap_or(program_path);
        return prepare_program(process, cwd, next_program, &next_refs, depth + 1, image);
    }

    Ok(PreparedProgram {
        argv: argv.iter().map(|arg| (*arg).to_string()).collect(),
        path: path.clone(),
        exec_root: derive_exec_root_from_path(path.as_str()),
    })
}

fn parse_shebang_argv(
    script_path: &str,
    image: &[u8],
    argv: &[&str],
) -> Result<Option<Vec<String>>, String> {
    if image.len() < 2 || &image[..2] != b"#!" {
        return Ok(None);
    }

    let line_end = image
        .iter()
        .position(|&byte| byte == b'\n')
        .unwrap_or(image.len());
    let line = core::str::from_utf8(&image[2..line_end])
        .map_err(|_| format!("invalid shebang in {script_path}"))?
        .trim_end_matches('\r')
        .trim();
    if line.is_empty() {
        return Err(format!("empty shebang interpreter in {script_path}"));
    }

    let mut parts = line.split_whitespace();
    let raw_interpreter = parts.next().unwrap();
    let mut next_argv = resolve_script_interpreter(script_path, raw_interpreter)?;
    next_argv.extend(parts.map(str::to_string));
    next_argv.push(script_path.to_string());
    next_argv.extend(argv.iter().skip(1).map(|arg| (*arg).to_string()));
    Ok(Some(next_argv))
}

fn resolve_script_interpreter(
    script_path: &str,
    raw_interpreter: &str,
) -> Result<Vec<String>, String> {
    let base = script_dir(script_path);
    let resolved = resolve_host_path(base, raw_interpreter)?;
    if matches!(std::fs::metadata(&resolved), Ok(meta) if meta.is_file()) {
        return Ok(vec![resolved]);
    }

    if raw_interpreter == "/bin/sh" || raw_interpreter == "/busybox" {
        if let Some(busybox) = find_busybox_for_script(script_path) {
            return Ok(vec![busybox, "sh".into()]);
        }
    } else if raw_interpreter == "/bin/busybox" {
        if let Some(busybox) = find_busybox_for_script(script_path) {
            return Ok(vec![busybox]);
        }
    }

    Err(format!("script interpreter not found: {raw_interpreter}"))
}

fn find_busybox_for_script(script_path: &str) -> Option<String> {
    let mut candidates = Vec::new();
    match derive_exec_root_from_path(script_path).as_str() {
        "/musl" => candidates.push("/musl/busybox"),
        "/glibc" => candidates.push("/glibc/busybox"),
        _ => {}
    }
    candidates.push("/musl/busybox");
    candidates.push("/glibc/busybox");

    candidates.into_iter().find_map(|path| {
        matches!(std::fs::metadata(path), Ok(meta) if meta.is_file()).then(|| path.to_string())
    })
}

fn script_dir(path: &str) -> String {
    match path.rfind('/') {
        Some(0) | None => "/".into(),
        Some(idx) => path[..idx].to_string(),
    }
}

fn analyze_elf(elf: &ElfFile<'_>, preferred_base: usize) -> Result<ElfLoadInfo, String> {
    let elf_type = elf.header.pt2.type_().as_type();
    let expected_machine = if cfg!(target_arch = "riscv64") {
        Machine::RISC_V
    } else {
        Machine::Other(258)
    };
    if elf.header.pt2.machine().as_machine() != expected_machine {
        return Err("ELF machine does not match current architecture".into());
    }
    let mut min_load_addr: Option<usize> = None;
    let mut max_segment_end = 0usize;
    let mut interpreter = None;
    for ph in elf.program_iter() {
        match ph.get_type().map_err(str_err)? {
            PhType::Load => {
                let seg_start = align_down(ph.virtual_addr() as usize, PAGE_SIZE_4K);
                let seg_end = align_up(
                    ph.virtual_addr() as usize + ph.mem_size() as usize,
                    PAGE_SIZE_4K,
                );
                min_load_addr = Some(match min_load_addr {
                    Some(curr) => curr.min(seg_start),
                    None => seg_start,
                });
                max_segment_end = cmp::max(max_segment_end, seg_end);
            }
            PhType::Interp => interpreter = Some(read_interp_path(elf, &ph)?),
            _ => {}
        }
    }
    let Some(min_load_addr) = min_load_addr else {
        return Err("ELF has no LOAD segments".into());
    };

    let (load_bias, base) = match elf_type {
        ElfType::Executable => (0usize, 0usize),
        ElfType::SharedObject => {
            let mapped_min = align_up(cmp::max(preferred_base, min_load_addr), PAGE_SIZE_4K);
            let load_bias = mapped_min
                .checked_sub(min_load_addr)
                .ok_or_else(|| "failed to compute PIE load bias".to_string())?;
            (load_bias, load_bias)
        }
        _ => return Err("unsupported ELF type".into()),
    };

    Ok(ElfLoadInfo {
        load_bias,
        entry: load_bias + elf.header.pt2.entry_point() as usize,
        phdr: phdr_addr(elf, load_bias).unwrap_or(0),
        max_segment_end: load_bias + max_segment_end,
        base,
        interpreter,
    })
}

fn read_interp_path(elf: &ElfFile<'_>, ph: &ProgramHeader<'_>) -> Result<String, String> {
    let offset = ph.offset() as usize;
    let file_size = ph.file_size() as usize;
    let end = offset
        .checked_add(file_size)
        .ok_or_else(|| "PT_INTERP range overflow".to_string())?;
    let image = elf.input;
    if end > image.len() {
        return Err("PT_INTERP exceeds ELF image".into());
    }
    let raw = &image[offset..end];
    let path = raw.split(|byte| *byte == 0).next().unwrap_or(raw);
    let path = core::str::from_utf8(path).map_err(|_| "invalid PT_INTERP path".to_string())?;
    if path.is_empty() {
        return Err("empty PT_INTERP path".into());
    }
    Ok(path.to_string())
}

#[cfg(target_arch = "loongarch64")]
fn patch_loongarch_musl_main_syscall_stubs(
    exec_root: &str,
    image: &mut [u8],
) -> Result<(), String> {
    if exec_root != "/musl" {
        return Ok(());
    }
    validate_loongarch_musl_patch_manifest()?;
    ensure_loongarch_musl_patch_manifest_symbol("brk")?;
    let elf = ElfFile::new(image).map_err(|err| format!("invalid musl executable ELF: {err}"))?;
    let brk_offset = find_dynsym_file_offset(&elf, "brk")?
        .or_else(|| find_symbol_file_offset(&elf, "brk").ok().flatten());
    drop(elf);
    if let Some(offset) = brk_offset {
        patch_loongarch_musl_brk_stub(image, offset)?;
    }
    Ok(())
}

#[cfg(target_arch = "loongarch64")]
fn patch_loongarch_musl_syscall_stubs(
    exec_root: &str,
    raw_interp: &str,
    image: &mut [u8],
) -> Result<(), String> {
    if exec_root != "/musl" || !raw_interp.contains("ld-musl") {
        return Ok(());
    }
    validate_loongarch_musl_patch_manifest()?;
    let elf = ElfFile::new(image).map_err(|err| format!("invalid musl interpreter ELF: {err}"))?;
    let raw_syscall_patches = [
        ("sched_setparam", general::__NR_sched_setparam),
        ("sched_getparam", general::__NR_sched_getparam),
        ("sched_setscheduler", general::__NR_sched_setscheduler),
        ("sched_getscheduler", general::__NR_sched_getscheduler),
    ];
    let mut offsets = Vec::new();
    for (name, syscall_nr) in raw_syscall_patches {
        ensure_loongarch_musl_patch_manifest_symbol(name)?;
        let Some(offset) = find_dynsym_file_offset(&elf, name)? else {
            continue;
        };
        offsets.push((offset, syscall_nr));
    }
    ensure_loongarch_musl_patch_manifest_symbol("brk")?;
    ensure_loongarch_musl_patch_manifest_symbol("sbrk")?;
    ensure_loongarch_musl_patch_manifest_symbol("gethostname")?;
    let brk_offset = find_dynsym_file_offset(&elf, "brk")?;
    let sbrk_offset = find_dynsym_file_offset(&elf, "sbrk")?;
    let gethostname_offset = find_dynsym_file_offset(&elf, "gethostname")?;
    let syscall_ret_offset = find_symbol_file_offset(&elf, "__syscall_ret")?;
    drop(elf);

    for (offset, syscall_nr) in offsets {
        patch_loongarch_musl_syscall_wrapper(image, offset, syscall_nr)?;
    }
    if let Some(offset) = brk_offset {
        patch_loongarch_musl_brk_stub(image, offset)?;
    }
    if let (Some(offset), Some(syscall_ret_offset)) = (sbrk_offset, syscall_ret_offset) {
        patch_loongarch_musl_sbrk_stub(image, offset, syscall_ret_offset)?;
    }
    if let (Some(gethostname_offset), Some(syscall_ret_offset)) =
        (gethostname_offset, syscall_ret_offset)
    {
        patch_loongarch_musl_gethostname_wrapper(image, gethostname_offset, syscall_ret_offset)?;
    }
    patch_loongarch_musl_readlink_wrappers(image)?;
    Ok(())
}

#[cfg(target_arch = "riscv64")]
fn patch_riscv_musl_main_syscall_stubs(exec_root: &str, image: &mut [u8]) -> Result<(), String> {
    if exec_root != "/musl" {
        return Ok(());
    }
    validate_riscv_musl_patch_manifest()?;
    ensure_riscv_musl_patch_manifest_symbol("brk")?;
    let elf = ElfFile::new(image).map_err(|err| format!("invalid musl executable ELF: {err}"))?;
    let brk_offset = find_dynsym_file_offset(&elf, "brk")?
        .or_else(|| find_symbol_file_offset(&elf, "brk").ok().flatten());
    drop(elf);
    if let Some(offset) = brk_offset {
        patch_riscv_musl_brk_stub(image, offset)?;
    }
    Ok(())
}

#[cfg(target_arch = "riscv64")]
fn patch_riscv_musl_syscall_stubs(
    exec_root: &str,
    raw_interp: &str,
    image: &mut [u8],
) -> Result<(), String> {
    if exec_root != "/musl" || !raw_interp.contains("ld-musl") {
        return Ok(());
    }
    validate_riscv_musl_patch_manifest()?;
    ensure_riscv_musl_patch_manifest_symbol("brk")?;
    ensure_riscv_musl_patch_manifest_symbol("sbrk")?;
    ensure_riscv_musl_patch_manifest_symbol("nice")?;
    ensure_riscv_musl_patch_manifest_symbol("gethostname")?;
    let elf = ElfFile::new(image).map_err(|err| format!("invalid musl interpreter ELF: {err}"))?;
    let brk_offset = find_dynsym_file_offset(&elf, "brk")?;
    let sbrk_offset = find_dynsym_file_offset(&elf, "sbrk")?;
    let nice_offset = find_dynsym_file_offset(&elf, "nice")?;
    let getpriority_offset = find_dynsym_file_offset(&elf, "getpriority")?;
    let setpriority_offset = find_dynsym_file_offset(&elf, "setpriority")?;
    let errno_location_offset = find_dynsym_file_offset(&elf, "__errno_location")?;
    let gethostname_offset = find_dynsym_file_offset(&elf, "gethostname")?;
    let syscall_ret_offset = find_symbol_file_offset(&elf, "__syscall_ret")?;
    drop(elf);
    if let Some(offset) = brk_offset {
        patch_riscv_musl_brk_stub(image, offset)?;
    }
    if let (Some(offset), Some(syscall_ret_offset)) = (sbrk_offset, syscall_ret_offset) {
        patch_riscv_musl_sbrk_stub(image, offset, syscall_ret_offset)?;
    }
    if let (
        Some(nice_offset),
        Some(getpriority_offset),
        Some(setpriority_offset),
        Some(errno_location_offset),
    ) = (
        nice_offset,
        getpriority_offset,
        setpriority_offset,
        errno_location_offset,
    ) {
        patch_riscv_musl_nice_wrapper(
            image,
            nice_offset,
            getpriority_offset,
            setpriority_offset,
            errno_location_offset,
        )?;
    }
    if let (Some(gethostname_offset), Some(syscall_ret_offset)) =
        (gethostname_offset, syscall_ret_offset)
    {
        patch_riscv_musl_gethostname_wrapper(image, gethostname_offset, syscall_ret_offset)?;
    }
    Ok(())
}

#[cfg(any(target_arch = "loongarch64", target_arch = "riscv64"))]
fn find_dynsym_file_offset(elf: &ElfFile<'_>, name: &str) -> Result<Option<usize>, String> {
    let Some(dynsym) = elf.find_section_by_name(".dynsym") else {
        return Ok(None);
    };
    let SectionData::DynSymbolTable64(entries) = dynsym.get_data(elf).map_err(str_err)? else {
        return Ok(None);
    };
    for entry in entries {
        if entry.get_name(elf).unwrap_or("") == name {
            let value = entry.value() as usize;
            if value == 0 {
                continue;
            }
            return vaddr_to_file_offset(elf, value).map(Some);
        }
    }
    Ok(None)
}

#[cfg(any(target_arch = "loongarch64", target_arch = "riscv64"))]
fn find_symbol_file_offset(elf: &ElfFile<'_>, name: &str) -> Result<Option<usize>, String> {
    if let Some(offset) = find_dynsym_file_offset(elf, name)? {
        return Ok(Some(offset));
    }
    let Some(symtab) = elf.find_section_by_name(".symtab") else {
        return Ok(None);
    };
    let SectionData::SymbolTable64(entries) = symtab.get_data(elf).map_err(str_err)? else {
        return Ok(None);
    };
    for entry in entries {
        if entry.get_name(elf).unwrap_or("") == name {
            let value = entry.value() as usize;
            return vaddr_to_file_offset(elf, value).map(Some);
        }
    }
    Ok(None)
}

#[cfg(any(target_arch = "loongarch64", target_arch = "riscv64"))]
fn vaddr_to_file_offset(elf: &ElfFile<'_>, vaddr: usize) -> Result<usize, String> {
    for ph in elf.program_iter() {
        if ph.get_type().map_err(str_err)? != PhType::Load {
            continue;
        }
        let start = ph.virtual_addr() as usize;
        let file_size = ph.file_size() as usize;
        let end = start
            .checked_add(file_size)
            .ok_or_else(|| "ELF LOAD range overflow".to_string())?;
        if (start..end).contains(&vaddr) {
            return Ok(ph.offset() as usize + (vaddr - start));
        }
    }
    Err(format!(
        "symbol address {vaddr:#x} is outside LOAD segments"
    ))
}

#[cfg(target_arch = "riscv64")]
fn patch_riscv_musl_brk_stub(image: &mut [u8], offset: usize) -> Result<(), String> {
    const ENOSYS_BRK_STUB_PREFIX: [u8; 8] = [0x13, 0x01, 0x01, 0xff, 0x13, 0x05, 0x40, 0xff];
    const PATCH_LEN: usize = 20;
    let end = offset
        .checked_add(PATCH_LEN)
        .ok_or_else(|| "riscv musl brk stub patch range overflow".to_string())?;
    if end > image.len() {
        return Err("riscv musl brk stub patch exceeds image".into());
    }
    if image
        .get(offset..offset + ENOSYS_BRK_STUB_PREFIX.len())
        .is_some_and(|prefix| prefix == ENOSYS_BRK_STUB_PREFIX)
    {
        let patch = [
            0xaa, 0x85, // mv a1, a0 (save requested break).
            0x93, 0x08, 0x60, 0x0d, // li a7, __NR_brk (214).
            0x73, 0x00, 0x00, 0x00, // ecall.
            0x33, 0x35, 0xb5, 0x00, // sltu a0, a0, a1 (failure if ret < requested).
            0x33, 0x05, 0xa0, 0x40, // neg a0, a0 (0 on success, -1 on failure).
            0x82, 0x80, // ret.
        ];
        image[offset..end].copy_from_slice(&patch);
    }
    Ok(())
}

#[cfg(target_arch = "riscv64")]
fn patch_riscv_musl_sbrk_stub(
    image: &mut [u8],
    offset: usize,
    syscall_ret_offset: usize,
) -> Result<(), String> {
    const ENOMEM_SBRK_STUB: [u8; 24] = [
        0x63, 0x06, 0x05, 0x00, // beqz a0, +12
        0x13, 0x05, 0x40, 0xff, // li a0, -ENOMEM
        0x6f, 0xe0, 0xcf, 0xd2, // j __syscall_ret
        0x93, 0x08, 0x60, 0x0d, // li a7, __NR_brk
        0x73, 0x00, 0x00, 0x00, // ecall
        0x67, 0x80, 0x00, 0x00, // ret
    ];
    const HELPER_LEN: usize = 40;
    let end = offset
        .checked_add(ENOMEM_SBRK_STUB.len())
        .ok_or_else(|| "riscv musl sbrk stub prefix range overflow".to_string())?;
    if !image
        .get(offset..end)
        .is_some_and(|prefix| prefix == ENOMEM_SBRK_STUB)
    {
        return Ok(());
    }

    let helper_offset = reserve_elf_rx_patch_area(image, HELPER_LEN)?;
    let entry_jump = riscv_j(offset, helper_offset)?;
    image[offset..offset + size_of::<u32>()].copy_from_slice(&entry_jump.to_le_bytes());

    let branch_to_fail = riscv_bne(helper_offset + 24, helper_offset + 32, 10, 12)?;
    let syscall_ret_jump = riscv_j(helper_offset + 36, syscall_ret_offset)?;
    let helper = [
        0xaa,
        0x85, // mv a1, a0 (save increment).
        0x01,
        0x45, // li a0, 0 (query current break).
        0x93,
        0x08,
        0x60,
        0x0d, // li a7, __NR_brk (214).
        0x73,
        0x00,
        0x00,
        0x00, // ecall -> current break in a0.
        0x33,
        0x06,
        0xb5,
        0x00, // add a2, a0, a1 (target break).
        0xaa,
        0x85, // mv a1, a0 (save old break for sbrk return).
        0x32,
        0x85, // mv a0, a2 (request target break).
        0x73,
        0x00,
        0x00,
        0x00, // ecall.
        branch_to_fail.to_le_bytes()[0],
        branch_to_fail.to_le_bytes()[1],
        branch_to_fail.to_le_bytes()[2],
        branch_to_fail.to_le_bytes()[3],
        0x2e,
        0x85, // mv a0, a1 (return old break on success).
        0x82,
        0x80, // ret.
        0x13,
        0x05,
        0x40,
        0xff, // li a0, -ENOMEM.
        syscall_ret_jump.to_le_bytes()[0],
        syscall_ret_jump.to_le_bytes()[1],
        syscall_ret_jump.to_le_bytes()[2],
        syscall_ret_jump.to_le_bytes()[3],
    ];
    image[helper_offset..helper_offset + HELPER_LEN].copy_from_slice(&helper);
    Ok(())
}

#[cfg(target_arch = "riscv64")]
fn patch_riscv_musl_gethostname_wrapper(
    image: &mut [u8],
    offset: usize,
    syscall_ret_offset: usize,
) -> Result<(), String> {
    const KNOWN_PREFIX: [u32; 4] = [
        0xe501_0113, // addi sp, sp, -432
        0x1921_3823, // sd s2, 400(sp)
        0x0081_0913, // addi s2, sp, 8
        0x1891_3c23, // sd s1, 408(sp)
    ];
    if !riscv_words_match(image, offset, &KNOWN_PREFIX)? {
        return Ok(());
    }

    let patch_offset = offset + 0x7c;
    let success_offset = offset + 0x5c;
    let patch = [
        0xfdc0_0513u32,                                   // li a0, -ENAMETOOLONG
        riscv_jal(patch_offset + 4, syscall_ret_offset)?, // __syscall_ret(a0)
        riscv_j(patch_offset + 8, success_offset)?,       // common epilogue
    ];
    for (idx, word) in patch.iter().enumerate() {
        let word_offset = patch_offset + idx * size_of::<u32>();
        let end = word_offset
            .checked_add(size_of::<u32>())
            .ok_or_else(|| "riscv musl gethostname patch range overflow".to_string())?;
        let bytes = image
            .get_mut(word_offset..end)
            .ok_or_else(|| "riscv musl gethostname patch exceeds image".to_string())?;
        bytes.copy_from_slice(&word.to_le_bytes());
    }
    Ok(())
}

#[cfg(target_arch = "riscv64")]
fn riscv_words_match(image: &[u8], offset: usize, words: &[u32]) -> Result<bool, String> {
    for (idx, expected) in words.iter().enumerate() {
        let word_offset = offset
            .checked_add(idx * size_of::<u32>())
            .ok_or_else(|| "RISC-V prefix offset overflow".to_string())?;
        if read_u32_le(image, word_offset)? != *expected {
            return Ok(false);
        }
    }
    Ok(true)
}

#[cfg(target_arch = "riscv64")]
fn patch_riscv_musl_nice_wrapper(
    image: &mut [u8],
    offset: usize,
    getpriority_offset: usize,
    setpriority_offset: usize,
    errno_location_offset: usize,
) -> Result<(), String> {
    const KNOWN_PREFIX: [u8; 16] = [
        0x13, 0x01, 0x01, 0xfe, // addi sp, sp, -32
        0x23, 0x34, 0x91, 0x00, // sd s1, 8(sp)
        0x23, 0x3c, 0x11, 0x00, // sd ra, 24(sp)
        0x23, 0x38, 0x81, 0x00, // sd s0, 16(sp)
    ];
    const PATCH_LEN: usize = 31 * size_of::<u32>();
    let end = offset
        .checked_add(PATCH_LEN)
        .ok_or_else(|| "riscv musl nice patch range overflow".to_string())?;
    if end > image.len() {
        return Err("riscv musl nice patch exceeds image".into());
    }
    if !image
        .get(offset..offset + KNOWN_PREFIX.len())
        .is_some_and(|prefix| prefix == KNOWN_PREFIX)
    {
        return Ok(());
    }

    let words = [
        0xff01_0113,                                      // addi sp, sp, -16
        0x0081_3023,                                      // sd s0, 0(sp)
        0x0011_3423,                                      // sd ra, 8(sp)
        0x0005_0413,                                      // mv s0, a0 (save increment)
        0x0000_0593,                                      // li a1, 0 (PRIO_PROCESS)
        0x0000_0513,                                      // li a0, 0 (self)
        riscv_jal(offset + 0x18, getpriority_offset)?,    // getpriority(0, 0)
        0x00a4_043b,                                      // addw s0, s0, a0
        0xfec0_0293,                                      // li t0, -20
        0x0054_5463,                                      // bge s0, t0, +8
        0xfec0_0413,                                      // li s0, -20
        0x0130_0293,                                      // li t0, 19
        0x0082_d463,                                      // bge t0, s0, +8
        0x0130_0413,                                      // li s0, 19
        0x0004_0613,                                      // mv a2, s0
        0x0000_0593,                                      // li a1, 0
        0x0000_0513,                                      // li a0, 0
        riscv_jal(offset + 0x44, setpriority_offset)?,    // setpriority(0, 0, nice)
        0x0205_0063,                                      // beqz a0, success
        riscv_jal(offset + 0x4c, errno_location_offset)?, // __errno_location()
        0x0005_2283,                                      // lw t0, 0(a0)
        0x00d0_0313,                                      // li t1, EACCES
        0x0062_9663,                                      // bne t0, t1, fail
        0x0010_0293,                                      // li t0, EPERM
        0x0055_2023,                                      // sw t0, 0(a0)
        0xfff0_0413,                                      // li s0, -1
        0x0081_3083,                                      // ld ra, 8(sp)
        0x0004_0513,                                      // mv a0, s0
        0x0001_3403,                                      // ld s0, 0(sp)
        0x0101_0113,                                      // addi sp, sp, 16
        0x0000_8067,                                      // ret
    ];
    for (idx, word) in words.iter().enumerate() {
        let word_offset = offset + idx * size_of::<u32>();
        image[word_offset..word_offset + size_of::<u32>()].copy_from_slice(&word.to_le_bytes());
    }
    Ok(())
}

#[cfg(target_arch = "riscv64")]
fn riscv_jal(from_offset: usize, to_offset: usize) -> Result<u32, String> {
    riscv_jal_with_rd(1, from_offset, to_offset)
}

#[cfg(target_arch = "riscv64")]
fn riscv_j(from_offset: usize, to_offset: usize) -> Result<u32, String> {
    riscv_jal_with_rd(0, from_offset, to_offset)
}

#[cfg(target_arch = "riscv64")]
fn riscv_bne(from_offset: usize, to_offset: usize, rs1: u32, rs2: u32) -> Result<u32, String> {
    riscv_branch(0x1, from_offset, to_offset, rs1, rs2)
}

#[cfg(target_arch = "riscv64")]
fn riscv_branch(
    funct3: u32,
    from_offset: usize,
    to_offset: usize,
    rs1: u32,
    rs2: u32,
) -> Result<u32, String> {
    if funct3 > 0x7 || rs1 > 31 || rs2 > 31 {
        return Err("RISC-V branch register/function out of range".into());
    }
    let delta = (to_offset as i64)
        .checked_sub(from_offset as i64)
        .ok_or_else(|| "RISC-V branch delta overflow".to_string())?;
    if delta % 2 != 0 {
        return Err(format!("unaligned RISC-V branch delta: {delta}"));
    }
    if !(-(1 << 12)..(1 << 12)).contains(&delta) {
        return Err(format!("RISC-V branch delta out of range: {delta}"));
    }
    let imm = (delta as u32) & 0x1fff;
    Ok(((imm & 0x1000) << 19)
        | ((imm & 0x07e0) << 20)
        | (rs2 << 20)
        | (rs1 << 15)
        | (funct3 << 12)
        | ((imm & 0x001e) << 7)
        | ((imm & 0x0800) >> 4)
        | 0x63)
}

#[cfg(target_arch = "riscv64")]
fn riscv_jal_with_rd(rd: u32, from_offset: usize, to_offset: usize) -> Result<u32, String> {
    if rd > 31 {
        return Err(format!("RISC-V jal register out of range: {rd}"));
    }
    let delta = (to_offset as i64)
        .checked_sub(from_offset as i64)
        .ok_or_else(|| "RISC-V jal delta overflow".to_string())?;
    if delta % 2 != 0 {
        return Err(format!("unaligned RISC-V jal delta: {delta}"));
    }
    if !(-(1 << 20)..(1 << 20)).contains(&delta) {
        return Err(format!("RISC-V jal delta out of range: {delta}"));
    }
    let imm = (delta as u32) & 0x001f_ffff;
    Ok(((imm & 0x0010_0000) << 11)
        | (imm & 0x000f_f000)
        | ((imm & 0x0000_0800) << 9)
        | ((imm & 0x0000_07fe) << 20)
        | (rd << 7)
        | 0x6f)
}

#[cfg(target_arch = "loongarch64")]
fn patch_loongarch_musl_brk_stub(image: &mut [u8], offset: usize) -> Result<(), String> {
    const ENOSYS_BRK_STUB_PREFIX: [u8; 4] = [0x63, 0xc0, 0xff, 0x02];
    const PATCH_LEN: usize = 24;

    let end = offset
        .checked_add(PATCH_LEN)
        .ok_or_else(|| "loongarch musl brk stub patch range overflow".to_string())?;
    if end > image.len() {
        return Err("loongarch musl brk stub patch exceeds image".into());
    }
    if image
        .get(offset..offset + ENOSYS_BRK_STUB_PREFIX.len())
        .is_some_and(|prefix| prefix == ENOSYS_BRK_STUB_PREFIX)
    {
        let load_syscall_nr = loongarch_addi_w_a7(general::__NR_brk)?;
        let patch = [
            0x0015_0085u32.to_le_bytes(), // move a1, a0 (save requested break).
            load_syscall_nr.to_le_bytes(),
            0x002b_0000u32.to_le_bytes(), // syscall 0.
            0x0012_9484u32.to_le_bytes(), // sltu a0, a0, a1.
            0x0011_9004u32.to_le_bytes(), // sub.d a0, zero, a0.
            0x4c00_0020u32.to_le_bytes(), // ret.
        ]
        .concat();
        image[offset..end].copy_from_slice(&patch);
    }
    Ok(())
}

#[cfg(target_arch = "loongarch64")]
fn patch_loongarch_musl_sbrk_stub(
    image: &mut [u8],
    offset: usize,
    syscall_ret_offset: usize,
) -> Result<(), String> {
    const ENOMEM_SBRK_STUB: [u8; 24] = [
        0x80, 0x10, 0x00, 0x44, // bnez a0, +16
        0x0b, 0x58, 0x83, 0x02, // a7 = __NR_brk
        0x00, 0x00, 0x2b, 0x00, // syscall 0
        0x20, 0x00, 0x00, 0x4c, // ret
        0x04, 0xd0, 0xbf, 0x02, // a0 = -ENOMEM
        0xff, 0x3b, 0xe3, 0x53, // b __syscall_ret
    ];
    const SYSCALL: u32 = 0x002b_0000;
    const RET: u32 = 0x4c00_0020;
    const HELPER_LEN: usize = 52;
    let end = offset
        .checked_add(ENOMEM_SBRK_STUB.len())
        .ok_or_else(|| "loongarch musl sbrk stub prefix range overflow".to_string())?;
    if !image
        .get(offset..end)
        .is_some_and(|prefix| prefix == ENOMEM_SBRK_STUB)
    {
        return Ok(());
    }

    let helper_offset = reserve_elf_rx_patch_area(image, HELPER_LEN)?;
    let entry_jump = loongarch_b(offset, helper_offset)?;
    image[offset..offset + size_of::<u32>()].copy_from_slice(&entry_jump.to_le_bytes());

    let load_syscall_nr = loongarch_addi_w_a7(general::__NR_brk)?;
    let branch_to_fail = loongarch_bne(helper_offset + 32, helper_offset + 44, 4, 6)?;
    let syscall_ret_jump = loongarch_b(helper_offset + 48, syscall_ret_offset)?;
    let patch = [
        loongarch_move(5, 4).to_le_bytes(), // move a1, a0 (save increment).
        loongarch_addi_w(4, 0)?.to_le_bytes(), // a0 = 0 (query current break).
        load_syscall_nr.to_le_bytes(),
        SYSCALL.to_le_bytes(),
        loongarch_add_d(6, 4, 5).to_le_bytes(), // a2 = current break + increment.
        loongarch_move(5, 4).to_le_bytes(),     // a1 = old break.
        loongarch_move(4, 6).to_le_bytes(),     // a0 = target break.
        SYSCALL.to_le_bytes(),
        branch_to_fail.to_le_bytes(),
        loongarch_move(4, 5).to_le_bytes(), // return old break on success.
        RET.to_le_bytes(),
        loongarch_addi_w(4, -12)?.to_le_bytes(), // a0 = -ENOMEM.
        syscall_ret_jump.to_le_bytes(),
    ]
    .concat();
    image[helper_offset..helper_offset + HELPER_LEN].copy_from_slice(&patch);
    Ok(())
}

#[cfg(target_arch = "loongarch64")]
fn patch_loongarch_musl_syscall_wrapper(
    image: &mut [u8],
    offset: usize,
    syscall_nr: u32,
) -> Result<(), String> {
    const ENOSYS_STUB_PREFIX: [u8; 8] = [0x63, 0xc0, 0xff, 0x02, 0x04, 0x68, 0xbf, 0x02];
    const SYSCALL: u32 = 0x002b_0000;
    const RET: u32 = 0x4c00_0020;
    const PATCH_LEN: usize = 16;

    let end = offset
        .checked_add(PATCH_LEN)
        .ok_or_else(|| "musl sched wrapper patch range overflow".to_string())?;
    if end > image.len() {
        return Err("musl sched wrapper patch exceeds image".into());
    }
    let known_stub = image
        .get(offset..offset + ENOSYS_STUB_PREFIX.len())
        .is_some_and(|prefix| prefix == ENOSYS_STUB_PREFIX);
    if known_stub {
        // These exported musl libc wrappers originally call __syscall_ret with
        // -ENOSYS.  Keep that errno-normalizing tail after replacing the ENOSYS
        // immediate with a real syscall, so libc calls return -1 and set errno
        // while raw syscall tests still observe raw -errno.
        let original_call_offset = offset + 12;
        let original_call = read_u32_le(image, original_call_offset)?;
        let syscall_ret_offset =
            loongarch_branch_target_offset(original_call_offset, original_call)?;
        let tail_call = loongarch_b(offset + 8, syscall_ret_offset)?;
        let load_syscall_nr = loongarch_addi_w_a7(syscall_nr)?;
        let patch = [
            load_syscall_nr.to_le_bytes(),
            SYSCALL.to_le_bytes(),
            tail_call.to_le_bytes(),
            RET.to_le_bytes(),
        ]
        .concat();
        image[offset..end].copy_from_slice(&patch);
    }
    Ok(())
}

#[cfg(target_arch = "loongarch64")]
fn patch_loongarch_musl_gethostname_wrapper(
    image: &mut [u8],
    offset: usize,
    syscall_ret_offset: usize,
) -> Result<(), String> {
    const KNOWN_PREFIX: [u32; 4] = [
        0x02f9_4063, // addi.d sp, sp, -432
        0x29c6_4079, // st.d s2, sp, 400
        0x02c0_2079, // addi.d s2, sp, 8
        0x29c6_6078, // st.d s1, sp, 408
    ];
    if !loongarch_words_match(image, offset, &KNOWN_PREFIX)? {
        return Ok(());
    }

    let patch_offset = offset + 0x80;
    let success_offset = offset + 0x60;
    let patch = [
        loongarch_addi_w(4, -36)?.to_le_bytes(), // a0 = -ENAMETOOLONG
        loongarch_bl(patch_offset + 4, syscall_ret_offset)?.to_le_bytes(),
        loongarch_b(patch_offset + 8, success_offset)?.to_le_bytes(),
    ]
    .concat();
    let end = patch_offset
        .checked_add(patch.len())
        .ok_or_else(|| "loongarch musl gethostname patch range overflow".to_string())?;
    let bytes = image
        .get_mut(patch_offset..end)
        .ok_or_else(|| "loongarch musl gethostname patch exceeds image".to_string())?;
    bytes.copy_from_slice(&patch);
    Ok(())
}

#[cfg(target_arch = "loongarch64")]
fn patch_loongarch_musl_readlink_wrappers(image: &mut [u8]) -> Result<(), String> {
    ensure_loongarch_musl_patch_manifest_symbol("readlink")?;
    ensure_loongarch_musl_patch_manifest_symbol("readlinkat")?;
    let elf = ElfFile::new(image).map_err(|err| format!("invalid musl interpreter ELF: {err}"))?;
    let Some(readlink_offset) = find_dynsym_file_offset(&elf, "readlink")? else {
        return Ok(());
    };
    let Some(readlinkat_offset) = find_dynsym_file_offset(&elf, "readlinkat")? else {
        return Ok(());
    };
    drop(elf);

    const READLINK_KNOWN_PREFIX: [u32; 4] = [
        0x02ff_8063, // addi.d sp, sp, -32
        0x29c0_6061, // st.d ra, sp, 24
        0x0015_00ac, // move a0, a1
        0x0015_00c7, // move a3, a2
    ];
    const READLINKAT_KNOWN_PREFIX: [u32; 4] = [
        0x02ff_8063, // addi.d sp, sp, -32
        0x29c0_6061, // st.d ra, sp, 24
        0x02c0_2068, // ld.d s0, sp, 8
        0x4000_24e0, // readlinkat zero-length fast path branch
    ];
    if !loongarch_words_match(image, readlink_offset, &READLINK_KNOWN_PREFIX)?
        || !loongarch_words_match(image, readlinkat_offset, &READLINKAT_KNOWN_PREFIX)?
    {
        return Ok(());
    }

    let syscall_ret_offset = {
        let call_offset = readlinkat_offset + 0x20;
        let call = read_u32_le(image, call_offset)?;
        loongarch_branch_target_offset(call_offset, call)?
    };

    patch_loongarch_musl_readlink(image, readlink_offset, syscall_ret_offset)?;
    patch_loongarch_musl_readlinkat(image, readlinkat_offset, syscall_ret_offset)
}

#[cfg(target_arch = "loongarch64")]
fn patch_loongarch_musl_readlink(
    image: &mut [u8],
    offset: usize,
    syscall_ret_offset: usize,
) -> Result<(), String> {
    const PATCH_LEN: usize = 32;
    let end = offset
        .checked_add(PATCH_LEN)
        .ok_or_else(|| "loongarch musl readlink patch range overflow".to_string())?;
    if end > image.len() {
        return Err("loongarch musl readlink patch exceeds image".into());
    }
    let tail_call = loongarch_b(offset + 24, syscall_ret_offset)?;
    let load_at_fdcwd = loongarch_addi_w(4, -100)?;
    let load_syscall_nr = loongarch_addi_w_a7(general::__NR_readlinkat)?;
    let patch = [
        loongarch_move(7, 6).to_le_bytes(), // move a3, a2 (bufsiz)
        loongarch_move(6, 5).to_le_bytes(), // move a2, a1 (buf)
        loongarch_move(5, 4).to_le_bytes(), // move a1, a0 (path)
        load_at_fdcwd.to_le_bytes(),        // a0 = AT_FDCWD
        load_syscall_nr.to_le_bytes(),
        0x002b_0000u32.to_le_bytes(), // syscall 0
        tail_call.to_le_bytes(),
        0x4c00_0020u32.to_le_bytes(), // ret (unreachable padding)
    ]
    .concat();
    image[offset..end].copy_from_slice(&patch);
    Ok(())
}

#[cfg(target_arch = "loongarch64")]
fn patch_loongarch_musl_readlinkat(
    image: &mut [u8],
    offset: usize,
    syscall_ret_offset: usize,
) -> Result<(), String> {
    const PATCH_LEN: usize = 16;
    let end = offset
        .checked_add(PATCH_LEN)
        .ok_or_else(|| "loongarch musl readlinkat patch range overflow".to_string())?;
    if end > image.len() {
        return Err("loongarch musl readlinkat patch exceeds image".into());
    }
    let tail_call = loongarch_b(offset + 8, syscall_ret_offset)?;
    let load_syscall_nr = loongarch_addi_w_a7(general::__NR_readlinkat)?;
    let patch = [
        load_syscall_nr.to_le_bytes(),
        0x002b_0000u32.to_le_bytes(), // syscall 0
        tail_call.to_le_bytes(),
        0x4c00_0020u32.to_le_bytes(), // ret (unreachable padding)
    ]
    .concat();
    image[offset..end].copy_from_slice(&patch);
    Ok(())
}

#[cfg(target_arch = "loongarch64")]
fn loongarch_words_match(image: &[u8], offset: usize, words: &[u32]) -> Result<bool, String> {
    for (idx, expected) in words.iter().enumerate() {
        let word_offset = offset
            .checked_add(idx * size_of::<u32>())
            .ok_or_else(|| "LoongArch prefix offset overflow".to_string())?;
        if read_u32_le(image, word_offset)? != *expected {
            return Ok(false);
        }
    }
    Ok(true)
}

#[cfg(any(target_arch = "loongarch64", target_arch = "riscv64"))]
fn read_u32_le(image: &[u8], offset: usize) -> Result<u32, String> {
    let end = offset
        .checked_add(size_of::<u32>())
        .ok_or_else(|| "LoongArch instruction read range overflow".to_string())?;
    let bytes = image
        .get(offset..end)
        .ok_or_else(|| "LoongArch instruction read exceeds image".to_string())?;
    Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
}

#[cfg(any(target_arch = "loongarch64", target_arch = "riscv64"))]
fn read_u16_le(image: &[u8], offset: usize) -> Result<u16, String> {
    let end = offset
        .checked_add(size_of::<u16>())
        .ok_or_else(|| "ELF halfword read range overflow".to_string())?;
    let bytes = image
        .get(offset..end)
        .ok_or_else(|| "ELF halfword read exceeds image".to_string())?;
    Ok(u16::from_le_bytes(bytes.try_into().unwrap()))
}

#[cfg(any(target_arch = "loongarch64", target_arch = "riscv64"))]
fn read_u64_le(image: &[u8], offset: usize) -> Result<u64, String> {
    let end = offset
        .checked_add(size_of::<u64>())
        .ok_or_else(|| "ELF word read range overflow".to_string())?;
    let bytes = image
        .get(offset..end)
        .ok_or_else(|| "ELF word read exceeds image".to_string())?;
    Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
}

#[cfg(any(target_arch = "loongarch64", target_arch = "riscv64"))]
fn write_u64_le(image: &mut [u8], offset: usize, value: u64) -> Result<(), String> {
    let end = offset
        .checked_add(size_of::<u64>())
        .ok_or_else(|| "ELF word write range overflow".to_string())?;
    let bytes = image
        .get_mut(offset..end)
        .ok_or_else(|| "ELF word write exceeds image".to_string())?;
    bytes.copy_from_slice(&value.to_le_bytes());
    Ok(())
}

#[cfg(any(target_arch = "loongarch64", target_arch = "riscv64"))]
fn reserve_elf_rx_patch_area(image: &mut [u8], patch_len: usize) -> Result<usize, String> {
    const PT_LOAD: u32 = 1;
    const PF_X: u32 = 1;
    if image.len() < 64 || image.get(0..4) != Some(b"\x7fELF") {
        return Err("invalid ELF image for RX patch reservation".into());
    }
    let phoff = read_u64_le(image, 32)? as usize;
    let phentsize = read_u16_le(image, 54)? as usize;
    let phnum = read_u16_le(image, 56)? as usize;
    if phentsize < 56 {
        return Err("unsupported ELF program header size".into());
    }

    let mut load_offsets = Vec::new();
    for idx in 0..phnum {
        let ph = phoff
            .checked_add(idx * phentsize)
            .ok_or_else(|| "ELF program header offset overflow".to_string())?;
        let p_type = read_u32_le(image, ph)?;
        if p_type == PT_LOAD {
            load_offsets.push(read_u64_le(image, ph + 8)? as usize);
        }
    }

    for idx in 0..phnum {
        let ph = phoff
            .checked_add(idx * phentsize)
            .ok_or_else(|| "ELF program header offset overflow".to_string())?;
        let p_type = read_u32_le(image, ph)?;
        let p_flags = read_u32_le(image, ph + 4)?;
        if p_type != PT_LOAD || p_flags & PF_X == 0 {
            continue;
        }
        let p_offset = read_u64_le(image, ph + 8)? as usize;
        let p_filesz = read_u64_le(image, ph + 32)? as usize;
        let p_memsz = read_u64_le(image, ph + 40)? as usize;
        if p_memsz < p_filesz {
            continue;
        }
        let old_end = p_offset
            .checked_add(p_filesz)
            .ok_or_else(|| "ELF LOAD end overflow".to_string())?;
        let patch_offset = align_up(old_end, size_of::<u32>());
        let patch_end = patch_offset
            .checked_add(patch_len)
            .ok_or_else(|| "ELF RX patch end overflow".to_string())?;
        let next_load = load_offsets
            .iter()
            .copied()
            .filter(|off| *off > p_offset)
            .min()
            .unwrap_or(image.len());
        if patch_end > next_load || patch_end > image.len() {
            continue;
        }
        let new_filesz = patch_end
            .checked_sub(p_offset)
            .ok_or_else(|| "ELF RX patch size underflow".to_string())?;
        let new_memsz = p_memsz.max(new_filesz);
        write_u64_le(image, ph + 32, new_filesz as u64)?;
        write_u64_le(image, ph + 40, new_memsz as u64)?;
        return Ok(patch_offset);
    }

    Err("no executable ELF LOAD segment has padding for musl sbrk patch".into())
}

#[cfg(target_arch = "loongarch64")]
fn loongarch_bl(from_offset: usize, to_offset: usize) -> Result<u32, String> {
    loongarch_branch(0x15, from_offset, to_offset)
}

#[cfg(target_arch = "loongarch64")]
fn loongarch_branch_target_offset(branch_offset: usize, instruction: u32) -> Result<usize, String> {
    const BL_OPCODE: u32 = 0x15;
    if instruction >> 26 != BL_OPCODE {
        return Err(format!(
            "expected LoongArch bl in musl ENOSYS stub at {branch_offset:#x}, got {instruction:#x}"
        ));
    }
    let raw = ((instruction >> 10) & 0xffff) | ((instruction & 0x3ff) << 16);
    let signed = sign_extend_i64(raw as i64, 26)
        .checked_shl(2)
        .ok_or_else(|| "LoongArch branch offset overflow".to_string())?;
    let target = (branch_offset as i64)
        .checked_add(signed)
        .ok_or_else(|| "LoongArch branch target overflow".to_string())?;
    if target < 0 {
        return Err("LoongArch branch target is negative".into());
    }
    Ok(target as usize)
}

#[cfg(target_arch = "loongarch64")]
fn loongarch_b(from_offset: usize, to_offset: usize) -> Result<u32, String> {
    loongarch_branch(0x14, from_offset, to_offset)
}

#[cfg(target_arch = "loongarch64")]
fn loongarch_bne(from_offset: usize, to_offset: usize, rj: u32, rd: u32) -> Result<u32, String> {
    if rj > 31 || rd > 31 {
        return Err("LoongArch branch register out of range".into());
    }
    let delta = (to_offset as i64)
        .checked_sub(from_offset as i64)
        .ok_or_else(|| "LoongArch conditional branch delta overflow".to_string())?;
    if delta % 4 != 0 {
        return Err(format!(
            "unaligned LoongArch conditional branch delta: {delta}"
        ));
    }
    let words = delta / 4;
    if !(-(1 << 15)..(1 << 15)).contains(&words) {
        return Err(format!(
            "LoongArch conditional branch delta out of range: {delta}"
        ));
    }
    let raw = (words as u32) & 0xffff;
    Ok((0x17 << 26) | (raw << 10) | (rj << 5) | rd)
}

#[cfg(target_arch = "loongarch64")]
fn loongarch_branch(opcode: u32, from_offset: usize, to_offset: usize) -> Result<u32, String> {
    const B_OPCODE: u32 = 0x14;
    const BL_OPCODE: u32 = 0x15;
    if opcode != B_OPCODE && opcode != BL_OPCODE {
        return Err(format!("unsupported LoongArch branch opcode: {opcode:#x}"));
    }
    let delta = (to_offset as i64)
        .checked_sub(from_offset as i64)
        .ok_or_else(|| "LoongArch branch delta overflow".to_string())?;
    if delta % 4 != 0 {
        return Err(format!("unaligned LoongArch branch delta: {delta}"));
    }
    let words = delta / 4;
    if !(-(1 << 25)..(1 << 25)).contains(&words) {
        return Err(format!("LoongArch branch delta out of range: {delta}"));
    }
    let raw = (words as u32) & 0x03ff_ffff;
    Ok((opcode << 26) | ((raw & 0xffff) << 10) | ((raw >> 16) & 0x3ff))
}

#[cfg(target_arch = "loongarch64")]
fn sign_extend_i64(value: i64, bits: u32) -> i64 {
    let shift = 64 - bits;
    (value << shift) >> shift
}

#[cfg(target_arch = "loongarch64")]
fn loongarch_addi_w_a7(imm: u32) -> Result<u32, String> {
    if imm > 0xfff {
        return Err(format!("LoongArch addi.w immediate out of range: {imm}"));
    }
    Ok(0x0280_000b | (imm << 10))
}

#[cfg(target_arch = "loongarch64")]
fn loongarch_addi_w(rd: u32, imm: i32) -> Result<u32, String> {
    if rd > 31 {
        return Err(format!("LoongArch register out of range: {rd}"));
    }
    if !(-2048..=2047).contains(&imm) {
        return Err(format!("LoongArch addi.w immediate out of range: {imm}"));
    }
    Ok(0x0280_0000 | (((imm as u32) & 0xfff) << 10) | rd)
}

#[cfg(target_arch = "loongarch64")]
fn loongarch_move(rd: u32, rj: u32) -> u32 {
    0x0015_0000 | (rj << 5) | rd
}

#[cfg(target_arch = "loongarch64")]
fn loongarch_add_d(rd: u32, rj: u32, rk: u32) -> u32 {
    0x0010_8000 | (rk << 10) | (rj << 5) | rd
}

fn map_elf_image(
    aspace: &mut AddrSpace,
    image: &[u8],
    elf: &ElfFile<'_>,
    info: &ElfLoadInfo,
) -> Result<Vec<LoadedMapping>, String> {
    let mut mappings = Vec::new();
    for ph in elf.program_iter() {
        if ph.get_type().map_err(str_err)? == PhType::Load {
            if let Some(mapping) = map_load_segment(aspace, image, &ph, info.load_bias)? {
                mappings.push(mapping);
            }
        }
    }
    mappings.sort_by_key(|mapping| mapping.start);
    Ok(mappings)
}

fn map_load_segment(
    aspace: &mut AddrSpace,
    image: &[u8],
    ph: &ProgramHeader<'_>,
    load_bias: usize,
) -> Result<Option<LoadedMapping>, String> {
    let start = load_bias + ph.virtual_addr() as usize;
    let mem_size = ph.mem_size() as usize;
    if mem_size == 0 {
        return Ok(None);
    }
    let seg_start = align_down(start, PAGE_SIZE_4K);
    let seg_end = align_up(start + mem_size, PAGE_SIZE_4K);
    let seg_size = seg_end - seg_start;
    aspace
        .map_alloc(
            VirtAddr::from(seg_start),
            seg_size,
            flags_from_ph(ph.flags()),
            true,
        )
        .map_err(|err| format!("failed to map ELF segment at {seg_start:#x}: {err}"))?;

    let file_size = ph.file_size() as usize;
    if file_size != 0 {
        let offset = ph.offset() as usize;
        let end = offset
            .checked_add(file_size)
            .ok_or_else(|| "ELF segment range overflow".to_string())?;
        if end > image.len() {
            return Err("ELF segment exceeds image size".into());
        }
        let data = &image[offset..offset + file_size];
        aspace
            .write(VirtAddr::from(start), data)
            .map_err(|err| format!("failed to write ELF segment at {start:#x}: {err}"))?;
    }
    Ok(Some(LoadedMapping {
        start: seg_start,
        size: seg_size,
        prot: prot_from_ph(ph.flags()),
    }))
}

fn phdr_addr(elf: &ElfFile<'_>, load_bias: usize) -> Option<usize> {
    let phoff = elf.header.pt2.ph_offset() as usize;
    for ph in elf.program_iter() {
        if ph.get_type().ok()? != PhType::Load {
            continue;
        }
        let seg_offset = ph.offset() as usize;
        let seg_end = seg_offset.checked_add(ph.file_size() as usize)?;
        if (seg_offset..seg_end).contains(&phoff) {
            return Some(load_bias + ph.virtual_addr() as usize + (phoff - seg_offset));
        }
    }
    None
}

fn build_initial_stack(
    aspace: &mut AddrSpace,
    stack_base: usize,
    stack_top: usize,
    argv: &[&str],
    env: &[&str],
    execfn: &str,
    entry: usize,
    interp_base: usize,
    phdr: usize,
    phent: usize,
    phnum: usize,
) -> Result<usize, String> {
    let mut sp = stack_top;
    let random_bytes = [0x55u8; 16];
    let random_ptr = push_stack_bytes(aspace, stack_base, &mut sp, &random_bytes, 16)?;
    let mut execfn_bytes = execfn.as_bytes().to_vec();
    execfn_bytes.push(0);
    let execfn_ptr = push_stack_bytes(aspace, stack_base, &mut sp, &execfn_bytes, 1)?;
    let mut platform_bytes = AUX_PLATFORM.as_bytes().to_vec();
    platform_bytes.push(0);
    let platform_ptr = push_stack_bytes(aspace, stack_base, &mut sp, &platform_bytes, 1)?;

    let mut arg_ptrs = Vec::with_capacity(argv.len());
    for arg in argv.iter().rev() {
        let mut bytes = arg.as_bytes().to_vec();
        bytes.push(0);
        let ptr = push_stack_bytes(aspace, stack_base, &mut sp, &bytes, 1)?;
        arg_ptrs.push(ptr);
    }
    arg_ptrs.reverse();

    let mut env_ptrs = Vec::with_capacity(env.len());
    for item in env.iter().rev() {
        let mut bytes = item.as_bytes().to_vec();
        bytes.push(0);
        let ptr = push_stack_bytes(aspace, stack_base, &mut sp, &bytes, 1)?;
        env_ptrs.push(ptr);
    }
    env_ptrs.reverse();

    let aux = [
        AuxEntry {
            key: auxvec::AT_PAGESZ as usize,
            value: PAGE_SIZE_4K,
        },
        AuxEntry {
            key: auxvec::AT_UID as usize,
            value: 0,
        },
        AuxEntry {
            key: auxvec::AT_EUID as usize,
            value: 0,
        },
        AuxEntry {
            key: auxvec::AT_GID as usize,
            value: 0,
        },
        AuxEntry {
            key: auxvec::AT_EGID as usize,
            value: 0,
        },
        AuxEntry {
            key: auxvec::AT_SECURE as usize,
            value: 0,
        },
        AuxEntry {
            key: auxvec::AT_FLAGS as usize,
            value: 0,
        },
        AuxEntry {
            key: auxvec::AT_CLKTCK as usize,
            value: AUX_CLOCK_TICKS,
        },
        AuxEntry {
            key: auxvec::AT_HWCAP as usize,
            value: 0,
        },
        AuxEntry {
            key: auxvec::AT_HWCAP2 as usize,
            value: 0,
        },
        AuxEntry {
            key: auxvec::AT_PLATFORM as usize,
            value: platform_ptr,
        },
        AuxEntry {
            key: auxvec::AT_BASE_PLATFORM as usize,
            value: platform_ptr,
        },
        AuxEntry {
            key: auxvec::AT_RANDOM as usize,
            value: random_ptr,
        },
        AuxEntry {
            key: auxvec::AT_PHDR as usize,
            value: phdr,
        },
        AuxEntry {
            key: auxvec::AT_PHENT as usize,
            value: phent,
        },
        AuxEntry {
            key: auxvec::AT_PHNUM as usize,
            value: phnum,
        },
        AuxEntry {
            key: auxvec::AT_BASE as usize,
            value: interp_base,
        },
        AuxEntry {
            key: auxvec::AT_ENTRY as usize,
            value: entry,
        },
        AuxEntry {
            key: auxvec::AT_EXECFN as usize,
            value: execfn_ptr,
        },
        AuxEntry {
            key: auxvec::AT_NULL as usize,
            value: 0,
        },
    ];

    let mut words = Vec::with_capacity(1 + arg_ptrs.len() + 1 + env_ptrs.len() + 1 + aux.len() * 2);
    words.push(argv.len());
    words.extend(arg_ptrs.iter().copied());
    words.push(0);
    words.extend(env_ptrs.iter().copied());
    words.push(0);
    for item in aux {
        words.push(item.key);
        words.push(item.value);
    }
    let bytes = words_to_bytes(&words);
    sp = align_down(sp.saturating_sub(bytes.len()), 16);
    let end = sp + bytes.len();
    if sp < stack_base || end > stack_top {
        return Err("user stack overflow".into());
    }
    aspace
        .populate_range(VirtAddr::from(sp), bytes.len(), PageFaultFlags::WRITE)
        .map_err(|err| format!("failed to populate user stack pages: {err}"))?;
    aspace
        .write(VirtAddr::from(sp), &bytes)
        .map_err(|err| format!("failed to populate user stack: {err}"))?;
    Ok(sp)
}

fn push_stack_bytes(
    aspace: &mut AddrSpace,
    stack_base: usize,
    sp: &mut usize,
    data: &[u8],
    align: usize,
) -> Result<usize, String> {
    *sp = align_down(sp.saturating_sub(data.len()), align.max(1));
    if *sp < stack_base {
        return Err("user stack overflow".into());
    }
    aspace
        .populate_range(VirtAddr::from(*sp), data.len(), PageFaultFlags::WRITE)
        .map_err(|err| format!("failed to populate user stack pages: {err}"))?;
    aspace
        .write(VirtAddr::from(*sp), data)
        .map_err(|err| format!("failed to write user stack data: {err}"))?;
    Ok(*sp)
}

fn words_to_bytes(words: &[usize]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(words.len() * size_of::<usize>());
    for word in words {
        bytes.extend_from_slice(&word.to_ne_bytes());
    }
    bytes
}

fn flags_from_ph(flags: PhFlags) -> MappingFlags {
    let mut out = MappingFlags::USER;
    if flags.is_read() || flags.is_execute() {
        out |= MappingFlags::READ;
    }
    if flags.is_write() {
        out |= MappingFlags::WRITE;
    }
    if flags.is_execute() {
        out |= MappingFlags::EXECUTE;
    }
    out
}

fn prot_from_ph(flags: PhFlags) -> u32 {
    let mut out = 0u32;
    if flags.is_read() || flags.is_execute() {
        out |= general::PROT_READ;
    }
    if flags.is_write() {
        out |= general::PROT_WRITE;
    }
    if flags.is_execute() {
        out |= general::PROT_EXEC;
    }
    out
}
