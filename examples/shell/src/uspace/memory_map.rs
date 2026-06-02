use core::sync::atomic::Ordering;

use axerrno::LinuxError;
use axhal::context::TrapFrame;
use axhal::paging::MappingFlags;
use axhal::trap::{register_trap_handler, PageFaultFlags, PAGE_FAULT};
use linux_raw_sys::general;
use memory_addr::{PageIter4K, VirtAddr, VirtAddrRange, PAGE_SIZE_4K};
use std::vec::Vec;

use super::linux_abi::{neg_errno, SIGSEGV_NUM, USER_MMAP_BASE, USER_STACK_SIZE, USER_STACK_TOP};
use super::process_lifecycle::{terminate_current_thread, terminate_current_thread_for_exit_group};
use super::signal_abi::queue_current_synchronous_signal;
use super::task_context::current_process;
use super::task_context::current_task_ext;
use super::user_memory::{validate_user_write, write_user_bytes};
use super::UserProcess;

macro_rules! user_trace {
    ($($arg:tt)*) => {};
}

pub(super) fn sys_brk(process: &UserProcess, addr: usize) -> isize {
    let mut brk = process.brk.lock();
    if addr == 0 {
        return brk.end as isize;
    }
    if addr < brk.start || addr > brk.limit {
        return brk.end as isize;
    }
    brk.end = addr;
    brk.end as isize
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
    let Some(process) = current_process() else {
        return false;
    };
    if let Some(code) = process.pending_exit_group() {
        user_trace!(
            "user-exit-group-pf: tid={} code={code} fault_vaddr={vaddr:#x} flags={flags:?}",
            current_tid(),
        );
        terminate_current_thread_for_exit_group(process.as_ref(), code);
    }
    let should_trace = _from_user
        && flags.contains(PageFaultFlags::WRITE)
        && vaddr.as_usize() >= USER_MMAP_BASE
        && vaddr.as_usize() < USER_STACK_TOP;
    let handled = {
        let mut aspace = process.aspace.lock();
        if should_trace {
            let _query = aspace
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
            let _query = aspace
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
        terminate_current_thread(process.as_ref(), 128 + signal);
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
    if len == 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let Some(size) = align_up_checked(len, PAGE_SIZE_4K) else {
        return neg_errno(LinuxError::ENOMEM);
    };
    if size == 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let flags_u32 = flags as u32;
    match flags_u32 & general::MAP_TYPE {
        general::MAP_SHARED | general::MAP_PRIVATE | general::MAP_SHARED_VALIDATE => {}
        _ => return neg_errno(LinuxError::EINVAL),
    }
    let anonymous = flags_u32 & general::MAP_ANONYMOUS != 0;
    let shared = flags_u32 & general::MAP_SHARED != 0;
    let map_fixed = flags_u32 & general::MAP_FIXED != 0;
    let locked = flags_u32 & general::MAP_LOCKED != 0;
    let request_addr = if addr == 0 {
        None
    } else {
        Some(align_down(addr, PAGE_SIZE_4K))
    };
    let map_flags = mmap_prot_to_flags(prot as u32);
    let mmap_limit_end = USER_STACK_TOP - USER_STACK_SIZE;
    let target = if let Some(start) = request_addr {
        let Some(end) = start.checked_add(size) else {
            return neg_errno(LinuxError::ENOMEM);
        };
        if start < USER_MMAP_BASE || end > mmap_limit_end {
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
    let populate = !anonymous || shared || locked;
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
    if !anonymous {
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
    }
    if shared && map_flags.contains(MappingFlags::WRITE) {
        process.record_shared_mmap(target, size, map_flags);
    }
    process.record_mmap_region(target, size, prot as u32, shared, locked);
    if let Some((start, end)) = sigbus_range {
        process.record_mmap_sigbus_range(start, end);
    }
    target as isize
}

pub(super) fn sys_munmap(process: &UserProcess, tf: &TrapFrame, addr: usize, len: usize) -> isize {
    if len == 0 {
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
        let _query = process
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
    0
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
        if aspace.page_table().query(page).is_err() {
            return neg_errno(LinuxError::ENOMEM);
        }
        residency.push(1);
    }
    drop(aspace);

    write_user_bytes(process, vec, residency.as_slice()).map_or_else(neg_errno, |_| 0)
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
    let prot_flags = mmap_prot_to_flags(_prot as u32);
    let mut aspace = _process.aspace.lock();
    match aspace.protect(VirtAddr::from(start), end - start, prot_flags) {
        Ok(()) => {
            _process.protect_mmap_region(start, end, _prot as u32);
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
