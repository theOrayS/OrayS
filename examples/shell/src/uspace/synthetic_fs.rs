use core::sync::atomic::Ordering;

use axerrno::LinuxError;
use axtask::AxTaskRef;
use linux_raw_sys::general;
use memory_addr::{VirtAddr, PAGE_SIZE_4K};
use std::string::{String, ToString};
use std::sync::Arc;
use std::vec::Vec;

use super::fd_table::{
    FdEntry, MemoryFileEntry, PathEntry, ProcPagemapEntry, ProcTimerSlackEntry, SyntheticDirEntry,
    SyntheticDirent,
};
use super::futex::futex_waiter_is_queued;
use super::linux_abi::{
    DEFAULT_GROUP_CONTENT, DEFAULT_PASSWD_CONTENT, ETC_GROUP_PATH, ETC_PASSWD_PATH,
    PROC_SELF_MAPS_PATH, USER_ASPACE_BASE, USER_STACK_SIZE, USER_STACK_TOP,
};
use super::memory_map::{align_down, align_up};
use super::runtime_paths::normalize_path;
use super::sysv_shm;
use super::task_context::task_ext;
use super::task_registry::{
    user_thread_entries_by_process_pid, user_thread_entry_by_process_pid, user_thread_entry_by_tid,
};
use super::UserProcess;

const PROC_SELF_PAGEMAP_PATH: &str = "/proc/self/pagemap";
const PROC_SELF_SMAPS_PATH: &str = "/proc/self/smaps";
const PROC_SELF_TIMERSLACK_PATH: &str = "/proc/self/timerslack_ns";
const SYNTHETIC_INIT_PID: i32 = 1;

fn proc_maps_perms(prot: u32, shared: bool) -> String {
    let mut perms = String::new();
    perms.push(if prot & general::PROT_READ != 0 {
        'r'
    } else {
        '-'
    });
    perms.push(if prot & general::PROT_WRITE != 0 {
        'w'
    } else {
        '-'
    });
    perms.push(if prot & general::PROT_EXEC != 0 {
        'x'
    } else {
        '-'
    });
    perms.push(if shared { 's' } else { 'p' });
    perms
}

fn proc_self_maps_content(process: &UserProcess) -> Vec<u8> {
    let exec_path = process.exec_path();
    let brk = *process.brk.lock();
    let text_start = USER_ASPACE_BASE;
    let text_end = text_start + PAGE_SIZE_4K;
    let heap_start = align_down(brk.start, PAGE_SIZE_4K);
    let heap_end = align_up(brk.end.max(brk.start + PAGE_SIZE_4K), PAGE_SIZE_4K);
    let stack_top = align_down(USER_STACK_TOP, PAGE_SIZE_4K);
    let stack_base = stack_top - USER_STACK_SIZE;
    let mut content = format!(
        "{text_start:08x}-{text_end:08x} r-xp 00000000 00:00 0 {exec_path}\n\
         {heap_start:08x}-{heap_end:08x} rw-p 00000000 00:00 0 [heap]\n\
         {stack_base:08x}-{stack_top:08x} rw-p 00000000 00:00 0 [stack]\n"
    );
    for region in process.mmap_regions() {
        content.push_str(&format!(
            "{:08x}-{:08x} {} 00000000 00:00 0\n",
            region.start,
            region.end(),
            proc_maps_perms(region.prot, region.shared)
        ));
    }
    content.into_bytes()
}

pub(super) fn is_proc_self_maps_path(path: &str) -> bool {
    normalize_path("/", path).as_deref() == Some(PROC_SELF_MAPS_PATH)
}

pub(super) fn synthetic_file_is_writable_open(flags: u32) -> bool {
    let access = flags & general::O_ACCMODE;
    access == general::O_WRONLY
        || access == general::O_RDWR
        || flags & (general::O_TRUNC | general::O_CREAT) != 0
}

pub(super) fn proc_self_maps_is_writable_open(flags: u32) -> bool {
    synthetic_file_is_writable_open(flags)
}

pub(super) fn proc_self_maps_fd_entry(process: &UserProcess) -> FdEntry {
    FdEntry::MemoryFile(MemoryFileEntry {
        path: PROC_SELF_MAPS_PATH.into(),
        data: Arc::new(proc_self_maps_content(process)),
        offset: 0,
    })
}

