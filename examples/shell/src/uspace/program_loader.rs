use core::cmp;
use core::mem::size_of;

use axerrno::{AxError, LinuxError};
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
use xmas_elf::ElfFile;

use super::linux_abi::{
    AUX_CLOCK_TICKS, AUX_PLATFORM, MAX_SCRIPT_INTERPRETER_DEPTH, TESTSUITE_STAGE_ROOT,
    USER_BRK_GROW_SIZE, USER_MMAP_BASE, USER_PIE_LOAD_BASE, USER_STACK_SIZE, USER_STACK_TOP,
};
use super::memory_map::{align_down, align_up, user_mapping_flags};
use super::perf_counters;
use super::runtime_paths::{
    try_derive_exec_root_from_path, try_resolve_host_path, try_resolve_runtime_support_file,
    try_runtime_absolute_path_candidates, try_staged_cwd_absolute_path_candidates,
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
const EXEC_IMAGE_READ_CHUNK: usize = 64 * 1024;
pub(super) const EXEC_LOADER_ENOMEM_PREFIX: &str = "exec-loader-ENOMEM: ";

pub(super) fn exec_loader_enomem(message: String) -> String {
    let mut tagged = String::new();
    let _ = tagged.try_reserve_exact(EXEC_LOADER_ENOMEM_PREFIX.len() + message.len());
    tagged.push_str(EXEC_LOADER_ENOMEM_PREFIX);
    tagged.push_str(message.as_str());
    tagged
}

fn exec_loader_enomem_context(context: &str) -> String {
    let mut tagged = String::new();
    let _ = tagged.try_reserve_exact(EXEC_LOADER_ENOMEM_PREFIX.len() + context.len());
    tagged.push_str(EXEC_LOADER_ENOMEM_PREFIX);
    tagged.push_str(context);
    tagged
}

fn exec_loader_join(parts: &[&str], context: &str) -> Result<String, String> {
    let len = parts.iter().try_fold(0usize, |len, part| {
        len.checked_add(part.len())
            .ok_or_else(|| exec_loader_enomem_context(context))
    })?;
    let mut out = String::new();
    out.try_reserve_exact(len)
        .map_err(|_| exec_loader_enomem_context(context))?;
    for part in parts {
        out.push_str(part);
    }
    Ok(out)
}

fn exec_loader_reserve<T>(
    vec: &mut Vec<T>,
    additional: usize,
    context: &str,
) -> Result<(), String> {
    vec.try_reserve_exact(additional)
        .map_err(|_| exec_loader_enomem_context(context))
}

fn exec_loader_owned_string(value: &str, context: &str) -> Result<String, String> {
    let mut out = String::new();
    out.try_reserve_exact(value.len())
        .map_err(|_| exec_loader_enomem_context(context))?;
    out.push_str(value);
    Ok(out)
}

fn exec_loader_push_string(
    values: &mut Vec<String>,
    value: &str,
    context: &str,
) -> Result<(), String> {
    exec_loader_reserve(values, 1, context)?;
    values.push(exec_loader_owned_string(value, context)?);
    Ok(())
}

fn exec_loader_owned_strings(items: &[&str], context: &str) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    exec_loader_reserve(&mut out, items.len(), context)?;
    for item in items {
        out.push(exec_loader_owned_string(item, context)?);
    }
    Ok(out)
}

fn stack_bytes_with_nul(value: &str, context: &str) -> Result<Vec<u8>, String> {
    let total = value
        .len()
        .checked_add(1)
        .ok_or_else(|| "user stack string length overflow".to_string())?;
    let mut bytes = Vec::new();
    exec_loader_reserve(&mut bytes, total, context)?;
    bytes.extend_from_slice(value.as_bytes());
    bytes.push(0);
    Ok(bytes)
}

