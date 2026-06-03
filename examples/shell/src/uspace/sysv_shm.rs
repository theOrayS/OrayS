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
use std::string::String;
use std::vec::Vec;

use super::UserProcess;
use super::linux_abi::{
    SYSV_IPC_CREAT, SYSV_IPC_EXCL, SYSV_IPC_INFO, SYSV_IPC_PRIVATE, SYSV_IPC_RMID, SYSV_IPC_SET,
    SYSV_IPC_STAT, SYSV_SHM_EXEC, SYSV_SHM_HUGETLB, SYSV_SHM_INFO, SYSV_SHM_LOCK,
    SYSV_SHM_MAX_SEGMENTS, SYSV_SHM_MAX_SIZE, SYSV_SHM_RDONLY, SYSV_SHM_REMAP, SYSV_SHM_RND,
    SYSV_SHM_STAT, SYSV_SHM_STAT_ANY, SYSV_SHM_UNLOCK, USER_MMAP_BASE, USER_STACK_SIZE,
    USER_STACK_TOP, neg_errno,
};
use super::memory_map::{align_down, align_up_checked, sys_munmap, user_mapping_flags};
use super::user_memory::{validate_user_read, write_user_value};

#[derive(Clone)]
struct SysvShmSegment {
    key: i32,
    mode: u32,
    uid: u32,
    gid: u32,
    cuid: u32,
    cgid: u32,
    requested_size: usize,
    size: usize,
    backing_vaddr: usize,
    attach_count: usize,
    create_pid: i32,
    last_pid: i32,
    attach_time: isize,
    detach_time: isize,
    change_time: isize,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct UserIpcPerm64 {
    key: i32,
    uid: u32,
    gid: u32,
    cuid: u32,
    cgid: u32,
    mode: u32,
    seq: u16,
    pad2: u16,
    unused1: usize,
    unused2: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct UserShmidDs64 {
    shm_perm: UserIpcPerm64,
    shm_segsz: usize,
    shm_atime: isize,
    shm_dtime: isize,
    shm_ctime: isize,
    shm_cpid: i32,
    shm_lpid: i32,
    shm_nattch: usize,
    unused4: usize,
    unused5: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct UserShminfo64 {
    shmmax: usize,
    shmmin: usize,
    shmmni: usize,
    shmseg: usize,
    shmall: usize,
    unused: [usize; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct UserShmInfo64 {
    used_ids: i32,
    pad: i32,
    shm_tot: usize,
    shm_rss: usize,
    shm_swp: usize,
    swap_attempts: usize,
    swap_successes: usize,
}

// Linux 64-bit IPC user ABI: asm-generic/{ipcbuf,shmbuf}.h.  RISC-V and
// LoongArch both use this shmid64_ds layout for shmctl(IPC_STAT).
const _: [(); 48] = [(); size_of::<UserIpcPerm64>()];
const _: [(); 112] = [(); size_of::<UserShmidDs64>()];
const _: [(); 72] = [(); size_of::<UserShminfo64>()];
const _: [(); 48] = [(); size_of::<UserShmInfo64>()];

static NEXT_SYSV_SHM_ID: AtomicI32 = AtomicI32::new(1);

fn table() -> &'static Mutex<BTreeMap<i32, SysvShmSegment>> {
    static SYSV_SHM: LazyInit<Mutex<BTreeMap<i32, SysvShmSegment>>> = LazyInit::new();
    if !SYSV_SHM.is_inited() {
        SYSV_SHM.init_once(Mutex::new(BTreeMap::new()));
    }
    &SYSV_SHM
}

fn removed_segments() -> &'static Mutex<BTreeMap<i32, SysvShmSegment>> {
    static REMOVED_SYSV_SHM: LazyInit<Mutex<BTreeMap<i32, SysvShmSegment>>> = LazyInit::new();
    if !REMOVED_SYSV_SHM.is_inited() {
        REMOVED_SYSV_SHM.init_once(Mutex::new(BTreeMap::new()));
    }
    &REMOVED_SYSV_SHM
}

fn current_time_secs() -> isize {
    axhal::time::wall_time().as_secs().min(isize::MAX as u64) as isize
}

fn parse_leading_usize(raw: &str) -> Option<usize> {
    let mut value = 0usize;
    let mut seen_digit = false;
    for byte in raw.bytes() {
        if !seen_digit && byte.is_ascii_whitespace() {
            continue;
        }
        if byte.is_ascii_digit() {
            seen_digit = true;
            value = value.checked_mul(10)?.checked_add((byte - b'0') as usize)?;
        } else {
            break;
        }
    }
    seen_digit.then_some(value)
}

fn read_proc_sys_usize(path: &str, default: usize) -> usize {
    axfs::api::read_to_string(path)
        .ok()
        .and_then(|raw| parse_leading_usize(&raw))
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

fn configured_shm_max_size() -> usize {
    read_proc_sys_usize("/proc/sys/kernel/shmmax", SYSV_SHM_MAX_SIZE).min(SYSV_SHM_MAX_SIZE)
}

fn configured_shm_total_pages() -> usize {
    read_proc_sys_usize("/proc/sys/kernel/shmall", SYSV_SHM_MAX_SIZE / PAGE_SIZE_4K)
}

fn shm_low_boundary() -> usize {
    if cfg!(target_arch = "loongarch64") {
        64 * 1024
    } else {
        PAGE_SIZE_4K
    }
}

fn segment_mode_allows(
    segment: &SysvShmSegment,
    process: &UserProcess,
    read: bool,
    write: bool,
) -> bool {
    if process.uid() == 0 {
        return true;
    }
    let shift = if process.uid() == segment.uid {
        6
    } else if process.gid() == segment.gid {
        3
    } else {
        0
    };
    let perms = (segment.mode >> shift) & 0o7;
    (!read || perms & 0o4 != 0) && (!write || perms & 0o2 != 0)
}

fn segment_control_allowed(segment: &SysvShmSegment, process: &UserProcess) -> bool {
    process.uid() == 0 || process.uid() == segment.uid || process.uid() == segment.cuid
}

fn requested_access(flags: i32) -> (bool, bool) {
    let mode = flags as u32 & 0o777;
    (mode & 0o444 != 0, mode & 0o222 != 0)
}

fn get_or_create(
    process: &UserProcess,
    key: usize,
    size: usize,
    shmflg: usize,
) -> Result<i32, LinuxError> {
    let key = key as i32;
    let flags = shmflg as i32;
    let requested_size = size;
    let mut table = table().lock();
    if key != SYSV_IPC_PRIVATE {
        if let Some((shmid, segment)) = table.iter().find(|(_, segment)| segment.key == key) {
            if flags & SYSV_IPC_CREAT != 0 && flags & SYSV_IPC_EXCL != 0 {
                return Err(LinuxError::EEXIST);
            }
            if size > segment.requested_size {
                return Err(LinuxError::EINVAL);
            }
            let (read, write) = requested_access(flags);
            if (read || write) && !segment_mode_allows(segment, process, read, write) {
                return Err(LinuxError::EACCES);
            }
            return Ok(*shmid);
        }
        if flags & SYSV_IPC_CREAT == 0 {
            return Err(LinuxError::ENOENT);
        }
    }

    if flags & SYSV_SHM_HUGETLB != 0 {
        return Err(LinuxError::EINVAL);
    }
    if table.len() >= SYSV_SHM_MAX_SEGMENTS {
        return Err(LinuxError::ENOSPC);
    }
    if size == 0 || size > configured_shm_max_size() {
        return Err(LinuxError::EINVAL);
    }
    let size = align_up_checked(size, PAGE_SIZE_4K).ok_or(LinuxError::ENOMEM)?;
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
            mode: (flags as u32) & 0o777,
            uid: process.uid(),
            gid: process.gid(),
            cuid: process.uid(),
            cgid: process.gid(),
            requested_size,
            size,
            backing_vaddr,
            attach_count: 0,
            create_pid: process.pid(),
            last_pid: 0,
            attach_time: 0,
            detach_time: 0,
            change_time: current_time_secs(),
        },
    );
    Ok(shmid)
}

fn lookup_for_attach(
    process: &UserProcess,
    shmid: i32,
    readonly: bool,
) -> Result<(usize, usize), LinuxError> {
    let table = table().lock();
    let segment = table.get(&shmid).ok_or(LinuxError::EINVAL)?;
    if !segment_mode_allows(segment, process, true, !readonly) {
        return Err(LinuxError::EACCES);
    }
    Ok((segment.size, segment.backing_vaddr))
}

fn contains(shmid: i32) -> bool {
    table().lock().contains_key(&shmid)
}

fn free_segment(segment: SysvShmSegment) {
    global_allocator().dealloc_pages(segment.backing_vaddr, segment.size / PAGE_SIZE_4K);
}

fn increment_attach(shmid: i32) -> bool {
    if let Some(segment) = table().lock().get_mut(&shmid) {
        segment.attach_count = segment.attach_count.saturating_add(1);
        segment.attach_time = current_time_secs();
        return true;
    }
    if let Some(segment) = removed_segments().lock().get_mut(&shmid) {
        segment.attach_count = segment.attach_count.saturating_add(1);
        segment.attach_time = current_time_secs();
        return true;
    }
    false
}

fn decrement_attach(shmid: i32, pid: i32) {
    if let Some(segment) = table().lock().get_mut(&shmid) {
        segment.attach_count = segment.attach_count.saturating_sub(1);
        segment.detach_time = current_time_secs();
        segment.last_pid = pid;
        return;
    }

    let mut removed = removed_segments().lock();
    let Some(segment) = removed.get_mut(&shmid) else {
        return;
    };
    segment.attach_count = segment.attach_count.saturating_sub(1);
    segment.detach_time = current_time_secs();
    segment.last_pid = pid;
    if segment.attach_count == 0 {
        if let Some(segment) = removed.remove(&shmid) {
            free_segment(segment);
        }
    }
}

pub(super) fn retain_attachments(attachments: &BTreeMap<usize, (i32, usize)>) {
    for (shmid, _) in attachments.values() {
        increment_attach(*shmid);
    }
}

pub(super) fn release_process_attachments(process: &UserProcess) {
    let attachments = {
        let mut attachments = process.shm_attachments.lock();
        core::mem::take(&mut *attachments)
    };
    for (shmid, _) in attachments.values() {
        decrement_attach(*shmid, process.pid());
    }
}

fn stat_from_segment(segment: &SysvShmSegment) -> UserShmidDs64 {
    UserShmidDs64 {
        shm_perm: UserIpcPerm64 {
            key: segment.key,
            uid: segment.uid,
            gid: segment.gid,
            cuid: segment.cuid,
            cgid: segment.cgid,
            mode: segment.mode,
            ..UserIpcPerm64::default()
        },
        shm_segsz: segment.requested_size,
        shm_atime: segment.attach_time,
        shm_dtime: segment.detach_time,
        shm_ctime: segment.change_time,
        shm_cpid: segment.create_pid,
        shm_lpid: segment.last_pid,
        shm_nattch: segment.attach_count,
        ..UserShmidDs64::default()
    }
}

fn stat(process: &UserProcess, shmid: i32) -> Result<UserShmidDs64, LinuxError> {
    let table = table().lock();
    let segment = table.get(&shmid).ok_or(LinuxError::EINVAL)?;
    if !segment_mode_allows(segment, process, true, false) {
        return Err(LinuxError::EACCES);
    }
    Ok(stat_from_segment(segment))
}

fn stat_by_index(process: &UserProcess, index: i32) -> Result<(i32, UserShmidDs64), LinuxError> {
    let table = table().lock();
    let segment = table.get(&index).ok_or(LinuxError::EINVAL)?;
    if !segment_mode_allows(segment, process, true, false) {
        return Err(LinuxError::EACCES);
    }
    Ok((index, stat_from_segment(segment)))
}

fn stat_by_index_any(index: i32) -> Result<(i32, UserShmidDs64), LinuxError> {
    let table = table().lock();
    let segment = table.get(&index).ok_or(LinuxError::EINVAL)?;
    Ok((index, stat_from_segment(segment)))
}

fn active_shm_max_id() -> i32 {
    table().lock().keys().copied().max().unwrap_or(0)
}

fn active_shm_snapshot() -> (i32, i32, usize) {
    let table = table().lock();
    let max_id = table.keys().copied().max().unwrap_or(0);
    let used_ids = table.len().min(i32::MAX as usize) as i32;
    let pages = table
        .values()
        .map(|segment| segment.size / PAGE_SIZE_4K)
        .sum();
    (max_id, used_ids, pages)
}

pub(super) fn proc_sysvipc_shm_content() -> Vec<u8> {
    let table = table().lock();
    let mut content = String::from(
        "       key      shmid perms                  size  cpid  lpid nattch   uid   gid  cuid  cgid      atime      dtime      ctime       rss      swap\n",
    );
    for (shmid, segment) in table.iter() {
        let rss_bytes = segment.size;
        content.push_str(&format!(
            "{key:10} {shmid:10} {mode:5o} {size:21} {cpid:5} {lpid:5} {nattch:6} {uid:5} {gid:5} {cuid:5} {cgid:5} {atime:10} {dtime:10} {ctime:10} {rss:10} {swap:10}\n",
            key = segment.key,
            shmid = shmid,
            mode = segment.mode,
            size = segment.requested_size,
            cpid = segment.create_pid,
            lpid = segment.last_pid,
            nattch = segment.attach_count,
            uid = segment.uid,
            gid = segment.gid,
            cuid = segment.cuid,
            cgid = segment.cgid,
            atime = segment.attach_time,
            dtime = segment.detach_time,
            ctime = segment.change_time,
            rss = rss_bytes,
            swap = 0,
        ));
    }
    content.into_bytes()
}

fn control_allowed(process: &UserProcess, shmid: i32) -> Result<bool, LinuxError> {
    let table = table().lock();
    let segment = table.get(&shmid).ok_or(LinuxError::EINVAL)?;
    Ok(segment_control_allowed(segment, process))
}

fn remove(shmid: i32) {
    if let Some(segment) = table().lock().remove(&shmid) {
        // System V IPC_RMID removes the identifier immediately but keeps the
        // backing object alive while existing attachments still reference it.
        if segment.attach_count == 0 {
            free_segment(segment);
        } else {
            removed_segments().lock().insert(shmid, segment);
        }
    }
}

fn range_has_mapping(process: &UserProcess, start: usize, size: usize) -> bool {
    let Some(end) = start.checked_add(size) else {
        return true;
    };
    let aspace = process.aspace.lock();
    let mut page = start;
    while page < end {
        if aspace.query_address(VirtAddr::from(page)).area_found {
            return true;
        }
        page = page.saturating_add(PAGE_SIZE_4K);
    }
    false
}

pub(super) fn sys_shmget(_process: &UserProcess, key: usize, size: usize, shmflg: usize) -> isize {
    match get_or_create(_process, key, size, shmflg) {
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
    let shmflg = shmflg as i32;
    const SYSV_SHMAT_KNOWN_FLAGS: i32 =
        SYSV_SHM_RDONLY | SYSV_SHM_RND | SYSV_SHM_REMAP | SYSV_SHM_EXEC;
    if shmflg & !SYSV_SHMAT_KNOWN_FLAGS != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    if shmaddr == 0 && shmflg & SYSV_SHM_REMAP != 0 {
        return neg_errno(LinuxError::EINVAL);
    }

    let readonly = shmflg & SYSV_SHM_RDONLY != 0;
    let (size, backing_vaddr) = match lookup_for_attach(process, shmid, readonly) {
        Ok(segment) => segment,
        Err(err) => return neg_errno(err),
    };

    let map_flags = if readonly {
        user_mapping_flags(true, false, shmflg & SYSV_SHM_EXEC != 0)
    } else {
        user_mapping_flags(true, true, shmflg & SYSV_SHM_EXEC != 0)
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
        } else if shmflg & SYSV_SHM_RND != 0 {
            align_down(shmaddr, shm_low_boundary())
        } else if shmaddr & (PAGE_SIZE_4K - 1) != 0 {
            return neg_errno(LinuxError::EINVAL);
        } else {
            shmaddr
        };
        let Some(end) = start.checked_add(size) else {
            return neg_errno(LinuxError::ENOMEM);
        };
        if start == 0 && shmflg & SYSV_SHM_REMAP != 0 {
            return neg_errno(LinuxError::EINVAL);
        }
        if start < USER_MMAP_BASE || end > USER_STACK_TOP - USER_STACK_SIZE {
            return neg_errno(LinuxError::ENOMEM);
        }
        start
    };
    if shmaddr != 0 && shmflg & SYSV_SHM_REMAP == 0 && range_has_mapping(process, target, size) {
        return neg_errno(LinuxError::EINVAL);
    }
    let paddr = virt_to_phys(VirtAddr::from(backing_vaddr));
    let map_result = {
        let mut aspace = process.aspace.lock();
        if shmaddr != 0 && shmflg & SYSV_SHM_REMAP != 0 {
            let _ = aspace.unmap(VirtAddr::from(target), size);
        }
        aspace.map_linear(VirtAddr::from(target), paddr, size, map_flags)
    };
    if let Err(err) = map_result {
        return neg_errno(LinuxError::from(err));
    }
    if !increment_attach(shmid) {
        let _ = process.aspace.lock().unmap(VirtAddr::from(target), size);
        return neg_errno(LinuxError::EINVAL);
    }
    if let Some((old_shmid, _)) = process.shm_attachments.lock().insert(target, (shmid, size)) {
        decrement_attach(old_shmid, process.pid());
    }
    target as isize
}

pub(super) fn sys_shmdt(process: &UserProcess, tf: &TrapFrame, shmaddr: usize) -> isize {
    let Some((shmid, size)) = process.shm_attachments.lock().remove(&shmaddr) else {
        return neg_errno(LinuxError::EINVAL);
    };
    let ret = sys_munmap(process, tf, shmaddr, size);
    if ret == 0 {
        decrement_attach(shmid, process.pid());
    } else {
        process
            .shm_attachments
            .lock()
            .insert(shmaddr, (shmid, size));
    }
    ret
}

pub(super) fn sys_shmctl(process: &UserProcess, shmid: usize, cmd: usize, buf: usize) -> isize {
    let shmid = shmid as i32;
    let cmd = cmd as i32;
    match cmd {
        SYSV_IPC_INFO => {
            let info = UserShminfo64 {
                shmmax: configured_shm_max_size(),
                shmmin: 1,
                shmmni: SYSV_SHM_MAX_SEGMENTS,
                shmseg: SYSV_SHM_MAX_SEGMENTS,
                shmall: configured_shm_total_pages(),
                ..UserShminfo64::default()
            };
            let ret = write_user_value(process, buf, &info);
            if ret != 0 {
                return ret;
            }
            active_shm_max_id() as isize
        }
        SYSV_SHM_INFO => {
            let (max_id, used_ids, pages) = active_shm_snapshot();
            let info = UserShmInfo64 {
                used_ids,
                shm_tot: pages,
                shm_rss: pages,
                ..UserShmInfo64::default()
            };
            let ret = write_user_value(process, buf, &info);
            if ret != 0 {
                return ret;
            }
            max_id as isize
        }
        SYSV_IPC_RMID => {
            if !contains(shmid) {
                return neg_errno(LinuxError::EINVAL);
            }
            match control_allowed(process, shmid) {
                Ok(true) => {}
                Ok(false) => return neg_errno(LinuxError::EPERM),
                Err(err) => return neg_errno(err),
            }
            remove(shmid);
            0
        }
        SYSV_IPC_STAT => {
            if buf == 0 {
                return neg_errno(LinuxError::EFAULT);
            }
            let stat = match stat(process, shmid) {
                Ok(stat) => stat,
                Err(err) => return neg_errno(err),
            };
            let ret = write_user_value(process, buf, &stat);
            if ret != 0 {
                return ret;
            }
            0
        }
        SYSV_SHM_STAT => {
            if buf == 0 {
                return neg_errno(LinuxError::EFAULT);
            }
            let (id, stat) = match stat_by_index(process, shmid) {
                Ok(stat) => stat,
                Err(err) => return neg_errno(err),
            };
            let ret = write_user_value(process, buf, &stat);
            if ret != 0 {
                return ret;
            }
            id as isize
        }
        SYSV_SHM_STAT_ANY => {
            if buf == 0 {
                return neg_errno(LinuxError::EFAULT);
            }
            let (id, stat) = match stat_by_index_any(shmid) {
                Ok(stat) => stat,
                Err(err) => return neg_errno(err),
            };
            let ret = write_user_value(process, buf, &stat);
            if ret != 0 {
                return ret;
            }
            id as isize
        }
        SYSV_IPC_SET => {
            if let Err(err) = validate_user_read(process, buf, size_of::<UserShmidDs64>()) {
                return neg_errno(err);
            }
            match control_allowed(process, shmid) {
                Ok(true) => 0,
                Ok(false) => neg_errno(LinuxError::EPERM),
                Err(err) => neg_errno(err),
            }
        }
        SYSV_SHM_LOCK | SYSV_SHM_UNLOCK => match control_allowed(process, shmid) {
            Ok(true) => neg_errno(LinuxError::EPERM),
            Ok(false) => neg_errno(LinuxError::EPERM),
            Err(err) => neg_errno(err),
        },
        _ => neg_errno(LinuxError::EINVAL),
    }
}