pub(super) fn proc_self_maps_path_entry(process: &UserProcess) -> FdEntry {
    let content_len = proc_self_maps_content(process).len();
    FdEntry::Path(PathEntry::synthetic_file(PROC_SELF_MAPS_PATH, content_len))
}

fn proc_smaps_content_for_target(target: &UserProcess) -> Vec<u8> {
    let mut regions = target.mmap_regions();
    regions.sort_by_key(|region| region.start);
    let aspace = target.aspace.lock();
    let mut content = String::new();
    for region in regions {
        let mut rss_pages = 0usize;
        let mut page = align_down(region.start, PAGE_SIZE_4K);
        let end = align_up(region.end(), PAGE_SIZE_4K);
        while page < end {
            if aspace.page_table().query(VirtAddr::from(page)).is_ok() {
                rss_pages = rss_pages.saturating_add(1);
            }
            match page.checked_add(PAGE_SIZE_4K) {
                Some(next) => page = next,
                None => break,
            }
        }
        let size_kb = region.size.div_ceil(1024);
        let rss_kb = rss_pages.saturating_mul(PAGE_SIZE_4K / 1024);
        let locked_kb = if region.locked { size_kb } else { 0 };
        let vm_flags = if region.locked { "lo" } else { "" };
        content.push_str(&format!(
            "{:08x}-{:08x} {} 00000000 00:00 0\n\
             Size:           {:>8} kB\n\
             Rss:            {:>8} kB\n\
             Pss:            {:>8} kB\n\
             Shared_Clean:   {:>8} kB\n\
             Shared_Dirty:   {:>8} kB\n\
             Private_Clean:  {:>8} kB\n\
             Private_Dirty:  {:>8} kB\n\
             Referenced:     {:>8} kB\n\
             Anonymous:      {:>8} kB\n\
             Locked:         {:>8} kB\n\
             VmFlags: {}\n",
            region.start,
            region.end(),
            proc_maps_perms(region.prot, region.shared),
            size_kb,
            rss_kb,
            rss_kb,
            0,
            0,
            if region.anonymous { 0 } else { rss_kb },
            if region.anonymous { rss_kb } else { 0 },
            rss_kb,
            if region.anonymous { rss_kb } else { 0 },
            locked_kb,
            vm_flags
        ));
    }
    content.into_bytes()
}

fn proc_smaps_content(process: &UserProcess, path: &str) -> Option<(String, Vec<u8>)> {
    let normalized = normalize_path("/", path)?;
    if normalized == PROC_SELF_SMAPS_PATH {
        return Some((normalized, proc_smaps_content_for_target(process)));
    }
    let rest = normalized.strip_prefix("/proc/")?;
    let pid_text = rest.strip_suffix("/smaps")?;
    let pid = pid_text.parse::<i32>().ok()?;
    if pid == process.pid() {
        return Some((normalized, proc_smaps_content_for_target(process)));
    }
    if let Some(entry) = process.child_thread_entry_by_pid(pid) {
        return Some((
            normalized,
            proc_smaps_content_for_target(entry.process.as_ref()),
        ));
    }
    let entry = user_thread_entry_by_process_pid(pid)?;
    Some((
        normalized,
        proc_smaps_content_for_target(entry.process.as_ref()),
    ))
}

pub(super) fn proc_smaps_fd_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, data) = proc_smaps_content(process, path)?;
    Some(FdEntry::MemoryFile(MemoryFileEntry {
        path,
        data: Arc::new(data),
        offset: 0,
    }))
}

pub(super) fn proc_smaps_path_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, data) = proc_smaps_content(process, path)?;
    Some(FdEntry::Path(PathEntry::synthetic_file(
        path.as_str(),
        data.len(),
    )))
}

fn proc_pagemap_target_path(process: &UserProcess, path: &str) -> Option<(String, i32)> {
    let normalized = normalize_path("/", path)?;
    if normalized == PROC_SELF_PAGEMAP_PATH {
        return Some((normalized, process.pid()));
    }
    let rest = normalized.strip_prefix("/proc/")?;
    let pid_text = rest.strip_suffix("/pagemap")?;
    pid_text.parse::<i32>().ok().map(|pid| (normalized, pid))
}