pub(super) fn exec_loader_string_refs<'a>(
    items: &'a [String],
    context: &str,
) -> Result<Vec<&'a str>, String> {
    let mut refs = Vec::new();
    exec_loader_reserve(&mut refs, items.len(), context)?;
    refs.extend(items.iter().map(String::as_str));
    Ok(refs)
}

pub(super) fn exec_loader_axerr(context: String, err: AxError) -> String {
    if LinuxError::from(err) == LinuxError::ENOMEM {
        exec_loader_enomem(context)
    } else {
        let message = format!("{context}: {err}");
        message
    }
}

#[derive(Default)]
struct RuntimeLoaderCache {
    musl: Option<bool>,
    glibc: Option<bool>,
}

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

fn default_exec_env(exec_root: &str, cwd: &str) -> Result<Vec<String>, String> {
    let path = if exec_root == "/glibc" {
        "PATH=/glibc:/musl"
    } else {
        "PATH=/musl:/glibc"
    };
    let pwd = if cwd.is_empty() { "/" } else { cwd };
    let pwd_entry = {
        let mut entry = exec_loader_owned_string("PWD=", "prepare default PWD env")?;
        entry.try_reserve_exact(pwd.len()).map_err(|_| {
            exec_loader_enomem("not enough kernel memory to prepare default PWD env".to_string())
        })?;
        entry.push_str(pwd);
        entry
    };
    let mut env = exec_loader_owned_strings(&[path, "HOME=/"], "prepare default exec env")?;
    exec_loader_reserve(&mut env, 1, "append default PWD env")?;
    env.push(pwd_entry);
    if runtime_has_musl_loader(exec_root)? {
        if let Some(preload) = runtime_compat_preload_path(exec_root)? {
            let mut entry =
                exec_loader_owned_string("LD_PRELOAD=", "prepare default LD_PRELOAD env")?;
            entry.try_reserve_exact(preload.len()).map_err(|_| {
                exec_loader_enomem(
                    "not enough kernel memory to prepare default LD_PRELOAD env".to_string(),
                )
            })?;
            entry.push_str(preload.as_str());
            exec_loader_reserve(&mut env, 1, "append default LD_PRELOAD env")?;
            env.push(entry);
        }
    }
    Ok(env)
}

fn runtime_compat_preload_path(exec_root: &str) -> Result<Option<String>, String> {
    let root = exec_root.trim_end_matches('/');
    let runtime_path = exec_loader_join(&[root, "/liboscompat.so"], "prepare LD_PRELOAD path")?;
    if matches!(File::open(runtime_path.as_str()), Ok(_)) {
        return Ok(Some(runtime_path));
    }
    if root == "/musl" {
        let staged_path = exec_loader_join(
            &[TESTSUITE_STAGE_ROOT, "/m/liboscompat.so"],
            "prepare staged LD_PRELOAD path",
        )?;
        if matches!(File::open(staged_path.as_str()), Ok(_)) {
            return Ok(Some(staged_path));
        }
    }
    Ok(None)
}

fn runtime_file_contains_ascii(path: &str, needle: &[u8]) -> bool {
    if needle.is_empty() {
        return true;
    }
    if needle.len() > 128 {
        return false;
    }

    let Ok(mut file) = File::open(path) else {
        return false;
    };
    let mut buffer = [0u8; 512];
    let mut window = [0u8; 640];
    let mut tail = [0u8; 128];
    let mut tail_len = 0usize;
    loop {
        let Ok(len) = file.read(&mut buffer) else {
            return false;
        };
        if len == 0 {
            return false;
        }
        window[..tail_len].copy_from_slice(&tail[..tail_len]);
        window[tail_len..tail_len + len].copy_from_slice(&buffer[..len]);
        let window_len = tail_len + len;
        if window[..window_len]
            .windows(needle.len())
            .any(|candidate| candidate == needle)
        {
            return true;
        }
        let keep = needle.len().saturating_sub(1).min(window_len);
        if keep > 0 {
            tail[..keep].copy_from_slice(&window[window_len - keep..window_len]);
        }
        tail_len = keep;
    }
}

