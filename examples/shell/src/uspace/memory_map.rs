use core::sync::atomic::Ordering;

use axerrno::LinuxError;
use axhal::context::TrapFrame;
use axhal::paging::MappingFlags;
use axhal::trap::{PAGE_FAULT, PageFaultFlags, register_trap_handler};
use linux_raw_sys::general;
use memory_addr::{PAGE_SIZE_4K, PageIter4K, VirtAddr, VirtAddrRange};
use std::collections::BTreeMap;
use std::vec::Vec;

use super::UserProcess;
use super::fd_table::{read_mmap_file_backing, write_mmap_file_backing};
use super::linux_abi::{
    SIGSEGV_NUM, USER_ASPACE_BASE, USER_MMAP_BASE, USER_STACK_SIZE, USER_STACK_TOP, neg_errno,
};
use super::process_lifecycle::{terminate_current_thread, terminate_current_thread_for_exit_group};
use super::signal_abi::queue_current_synchronous_signal;
use super::task_context::{current_task_ext, current_tid, user_pc};
use super::user_memory::{
    MAX_USER_IO_CHUNK, read_user_bytes, validate_user_write, write_user_bytes,
};

pub(super) fn sys_brk(process: &UserProcess, addr: usize) -> isize {
    let mut brk = process.brk.lock();
    if addr == 0 {
        return brk.end as isize;
    }
    if addr < brk.start || addr > brk.limit {
        return brk.end as isize;
    }
    brk.end = addr;
    let heap_kb = rss_kb_from_bytes(brk.end.saturating_sub(brk.start));
    let new_end = brk.end;
    drop(brk);
    process.record_self_maxrss_kb(heap_kb);
    new_end as isize
}

fn rss_kb_from_bytes(bytes: usize) -> usize {
    let pages = bytes.saturating_add(PAGE_SIZE_4K - 1) / PAGE_SIZE_4K;
    pages.saturating_mul(PAGE_SIZE_4K / 1024)
}

pub(super) fn mmap_prot_to_flags(prot: u32) -> MappingFlags {
    let mut flags = MappingFlags::USER;
    if prot & general::PROT_READ != 0 {
        flags |= MappingFlags::READ;
    }
    if prot & general::PROT_WRITE != 0 {
        flags |= MappingFlags::READ | MappingFlags::WRITE;
    }
    if prot & general::PROT_EXEC != 0 {
        flags |= MappingFlags::READ | MappingFlags::EXECUTE;
    }
    flags
}

pub(super) fn user_mapping_flags(read: bool, write: bool, exec: bool) -> MappingFlags {
    let mut flags = MappingFlags::USER;
    if read {
        flags |= MappingFlags::READ;
    }
    if write {
        flags |= MappingFlags::WRITE;
    }
    if exec {
        flags |= MappingFlags::EXECUTE;
    }
    flags
}

pub(super) fn align_down(value: usize, align: usize) -> usize {
    value & !(align - 1)
}

pub(super) fn align_up(value: usize, align: usize) -> usize {
    align_up_checked(value, align).unwrap_or(usize::MAX)
}

pub(super) fn align_up_checked(value: usize, align: usize) -> Option<usize> {
    if value == 0 {
        Some(0)
    } else {
        value
            .checked_add(align - 1)
            .map(|aligned| align_down(aligned, align))
    }
}

#[register_trap_handler(PAGE_FAULT)]
fn user_page_fault(vaddr: VirtAddr, flags: PageFaultFlags, _from_user: bool) -> bool {
    let Some(ext) = current_task_ext() else {
        return false;
    };
    let process = ext.process.as_ref();
    if let Some(code) = process.pending_exit_group() {
        user_trace!(
            "user-exit-group-pf: tid={} code={code} fault_vaddr={vaddr:#x} flags={flags:?}",
            current_tid(),
        );
        terminate_current_thread_for_exit_group(process, code);
    }
    let should_trace = _from_user
        && flags.contains(PageFaultFlags::WRITE)
        && vaddr.as_usize() >= USER_MMAP_BASE
        && vaddr.as_usize() < USER_STACK_TOP;
    if _from_user {
        let _ = process.handle_mmap_grow_down_fault(vaddr.as_usize(), flags);
    }
    let handled = {
        let mut aspace = process.aspace.lock();
        if should_trace {
            let query = aspace
                .page_table()
                .query(VirtAddr::from(align_down(vaddr.as_usize(), PAGE_SIZE_4K)));
            user_trace!(
                "user-pf: vaddr={:#x} flags={flags:?} satp={:#x} aspace_root={:#x} query_before={query:?}",
                vaddr,
                axhal::asm::read_user_page_table(),
                aspace.page_table_root(),
            );
        }
        let handled = aspace.handle_page_fault(vaddr, flags);
        if should_trace {
            let query = aspace
                .page_table()
                .query(VirtAddr::from(align_down(vaddr.as_usize(), PAGE_SIZE_4K)));
            user_trace!("user-pf: handled={handled} query_after={query:?}");
        }
        handled
    };
    if !handled && _from_user {
        let signal = if process.fault_in_mmap_sigbus_range(vaddr.as_usize()) {
            general::SIGBUS as i32
        } else {
            SIGSEGV_NUM
        };
        if queue_current_synchronous_signal(signal) {
            return true;
        }
        process.request_signal_exit_group(signal);
        terminate_current_thread(process, 128 + signal);
    }
    #[cfg(target_arch = "loongarch64")]
    if handled {
        axhal::asm::flush_tlb(None);
    }
    handled
}