fn proc_pagemap_snapshot(target: &UserProcess, path: String) -> ProcPagemapEntry {
    let brk = *target.brk.lock();
    let text_start = USER_ASPACE_BASE;
    let text_end = text_start + PAGE_SIZE_4K;
    let heap_start = align_down(brk.start, PAGE_SIZE_4K);
    let heap_end = align_up(brk.end.max(brk.start + PAGE_SIZE_4K), PAGE_SIZE_4K);
    let stack_top = align_down(USER_STACK_TOP, PAGE_SIZE_4K);
    let stack_base = stack_top.saturating_sub(USER_STACK_SIZE);
    let mut ranges = Vec::new();
    ranges.push((text_start, text_end));
    ranges.push((heap_start, heap_end));
    ranges.push((stack_base, stack_top));
    for region in target.mmap_regions() {
        ranges.push((region.start, region.end()));
    }
    ranges.sort_by_key(|(start, _)| *start);

    let aspace = target.aspace.lock();
    let mut present_ranges = Vec::<(u64, u64)>::new();
    for (start, end) in ranges {
        let mut page = align_down(start, PAGE_SIZE_4K);
        let end = align_up(end.max(start), PAGE_SIZE_4K);
        while page < end {
            if aspace.page_table().query(VirtAddr::from(page)).is_ok() {
                push_present_pagemap_page(&mut present_ranges, (page / PAGE_SIZE_4K) as u64);
            }
            match page.checked_add(PAGE_SIZE_4K) {
                Some(next) => page = next,
                None => break,
            }
        }
    }
    let max_page = USER_STACK_TOP.div_ceil(PAGE_SIZE_4K) as u64;
    ProcPagemapEntry {
        path,
        present_ranges: Arc::new(present_ranges),
        offset: 0,
        size: max_page.saturating_mul(core::mem::size_of::<u64>() as u64),
    }
}

fn push_present_pagemap_page(ranges: &mut Vec<(u64, u64)>, page_index: u64) {
    if let Some((_, end)) = ranges.last_mut() {
        if *end == page_index {
            *end = end.saturating_add(1);
            return;
        }
        if *end > page_index {
            return;
        }
    }
    ranges.push((page_index, page_index.saturating_add(1)));
}

pub(super) fn proc_pagemap_fd_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, pid) = proc_pagemap_target_path(process, path)?;
    if pid == process.pid() {
        return Some(FdEntry::ProcPagemap(proc_pagemap_snapshot(process, path)));
    }
    user_thread_entry_by_process_pid(pid)
        .map(|entry| FdEntry::ProcPagemap(proc_pagemap_snapshot(entry.process.as_ref(), path)))
}

pub(super) fn proc_pagemap_path_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, pid) = proc_pagemap_target_path(process, path)?;
    if pid != process.pid() && user_thread_entry_by_process_pid(pid).is_none() {
        return None;
    }
    let max_page = USER_STACK_TOP.div_ceil(PAGE_SIZE_4K);
    Some(FdEntry::Path(PathEntry::synthetic_file(
        path.as_str(),
        max_page.saturating_mul(core::mem::size_of::<u64>()),
    )))
}

fn proc_timerslack_target(process: &UserProcess, path: &str) -> Option<(String, i32, u64)> {
    let normalized = normalize_path("/", path)?;
    let pid = if normalized == PROC_SELF_TIMERSLACK_PATH {
        process.pid()
    } else {
        let rest = normalized.strip_prefix("/proc/")?;
        let pid_text = rest.strip_suffix("/timerslack_ns")?;
        pid_text.parse::<i32>().ok()?
    };
    let timer_slack_ns = if pid == process.pid() {
        process.timer_slack_ns()
    } else {
        user_thread_entry_by_process_pid(pid)?
            .process
            .timer_slack_ns()
    };
    Some((normalized, pid, timer_slack_ns))
}

pub(super) fn proc_timerslack_fd_entry(
    process: &UserProcess,
    path: &str,
    status_flags: u32,
) -> Option<FdEntry> {
    let (path, pid, _) = proc_timerslack_target(process, path)?;
    Some(FdEntry::ProcTimerSlack(ProcTimerSlackEntry {
        path,
        target_pid: pid,
        offset: 0,
        status_flags,
    }))
}

