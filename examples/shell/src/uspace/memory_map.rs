use core::sync::atomic::Ordering;

use axerrno::LinuxError;
use axhal::context::TrapFrame;
use axhal::paging::MappingFlags;
use axhal::trap::{PAGE_FAULT, PageFaultFlags, register_trap_handler};
use linux_raw_sys::general;
use memory_addr::{PAGE_SIZE_4K, PageIter4K, VirtAddr};

use super::UserProcess;
use super::linux_abi::{USER_MMAP_BASE, USER_STACK_SIZE, USER_STACK_TOP, neg_errno};
use super::process_lifecycle::{terminate_current_thread, terminate_current_thread_for_exit_group};
use super::task_context::current_process;
use super::task_context::current_task_ext;

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
    if value == 0 {
        0
    } else {
        align_down(value + align - 1, align)
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
        terminate_current_thread(process.as_ref(), 128 + 11);
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
    let size = align_up(len.max(1), PAGE_SIZE_4K);
    let anonymous = flags as u32 & general::MAP_ANONYMOUS != 0;
    let map_fixed = flags as u32 & general::MAP_FIXED != 0;
    let request_addr = if addr == 0 {
        None
    } else {
        Some(align_down(addr, PAGE_SIZE_4K))
    };
    let map_flags = mmap_prot_to_flags(prot as u32);
    let target = {
        let mut brk = process.brk.lock();
        let start = request_addr.unwrap_or_else(|| {
            let start = align_up(brk.next_mmap, PAGE_SIZE_4K);
            brk.next_mmap = start + size + PAGE_SIZE_4K;
            start
        });
        if start < USER_MMAP_BASE || start + size >= USER_STACK_TOP - USER_STACK_SIZE {
            return neg_errno(LinuxError::ENOMEM);
        }
        start
    };
    if anonymous && size <= 0x40000 {
        user_trace!("user-mmap: target={target:#x} len={size:#x} prot={prot:#x} flags={flags:#x}");
    }
    let populate = !anonymous;
    {
        let mut aspace = process.aspace.lock();
        if map_fixed {
            let _ = aspace.unmap(VirtAddr::from(target), size);
        }
        if let Err(err) = aspace.map_alloc(VirtAddr::from(target), size, map_flags, populate) {
            return neg_errno(LinuxError::from(err));
        }
    }

    if !anonymous {
        let file_bytes = {
            let mut table = process.fds.lock();
            match table.read_file_at(fd as i32, offset as u64, len) {
                Ok(bytes) => bytes,
                Err(err) => return neg_errno(err),
            }
        };
        if let Err(err) = process
            .aspace
            .lock()
            .write(VirtAddr::from(target), &file_bytes)
        {
            return neg_errno(LinuxError::from(err));
        }
    }
    target as isize
}

pub(super) fn sys_munmap(process: &UserProcess, tf: &TrapFrame, addr: usize, len: usize) -> isize {
    if len == 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let start = align_down(addr, PAGE_SIZE_4K);
    let end = align_up(addr.saturating_add(len), PAGE_SIZE_4K);
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
    let unmap_result = process
        .aspace
        .lock()
        .unmap(VirtAddr::from(start), end - start);
    match unmap_result {
        Ok(()) => 0,
        Err(err) => neg_errno(LinuxError::from(err)),
    }
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
    let start = align_down(_addr, PAGE_SIZE_4K);
    let end = align_up(_addr.saturating_add(_len), PAGE_SIZE_4K);
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