pub(super) fn sys_mmap(
    process: &UserProcess,
    addr: usize,
    len: usize,
    prot: usize,
    flags: usize,
    fd: usize,
    offset: usize,
) -> isize {
    let flags_u32 = flags as u32;
    match flags_u32 & general::MAP_TYPE {
        general::MAP_SHARED | general::MAP_PRIVATE | general::MAP_SHARED_VALIDATE => {}
        _ => return neg_errno(LinuxError::EINVAL),
    }
    const MMAP_COMMON_SUPPORTED_FLAGS: u32 = general::MAP_TYPE
        | general::MAP_FIXED
        | general::MAP_ANONYMOUS
        | general::MAP_POPULATE
        | general::MAP_NONBLOCK
        | general::MAP_STACK
        | general::MAP_FIXED_NOREPLACE
        | general::MAP_GROWSDOWN
        | general::MAP_DENYWRITE
        | general::MAP_EXECUTABLE
        | general::MAP_LOCKED
        | general::MAP_NORESERVE;
    if flags_u32 & general::MAP_TYPE == general::MAP_SHARED_VALIDATE
        && flags_u32 & !MMAP_COMMON_SUPPORTED_FLAGS != 0
    {
        return neg_errno(LinuxError::EOPNOTSUPP);
    }
    let anonymous = flags_u32 & general::MAP_ANONYMOUS != 0;
    let shared = flags_u32 & general::MAP_SHARED != 0;
    let map_fixed = flags_u32 & general::MAP_FIXED != 0;
    let locked =
        flags_u32 & general::MAP_LOCKED != 0 || process.mlock_future.load(Ordering::Acquire);
    if len == 0 {
        if !anonymous {
            if let Err(err) = process.fds.lock().mmap_validate_file_fd_exists(fd as i32) {
                return neg_errno(err);
            }
        }
        return neg_errno(LinuxError::EINVAL);
    }
    let dev_zero = !anonymous && process.fds.lock().is_dev_zero(fd as i32);
    if !anonymous {
        if let Err(err) = process.fds.lock().mmap_validate_file_fd(fd as i32) {
            return neg_errno(err);
        }
    }
    let map_flags = mmap_prot_to_flags(prot as u32);
    let shared_write_allowed = if anonymous || !shared {
        true
    } else {
        match process.fds.lock().mmap_fd_allows_shared_write(fd as i32) {
            Ok(allowed) => allowed,
            Err(err) => return neg_errno(err),
        }
    };
    if map_flags.contains(MappingFlags::WRITE) && !shared_write_allowed {
        return neg_errno(LinuxError::EACCES);
    }
    let Some(size) = align_up_checked(len, PAGE_SIZE_4K) else {
        return neg_errno(LinuxError::ENOMEM);
    };
    if size == 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    // Linux's vm.max_map_count limits the number of visible VMA entries.
    // Our synthetic /proc/<pid>/maps always exposes text/heap/stack plus
    // recorded mmap regions.  Allow the mapping that crosses the limit, then
    // fail the next one, matching the Linux boundary where the
    // stopped child has max_map_count + 1 map lines.
    let visible_map_count = process.mmap_regions().len().saturating_add(3);
    if visible_map_count > super::synthetic_fs::proc_sys_vm_max_map_count() {
        return neg_errno(LinuxError::ENOMEM);
    }
    let request_addr = if addr == 0 {
        None
    } else {
        Some(align_down(addr, PAGE_SIZE_4K))
    };
    let mmap_limit_end = USER_STACK_TOP - USER_STACK_SIZE;
    let exact_fixed = flags_u32 & (general::MAP_FIXED | general::MAP_FIXED_NOREPLACE) != 0;
    let target = if let Some(start) = request_addr {
        let Some(end) = start.checked_add(size) else {
            return neg_errno(LinuxError::ENOMEM);
        };
        let min_start = if exact_fixed {
            USER_ASPACE_BASE
        } else {
            USER_MMAP_BASE
        };
        if start < min_start || end > mmap_limit_end {
            return neg_errno(LinuxError::ENOMEM);
        }
        start
    } else {
        let mut brk = process.brk.lock();
        let Some(hint) = align_up_checked(brk.next_mmap, PAGE_SIZE_4K) else {
            return neg_errno(LinuxError::ENOMEM);
        };
        let Some(limit_size) = mmap_limit_end.checked_sub(USER_MMAP_BASE) else {
            return neg_errno(LinuxError::ENOMEM);
        };
        let limit = VirtAddrRange::from_start_size(VirtAddr::from(USER_MMAP_BASE), limit_size);
        let start = {
            let aspace = process.aspace.lock();
            aspace
                .find_free_area(VirtAddr::from(hint), size, limit)
                .or_else(|| aspace.find_free_area(VirtAddr::from(USER_MMAP_BASE), size, limit))
        };
        let Some(start) = start.map(|addr| addr.as_usize()) else {
            return neg_errno(LinuxError::ENOMEM);
        };
        brk.next_mmap = start
            .checked_add(size)
            .and_then(|end| end.checked_add(PAGE_SIZE_4K))
            .filter(|next| *next < mmap_limit_end)
            .unwrap_or(USER_MMAP_BASE);
        start
    };
    let Some(target_end) = target.checked_add(size) else {
        return neg_errno(LinuxError::ENOMEM);
    };
    if anonymous && size <= 0x40000 {
        user_trace!("user-mmap: target={target:#x} len={size:#x} prot={prot:#x} flags={flags:#x}");
    }
    let file_backed = !anonymous && !dev_zero;
    let mut file_backing = if file_backed && shared && shared_write_allowed {
        let file = match process.fds.lock().mmap_file_backing(fd as i32) {
            Ok(file) => file,
            Err(err) => return neg_errno(err),
        };
        Some(super::UserMmapFileBacking {
            file,
            offset: offset as u64,
            valid_len: 0,
        })
    } else {
        None
    };
    if let Some(mut cache) = file_backing
        .as_ref()
        .and_then(|backing| process.take_exec_shared_mmap_cache(&backing.file, offset as u64, size))
    {
        let mut pages = BTreeMap::new();
        for (delta, frame, _) in cache.pages.iter().copied() {
            let Some(page_addr) = target.checked_add(delta) else {
                cache.release_retained_frames();
                return neg_errno(LinuxError::ENOMEM);
            };
            pages.insert(page_addr, (frame, map_flags));
        }
        if pages.len() != size / PAGE_SIZE_4K {
            cache.release_retained_frames();
            return neg_errno(LinuxError::ENOMEM);
        }
        {
            let mut aspace = process.aspace.lock();
            if map_fixed {
                process.forget_mmap_region(target, target_end);
                let _ = aspace.unmap(VirtAddr::from(target), size);
            }
            if let Err(err) =
                aspace.map_retained_shared_frames(VirtAddr::from(target), size, map_flags, pages)
            {
                cache.release_retained_frames();
                return neg_errno(LinuxError::from(err));
            }
        }
        cache.disarm_retained_frames();
        if let Some(backing) = file_backing.as_mut() {
            backing.valid_len = cache.valid_len.min(len);
        }
        if shared && map_flags.contains(MappingFlags::WRITE) {
            process.record_shared_mmap(target, size, map_flags);
        }
        process.record_mmap_region(
            target,
            size,
            prot as u32,
            shared,
            anonymous || dev_zero,
            locked,
            flags_u32 & general::MAP_GROWSDOWN != 0,
            shared_write_allowed,
            file_backing,
        );
        process.record_self_maxrss_kb(rss_kb_from_bytes(size));
        return target as isize;
    }
    let populate = file_backed || shared || locked;
    {
        let mut aspace = process.aspace.lock();
        if map_fixed {
            process.forget_mmap_region(target, target_end);
            let _ = aspace.unmap(VirtAddr::from(target), size);
        }
        if let Err(err) = aspace.map_alloc(VirtAddr::from(target), size, map_flags, populate) {
            return neg_errno(LinuxError::from(err));
        }
    }

    let mut sigbus_range = None;
    if file_backed {
        const FILE_MMAP_COPY_CHUNK: usize = PAGE_SIZE_4K;
        let mut copied = 0usize;
        let mut buf = [0u8; FILE_MMAP_COPY_CHUNK];
        while copied < len {
            let chunk_len = core::cmp::min(len - copied, buf.len());
            let Some(file_offset) = (offset as u64).checked_add(copied as u64) else {
                let _ = process.aspace.lock().unmap(VirtAddr::from(target), size);
                return neg_errno(LinuxError::EINVAL);
            };
            let read = {
                let mut table = process.fds.lock();
                match table.mmap_read_file_at_into_fd(
                    process,
                    fd as i32,
                    file_offset,
                    &mut buf[..chunk_len],
                ) {
                    Ok(read) => read,
                    Err(err) => {
                        let _ = process.aspace.lock().unmap(VirtAddr::from(target), size);
                        return neg_errno(err);
                    }
                }
            };
            if read == 0 {
                break;
            }
            let Some(dst) = target.checked_add(copied) else {
                let _ = process.aspace.lock().unmap(VirtAddr::from(target), size);
                return neg_errno(LinuxError::EINVAL);
            };
            if let Err(err) = process
                .aspace
                .lock()
                .write(VirtAddr::from(dst), &buf[..read])
            {
                let _ = process.aspace.lock().unmap(VirtAddr::from(target), size);
                return neg_errno(LinuxError::from(err));
            }
            copied += read;
            if read < chunk_len {
                break;
            }
        }
        if copied < len {
            let Some(valid_len) = align_up_checked(copied, PAGE_SIZE_4K) else {
                let _ = process.aspace.lock().unmap(VirtAddr::from(target), size);
                return neg_errno(LinuxError::ENOMEM);
            };
            let Some(invalid_start) = target.checked_add(valid_len) else {
                let _ = process.aspace.lock().unmap(VirtAddr::from(target), size);
                return neg_errno(LinuxError::ENOMEM);
            };
            if invalid_start < target_end {
                if let Err(err) = process.aspace.lock().protect(
                    VirtAddr::from(invalid_start),
                    target_end - invalid_start,
                    MappingFlags::USER,
                ) {
                    let _ = process.aspace.lock().unmap(VirtAddr::from(target), size);
                    return neg_errno(LinuxError::from(err));
                }
                sigbus_range = Some((invalid_start, target_end));
            }
        }
        if let Some(backing) = file_backing.as_mut() {
            backing.valid_len = copied.min(len);
        }
    }
    if shared && map_flags.contains(MappingFlags::WRITE) {
        process.record_shared_mmap(target, size, map_flags);
    }
    process.record_mmap_region(
        target,
        size,
        prot as u32,
        shared,
        anonymous || dev_zero,
        locked,
        flags_u32 & general::MAP_GROWSDOWN != 0,
        shared_write_allowed,
        file_backing,
    );
    process.record_self_maxrss_kb(rss_kb_from_bytes(size));
    if let Some((start, end)) = sigbus_range {
        process.record_mmap_sigbus_range(start, end);
    }
    target as isize
}

