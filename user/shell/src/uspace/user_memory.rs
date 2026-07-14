use core::mem::{MaybeUninit, forget, size_of};
use core::slice;
use core::sync::atomic::{AtomicU64, Ordering};

use axerrno::LinuxError;
use axhal::paging::MappingFlags;
use axhal::trap::PageFaultFlags;
use linux_raw_sys::general;
use memory_addr::{MemoryAddr, PAGE_SIZE_4K, PageIter4K, VirtAddr};
use orays_linux::backend::UserMemoryBackend;
use orays_linux::user::{Read, UserRange, Write};
use std::string::String;
use std::vec::Vec;

use super::linux_abi::IOV_MAX;
use super::perf_counters;
use super::{UserProcess, neg_errno};

pub(super) const MAX_USER_IO_CHUNK: usize = 64 * 1024;

static PSEUDO_RANDOM_STATE: AtomicU64 = AtomicU64::new(0x9e37_79b9_7f4a_7c15);

fn next_pseudo_random_u64() -> u64 {
    loop {
        let current = PSEUDO_RANDOM_STATE.load(Ordering::Acquire);
        let time_mix =
            axhal::time::wall_time_nanos() ^ axhal::time::monotonic_time_nanos().rotate_left(17);
        let mut next = current ^ time_mix ^ 0xbf58_476d_1ce4_e5b9;
        next ^= next << 13;
        next ^= next >> 7;
        next ^= next << 17;
        if next == 0 {
            next = 0x94d0_49bb_1331_11eb;
        }
        if PSEUDO_RANDOM_STATE
            .compare_exchange(current, next, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            return next;
        }
    }
}

pub(super) fn fill_pseudo_random_bytes(dst: &mut [u8]) {
    let mut offset = 0;
    while offset < dst.len() {
        let bytes = next_pseudo_random_u64().to_ne_bytes();
        let len = (dst.len() - offset).min(bytes.len());
        dst[offset..offset + len].copy_from_slice(&bytes[..len]);
        offset += len;
    }
}

pub(super) fn validate_user_read(
    process: &UserProcess,
    ptr: usize,
    len: usize,
) -> Result<(), LinuxError> {
    validate_user_access(process, ptr, len, false)
}

pub(super) fn validate_user_write(
    process: &UserProcess,
    ptr: usize,
    len: usize,
) -> Result<(), LinuxError> {
    validate_user_access(process, ptr, len, true)
}

pub(super) fn fault_in_user_read(
    process: &UserProcess,
    ptr: usize,
    len: usize,
) -> Result<(), LinuxError> {
    fault_in_user_range(process, ptr, len, false)
}

pub(super) fn fault_in_user_write(
    process: &UserProcess,
    ptr: usize,
    len: usize,
) -> Result<(), LinuxError> {
    fault_in_user_range(process, ptr, len, true)
}

pub(super) fn sys_getrandom(process: &UserProcess, buf: usize, len: usize, flags: usize) -> isize {
    const GRND_NONBLOCK: usize = 0x0001;
    const GRND_RANDOM: usize = 0x0002;
    const GRND_INSECURE: usize = 0x0004;
    if flags & !(GRND_NONBLOCK | GRND_RANDOM | GRND_INSECURE) != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    if let Err(err) = validate_user_write(process, buf, len) {
        return neg_errno(err);
    }

    let mut filled = 0usize;
    let mut chunk = [0u8; 256];
    while filled < len {
        let chunk_len = (len - filled).min(chunk.len());
        fill_pseudo_random_bytes(&mut chunk[..chunk_len]);
        let dst = match buf.checked_add(filled) {
            Some(dst) => dst,
            None => return neg_errno(LinuxError::EFAULT),
        };
        if let Err(err) = write_user_bytes(process, dst, &chunk[..chunk_len]) {
            return neg_errno(err);
        }
        filled += chunk_len;
    }
    filled as isize
}

fn validate_user_access(
    process: &UserProcess,
    ptr: usize,
    len: usize,
    write: bool,
) -> Result<(), LinuxError> {
    fault_in_user_range(process, ptr, len, write)
}