fn runtime_has_musl_loader(exec_root: &str) -> Result<bool, String> {
    let root = exec_root.trim_end_matches('/');
    if let Some(root) = cacheable_runtime_loader_root(root) {
        let cache = runtime_loader_cache();
        {
            let cache = cache.lock();
            let cached = match root {
                "/musl" => cache.musl,
                "/glibc" => cache.glibc,
                _ => None,
            };
            if let Some(cached) = cached {
                return Ok(cached);
            }
        }
        let has_loader = runtime_has_musl_loader_uncached(root)?;
        let mut cache = cache.lock();
        match root {
            "/musl" => cache.musl = Some(has_loader),
            "/glibc" => cache.glibc = Some(has_loader),
            _ => {}
        }
        return Ok(has_loader);
    }
    runtime_has_musl_loader_uncached(root)
}

// Only suite image roots are cacheable here.  Do not cache `/tmp` stage
// artifacts or LD_PRELOAD candidates: `liboscompat.so` can be generated during
// evaluator setup and must remain visible to later exec probes.
fn cacheable_runtime_loader_root(root: &str) -> Option<&'static str> {
    match root {
        "/musl" => Some("/musl"),
        "/glibc" => Some("/glibc"),
        _ => None,
    }
}

fn runtime_loader_cache() -> &'static Mutex<RuntimeLoaderCache> {
    static CACHE: LazyInit<Mutex<RuntimeLoaderCache>> = LazyInit::new();
    let _ = CACHE.call_once(|| Mutex::new(RuntimeLoaderCache::default()));
    &CACHE
}