pub(super) fn sys_mremap(
    process: &UserProcess,
    old_addr: usize,
    old_size: usize,
    new_size: usize,
    flags: usize,
    new_addr: usize,
) -> isize {
    const SUPPORTED_FLAGS: u32 = general::MREMAP_MAYMOVE | general::MREMAP_FIXED;

    let flags = flags as u32;
    if old_addr & (PAGE_SIZE_4K - 1) != 0 || flags & !SUPPORTED_FLAGS != 0 || new_size == 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    if flags & general::MREMAP_FIXED != 0 {
        if flags & general::MREMAP_MAYMOVE == 0 || new_addr & (PAGE_SIZE_4K - 1) != 0 {
            return neg_errno(LinuxError::EINVAL);
        }
    }

    let Some(old_size) = align_up_checked(old_size, PAGE_SIZE_4K) else {
        return neg_errno(LinuxError::ENOMEM);
    };
    let Some(new_size) = align_up_checked(new_size, PAGE_SIZE_4K) else {
        return neg_errno(LinuxError::ENOMEM);
    };
    if old_size == 0 || new_size == 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let Some(old_end) = old_addr.checked_add(old_size) else {
        return neg_errno(LinuxError::ENOMEM);
    };
    if !mremap_mapped_range(process, old_addr, old_end) {
        return neg_errno(LinuxError::EFAULT);
    }
    let Some(source) = mremap_source_region(process, old_addr, old_end) else {
        return neg_errno(LinuxError::ENOMEM);
    };

    if flags & general::MREMAP_FIXED != 0 {
        let Some(target_end) = new_addr.checked_add(new_size) else {
            return neg_errno(LinuxError::ENOMEM);
        };
        if new_addr < USER_MMAP_BASE
            || target_end > mmap_limit_end()
            || ranges_overlap(old_addr, old_end, new_addr, target_end)
        {
            return neg_errno(LinuxError::EINVAL);
        }
        return mremap_move(process, source, old_addr, old_size, new_addr, new_size)
            .map_or_else(neg_errno, |addr| addr as isize);
    }

    if new_size <= old_size {
        return mremap_shrink_in_place(process, source, old_addr, old_size, new_size);
    }

    if let Some(result) = mremap_try_expand_in_place(process, &source, old_addr, old_size, new_size)
    {
        return result.map_or_else(neg_errno, |addr| addr as isize);
    }

    if flags & general::MREMAP_MAYMOVE == 0 {
        return neg_errno(LinuxError::ENOMEM);
    }

    let target = match mremap_find_free_area(process, new_size) {
        Some(target) => target,
        None => return neg_errno(LinuxError::ENOMEM),
    };

    mremap_move(process, source, old_addr, old_size, target, new_size)
        .map_or_else(neg_errno, |addr| addr as isize)
}