fn user_range_fits(ptr: usize, len: usize) -> bool {
    len == 0 || ptr.checked_add(len).is_some()
}

fn fault_in_user_range(
    process: &UserProcess,
    ptr: usize,
    len: usize,
    write: bool,
) -> Result<(), LinuxError> {
    if len == 0 {
        return Ok(());
    }
    if ptr == 0
        || !user_range_fits(ptr, len)
        || user_range_crosses_uncommitted_brk(process, ptr, len)
    {
        return Err(LinuxError::EFAULT);
    }
    perf_counters::record_user_copy_fault();
    let end = ptr.checked_add(len).ok_or(LinuxError::EFAULT)?;
    let access_flags = if write {
        MappingFlags::READ | MappingFlags::WRITE
    } else {
        MappingFlags::READ
    };
    let fault_flags = if write {
        PageFaultFlags::WRITE
    } else {
        PageFaultFlags::READ
    };

    // Real Linux copy_to/from_user can fault in legal lazy user pages while it
    // copies.  Our copy helpers run in kernel context, so they must explicitly
    // apply MAP_GROWSDOWN and lazy mmap population before calling
    // AddrSpace::read/write; otherwise a valid stack/TLS/futex pointer can be
    // misreported as EFAULT and produce an axmm BadAddress warning storm under
    // thread-heavy glibc workloads.
    let start_page = VirtAddr::from(ptr.align_down_4k());
    let end_page = if end.is_aligned_4k() {
        end
    } else {
        end.checked_add(PAGE_SIZE_4K - 1)
            .ok_or(LinuxError::EFAULT)?
            .align_down_4k()
    };
    let end_page = VirtAddr::from(end_page);
    if let Some(pages) = PageIter4K::new(start_page, end_page) {
        for page in pages {
            let _ = process.handle_mmap_grow_down_fault(page.as_usize(), fault_flags);
        }
    }

    let mut aspace = process.aspace.lock();
    if !aspace.can_access_range(VirtAddr::from(ptr), len, access_flags) {
        return Err(LinuxError::EFAULT);
    }
    aspace
        .populate_range(VirtAddr::from(ptr), len, fault_flags)
        .map_err(|_| LinuxError::EFAULT)
}

fn user_range_crosses_uncommitted_brk(process: &UserProcess, ptr: usize, len: usize) -> bool {
    if len == 0 {
        return false;
    }
    let Some(end) = ptr.checked_add(len) else {
        return true;
    };
    let brk = process.brk.lock();
    ptr < brk.limit && end > brk.end
}

pub(super) fn read_user_bytes(
    process: &UserProcess,
    ptr: usize,
    len: usize,
) -> Result<Vec<u8>, LinuxError> {
    if len == 0 {
        return Ok(Vec::new());
    }
    fault_in_user_read(process, ptr, len)?;

    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(len)
        .map_err(|_| LinuxError::ENOMEM)?;
    bytes.resize(len, 0);
    process
        .aspace
        .lock()
        .read(VirtAddr::from(ptr), &mut bytes)
        .map_err(|_| LinuxError::EFAULT)?;
    perf_counters::record_user_copy_read(len);
    Ok(bytes)
}

pub(super) fn read_user_bytes_into(
    process: &UserProcess,
    ptr: usize,
    dst: &mut [u8],
) -> Result<(), LinuxError> {
    if dst.is_empty() {
        return Ok(());
    }
    fault_in_user_read(process, ptr, dst.len())?;
    process
        .aspace
        .lock()
        .read(VirtAddr::from(ptr), dst)
        .map_err(|_| LinuxError::EFAULT)?;
    perf_counters::record_user_copy_read(dst.len());
    Ok(())
}