pub(super) fn proc_timerslack_path_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, _, timer_slack_ns) = proc_timerslack_target(process, path)?;
    let size = format!("{timer_slack_ns}\n").len();
    Some(FdEntry::Path(PathEntry::synthetic_file_with_mode(
        path.as_str(),
        size,
        0o644,
    )))
}

fn proc_stat_target_process(process: &UserProcess, path: &str) -> Option<(i32, UserProcessStat)> {
    let normalized = normalize_path("/", path)?;
    if let Some(tid_text) = normalized
        .strip_prefix("/proc/self/task/")
        .and_then(|rest| rest.strip_suffix("/stat"))
    {
        if tid_text.contains('/') {
            return None;
        }
        let tid = tid_text.parse::<i32>().ok()?;
        return proc_task_thread_stat(process, process.pid(), tid);
    }
    if let Some(rest) = normalized.strip_prefix("/proc/") {
        let (pid_text, after_pid) = rest.split_once('/')?;
        if let Some(tid_text) = after_pid
            .strip_prefix("task/")
            .and_then(|rest| rest.strip_suffix("/stat"))
        {
            if tid_text.contains('/') {
                return None;
            }
            let pid = pid_text.parse::<i32>().ok()?;
            let tid = tid_text.parse::<i32>().ok()?;
            return proc_task_thread_stat(process, pid, tid);
        }
    }
    let pid = if normalized == "/proc/self/stat" {
        process.pid()
    } else {
        let rest = normalized.strip_prefix("/proc/")?;
        let pid_text = rest.strip_suffix("/stat")?;
        pid_text.parse::<i32>().ok()?
    };
    if pid == process.pid() {
        return Some((pid, UserProcessStat::from(process)));
    }
    if let Some(entry) = process.child_thread_entry_by_pid(pid) {
        return Some((pid, UserProcessStat::from(entry.process.as_ref())));
    }
    if pid == SYNTHETIC_INIT_PID {
        return Some((pid, UserProcessStat::synthetic_init()));
    }
    user_thread_entry_by_process_pid(pid)
        .map(|entry| (pid, UserProcessStat::from(entry.process.as_ref())))
}

fn proc_task_thread_stat(
    process: &UserProcess,
    pid: i32,
    tid: i32,
) -> Option<(i32, UserProcessStat)> {
    if !live_process_exists(process, pid) {
        return None;
    }
    let entry = user_thread_entry_by_tid(tid)?;
    (entry.process.pid() == pid).then(|| {
        (
            tid,
            UserProcessStat::from_task(entry.process.as_ref(), &entry.task),
        )
    })
}

struct UserProcessStat {
    ppid: i32,
    pgid: i32,
    sid: i32,
    state: char,
    comm: String,
    locked_mmap_kb: usize,
}

impl UserProcessStat {
    fn from(process: &UserProcess) -> Self {
        Self::with_state(process, process_state(process))
    }

    fn from_task(process: &UserProcess, task: &AxTaskRef) -> Self {
        Self::with_state(process, task_state(process, task))
    }

    fn with_state(process: &UserProcess, state: char) -> Self {
        let exec_path = process.exec_path();
        let comm = exec_path
            .rsplit('/')
            .next()
            .filter(|name| !name.is_empty())
            .unwrap_or("user")
            .chars()
            .take(15)
            .collect();
        Self {
            ppid: process.ppid(),
            pgid: process.pgid(),
            sid: process.sid(),
            state,
            comm,
            locked_mmap_kb: process.locked_mmap_kb(),
        }
    }

    fn synthetic_init() -> Self {
        Self {
            ppid: 0,
            pgid: SYNTHETIC_INIT_PID,
            sid: SYNTHETIC_INIT_PID,
            state: 'S',
            comm: "init".into(),
            locked_mmap_kb: 0,
        }
    }
}

fn process_state(process: &UserProcess) -> char {
    if process.live_threads.load(Ordering::Acquire) == 0 {
        'Z'
    } else if process.is_child_wait_blocked()
        || process.is_syscall_wait_blocked()
        || process_has_futex_waiter(process)
        || process_has_signal_waiter(process)
    {
        'S'
    } else {
        'R'
    }
}