fn mmap_limit_end() -> usize {
    USER_STACK_TOP - USER_STACK_SIZE
}

fn ranges_overlap(
    left_start: usize,
    left_end: usize,
    right_start: usize,
    right_end: usize,
) -> bool {
    left_start < right_end && right_start < left_end
}

fn mremap_source_region(
    process: &UserProcess,
    start: usize,
    end: usize,
) -> Option<super::UserMmapRegion> {
    process
        .mmap_regions()
        .into_iter()
        .find(|region| region.start <= start && region.end() >= end)
}

fn mremap_mapped_range(process: &UserProcess, start: usize, end: usize) -> bool {
    let size = end - start;
    let aspace = process.aspace.lock();
    aspace.contains_range(VirtAddr::from(start), size)
        && aspace.can_access_range(VirtAddr::from(start), size, MappingFlags::empty())
}

fn mremap_shrink_in_place(
    process: &UserProcess,
    source: super::UserMmapRegion,
    old_addr: usize,
    old_size: usize,
    new_size: usize,
) -> isize {
    let new_end = old_addr + new_size;
    let preserved_sigbus = process.mmap_sigbus_segments(old_addr, new_end);
    if new_size < old_size {
        let tail_start = old_addr + new_size;
        let tail_size = old_size - new_size;
        process.forget_mmap_region(tail_start, old_addr + old_size);
        process.forget_mmap_range(tail_start, old_addr + old_size);
        let unmap_result = process
            .aspace
            .lock()
            .unmap(VirtAddr::from(tail_start), tail_size);
        if let Err(err) = unmap_result {
            return neg_errno(LinuxError::from(err));
        }
    }
    let mut region = source.subregion(old_addr, old_addr + new_size, source.prot);
    region.start = old_addr;
    region.size = new_size;
    let region_size = region.size;
    process.record_mmap_region_entry(region);
    process.record_mmap_sigbus_ranges(preserved_sigbus);
    process.record_self_maxrss_kb(rss_kb_from_bytes(region_size));
    old_addr as isize
}

fn mremap_try_expand_in_place(
    process: &UserProcess,
    source: &super::UserMmapRegion,
    old_addr: usize,
    old_size: usize,
    new_size: usize,
) -> Option<Result<usize, LinuxError>> {
    let old_end = old_addr.checked_add(old_size)?;
    let new_end = old_addr.checked_add(new_size)?;
    if new_end > mmap_limit_end() {
        return Some(Err(LinuxError::ENOMEM));
    }
    let extension = new_size.checked_sub(old_size)?;
    if extension == 0 {
        return Some(Ok(old_addr));
    }
    let preserved_sigbus = process.mmap_sigbus_segments(old_addr, old_end);
    let map_flags = mmap_prot_to_flags(source.prot);
    let populate = source.file_backing.is_some() || source.shared || source.locked;
    let mut aspace = process.aspace.lock();
    if aspace
        .map_alloc(VirtAddr::from(old_end), extension, map_flags, populate)
        .is_err()
    {
        return None;
    }
    drop(aspace);

    let mut region = source.subregion(old_addr, old_addr + old_size, source.prot);
    region.start = old_addr;
    region.size = new_size;
    if let Some(backing) = region.file_backing.as_mut() {
        backing.valid_len = backing.valid_len.max(new_size);
    }
    let region_size = region.size;
    process.record_mmap_region_entry(region);
    process.record_mmap_sigbus_ranges(preserved_sigbus);
    process.record_self_maxrss_kb(rss_kb_from_bytes(region_size));
    if source.shared && map_flags.contains(MappingFlags::WRITE) {
        process.record_shared_mmap(old_end, extension, map_flags);
    }
    Some(Ok(old_addr))
}

fn mremap_find_free_area(process: &UserProcess, size: usize) -> Option<usize> {
    let limit_size = mmap_limit_end().checked_sub(USER_MMAP_BASE)?;
    let limit = VirtAddrRange::from_start_size(VirtAddr::from(USER_MMAP_BASE), limit_size);
    let mut brk = process.brk.lock();
    let hint = align_up_checked(brk.next_mmap, PAGE_SIZE_4K)?;
    let start = {
        let aspace = process.aspace.lock();
        aspace
            .find_free_area(VirtAddr::from(hint), size, limit)
            .or_else(|| aspace.find_free_area(VirtAddr::from(USER_MMAP_BASE), size, limit))
    }?;
    let start = start.as_usize();
    brk.next_mmap = start
        .checked_add(size)
        .and_then(|end| end.checked_add(PAGE_SIZE_4K))
        .filter(|next| *next < mmap_limit_end())
        .unwrap_or(USER_MMAP_BASE);
    Some(start)
}

fn mremap_move(
    process: &UserProcess,
    source: super::UserMmapRegion,
    old_addr: usize,
    old_size: usize,
    target: usize,
    new_size: usize,
) -> Result<usize, LinuxError> {
    let target_end = target.checked_add(new_size).ok_or(LinuxError::ENOMEM)?;
    if target < USER_MMAP_BASE || target_end > mmap_limit_end() {
        return Err(LinuxError::ENOMEM);
    }
    let copy_size = old_size.min(new_size);
    let map_flags = mmap_prot_to_flags(source.prot);
    let populate = source.file_backing.is_some() || source.shared || source.locked;
    let moved_sigbus: Vec<(usize, usize)> = process
        .mmap_sigbus_segments(old_addr, old_addr + copy_size)
        .into_iter()
        .filter_map(|(start, end)| {
            let mapped_start = target.checked_add(start.checked_sub(old_addr)?)?;
            let mapped_end = target.checked_add(end.checked_sub(old_addr)?)?;
            (mapped_end > mapped_start).then_some((mapped_start, mapped_end))
        })
        .collect();

    process.forget_mmap_region(target, target_end);
    process.forget_mmap_range(target, target_end);
    {
        let mut aspace = process.aspace.lock();
        let _ = aspace.unmap(VirtAddr::from(target), new_size);
        aspace
            .map_alloc(VirtAddr::from(target), new_size, map_flags, populate)
            .map_err(LinuxError::from)?;
    }

    if let Err(err) = mremap_copy_user_range(process, old_addr, target, copy_size) {
        let _ = process
            .aspace
            .lock()
            .unmap(VirtAddr::from(target), new_size);
        return Err(err);
    }

    process.forget_mmap_region(old_addr, old_addr + old_size);
    process.forget_mmap_range(old_addr, old_addr + old_size);
    process
        .aspace
        .lock()
        .unmap(VirtAddr::from(old_addr), old_size)
        .map_err(LinuxError::from)?;

    let mut region = source.subregion(old_addr, old_addr + old_size, source.prot);
    region.start = target;
    region.size = new_size;
    if let Some(backing) = region.file_backing.as_mut() {
        backing.valid_len = backing.valid_len.max(new_size);
    }
    let region_size = region.size;
    process.record_mmap_region_entry(region);
    process.record_mmap_sigbus_ranges(moved_sigbus);
    process.record_self_maxrss_kb(rss_kb_from_bytes(region_size));
    if source.shared && map_flags.contains(MappingFlags::WRITE) {
        process.record_shared_mmap(target, new_size, map_flags);
    }
    Ok(target)
}