pub(super) fn read_iovec_entries(
    process: &UserProcess,
    iov: usize,
    iovcnt: usize,
) -> Result<Vec<general::iovec>, LinuxError> {
    if iovcnt > IOV_MAX {
        return Err(LinuxError::EINVAL);
    }
    let iov_bytes_len = iovcnt
        .checked_mul(size_of::<general::iovec>())
        .ok_or(LinuxError::EINVAL)?;
    let mut raw_entries: Vec<MaybeUninit<general::iovec>> = Vec::new();
    raw_entries
        .try_reserve_exact(iovcnt)
        .map_err(|_| LinuxError::ENOMEM)?;
    raw_entries.resize_with(iovcnt, MaybeUninit::uninit);
    // SAFETY: `raw_entries` reserves exactly `iovcnt` slots of `general::iovec`, and the
    // byte view spans those slots only. A failed copy returns before the entries are read.
    let dst =
        unsafe { slice::from_raw_parts_mut(raw_entries.as_mut_ptr() as *mut u8, iov_bytes_len) };
    read_user_bytes_into(process, iov, dst)?;
    let len = raw_entries.len();
    let cap = raw_entries.capacity();
    let ptr = raw_entries.as_mut_ptr() as *mut general::iovec;
    forget(raw_entries);
    // SAFETY: after `read_user_bytes_into` succeeds, every byte of each `iovec` slot has
    // been initialized from userspace, so ownership can move from `MaybeUninit<iovec>` to
    // `Vec<iovec>` without per-entry copies. `general::iovec` is a plain C ABI record.
    let entries = unsafe { Vec::from_raw_parts(ptr, len, cap) };
    perf_counters::record_iovec_table(entries.len());
    let mut total_len = 0usize;
    for entry in &entries {
        total_len = total_len
            .checked_add(entry.iov_len as usize)
            .ok_or(LinuxError::EINVAL)?;
        if total_len > isize::MAX as usize {
            return Err(LinuxError::EINVAL);
        }
    }
    Ok(entries)
}

pub(super) fn write_user_bytes(
    process: &UserProcess,
    ptr: usize,
    bytes: &[u8],
) -> Result<(), LinuxError> {
    if bytes.is_empty() {
        return Ok(());
    }
    fault_in_user_write(process, ptr, bytes.len())?;

    process
        .aspace
        .lock()
        .write(VirtAddr::from(ptr), bytes)
        .map_err(|_| LinuxError::EFAULT)?;
    perf_counters::record_user_copy_write(bytes.len());
    Ok(())
}

struct ProcessUserMemory<'a> {
    process: &'a UserProcess,
}

impl UserMemoryBackend for ProcessUserMemory<'_> {
    type Error = LinuxError;

    fn validate_read(&self, range: UserRange<Read>) -> Result<(), Self::Error> {
        validate_user_read(self.process, range.start().get(), range.len())
    }

    fn validate_write(&self, range: UserRange<Write>) -> Result<(), Self::Error> {
        validate_user_write(self.process, range.start().get(), range.len())
    }

    fn read_bytes(&self, src: UserRange<Read>, dst: &mut [u8]) -> Result<(), Self::Error> {
        if src.len() != dst.len() {
            return Err(LinuxError::EINVAL);
        }
        read_user_bytes_into(self.process, src.start().get(), dst)
    }

    fn write_bytes(&self, dst: UserRange<Write>, src: &[u8]) -> Result<(), Self::Error> {
        if dst.len() != src.len() {
            return Err(LinuxError::EINVAL);
        }
        write_user_bytes(self.process, dst.start().get(), src)
    }
}

pub(super) fn user_io_buffer(len: usize) -> Result<Vec<u8>, LinuxError> {
    let len = len.min(MAX_USER_IO_CHUNK);
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(len)
        .map_err(|_| LinuxError::ENOMEM)?;
    bytes.resize(len, 0);
    Ok(bytes)
}