fn task_state(process: &UserProcess, task: &AxTaskRef) -> char {
    if process.live_threads.load(Ordering::Acquire) == 0 {
        'Z'
    } else if task_has_futex_waiter(process, task) || task_has_signal_waiter(task) {
        'S'
    } else {
        'R'
    }
}

fn task_has_futex_waiter(process: &UserProcess, task: &AxTaskRef) -> bool {
    let Some(ext) = task_ext(task) else {
        return false;
    };
    let uaddr = ext.futex_wait.load(Ordering::Acquire);
    uaddr != 0 && futex_waiter_is_queued(process, uaddr)
}

fn task_has_signal_waiter(task: &AxTaskRef) -> bool {
    task_ext(task)
        .map(|ext| ext.signal_wait.load(Ordering::Acquire) || ext.poll_wait.load(Ordering::Acquire))
        .unwrap_or(false)
}

fn process_has_futex_waiter(process: &UserProcess) -> bool {
    user_thread_entries_by_process_pid(process.pid())
        .into_iter()
        .any(|entry| task_has_futex_waiter(process, &entry.task))
}

fn process_has_signal_waiter(process: &UserProcess) -> bool {
    user_thread_entries_by_process_pid(process.pid())
        .into_iter()
        .any(|entry| task_has_signal_waiter(&entry.task))
}

fn proc_pid_stat_content(process: &UserProcess, path: &str) -> Option<(String, Vec<u8>)> {
    let normalized = normalize_path("/", path)?;
    let (pid, stat) = proc_stat_target_process(process, normalized.as_str())?;
    let content = format!(
        "{pid} ({}) {} {} {} {} 0 -1 0 0 0 0 0 0 0 0 20 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n",
        stat.comm, stat.state, stat.ppid, stat.pgid, stat.sid
    );
    Some((normalized, content.into_bytes()))
}

pub(super) fn proc_pid_stat_fd_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, data) = proc_pid_stat_content(process, path)?;
    Some(FdEntry::MemoryFile(MemoryFileEntry {
        path,
        data: Arc::new(data),
        offset: 0,
    }))
}

pub(super) fn proc_pid_stat_path_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, data) = proc_pid_stat_content(process, path)?;
    Some(FdEntry::Path(PathEntry::synthetic_file(
        path.as_str(),
        data.len(),
    )))
}

fn proc_pid_status_content(process: &UserProcess, path: &str) -> Option<(String, Vec<u8>)> {
    let normalized = normalize_path("/", path)?;
    let (pid, stat) = if normalized == "/proc/self/status" {
        (process.pid(), UserProcessStat::from(process))
    } else {
        let rest = normalized.strip_prefix("/proc/")?;
        let pid_text = rest.strip_suffix("/status")?;
        let pid = pid_text.parse::<i32>().ok()?;
        if pid == process.pid() {
            (pid, UserProcessStat::from(process))
        } else if pid == SYNTHETIC_INIT_PID {
            (pid, UserProcessStat::synthetic_init())
        } else if let Some(entry) = process.child_thread_entry_by_pid(pid) {
            (pid, UserProcessStat::from(entry.process.as_ref()))
        } else {
            let entry = user_thread_entry_by_process_pid(pid)?;
            (pid, UserProcessStat::from(entry.process.as_ref()))
        }
    };
    let groups = process
        .groups()
        .into_iter()
        .map(|gid| gid.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    let groups = if groups.is_empty() {
        process.gid().to_string()
    } else {
        groups
    };
    let content = format!(
        "Name:\t{}\n\
         State:\t{} ({})\n\
         Tgid:\t{pid}\n\
         Pid:\t{pid}\n\
         PPid:\t{}\n\
         VmLck:\t{} kB\n\
         Uid:\t{}\t{}\t{}\t{}\n\
         Gid:\t{}\t{}\t{}\t{}\n\
         Groups:\t{groups}\n",
        stat.comm,
        stat.state,
        stat.state,
        stat.ppid,
        stat.locked_mmap_kb,
        process.real_uid(),
        process.uid(),
        process.saved_uid(),
        process.fs_uid(),
        process.real_gid(),
        process.gid(),
        process.saved_gid(),
        process.fs_gid(),
    );
    Some((normalized, content.into_bytes()))
}

pub(super) fn proc_pid_status_fd_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, data) = proc_pid_status_content(process, path)?;
    Some(FdEntry::MemoryFile(MemoryFileEntry {
        path,
        data: Arc::new(data),
        offset: 0,
    }))
}