fn mremap_copy_user_range(
    process: &UserProcess,
    src: usize,
    dst: usize,
    len: usize,
) -> Result<(), LinuxError> {
    let chunk = MAX_USER_IO_CHUNK.min(PAGE_SIZE_4K * 16);
    let mut buf = Vec::new();
    buf.try_reserve_exact(chunk)
        .map_err(|_| LinuxError::ENOMEM)?;
    buf.resize(chunk, 0);
    let mut copied = 0usize;
    while copied < len {
        let chunk_len = (len - copied).min(buf.len());
        let src_addr = src.checked_add(copied).ok_or(LinuxError::ENOMEM)?;
        let dst_addr = dst.checked_add(copied).ok_or(LinuxError::ENOMEM)?;
        let mut aspace = process.aspace.lock();
        aspace
            .read(VirtAddr::from(src_addr), &mut buf[..chunk_len])
            .map_err(LinuxError::from)?;
        aspace
            .write(VirtAddr::from(dst_addr), &buf[..chunk_len])
            .map_err(LinuxError::from)?;
        copied += chunk_len;
    }
    Ok(())
}

pub(super) fn sys_munmap(process: &UserProcess, tf: &TrapFrame, addr: usize, len: usize) -> isize {
    if len == 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    if addr % PAGE_SIZE_4K != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let Some(raw_end) = addr.checked_add(len) else {
        return neg_errno(LinuxError::EINVAL);
    };
    let start = align_down(addr, PAGE_SIZE_4K);
    let Some(end) = align_up_checked(raw_end, PAGE_SIZE_4K) else {
        return neg_errno(LinuxError::EINVAL);
    };
    if end <= start {
        return neg_errno(LinuxError::EINVAL);
    }
    let self_stack_unmap = (start..end).contains(&tf.regs.sp);
    if start >= USER_MMAP_BASE && end - start <= 0x40000 {
        let query = process
            .aspace
            .lock()
            .page_table()
            .query(VirtAddr::from(start));
        user_trace!(
            "user-munmap: tid={} start={start:#x} end={end:#x} sp={:#x} tp={:#x} ra={:#x} pc={:#x} query_before={query:?}",
            current_tid(),
            tf.regs.sp,
            tf.regs.tp,
            tf.regs.ra,
            user_pc(tf),
        );
    }
    if self_stack_unmap {
        if let Some(ext) = current_task_ext() {
            user_trace!(
                "thrmunmap: defer tid={} start={start:#x} end={end:#x} sp={:#x} tp={:#x}",
                current_tid(),
                tf.regs.sp,
                tf.regs.tp,
            );
            ext.deferred_unmap_start.store(start, Ordering::Release);
            ext.deferred_unmap_len.store(end - start, Ordering::Release);
            return 0;
        }
    }
    let _ = msync_file_backed_ranges(process, start, end);
    process.forget_mmap_region(start, end);
    process.forget_mmap_range(start, end);
    let unmap_result = process
        .aspace
        .lock()
        .unmap(VirtAddr::from(start), end - start);
    match unmap_result {
        Ok(()) => 0,
        Err(err) => neg_errno(LinuxError::from(err)),
    }
}

pub(super) fn sys_msync(process: &UserProcess, addr: usize, len: usize, flags: usize) -> isize {
    const MS_ASYNC: usize = 0x1;
    const MS_INVALIDATE: usize = 0x2;
    const MS_SYNC: usize = 0x4;
    const SUPPORTED_FLAGS: usize = MS_ASYNC | MS_INVALIDATE | MS_SYNC;

    if addr & (PAGE_SIZE_4K - 1) != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    if flags & !SUPPORTED_FLAGS != 0 || flags & (MS_ASYNC | MS_SYNC) == (MS_ASYNC | MS_SYNC) {
        return neg_errno(LinuxError::EINVAL);
    }
    if len == 0 {
        return 0;
    }
    let Some(raw_end) = addr.checked_add(len) else {
        return neg_errno(LinuxError::ENOMEM);
    };
    let Some(end) = align_up_checked(raw_end, PAGE_SIZE_4K) else {
        return neg_errno(LinuxError::ENOMEM);
    };
    if end <= addr {
        return neg_errno(LinuxError::ENOMEM);
    }

    let aspace = process.aspace.lock();
    for page in PageIter4K::new(VirtAddr::from(addr), VirtAddr::from(end)).unwrap() {
        if aspace.page_table().query(page).is_err() {
            return neg_errno(LinuxError::ENOMEM);
        }
    }
    drop(aspace);
    if flags & MS_INVALIDATE != 0 && process.mmap_range_has_locked(addr, end) {
        return neg_errno(LinuxError::EBUSY);
    }
    if flags & MS_INVALIDATE != 0 {
        return invalidate_file_backed_ranges(process, addr, end).map_or_else(neg_errno, |_| 0);
    }
    msync_file_backed_ranges(process, addr, end).map_or_else(neg_errno, |_| 0)
}