pub(super) fn with_readable_user_buffer(
    process: &UserProcess,
    ptr: usize,
    len: usize,
    f: impl FnOnce(&[u8]) -> Result<usize, LinuxError>,
) -> isize {
    let len = len.min(MAX_USER_IO_CHUNK);
    let bytes = match read_user_bytes(process, ptr, len) {
        Ok(bytes) => bytes,
        Err(err) => return neg_errno(err),
    };
    match f(&bytes) {
        Ok(v) => v as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn with_writable_user_buffer(
    process: &UserProcess,
    ptr: usize,
    len: usize,
    f: impl FnOnce(&mut [u8]) -> Result<usize, LinuxError>,
) -> isize {
    let len = len.min(MAX_USER_IO_CHUNK);
    if let Err(err) = validate_user_write(process, ptr, len) {
        return neg_errno(err);
    }
    let mut bytes = match user_io_buffer(len) {
        Ok(bytes) => bytes,
        Err(err) => return neg_errno(err),
    };
    match f(&mut bytes) {
        Ok(v) => {
            if v > len {
                return neg_errno(LinuxError::EINVAL);
            }
            match write_user_bytes(process, ptr, &bytes[..v]) {
                Ok(()) => v as isize,
                Err(err) => neg_errno(err),
            }
        }
        Err(err) => neg_errno(err),
    }
}

pub(super) fn clear_user_bytes(
    process: &UserProcess,
    ptr: usize,
    len: usize,
) -> Result<(), LinuxError> {
    const ZERO_CHUNK: [u8; 64] = [0; 64];

    if len == 0 {
        return Ok(());
    }
    fault_in_user_write(process, ptr, len)?;

    let mut aspace = process.aspace.lock();
    let mut offset = 0;
    while offset < len {
        let chunk_len = core::cmp::min(len - offset, ZERO_CHUNK.len());
        let dst = ptr.checked_add(offset).ok_or(LinuxError::EFAULT)?;
        aspace
            .write(VirtAddr::from(dst), &ZERO_CHUNK[..chunk_len])
            .map_err(|_| LinuxError::EFAULT)?;
        offset += chunk_len;
    }
    Ok(())
}

pub(super) fn write_user_value<T: Copy>(process: &UserProcess, ptr: usize, value: &T) -> isize {
    if fault_in_user_write(process, ptr, size_of::<T>()).is_err() {
        return neg_errno(LinuxError::EFAULT);
    }

    let src =
        unsafe { core::slice::from_raw_parts(value as *const T as *const u8, size_of::<T>()) };
    process
        .aspace
        .lock()
        .write(VirtAddr::from(ptr), src)
        .map_or_else(
            |_| neg_errno(LinuxError::EFAULT),
            |_| {
                perf_counters::record_user_copy_write(size_of::<T>());
                0
            },
        )
}

pub(super) fn read_user_value<T: Copy>(process: &UserProcess, ptr: usize) -> Result<T, LinuxError> {
    fault_in_user_read(process, ptr, size_of::<T>())?;

    let mut value = MaybeUninit::<T>::uninit();
    let dst =
        unsafe { core::slice::from_raw_parts_mut(value.as_mut_ptr() as *mut u8, size_of::<T>()) };
    process
        .aspace
        .lock()
        .read(VirtAddr::from(ptr), dst)
        .map_err(|_| LinuxError::EFAULT)?;
    perf_counters::record_user_copy_read(size_of::<T>());
    Ok(unsafe { value.assume_init() })
}

pub(super) fn read_user_word(process: &UserProcess, ptr: usize) -> Result<usize, LinuxError> {
    read_user_value(process, ptr)
}

pub(super) fn read_execve_argv(
    process: &UserProcess,
    argv_ptr: usize,
    default_argv0: &str,
) -> Result<Vec<String>, LinuxError> {
    const MAX_ARGC: usize = 256;

    if argv_ptr == 0 {
        return Ok(vec![default_argv0.into()]);
    }

    let mut argv = Vec::new();
    for idx in 0..MAX_ARGC {
        let item_ptr = read_user_word(process, argv_ptr + idx * size_of::<usize>())?;
        if item_ptr == 0 {
            break;
        }
        argv.push(read_cstr(process, item_ptr)?);
    }
    if argv.is_empty() {
        argv.push(default_argv0.into());
    }
    Ok(argv)
}

pub(super) fn read_execve_envp(
    process: &UserProcess,
    envp_ptr: usize,
) -> Result<Vec<String>, LinuxError> {
    const MAX_ENVC: usize = 512;

    if envp_ptr == 0 {
        return Ok(Vec::new());
    }

    let mut env = Vec::new();
    for idx in 0..MAX_ENVC {
        let item_ptr = read_user_word(process, envp_ptr + idx * size_of::<usize>())?;
        if item_ptr == 0 {
            break;
        }
        env.push(read_cstr(process, item_ptr)?);
    }
    Ok(env)
}

pub(super) fn read_cstr(process: &UserProcess, ptr: usize) -> Result<String, LinuxError> {
    const MAX_USER_CSTR_LEN: usize = 128 * 1024;
    const USER_CSTR_COPY_CHUNK: usize = 256;

    if ptr == 0 {
        return read_cstr_efault(process, ptr, ptr, "null pointer");
    }
    if !user_range_fits(ptr, 1) {
        return read_cstr_efault(process, ptr, ptr, "pointer overflow");
    }

    let mut bytes = Vec::new();
    let mut offset = 0usize;
    let mut chunk = [0u8; USER_CSTR_COPY_CHUNK];
    while offset < MAX_USER_CSTR_LEN {
        let addr = match ptr.checked_add(offset) {
            Some(addr) => addr,
            None => {
                let aspace = process.aspace.lock();
                log_read_cstr_efault(process, ptr, ptr, "string pointer overflow", &aspace);
                return Err(LinuxError::EFAULT);
            }
        };
        let remaining = MAX_USER_CSTR_LEN - offset;
        let chunk_len = match cstr_chunk_len(process, addr, remaining, chunk.len()) {
            Ok(len) if len > 0 => len,
            Ok(_) => {
                let aspace = process.aspace.lock();
                log_read_cstr_efault(process, ptr, addr, "range is not readable", &aspace);
                return Err(LinuxError::EFAULT);
            }
            Err(err) => {
                let aspace = process.aspace.lock();
                log_read_cstr_efault(process, ptr, addr, "range is not readable", &aspace);
                return Err(err);
            }
        };
        if let Err(err) = fault_in_user_read(process, addr, chunk_len) {
            let aspace = process.aspace.lock();
            log_read_cstr_efault(process, ptr, addr, "range is not readable", &aspace);
            return Err(err);
        }
        let mut aspace = process.aspace.lock();
        if aspace
            .read(VirtAddr::from(addr), &mut chunk[..chunk_len])
            .is_err()
        {
            log_read_cstr_efault(process, ptr, addr, "address-space read failed", &aspace);
            return Err(LinuxError::EFAULT);
        }
        if let Some(nul) = chunk[..chunk_len].iter().position(|&byte| byte == 0) {
            bytes.extend_from_slice(&chunk[..nul]);
            return String::from_utf8(bytes).map_err(|_| LinuxError::EINVAL);
        }
        bytes.extend_from_slice(&chunk[..chunk_len]);
        offset += chunk_len;
    }

    Err(LinuxError::EINVAL)
}

fn cstr_chunk_len(
    process: &UserProcess,
    addr: usize,
    remaining: usize,
    scratch_len: usize,
) -> Result<usize, LinuxError> {
    let page_remaining = PAGE_SIZE_4K - (addr & (PAGE_SIZE_4K - 1));
    let mut len = remaining.min(page_remaining).min(scratch_len);
    let brk = process.brk.lock();
    if addr < brk.limit {
        if addr >= brk.end {
            return Err(LinuxError::EFAULT);
        }
        len = len.min(brk.end - addr);
    }
    Ok(len)
}

fn read_cstr_efault(
    process: &UserProcess,
    ptr: usize,
    fault_addr: usize,
    reason: &'static str,
) -> Result<String, LinuxError> {
    let aspace = process.aspace.lock();
    log_read_cstr_efault(process, ptr, fault_addr, reason, &aspace);
    Err(LinuxError::EFAULT)
}

fn log_read_cstr_efault(
    process: &UserProcess,
    ptr: usize,
    fault_addr: usize,
    reason: &'static str,
    aspace: &axmm::AddrSpace,
) {
    let mapped = aspace.query_address(VirtAddr::from(fault_addr)).pte_mapped;
    user_trace!(
        "user-read-cstr: EFAULT pid={} ptr={:#x} fault={:#x} mapped={} reason={}",
        process.pid(),
        ptr,
        fault_addr,
        mapped,
        reason
    );
}