pub(super) fn proc_pid_status_path_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, data) = proc_pid_status_content(process, path)?;
    Some(FdEntry::Path(PathEntry::synthetic_file(
        path.as_str(),
        data.len(),
    )))
}

fn live_process_exists(process: &UserProcess, pid: i32) -> bool {
    pid == process.pid() || user_thread_entry_by_process_pid(pid).is_some()
}

fn thread_belongs_to_process(pid: i32, tid: i32) -> bool {
    user_thread_entry_by_tid(tid)
        .map(|entry| entry.process.pid() == pid)
        .unwrap_or(false)
}

fn parent_path(path: &str) -> String {
    match path.rsplit_once('/') {
        Some(("", _)) | None => "/".into(),
        Some((parent, _)) => parent.into(),
    }
}

fn proc_task_dir_snapshot(
    process: &UserProcess,
    path: &str,
) -> Option<(String, String, Vec<SyntheticDirent>)> {
    let normalized = normalize_path("/", path)?;
    if normalized == "/proc/self/task" {
        return proc_task_list_dir_snapshot(process, normalized, process.pid());
    }
    if let Some(tid_text) = normalized.strip_prefix("/proc/self/task/") {
        if tid_text.contains('/') {
            return None;
        }
        let tid = tid_text.parse::<i32>().ok()?;
        return proc_task_thread_dir_snapshot(process, normalized, process.pid(), tid);
    }

    let rest = normalized.strip_prefix("/proc/")?;
    let (pid_text, after_pid) = rest.split_once('/')?;
    let pid = pid_text.parse::<i32>().ok()?;
    if after_pid == "task" {
        return proc_task_list_dir_snapshot(process, normalized, pid);
    }
    let tid_text = after_pid.strip_prefix("task/")?;
    if tid_text.contains('/') {
        return None;
    }
    let tid = tid_text.parse::<i32>().ok()?;
    proc_task_thread_dir_snapshot(process, normalized, pid, tid)
}

fn proc_task_list_dir_snapshot(
    process: &UserProcess,
    path: String,
    pid: i32,
) -> Option<(String, String, Vec<SyntheticDirent>)> {
    if !live_process_exists(process, pid) {
        return None;
    }
    let mut tids = user_thread_entries_by_process_pid(pid)
        .into_iter()
        .map(|entry| entry.task.id().as_u64() as i32)
        .collect::<Vec<_>>();
    tids.sort_unstable();
    tids.dedup();
    let entries = tids
        .into_iter()
        .map(|tid| {
            let name = tid.to_string();
            SyntheticDirent::new(
                name,
                general::DT_DIR as u8,
                format!("{}/{}", path.as_str(), tid),
            )
        })
        .collect();
    Some((path.clone(), parent_path(path.as_str()), entries))
}

fn proc_task_thread_dir_snapshot(
    process: &UserProcess,
    path: String,
    pid: i32,
    tid: i32,
) -> Option<(String, String, Vec<SyntheticDirent>)> {
    if !live_process_exists(process, pid) || !thread_belongs_to_process(pid, tid) {
        return None;
    }
    let entries = vec![SyntheticDirent::new(
        "comm".into(),
        general::DT_REG as u8,
        format!("{}/comm", path.as_str()),
    )];
    Some((path.clone(), parent_path(path.as_str()), entries))
}

pub(super) fn proc_task_dir_fd_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, parent, entries) = proc_task_dir_snapshot(process, path)?;
    Some(FdEntry::SyntheticDir(SyntheticDirEntry::new(
        path, parent, entries,
    )))
}

pub(super) fn proc_task_dir_path_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, _, _) = proc_task_dir_snapshot(process, path)?;
    Some(FdEntry::Path(PathEntry::synthetic_dir(path.as_str())))
}