pub(super) fn sys_madvise(process: &UserProcess, addr: usize, len: usize, advice: usize) -> isize {
    if addr & (PAGE_SIZE_4K - 1) != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let advice = advice as u32;
    if !madvise_advice_known(advice) {
        return neg_errno(LinuxError::EINVAL);
    }
    if len == 0 {
        return 0;
    }
    let Some(raw_end) = addr.checked_add(len) else {
        return neg_errno(LinuxError::ENOMEM);
    };
    let Some(end) = align_up_checked(raw_end, PAGE_SIZE_4K) else {
        return neg_errno(LinuxError::ENOMEM);
    };
    if end <= addr || end > USER_STACK_TOP {
        return neg_errno(LinuxError::ENOMEM);
    }
    match advice {
        general::MADV_HWPOISON => {
            if !madvise_mapped_range(process, addr, end) {
                return neg_errno(LinuxError::ENOMEM);
            }
            let size = end - addr;
            match process
                .aspace
                .lock()
                .protect(VirtAddr::from(addr), size, MappingFlags::USER)
            {
                Ok(()) => {
                    process.record_mmap_sigbus_range(addr, end);
                    0
                }
                Err(err) => neg_errno(LinuxError::from(err)),
            }
        }
        general::MADV_DONTNEED => {
            if !madvise_mapped_range(process, addr, end) {
                return neg_errno(LinuxError::ENOMEM);
            }
            if process.mmap_range_has_locked(addr, end)
                || madvise_range_satisfies(process, addr, end, |region| {
                    region.shared && !region.may_write
                })
            {
                return neg_errno(LinuxError::EINVAL);
            }
            if madvise_range_is_private_anonymous(process, addr, end) {
                return zero_user_range(process, addr, end).map_or_else(neg_errno, |_| 0);
            }
            0
        }
        general::MADV_REMOVE => {
            if !madvise_mapped_range(process, addr, end) {
                return neg_errno(LinuxError::ENOMEM);
            }
            if !madvise_range_satisfies(process, addr, end, |region| {
                region.shared
                    && region.may_write
                    && region.prot & general::PROT_WRITE != 0
                    && region.file_backing.is_some()
            }) {
                return neg_errno(LinuxError::EINVAL);
            }
            zero_user_range(process, addr, end).map_or_else(neg_errno, |_| 0)
        }
        general::MADV_FREE | general::MADV_WIPEONFORK | general::MADV_KEEPONFORK => {
            match madvise_private_anonymous_error(process, addr, end) {
                None => {
                    if advice == general::MADV_WIPEONFORK {
                        process.set_mmap_wipe_on_fork_range(addr, end, true);
                    } else if advice == general::MADV_KEEPONFORK {
                        process.set_mmap_wipe_on_fork_range(addr, end, false);
                    }
                    0
                }
                Some(err) => neg_errno(err),
            }
        }
        general::MADV_DONTFORK | general::MADV_DOFORK => {
            if !madvise_mapped_range(process, addr, end) {
                return neg_errno(LinuxError::ENOMEM);
            }
            if !madvise_range_is_tracked(process, addr, end) {
                return neg_errno(LinuxError::ENOMEM);
            }
            process.set_mmap_dont_fork_range(addr, end, advice == general::MADV_DONTFORK);
            0
        }
        general::MADV_POPULATE_READ | general::MADV_POPULATE_WRITE => {
            let write = advice == general::MADV_POPULATE_WRITE;
            madvise_populate_range(process, addr, end, write).map_or_else(neg_errno, |_| 0)
        }
        general::MADV_MERGEABLE | general::MADV_UNMERGEABLE => {
            if !madvise_mapped_range(process, addr, end) {
                return neg_errno(LinuxError::ENOMEM);
            }
            if madvise_range_satisfies(process, addr, end, |region| {
                region.may_write && region.prot & general::PROT_WRITE != 0
            }) {
                0
            } else {
                neg_errno(LinuxError::EINVAL)
            }
        }
        _ => {
            if !madvise_mapped_range(process, addr, end) {
                return neg_errno(LinuxError::ENOMEM);
            }
            0
        }
    }
}

fn madvise_advice_known(advice: u32) -> bool {
    matches!(
        advice,
        general::MADV_NORMAL
            | general::MADV_RANDOM
            | general::MADV_SEQUENTIAL
            | general::MADV_WILLNEED
            | general::MADV_DONTNEED
            | general::MADV_FREE
            | general::MADV_REMOVE
            | general::MADV_DONTFORK
            | general::MADV_DOFORK
            | general::MADV_HWPOISON
            | general::MADV_MERGEABLE
            | general::MADV_UNMERGEABLE
            | general::MADV_HUGEPAGE
            | general::MADV_NOHUGEPAGE
            | general::MADV_DONTDUMP
            | general::MADV_DODUMP
            | general::MADV_WIPEONFORK
            | general::MADV_KEEPONFORK
            | general::MADV_COLD
            | general::MADV_PAGEOUT
            | general::MADV_POPULATE_READ
            | general::MADV_POPULATE_WRITE
            | general::MADV_DONTNEED_LOCKED
    )
}

fn madvise_range_is_private_anonymous(process: &UserProcess, start: usize, end: usize) -> bool {
    madvise_range_satisfies(process, start, end, |region| {
        !region.shared && region.anonymous
    })
}

fn madvise_range_is_tracked(process: &UserProcess, start: usize, end: usize) -> bool {
    madvise_range_satisfies(process, start, end, |_| true)
}

fn madvise_private_anonymous_error(
    process: &UserProcess,
    start: usize,
    end: usize,
) -> Option<LinuxError> {
    let mut cursor = start;
    let mut regions = process.mmap_regions();
    regions.sort_by_key(|region| region.start);
    for region in regions {
        let region_end = region.end();
        if region_end <= cursor || region.start >= end {
            continue;
        }
        if region.start > cursor {
            return Some(LinuxError::ENOMEM);
        }
        if region.shared || !region.anonymous {
            return Some(LinuxError::EINVAL);
        }
        cursor = cursor.max(region_end.min(end));
        if cursor >= end {
            return None;
        }
    }
    Some(LinuxError::ENOMEM)
}

fn madvise_populate_range(
    process: &UserProcess,
    start: usize,
    end: usize,
    write: bool,
) -> Result<(), LinuxError> {
    let size = end - start;
    let access = if write {
        MappingFlags::WRITE
    } else {
        MappingFlags::READ
    };
    let fault = if write {
        PageFaultFlags::WRITE
    } else {
        PageFaultFlags::READ
    };
    let mut aspace = process.aspace.lock();
    if !aspace.contains_range(VirtAddr::from(start), size)
        || !aspace.can_access_range(VirtAddr::from(start), size, access)
    {
        return Err(LinuxError::ENOMEM);
    }
    let Some(pages) = PageIter4K::new(VirtAddr::from(start), VirtAddr::from(end)) else {
        return Err(LinuxError::ENOMEM);
    };
    for page in pages {
        let needs_fault = aspace.page_table().query(page).is_err()
            || (write
                && !aspace
                    .page_table()
                    .query(page)
                    .map(|(_, flags, _)| flags.contains(MappingFlags::WRITE))
                    .unwrap_or(false));
        if needs_fault && !aspace.handle_page_fault(page, fault) {
            return Err(LinuxError::ENOMEM);
        }
    }
    Ok(())
}