fn runtime_has_musl_loader_uncached(exec_root: &str) -> Result<bool, String> {
    let lib_dir = exec_loader_join(
        &[exec_root.trim_end_matches('/'), "/lib"],
        "prepare runtime lib directory",
    )?;
    let Ok(entries) = std::fs::read_dir(&lib_dir) else {
        return Ok(false);
    };
    for entry in entries.filter_map(Result::ok) {
        let name = entry.file_name();
        let path = exec_loader_join(
            &[lib_dir.as_str(), "/", name.as_str()],
            "prepare runtime loader candidate",
        )?;
        if !matches!(std::fs::metadata(path.as_str()), Ok(metadata) if metadata.is_file()) {
            continue;
        }
        if name.starts_with("ld-musl-")
            || (name == "libc.so" && runtime_file_contains_ascii(&path, b"musl"))
        {
            return Ok(true);
        }
    }
    Ok(false)
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
            .map_err(|_| {
                exec_loader_enomem(format!("not enough kernel memory to read {label} {path}"))
            })?;
    }

    loop {
        if image.len() == MAX_EXEC_IMAGE_SIZE {
            let mut probe = [0u8; 1];
            let count = file
                .read(&mut probe)
                .map_err(|err| format!("failed to read {label} {path}: {err}"))?;
            if count == 0 {
                break;
            }
            return Err(format!(
                "{label} {path} is too large to load (exceeds {} bytes)",
                MAX_EXEC_IMAGE_SIZE
            ));
        }

        let old_len = image.len();
        let chunk_len = EXEC_IMAGE_READ_CHUNK.min(MAX_EXEC_IMAGE_SIZE - old_len);
        image.try_reserve_exact(chunk_len).map_err(|_| {
            exec_loader_enomem(format!("not enough kernel memory to read {label} {path}"))
        })?;
        image.resize(old_len + chunk_len, 0);
        let count = match file.read(&mut image[old_len..old_len + chunk_len]) {
            Ok(count) => count,
            Err(err) => {
                image.truncate(old_len);
                return Err(format!("failed to read {label} {path}: {err}"));
            }
        };
        if count == 0 {
            image.truncate(old_len);
            break;
        }
        image.truncate(old_len + count);
    }

    if image.len() < expected_len {
        image
            .try_reserve_exact(expected_len - image.len())
            .map_err(|_| {
                exec_loader_enomem(format!(
                    "not enough kernel memory to read sparse {label} {path}"
                ))
            })?;
        image.resize(expected_len, 0);
    }
    if let Some(process) = process {
        process.copy_path_sparse_data(path, 0, image.as_mut_slice());
    }
    perf_counters::record_exec_image(image.len());
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
    let elf = ElfFile::new(main_image.as_slice()).map_err(|err| format!("invalid ELF: {err}"))?;
    let main = analyze_elf(&elf, USER_PIE_LOAD_BASE)?;
    let exec_root = effective_exec_root(prepared.exec_root.as_str(), main.interpreter.as_deref())?;

    aspace.clear();

    let mut mappings = map_elf_image(aspace, main_image.as_slice(), &elf, &main)?;
    let mut max_mapped_end = main.max_segment_end;
    let mut runtime_entry = main.entry;
    let mut interp_base = 0usize;

    if let Some(raw_interp) = main.interpreter.as_deref() {
        let interp_path = try_resolve_runtime_support_file(exec_root.as_str(), raw_interp)?;
        let mut interp_image = interp_exec_image_buffer().lock();
        read_exec_image_into(
            process,
            interp_path.as_str(),
            "interpreter",
            &mut interp_image,
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
        let interp_mappings = map_elf_image(aspace, interp_image.as_slice(), &interp_elf, &interp)?;
        exec_loader_reserve(
            &mut mappings,
            interp_mappings.len(),
            "append interpreter ELF mappings",
        )?;
        mappings.extend(interp_mappings);
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
        .map_err(|err| exec_loader_axerr("failed to reserve brk area".to_string(), err))?;

    let stack_top = align_down(USER_STACK_TOP, PAGE_SIZE_4K);
    let stack_base = stack_top - USER_STACK_SIZE;
    aspace
        .map_alloc(
            VirtAddr::from(stack_base),
            USER_STACK_SIZE,
            user_mapping_flags(true, true, false),
            false,
        )
        .map_err(|err| exec_loader_axerr("failed to reserve user stack".to_string(), err))?;

    let argv_refs = exec_loader_string_refs(prepared.argv.as_slice(), "prepare argv references")?;
    let default_env;
    let env_refs = if let Some(env) = env_override {
        exec_loader_string_refs(env, "prepare env references")?
    } else {
        default_env = default_exec_env(exec_root.as_str(), cwd)?;
        exec_loader_string_refs(default_env.as_slice(), "prepare default env references")?
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

fn effective_exec_root(path_root: &str, interpreter: Option<&str>) -> Result<String, String> {
    if path_root != "/" {
        return exec_loader_owned_string(path_root, "copy exec root");
    }
    let Some(interpreter) = interpreter else {
        return exec_loader_owned_string(path_root, "copy exec root");
    };
    let name = interpreter.rsplit('/').next().unwrap_or(interpreter);
    if name.starts_with("ld-musl-") {
        exec_loader_owned_string("/musl", "copy interpreter exec root")
    } else if name.starts_with("ld-linux-") {
        exec_loader_owned_string("/glibc", "copy interpreter exec root")
    } else {
        exec_loader_owned_string(path_root, "copy exec root")
    }
}

fn resolve_program_exec_path(
    process: Option<&UserProcess>,
    cwd: &str,
    program_path: &str,
) -> Result<String, String> {
    let path = try_resolve_host_path(cwd, program_path)?;
    if matches!(File::open(&path), Ok(_)) {
        return Ok(path);
    }
    if !program_path.starts_with('/') {
        return Ok(path);
    }
    let Some(process) = process else {
        return Ok(path);
    };
    for candidate in try_staged_cwd_absolute_path_candidates(cwd, path.as_str())? {
        if candidate != path && matches!(File::open(&candidate), Ok(_)) {
            return Ok(candidate);
        }
    }
    let exec_root = process.exec_root();
    for candidate in try_runtime_absolute_path_candidates(exec_root.as_str(), path.as_str())? {
        if candidate != path && matches!(File::open(&candidate), Ok(_)) {
            return Ok(candidate);
        }
    }
    Ok(path)
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

    let path = resolve_program_exec_path(process, cwd, program_path)?;
    read_exec_image_into(process, path.as_str(), "program", image)?;

    if let Some(next_argv) =
        parse_shebang_argv(process, cwd, path.as_str(), image.as_slice(), argv)?
    {
        let next_refs =
            exec_loader_string_refs(next_argv.as_slice(), "prepare shebang argv references")?;
        let next_program = next_refs.first().copied().unwrap_or(program_path);
        return prepare_program(process, cwd, next_program, &next_refs, depth + 1, image);
    }

    Ok(PreparedProgram {
        argv: exec_loader_owned_strings(argv, "copy prepared argv")?,
        path: exec_loader_owned_string(path.as_str(), "copy prepared exec path")?,
        exec_root: try_derive_exec_root_from_path(path.as_str())?,
    })
}

fn parse_shebang_argv(
    process: Option<&UserProcess>,
    cwd: &str,
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
        .map_err(|_| {
            exec_loader_join(
                &["invalid shebang in ", script_path],
                "report invalid shebang",
            )
            .unwrap_or_else(|err| err)
        })?
        .trim_end_matches('\r')
        .trim();
    if line.is_empty() {
        return Err(exec_loader_join(
            &["empty shebang interpreter in ", script_path],
            "report empty shebang",
        )?);
    }

    let mut parts = line.split_whitespace();
    let raw_interpreter = parts.next().unwrap();
    let mut next_argv = resolve_script_interpreter(process, cwd, script_path, raw_interpreter)?;
    for part in parts {
        exec_loader_push_string(&mut next_argv, part, "copy shebang argument")?;
    }
    exec_loader_push_string(&mut next_argv, script_path, "copy shebang script path")?;
    for arg in argv.iter().skip(1) {
        exec_loader_push_string(&mut next_argv, arg, "copy shebang original argv")?;
    }
    Ok(Some(next_argv))
}

fn resolve_script_interpreter(
    process: Option<&UserProcess>,
    cwd: &str,
    script_path: &str,
    raw_interpreter: &str,
) -> Result<Vec<String>, String> {
    let base = script_dir(script_path)?;
    let resolved = try_resolve_host_path(base.as_str(), raw_interpreter)?;
    if matches!(std::fs::metadata(&resolved), Ok(meta) if meta.is_file()) {
        let mut argv = Vec::new();
        exec_loader_push_string(
            &mut argv,
            resolved.as_str(),
            "prepare script interpreter argv",
        )?;
        return Ok(argv);
    }
    let resolved = resolve_program_exec_path(process, cwd, raw_interpreter)?;
    if matches!(std::fs::metadata(&resolved), Ok(meta) if meta.is_file()) {
        let mut argv = Vec::new();
        exec_loader_push_string(
            &mut argv,
            resolved.as_str(),
            "prepare script interpreter argv",
        )?;
        let raw_name = raw_interpreter
            .rsplit('/')
            .next()
            .unwrap_or(raw_interpreter);
        let resolved_name = resolved.rsplit('/').next().unwrap_or(resolved.as_str());
        if resolved_name == "busybox" && matches!(raw_name, "sh" | "ash" | "bash") {
            exec_loader_push_string(&mut argv, "sh", "prepare busybox shell argv")?;
        }
        return Ok(argv);
    }

    Err(exec_loader_join(
        &["script interpreter not found: ", raw_interpreter],
        "report missing script interpreter",
    )?)
}

fn script_dir(path: &str) -> Result<String, String> {
    match path.rfind('/') {
        Some(0) | None => exec_loader_owned_string("/", "copy script directory"),
        Some(idx) => exec_loader_owned_string(&path[..idx], "copy script directory"),
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
    exec_loader_owned_string(path, "copy PT_INTERP path")
}

fn map_elf_image(
    aspace: &mut AddrSpace,
    image: &[u8],
    elf: &ElfFile<'_>,
    info: &ElfLoadInfo,
) -> Result<Vec<LoadedMapping>, String> {
    let mut mappings = Vec::new();
    mappings
        .try_reserve_exact(elf.header.pt2.ph_count() as usize)
        .map_err(|_| {
            exec_loader_enomem("not enough kernel memory to prepare ELF mappings".to_string())
        })?;
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
        .map_err(|err| {
            exec_loader_axerr(format!("failed to map ELF segment at {seg_start:#x}"), err)
        })?;

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
    let execfn_bytes = stack_bytes_with_nul(execfn, "copy AT_EXECFN string")?;
    let execfn_ptr = push_stack_bytes(aspace, stack_base, &mut sp, &execfn_bytes, 1)?;
    let platform_bytes = stack_bytes_with_nul(AUX_PLATFORM, "copy AUX_PLATFORM string")?;
    let platform_ptr = push_stack_bytes(aspace, stack_base, &mut sp, &platform_bytes, 1)?;

    let mut arg_ptrs = Vec::new();
    exec_loader_reserve(&mut arg_ptrs, argv.len(), "record argv pointers")?;
    for arg in argv.iter().rev() {
        let bytes = stack_bytes_with_nul(arg, "copy argv string")?;
        let ptr = push_stack_bytes(aspace, stack_base, &mut sp, &bytes, 1)?;
        arg_ptrs.push(ptr);
    }
    arg_ptrs.reverse();

    let mut env_ptrs = Vec::new();
    exec_loader_reserve(&mut env_ptrs, env.len(), "record env pointers")?;
    for item in env.iter().rev() {
        let bytes = stack_bytes_with_nul(item, "copy env string")?;
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

    let word_count = 1usize
        .checked_add(arg_ptrs.len())
        .and_then(|count| count.checked_add(1))
        .and_then(|count| count.checked_add(env_ptrs.len()))
        .and_then(|count| count.checked_add(1))
        .and_then(|count| {
            aux.len()
                .checked_mul(2)
                .and_then(|aux_words| count.checked_add(aux_words))
        })
        .ok_or_else(|| "initial user stack word count overflow".to_string())?;
    let mut words = Vec::new();
    exec_loader_reserve(&mut words, word_count, "prepare initial stack words")?;
    words.push(argv.len());
    words.extend(arg_ptrs.iter().copied());
    words.push(0);
    words.extend(env_ptrs.iter().copied());
    words.push(0);
    for item in aux {
        words.push(item.key);
        words.push(item.value);
    }
    let bytes = words_to_bytes(&words)?;
    sp = align_down(sp.saturating_sub(bytes.len()), 16);
    let end = sp + bytes.len();
    if sp < stack_base || end > stack_top {
        return Err("user stack overflow".into());
    }
    aspace
        .populate_range(VirtAddr::from(sp), bytes.len(), PageFaultFlags::WRITE)
        .map_err(|err| exec_loader_axerr("failed to populate user stack pages".to_string(), err))?;
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
        .map_err(|err| exec_loader_axerr("failed to populate user stack pages".to_string(), err))?;
    aspace
        .write(VirtAddr::from(*sp), data)
        .map_err(|err| format!("failed to write user stack data: {err}"))?;
    Ok(*sp)
}

fn words_to_bytes(words: &[usize]) -> Result<Vec<u8>, String> {
    let byte_len = words
        .len()
        .checked_mul(size_of::<usize>())
        .ok_or_else(|| "initial user stack byte count overflow".to_string())?;
    let mut bytes = Vec::new();
    exec_loader_reserve(&mut bytes, byte_len, "prepare initial stack bytes")?;
    for word in words {
        bytes.extend_from_slice(&word.to_ne_bytes());
    }
    Ok(bytes)
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
