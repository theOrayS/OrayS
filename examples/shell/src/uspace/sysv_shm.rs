use core::mem::size_of;
use core::ptr;
use core::sync::atomic::{AtomicI32, Ordering};

use axalloc::global_allocator;
use axerrno::LinuxError;
use axhal::context::TrapFrame;
use axhal::mem::virt_to_phys;
use axsync::Mutex;
use lazyinit::LazyInit;
use memory_addr::{PAGE_SIZE_4K, VirtAddr};
use std::collections::BTreeMap;
use std::vec::Vec;

use super::UserProcess;
use super::linux_abi::{
    SYSV_IPC_CREAT, SYSV_IPC_EXCL, SYSV_IPC_PRIVATE, SYSV_IPC_RMID, SYSV_IPC_SET, SYSV_IPC_STAT,
    SYSV_SHM_MAX_SIZE, SYSV_SHM_RDONLY, USER_MMAP_BASE, USER_STACK_SIZE, USER_STACK_TOP, neg_errno,
};
use super::memory_map::{align_down, align_up_checked, sys_munmap, user_mapping_flags};
use super::user_memory::clear_user_bytes;

#[derive(Clone)]
struct SysvShmSegment {
    key: i32,
    size: usize,
    backing_vaddr: usize,
}

static NEXT_SYSV_SHM_ID: AtomicI32 = AtomicI32::new(1);

fn table() -> &'static Mutex<BTreeMap<i32, SysvShmSegment>> {
    static SYSV_SHM: LazyInit<Mutex<BTreeMap<i32, SysvShmSegment>>> = LazyInit::new();
    if !SYSV_SHM.is_inited() {
        SYSV_SHM.init_once(Mutex::new(BTreeMap::new()));
    }
    &SYSV_SHM
}

fn removed_segments() -> &'static Mutex<Vec<SysvShmSegment>> {
    static REMOVED_SYSV_SHM: LazyInit<Mutex<Vec<SysvShmSegment>>> = LazyInit::new();
    if !REMOVED_SYSV_SHM.is_inited() {
        REMOVED_SYSV_SHM.init_once(Mutex::new(Vec::new()));
    }
    &REMOVED_SYSV_SHM
}

fn get_or_create(key: usize, size: usize, shmflg: usize) -> Result<i32, LinuxError> {
    let key = key as i32;
    let flags = shmflg as i32;
    let mut table = table().lock();
    if key != SYSV_IPC_PRIVATE {
        if let Some((shmid, segment)) = table.iter().find(|(_, segment)| segment.key == key) {
            if flags & SYSV_IPC_CREAT != 0 && flags & SYSV_IPC_EXCL != 0 {
                return Err(LinuxError::EINVAL);
            }
            if size > segment.size {
                return Err(LinuxError::EINVAL);
            }
            return Ok(*shmid);
        }
        if flags & SYSV_IPC_CREAT == 0 {
            return Err(LinuxError::ENOENT);
        }
    }

    let size = align_up_checked(size.max(1), PAGE_SIZE_4K).ok_or(LinuxError::ENOMEM)?;
    if size == 0 || size > SYSV_SHM_MAX_SIZE {
        return Err(LinuxError::ENOMEM);
    }
    let pages = size / PAGE_SIZE_4K;
    let backing_vaddr = global_allocator()
        .alloc_pages(pages, PAGE_SIZE_4K)
        .map_err(|_| LinuxError::ENOMEM)?;
    unsafe {
        ptr::write_bytes(backing_vaddr as *mut u8, 0, size);
    }
    let shmid = NEXT_SYSV_SHM_ID.fetch_add(1, Ordering::Relaxed);
    table.insert(
        shmid,
        SysvShmSegment {
            key,
            size,
            backing_vaddr,
        },
    );
    Ok(shmid)
}

fn lookup(shmid: i32) -> Option<(usize, usize)> {
    table()
        .lock()
        .get(&shmid)
        .map(|segment| (segment.size, segment.backing_vaddr))
}

fn contains(shmid: i32) -> bool {
    table().lock().contains_key(&shmid)
}

fn remove(shmid: i32) {
    if let Some(segment) = table().lock().remove(&shmid) {
        // System V IPC_RMID removes the identifier immediately but keeps the
        // backing object alive while existing attachments still reference it.
        // This evaluator does not yet track attachment reference counts, so
        // retire the segment from future lookups without freeing live mapped
        // pages out from under parent/child LTP result buffers.
        removed_segments().lock().push(segment);
    }
}

pub(super) fn sys_shmget(_process: &UserProcess, key: usize, size: usize, shmflg: usize) -> isize {
    match get_or_create(key, size, shmflg) {
        Ok(shmid) => shmid as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_shmat(
    process: &UserProcess,
    shmid: usize,
    shmaddr: usize,
    shmflg: usize,
) -> isize {
    let shmid = shmid as i32;
    let Some((size, backing_vaddr)) = lookup(shmid) else {
        return neg_errno(LinuxError::EINVAL);
    };
    let map_flags = if shmflg as i32 & SYSV_SHM_RDONLY != 0 {
        user_mapping_flags(true, false, false)
    } else {
        user_mapping_flags(true, true, false)
    };
    let target = {
        let mut brk = process.brk.lock();
        let start = if shmaddr == 0 {
            let Some(start) = align_up_checked(brk.next_mmap, PAGE_SIZE_4K) else {
                return neg_errno(LinuxError::ENOMEM);
            };
            brk.next_mmap = start
                .checked_add(size)
                .and_then(|end| end.checked_add(PAGE_SIZE_4K))
                .unwrap_or(usize::MAX);
            start
        } else {
            align_down(shmaddr, PAGE_SIZE_4K)
        };
        let Some(end) = start.checked_add(size) else {
            return neg_errno(LinuxError::ENOMEM);
        };
        if start < USER_MMAP_BASE || end > USER_STACK_TOP - USER_STACK_SIZE {
            return neg_errno(LinuxError::ENOMEM);
        }
        start
    };
    let paddr = virt_to_phys(VirtAddr::from(backing_vaddr));
    let map_result = {
        let mut aspace = process.aspace.lock();
        if shmaddr != 0 {
            let _ = aspace.unmap(VirtAddr::from(target), size);
        }
        aspace.map_linear(VirtAddr::from(target), paddr, size, map_flags)
    };
    if let Err(err) = map_result {
        return neg_errno(LinuxError::from(err));
    }
    process.shm_attachments.lock().insert(target, (shmid, size));
    target as isize
}

pub(super) fn sys_shmdt(process: &UserProcess, tf: &TrapFrame, shmaddr: usize) -> isize {
    let Some((_shmid, size)) = process.shm_attachments.lock().remove(&shmaddr) else {
        return neg_errno(LinuxError::EINVAL);
    };
    sys_munmap(process, tf, shmaddr, size)
}

pub(super) fn sys_shmctl(process: &UserProcess, shmid: usize, cmd: usize, buf: usize) -> isize {
    let shmid = shmid as i32;
    let cmd = cmd as i32;
    if !contains(shmid) {
        return neg_errno(LinuxError::EINVAL);
    }
    match cmd {
        SYSV_IPC_RMID => {
            remove(shmid);
            0
        }
        SYSV_IPC_STAT => {
            if buf != 0 {
                if let Err(err) = clear_user_bytes(process, buf, size_of::<usize>() * 16) {
                    return neg_errno(err);
                }
            }
            0
        }
        SYSV_IPC_SET => 0,
        _ => neg_errno(LinuxError::EINVAL),
    }
}