fn madvise_mapped_range(process: &UserProcess, start: usize, end: usize) -> bool {
    let size = end - start;
    let aspace = process.aspace.lock();
    aspace.contains_range(VirtAddr::from(start), size)
        && aspace.can_access_range(VirtAddr::from(start), size, MappingFlags::empty())
}

fn madvise_range_satisfies<F>(process: &UserProcess, start: usize, end: usize, mut pred: F) -> bool
where
    F: FnMut(&super::UserMmapRegion) -> bool,
{
    let mut cursor = start;
    let mut regions = process.mmap_regions();
    regions.sort_by_key(|region| region.start);
    for region in regions {
        let region_end = region.end();
        if region_end <= cursor || region.start >= end {
            continue;
        }
        if region.start > cursor || !pred(&region) {
            return false;
        }
        cursor = cursor.max(region_end.min(end));
        if cursor >= end {
            return true;
        }
    }
    false
}

fn zero_user_range(process: &UserProcess, start: usize, end: usize) -> Result<(), LinuxError> {
    let mut zeros = Vec::new();
    let chunk = MAX_USER_IO_CHUNK.min(PAGE_SIZE_4K * 16);
    zeros
        .try_reserve_exact(chunk)
        .map_err(|_| LinuxError::ENOMEM)?;
    zeros.resize(chunk, 0);
    let mut cursor = start;
    while cursor < end {
        let len = (end - cursor).min(zeros.len());
        process
            .aspace
            .lock()
            .write(VirtAddr::from(cursor), &zeros[..len])
            .map_err(LinuxError::from)?;
        cursor += len;
    }
    Ok(())
}

fn msync_file_backed_ranges(
    process: &UserProcess,
    start: usize,
    end: usize,
) -> Result<(), LinuxError> {
    for region in process.mmap_regions() {
        let Some(mut backing) = region.file_backing.clone() else {
            continue;
        };
        if !region.shared {
            continue;
        }
        let region_end = region.end();
        let flush_start = start.max(region.start);
        let flush_end = end
            .min(region_end)
            .min(region.start.saturating_add(backing.valid_len));
        if flush_end <= flush_start {
            continue;
        }

        let total_len = flush_end - flush_start;
        let mut copied = 0usize;
        while copied < total_len {
            let chunk_len = (total_len - copied).min(MAX_USER_IO_CHUNK);
            let chunk_addr = flush_start.checked_add(copied).ok_or(LinuxError::ENOMEM)?;
            let region_delta = chunk_addr
                .checked_sub(region.start)
                .ok_or(LinuxError::EINVAL)?;
            let file_offset = backing
                .offset
                .checked_add(region_delta as u64)
                .ok_or(LinuxError::EINVAL)?;
            let bytes = read_user_bytes(process, chunk_addr, chunk_len)?;

            let mut written = 0usize;
            while written < bytes.len() {
                let write_offset = file_offset
                    .checked_add(written as u64)
                    .ok_or(LinuxError::EINVAL)?;
                let n = write_mmap_file_backing(
                    process,
                    &mut backing.file,
                    write_offset,
                    &bytes[written..],
                )?;
                if n == 0 {
                    return Err(LinuxError::EIO);
                }
                written += n;
            }
            copied += chunk_len;
        }
    }
    Ok(())
}

fn invalidate_file_backed_ranges(
    process: &UserProcess,
    start: usize,
    end: usize,
) -> Result<(), LinuxError> {
    for region in process.mmap_regions() {
        let Some(mut backing) = region.file_backing.clone() else {
            continue;
        };
        if !region.shared {
            continue;
        }
        let region_end = region.end();
        let refresh_start = start.max(region.start);
        let refresh_end = end.min(region_end);
        if refresh_end <= refresh_start {
            continue;
        }

        let total_len = refresh_end - refresh_start;
        let mut copied = 0usize;
        while copied < total_len {
            let chunk_len = (total_len - copied).min(MAX_USER_IO_CHUNK);
            let chunk_addr = refresh_start
                .checked_add(copied)
                .ok_or(LinuxError::ENOMEM)?;
            let region_delta = chunk_addr
                .checked_sub(region.start)
                .ok_or(LinuxError::EINVAL)?;
            let file_offset = backing
                .offset
                .checked_add(region_delta as u64)
                .ok_or(LinuxError::EINVAL)?;
            let mut bytes = Vec::new();
            bytes
                .try_reserve_exact(chunk_len)
                .map_err(|_| LinuxError::ENOMEM)?;
            bytes.resize(chunk_len, 0);
            let _ = read_mmap_file_backing(process, &mut backing.file, file_offset, &mut bytes)?;
            write_user_bytes(process, chunk_addr, &bytes)?;
            copied += chunk_len;
        }
    }
    Ok(())
}

pub(super) fn sys_mincore(process: &UserProcess, addr: usize, len: usize, vec: usize) -> isize {
    if len == 0 {
        return 0;
    }
    if addr & (PAGE_SIZE_4K - 1) != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let Some(raw_end) = addr.checked_add(len) else {
        return neg_errno(LinuxError::ENOMEM);
    };
    let Some(end) = align_up_checked(raw_end, PAGE_SIZE_4K) else {
        return neg_errno(LinuxError::ENOMEM);
    };
    if end <= addr || end > USER_STACK_TOP {
        return neg_errno(LinuxError::ENOMEM);
    }
    let page_count = (end - addr) / PAGE_SIZE_4K;
    if let Err(err) = validate_user_write(process, vec, page_count) {
        return neg_errno(err);
    }

    let mut residency = Vec::new();
    if residency.try_reserve_exact(page_count).is_err() {
        return neg_errno(LinuxError::ENOMEM);
    }
    let aspace = process.aspace.lock();
    for page in PageIter4K::new(VirtAddr::from(addr), VirtAddr::from(end)).unwrap() {
        let query = aspace.query_address(page);
        if !query.area_found {
            return neg_errno(LinuxError::ENOMEM);
        }
        residency.push(u8::from(query.pte_mapped || query.shared_metadata));
    }
    drop(aspace);

    write_user_bytes(process, vec, residency.as_slice()).map_or_else(neg_errno, |_| 0)
}