fn proc_comm_target_process(process: &UserProcess, path: &str) -> Option<(String, Vec<u8>)> {
    let normalized = normalize_path("/", path)?;
    let process_comm_content = |target: &UserProcess| {
        let mut content = target.prctl_name();
        content.push('\n');
        Some((normalized.clone(), content.into_bytes()))
    };
    if normalized == "/proc/self/comm" {
        return process_comm_content(process);
    }
    let task_rest = if let Some(rest) = normalized.strip_prefix("/proc/self/task/") {
        Some((rest, process.pid()))
    } else if let Some(rest) = normalized.strip_prefix("/proc/") {
        if let Some((pid_text, rest)) = rest.split_once("/task/") {
            let pid = pid_text.parse::<i32>().ok()?;
            if !live_process_exists(process, pid) {
                return None;
            }
            Some((rest, pid))
        } else {
            None
        }
    } else {
        None
    };
    if let Some((rest, task_pid)) = task_rest {
        let tid_text = rest.strip_suffix("/comm")?;
        let tid = tid_text.parse::<i32>().ok()?;
        let target_name = if task_pid == process.pid() && tid == process.pid() {
            Some(process.prctl_name())
        } else if let Some(entry) = user_thread_entry_by_tid(tid) {
            (entry.process.pid() == task_pid).then(|| entry.process.prctl_name())
        } else {
            None
        }?;
        let mut content = target_name;
        content.push('\n');
        return Some((normalized, content.into_bytes()));
    }
    if let Some(rest) = normalized.strip_prefix("/proc/") {
        if let Some(pid_text) = rest.strip_suffix("/comm") {
            let pid = pid_text.parse::<i32>().ok()?;
            if pid == process.pid() {
                return process_comm_content(process);
            }
            let entry = user_thread_entry_by_process_pid(pid)?;
            return process_comm_content(entry.process.as_ref());
        }
    }
    None
}

pub(super) fn proc_comm_fd_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, data) = proc_comm_target_process(process, path)?;
    Some(FdEntry::MemoryFile(MemoryFileEntry {
        path,
        data: Arc::new(data),
        offset: 0,
    }))
}

pub(super) fn proc_comm_path_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, data) = proc_comm_target_process(process, path)?;
    Some(FdEntry::Path(PathEntry::synthetic_file(
        path.as_str(),
        data.len(),
    )))
}

pub(super) fn proc_exe_link_target(process: &UserProcess, path: &str) -> Option<String> {
    let pid_path = format!("/proc/{}/exe", process.pid());
    (path == "/proc/self/exe" || path == pid_path).then(|| process.exec_path())
}

pub(super) fn synthetic_userdb_content(path: &str) -> Option<(&'static str, &'static [u8])> {
    match normalize_path("/", path).as_deref() {
        Some(ETC_PASSWD_PATH) => Some((ETC_PASSWD_PATH, DEFAULT_PASSWD_CONTENT)),
        Some(ETC_GROUP_PATH) => Some((ETC_GROUP_PATH, DEFAULT_GROUP_CONTENT)),
        _ => None,
    }
}

pub(super) fn synthetic_userdb_fd_entry(path: &'static str, data: &'static [u8]) -> FdEntry {
    FdEntry::MemoryFile(MemoryFileEntry {
        path: path.into(),
        data: Arc::new(data.to_vec()),
        offset: 0,
    })
}

pub(super) fn synthetic_userdb_path_entry(path: &'static str, data: &'static [u8]) -> FdEntry {
    FdEntry::Path(PathEntry::synthetic_file(path, data.len()))
}

const KERNEL_CONFIG_BOOT_PATH: &str = "/boot/config-6.0.0";
const KERNEL_CONFIG_MODULE_PATH: &str = "/lib/modules/6.0.0/config";
const KERNEL_CONFIG_MODULE_BUILD_PATH: &str = "/lib/modules/6.0.0/build/.config";
const KERNEL_CONFIG_LIB_PATH: &str = "/lib/kernel/config-6.0.0";

const SYNTHETIC_KERNEL_CONFIG_CONTENT: &[u8] = b"\
# ArceOS synthetic kernel config for LTP feature probes.
# Keep these entries aligned with implemented kernel/user ABI support.
CONFIG_EVENTFD=y
";