pub(super) fn sys_mlock(process: &UserProcess, addr: usize, len: usize) -> isize {
    mlock_range(process, addr, len, true)
}

pub(super) fn sys_mlock2(process: &UserProcess, addr: usize, len: usize, flags: usize) -> isize {
    let flags = flags as u32;
    if flags & !general::MLOCK_ONFAULT != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    mlock_range(process, addr, len, flags & general::MLOCK_ONFAULT == 0)
}

fn mlock_range(process: &UserProcess, addr: usize, len: usize, populate: bool) -> isize {
    let (start, end) = match validate_lock_range(process, addr, len) {
        Ok(Some(range)) => range,
        Ok(None) => return 0,
        Err(err) => return neg_errno(err),
    };
    if let Err(err) = enforce_memlock_limit(process, end - start) {
        return neg_errno(err);
    }
    if populate {
        let mut aspace = process.aspace.lock();
        if aspace
            .populate_range(VirtAddr::from(start), end - start, PageFaultFlags::READ)
            .is_err()
        {
            return neg_errno(LinuxError::ENOMEM);
        }
    }
    process.set_mmap_lock_range(start, end, true);
    0
}

pub(super) fn sys_munlock(process: &UserProcess, addr: usize, len: usize) -> isize {
    match validate_lock_range(process, addr, len) {
        Ok(Some((start, end))) => {
            process.set_mmap_lock_range(start, end, false);
            0
        }
        Ok(None) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_mlockall(process: &UserProcess, flags: usize) -> isize {
    let supported = general::MCL_CURRENT | general::MCL_FUTURE | general::MCL_ONFAULT;
    if flags as u32 & !supported != 0
        || flags as u32 & (general::MCL_CURRENT | general::MCL_FUTURE) == 0
    {
        return neg_errno(LinuxError::EINVAL);
    }
    let current_lock_bytes = if flags as u32 & general::MCL_CURRENT != 0 {
        let brk = process.brk.lock();
        brk.end.saturating_sub(brk.start).max(PAGE_SIZE_4K)
    } else {
        0
    };
    if let Err(err) = enforce_memlock_limit(process, current_lock_bytes) {
        return neg_errno(err);
    }

    if flags as u32 & general::MCL_FUTURE != 0 {
        process.mlock_future.store(true, Ordering::Release);
    }
    if flags as u32 & general::MCL_CURRENT != 0 {
        process.set_all_mmap_locked(true);
        let accounted_kb = current_lock_bytes / 1024;
        process
            .mlockall_accounted_kb
            .store(accounted_kb.max(1), Ordering::Release);
    }
    0
}

fn enforce_memlock_limit(process: &UserProcess, bytes: usize) -> Result<(), LinuxError> {
    if bytes == 0 || process.uid() == 0 {
        return Ok(());
    }
    let limit = process.get_rlimit(general::RLIMIT_MEMLOCK).current();
    if limit == 0 {
        return Err(LinuxError::EPERM);
    }
    if bytes as u64 > limit {
        return Err(LinuxError::ENOMEM);
    }
    Ok(())
}

pub(super) fn sys_munlockall(process: &UserProcess) -> isize {
    process.mlock_future.store(false, Ordering::Release);
    process.set_all_mmap_locked(false);
    process.mlockall_accounted_kb.store(0, Ordering::Release);
    0
}

fn validate_lock_range(
    process: &UserProcess,
    addr: usize,
    len: usize,
) -> Result<Option<(usize, usize)>, LinuxError> {
    if len == 0 {
        return Ok(None);
    }
    let start = align_down(addr, PAGE_SIZE_4K);
    let raw_end = addr.checked_add(len).ok_or(LinuxError::ENOMEM)?;
    let end = align_up_checked(raw_end, PAGE_SIZE_4K).ok_or(LinuxError::ENOMEM)?;
    if end <= start || end > USER_STACK_TOP {
        return Err(LinuxError::ENOMEM);
    }

    let aspace = process.aspace.lock();
    for page in PageIter4K::new(VirtAddr::from(start), VirtAddr::from(end)).unwrap() {
        if !aspace.query_address(page).area_found {
            return Err(LinuxError::ENOMEM);
        }
    }
    Ok(Some((start, end)))
}

pub(super) fn sys_mprotect(
    _process: &UserProcess,
    _addr: usize,
    _len: usize,
    _prot: usize,
) -> isize {
    if _len == 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let Some(raw_end) = _addr.checked_add(_len) else {
        return neg_errno(LinuxError::EINVAL);
    };
    if _addr & (PAGE_SIZE_4K - 1) != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let start = _addr;
    let Some(end) = align_up_checked(raw_end, PAGE_SIZE_4K) else {
        return neg_errno(LinuxError::EINVAL);
    };
    if end <= start {
        return neg_errno(LinuxError::EINVAL);
    }
    if _len <= 0x40000 {
        user_trace!("user-mprotect: start={start:#x} end={end:#x} prot={_prot:#x}");
    }
    if _prot as u32 & general::PROT_WRITE != 0 && _process.mmap_range_denies_write(start, end) {
        return neg_errno(LinuxError::EACCES);
    }
    let prot_flags = mmap_prot_to_flags(_prot as u32);
    let mut aspace = _process.aspace.lock();
    let size = end - start;
    if !aspace.contains_range(VirtAddr::from(start), size)
        || !aspace.can_access_range(VirtAddr::from(start), size, MappingFlags::empty())
    {
        return neg_errno(LinuxError::ENOMEM);
    }
    match aspace.protect(VirtAddr::from(start), size, prot_flags) {
        Ok(()) => {
            _process.protect_mmap_region(start, end, _prot as u32);
            _process.protect_shared_mmap_range(start, end, prot_flags);
            // Thread stacks are typically created as PROT_NONE mappings and then
            // flipped to writable with mprotect(). Pre-fault only the stack-top
            // pages so the first user-space writes succeed without turning the
            // whole stack into eagerly allocated memory.
            if _prot as u32 & general::PROT_WRITE != 0 && end - start <= 0x40000 {
                let prefault_start = end.saturating_sub(PAGE_SIZE_4K * 2).max(start);
                for page in
                    PageIter4K::new(VirtAddr::from(prefault_start), VirtAddr::from(end)).unwrap()
                {
                    let _ = aspace.handle_page_fault(page, PageFaultFlags::WRITE);
                }
            }
            0
        }
        Err(err) => neg_errno(LinuxError::from(err)),
    }
}