pub(super) fn synthetic_kernel_config_content(path: &str) -> Option<(&'static str, &'static [u8])> {
    match normalize_path("/", path).as_deref() {
        Some(KERNEL_CONFIG_BOOT_PATH) => {
            Some((KERNEL_CONFIG_BOOT_PATH, SYNTHETIC_KERNEL_CONFIG_CONTENT))
        }
        Some(KERNEL_CONFIG_MODULE_PATH) => {
            Some((KERNEL_CONFIG_MODULE_PATH, SYNTHETIC_KERNEL_CONFIG_CONTENT))
        }
        Some(KERNEL_CONFIG_MODULE_BUILD_PATH) => Some((
            KERNEL_CONFIG_MODULE_BUILD_PATH,
            SYNTHETIC_KERNEL_CONFIG_CONTENT,
        )),
        Some(KERNEL_CONFIG_LIB_PATH) => {
            Some((KERNEL_CONFIG_LIB_PATH, SYNTHETIC_KERNEL_CONFIG_CONTENT))
        }
        _ => None,
    }
}

pub(super) fn synthetic_kernel_config_fd_entry(path: &'static str, data: &'static [u8]) -> FdEntry {
    FdEntry::MemoryFile(MemoryFileEntry {
        path: path.into(),
        data: Arc::new(data.to_vec()),
        offset: 0,
    })
}

pub(super) fn synthetic_kernel_config_path_entry(
    path: &'static str,
    data: &'static [u8],
) -> FdEntry {
    FdEntry::Path(PathEntry::synthetic_file(path, data.len()))
}

const PROC_SYSVIPC_SHM_PATH: &str = "/proc/sysvipc/shm";

pub(super) fn proc_sysvipc_shm_fd_entry(path: &str) -> Option<FdEntry> {
    let normalized = normalize_path("/", path)?;
    if normalized != PROC_SYSVIPC_SHM_PATH {
        return None;
    }
    Some(FdEntry::MemoryFile(MemoryFileEntry {
        path: normalized,
        data: Arc::new(sysv_shm::proc_sysvipc_shm_content()),
        offset: 0,
    }))
}

pub(super) fn proc_sysvipc_shm_path_entry(path: &str) -> Option<FdEntry> {
    let normalized = normalize_path("/", path)?;
    if normalized != PROC_SYSVIPC_SHM_PATH {
        return None;
    }
    let size = sysv_shm::proc_sysvipc_shm_content().len();
    Some(FdEntry::Path(PathEntry::synthetic_file(
        normalized.as_str(),
        size,
    )))
}

const PROC_SYS_KERNEL_CORE_PATTERN_PATH: &str = "/proc/sys/kernel/core_pattern";
const PROC_SYS_KERNEL_CORE_PATTERN_CONTENT: &[u8] = b"core\n";

pub(super) fn synthetic_proc_sys_content(path: &str) -> Option<(&'static str, &'static [u8])> {
    match normalize_path("/", path).as_deref() {
        Some(PROC_SYS_KERNEL_CORE_PATTERN_PATH) => Some((
            PROC_SYS_KERNEL_CORE_PATTERN_PATH,
            PROC_SYS_KERNEL_CORE_PATTERN_CONTENT,
        )),
        _ => None,
    }
}

pub(super) fn synthetic_proc_sys_fd_entry(path: &'static str, data: &'static [u8]) -> FdEntry {
    FdEntry::MemoryFile(MemoryFileEntry {
        path: path.into(),
        data: Arc::new(data.to_vec()),
        offset: 0,
    })
}

pub(super) fn synthetic_proc_sys_path_entry(path: &'static str, data: &'static [u8]) -> FdEntry {
    FdEntry::Path(PathEntry::synthetic_file(path, data.len()))
}

pub(super) fn dev_shm_host_path(path: &str) -> Option<String> {
    let normalized = normalize_path("/", path)?;
    let rel = normalized.strip_prefix("/dev/shm/")?;
    if rel.is_empty() {
        return None;
    }
    Some(format!("/tmp/shm/{rel}"))
}

pub(super) fn ensure_dev_shm_dir() -> Result<(), LinuxError> {
    ensure_host_dir("/tmp")?;
    ensure_host_dir("/tmp/shm")
}

fn ensure_host_dir(path: &str) -> Result<(), LinuxError> {
    if axfs::api::metadata(path).is_ok() {
        return Ok(());
    }
    axfs::api::create_dir(path).map_err(LinuxError::from)
}
