use core::sync::atomic::{AtomicUsize, Ordering};

use axalloc::global_allocator;
use axerrno::LinuxError;
use axtask::AxTaskRef;
use lazyinit::LazyInit;
use linux_raw_sys::general;
use memory_addr::{PAGE_SIZE_4K, VirtAddr};
use std::string::{String, ToString};
use std::sync::{Arc, Mutex};
use std::vec::Vec;

use super::UserProcess;
use super::fd_table::{
    FdEntry, MemoryFileEntry, PathEntry, ProcPagemapEntry, ProcTimerSlackEntry, SyntheticDirEntry,
    SyntheticDirent,
};
use super::futex::futex_waiter_is_queued;
use super::linux_abi::{
    DEFAULT_GROUP_CONTENT, DEFAULT_PASSWD_CONTENT, ETC_GROUP_PATH, ETC_PASSWD_PATH,
    PROC_SELF_MAPS_PATH, SYSV_SHM_MAX_SEGMENTS, SYSV_SHM_MAX_SIZE, USER_ASPACE_BASE,
    USER_STACK_SIZE, USER_STACK_TOP,
};
use super::memory_map::{align_down, align_up};
use super::runtime_paths::normalize_path;
use super::sysv_msg;
use super::sysv_sem;
use super::sysv_shm;
use super::task_context::task_ext;
use super::task_registry::{
    user_thread_entries_by_process_pid, user_thread_entry_by_process_pid, user_thread_entry_by_tid,
};

const PROC_SELF_PAGEMAP_PATH: &str = "/proc/self/pagemap";
const PROC_SELF_SMAPS_PATH: &str = "/proc/self/smaps";
const PROC_SELF_STATM_PATH: &str = "/proc/self/statm";
const PROC_SELF_TIMERSLACK_PATH: &str = "/proc/self/timerslack_ns";
const PROC_CMDLINE_PATH: &str = "/proc/cmdline";
const PROC_VERSION_PATH: &str = "/proc/version";
const PROC_MEMINFO_PATH: &str = "/proc/meminfo";
const PROC_UPTIME_PATH: &str = "/proc/uptime";
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
    let heap_end = align_up(
        brk.end.max(brk.start.saturating_add(PAGE_SIZE_4K)),
        PAGE_SIZE_4K,
    );
    let stack_top = align_down(USER_STACK_TOP, PAGE_SIZE_4K);
    let stack_base = stack_top - USER_STACK_SIZE;
    let regions = process.mmap_regions();
    let has_loaded_image_maps = regions
        .iter()
        .any(|region| region.start < heap_start && region.prot & general::PROT_EXEC != 0);
    let mut lines = Vec::<(usize, String)>::new();
    if !has_loaded_image_maps {
        lines.push((
            text_start,
            format!("{text_start:08x}-{text_end:08x} r-xp 00000000 00:00 0 {exec_path}\n"),
        ));
    }
    lines.push((
        heap_start,
        format!("{heap_start:08x}-{heap_end:08x} rw-p 00000000 00:00 0 [heap]\n"),
    ));
    lines.push((
        stack_base,
        format!("{stack_base:08x}-{stack_top:08x} rw-p 00000000 00:00 0 [stack]\n"),
    ));
    for region in regions {
        let path = if region.start < heap_start {
            exec_path.as_str()
        } else {
            ""
        };
        let suffix = if path.is_empty() {
            "\n".to_string()
        } else {
            format!(" {path}\n")
        };
        lines.push((
            region.start,
            format!(
                "{:08x}-{:08x} {} 00000000 00:00 0{}",
                region.start,
                region.end(),
                proc_maps_perms(region.prot, region.shared),
                suffix
            ),
        ));
    }
    lines.sort_by_key(|(start, _)| *start);
    let content = lines
        .into_iter()
        .map(|(_, line)| line)
        .collect::<Vec<_>>()
        .concat();
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

fn merged_page_ranges(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    for (start, end) in ranges.iter_mut() {
        *start = align_down(*start, PAGE_SIZE_4K);
        *end = align_up(*end, PAGE_SIZE_4K);
    }
    ranges.retain(|(start, end)| start < end);
    ranges.sort_by_key(|(start, _)| *start);

    let mut merged = Vec::<(usize, usize)>::new();
    for (start, end) in ranges {
        if let Some((_, previous_end)) = merged.last_mut()
            && start <= *previous_end
        {
            *previous_end = (*previous_end).max(end);
            continue;
        }
        merged.push((start, end));
    }
    merged
}

fn page_range_count(ranges: &[(usize, usize)]) -> usize {
    ranges.iter().fold(0_usize, |pages, (start, end)| {
        pages.saturating_add(end.saturating_sub(*start) / PAGE_SIZE_4K)
    })
}

fn proc_statm_content_for_target(target: &UserProcess) -> Vec<u8> {
    let regions = target.mmap_regions();
    let brk = *target.brk.lock();
    let heap_start = align_down(brk.start, PAGE_SIZE_4K);
    let heap_end = align_up(brk.end.max(brk.start + PAGE_SIZE_4K), PAGE_SIZE_4K);
    let stack_top = align_down(USER_STACK_TOP, PAGE_SIZE_4K);
    let stack_base = stack_top.saturating_sub(USER_STACK_SIZE);

    let mut visible_ranges = regions
        .iter()
        .map(|region| (region.start, region.end()))
        .collect::<Vec<_>>();
    visible_ranges.push((heap_start, heap_end));
    visible_ranges.push((stack_base, stack_top));
    let visible_ranges = merged_page_ranges(visible_ranges);

    let text_ranges = merged_page_ranges(
        regions
            .iter()
            .filter(|region| region.prot & general::PROT_EXEC != 0)
            .map(|region| (region.start, region.end()))
            .collect(),
    );
    let mut data_ranges = regions
        .iter()
        .filter(|region| region.prot & general::PROT_WRITE != 0)
        .map(|region| (region.start, region.end()))
        .collect::<Vec<_>>();
    data_ranges.push((heap_start, heap_end));
    data_ranges.push((stack_base, stack_top));
    let data_ranges = merged_page_ranges(data_ranges);
    let shared_ranges = merged_page_ranges(
        regions
            .iter()
            .filter(|region| region.shared)
            .map(|region| (region.start, region.end()))
            .collect(),
    );

    let aspace = target.aspace.lock();
    let resident_page_count = |ranges: &[(usize, usize)]| {
        let mut resident = 0_usize;
        for (start, end) in ranges {
            let mut page = *start;
            while page < *end {
                if aspace.page_table().query(VirtAddr::from(page)).is_ok() {
                    resident = resident.saturating_add(1);
                }
                let Some(next_page) = page.checked_add(PAGE_SIZE_4K) else {
                    break;
                };
                page = next_page;
            }
        }
        resident
    };
    let resident_pages = resident_page_count(visible_ranges.as_slice());
    let shared_pages = resident_page_count(shared_ranges.as_slice());

    format!(
        "{} {} {} {} 0 {} 0\n",
        page_range_count(visible_ranges.as_slice()),
        resident_pages,
        shared_pages,
        page_range_count(text_ranges.as_slice()),
        page_range_count(data_ranges.as_slice()),
    )
    .into_bytes()
}

fn proc_statm_content(process: &UserProcess, path: &str) -> Option<(String, Vec<u8>)> {
    let normalized = normalize_path("/", path)?;
    let pid = if normalized == PROC_SELF_STATM_PATH {
        process.pid()
    } else {
        let rest = normalized.strip_prefix("/proc/")?;
        let pid_text = rest.strip_suffix("/statm")?;
        pid_text.parse::<i32>().ok()?
    };
    if pid == process.pid() {
        return Some((normalized, proc_statm_content_for_target(process)));
    }
    if let Some(entry) = process.child_thread_entry_by_pid(pid) {
        return Some((
            normalized,
            proc_statm_content_for_target(entry.process.as_ref()),
        ));
    }
    let entry = user_thread_entry_by_process_pid(pid)?;
    Some((
        normalized,
        proc_statm_content_for_target(entry.process.as_ref()),
    ))
}

pub(super) fn proc_statm_fd_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, data) = proc_statm_content(process, path)?;
    Some(FdEntry::MemoryFile(MemoryFileEntry {
        path,
        data: Arc::new(data),
        offset: 0,
    }))
}

pub(super) fn proc_statm_path_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, data) = proc_statm_content(process, path)?;
    Some(FdEntry::Path(PathEntry::synthetic_file(
        path.as_str(),
        data.len(),
    )))
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
    vm_peak_kb: usize,
    vm_rss_kb: usize,
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
            vm_peak_kb: 0,
            vm_rss_kb: 0,
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
            vm_peak_kb: 0,
            vm_rss_kb: 0,
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
         VmPeak:\t{} kB\n\
         VmSize:\t{} kB\n\
         VmHWM:\t{} kB\n\
         VmRSS:\t{} kB\n\
         VmData:\t{} kB\n\
         VmStk:\t0 kB\n\
         VmExe:\t0 kB\n\
         VmLib:\t0 kB\n\
         VmPTE:\t0 kB\n\
         VmLck:\t{} kB\n\
         VmSwap:\t0 kB\n\
         Uid:\t{}\t{}\t{}\t{}\n\
         Gid:\t{}\t{}\t{}\t{}\n\
         Groups:\t{groups}\n",
        stat.comm,
        stat.state,
        stat.state,
        stat.ppid,
        stat.vm_peak_kb,
        stat.vm_rss_kb,
        stat.vm_peak_kb,
        stat.vm_rss_kb,
        stat.vm_rss_kb,
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
    let process_comm = |target: &UserProcess| {
        user_thread_entry_by_tid(target.pid())
            .filter(|entry| entry.process.pid() == target.pid())
            .and_then(|entry| task_ext(&entry.task).map(|ext| ext.comm()))
            .unwrap_or_else(|| target.prctl_name())
    };
    let process_comm_content = |target: &UserProcess| {
        let mut content = process_comm(target);
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
        let target_name = user_thread_entry_by_tid(tid)
            .filter(|entry| entry.process.pid() == task_pid)
            .and_then(|entry| task_ext(&entry.task).map(|ext| ext.comm()))
            .or_else(|| {
                (task_pid == process.pid() && tid == process.pid()).then(|| process.prctl_name())
            })?;
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

const SYNTHETIC_PROC_VERSION_CONTENT: &[u8] =
    b"Linux version 6.0.0 (ArceOS compatibility kernel) #1 SMP PREEMPT\n";
const SYNTHETIC_PROC_CMDLINE_CONTENT: &[u8] = b"root=/dev/vda rw console=ttyS0\n";

pub(super) fn synthetic_proc_version_content(path: &str) -> Option<(&'static str, &'static [u8])> {
    match normalize_path("/", path).as_deref() {
        Some(PROC_VERSION_PATH) => Some((PROC_VERSION_PATH, SYNTHETIC_PROC_VERSION_CONTENT)),
        Some(PROC_CMDLINE_PATH) => Some((PROC_CMDLINE_PATH, SYNTHETIC_PROC_CMDLINE_CONTENT)),
        _ => None,
    }
}

pub(super) fn synthetic_proc_version_fd_entry(path: &'static str, data: &'static [u8]) -> FdEntry {
    FdEntry::MemoryFile(MemoryFileEntry {
        path: path.into(),
        data: Arc::new(data.to_vec()),
        offset: 0,
    })
}

pub(super) fn synthetic_proc_version_path_entry(
    path: &'static str,
    data: &'static [u8],
) -> FdEntry {
    FdEntry::Path(PathEntry::synthetic_file(path, data.len()))
}

fn proc_meminfo_content() -> Vec<u8> {
    let alloc = global_allocator();
    let free_pages = alloc.available_pages();
    let used_pages = alloc.used_pages();
    let total_pages = used_pages.saturating_add(free_pages).max(1);
    let total_kb = total_pages.saturating_mul(PAGE_SIZE_4K / 1024);
    let free_kb = free_pages.saturating_mul(PAGE_SIZE_4K / 1024);
    let used_kb = used_pages.saturating_mul(PAGE_SIZE_4K / 1024);
    let ratio = PROC_SYS_VM_OVERCOMMIT_RATIO.load(Ordering::Acquire).max(1);
    let commit_limit_kb = total_kb.saturating_mul(ratio) / 100;
    let committed_as_kb = used_kb.min(commit_limit_kb / 2);

    format!(
        "MemTotal:       {total_kb:>8} kB\n\
         MemFree:        {free_kb:>8} kB\n\
         MemAvailable:   {free_kb:>8} kB\n\
         Buffers:               0 kB\n\
         Cached:                0 kB\n\
         SwapCached:            0 kB\n\
         Active:                0 kB\n\
         Inactive:              0 kB\n\
         SwapTotal:             0 kB\n\
         SwapFree:              0 kB\n\
         Dirty:                 0 kB\n\
         Writeback:             0 kB\n\
         AnonPages:      {used_kb:>8} kB\n\
         Mapped:                0 kB\n\
         Shmem:                 0 kB\n\
         KReclaimable:          0 kB\n\
         Slab:                  0 kB\n\
         SReclaimable:          0 kB\n\
         SUnreclaim:            0 kB\n\
         PageTables:            0 kB\n\
         CommitLimit:    {commit_limit_kb:>8} kB\n\
         Committed_AS:   {committed_as_kb:>8} kB\n\
         VmallocTotal:   34359738367 kB\n\
         VmallocUsed:           0 kB\n\
         VmallocChunk:          0 kB\n"
    )
    .into_bytes()
}

pub(super) fn proc_meminfo_fd_entry(path: &str) -> Option<FdEntry> {
    (normalize_path("/", path).as_deref() == Some(PROC_MEMINFO_PATH)).then(|| {
        FdEntry::MemoryFile(MemoryFileEntry {
            path: PROC_MEMINFO_PATH.into(),
            data: Arc::new(proc_meminfo_content()),
            offset: 0,
        })
    })
}

pub(super) fn proc_meminfo_path_entry(path: &str) -> Option<FdEntry> {
    (normalize_path("/", path).as_deref() == Some(PROC_MEMINFO_PATH)).then(|| {
        FdEntry::Path(PathEntry::synthetic_file(
            PROC_MEMINFO_PATH,
            proc_meminfo_content().len(),
        ))
    })
}

fn proc_uptime_content() -> Vec<u8> {
    const NANOS_PER_CENTISECOND: u64 = 10_000_000;

    let uptime_centiseconds = axhal::time::monotonic_time()
        .as_nanos()
        .saturating_div(u128::from(NANOS_PER_CENTISECOND));
    let idle_centiseconds = axhal::time::ticks_to_nanos(axtask::idle_runtime_ticks())
        .saturating_div(NANOS_PER_CENTISECOND);

    format!(
        "{}.{:02} {}.{:02}\n",
        uptime_centiseconds / 100,
        uptime_centiseconds % 100,
        idle_centiseconds / 100,
        idle_centiseconds % 100,
    )
    .into_bytes()
}

pub(super) fn proc_uptime_fd_entry(path: &str) -> Option<FdEntry> {
    (normalize_path("/", path).as_deref() == Some(PROC_UPTIME_PATH)).then(|| {
        FdEntry::MemoryFile(MemoryFileEntry {
            path: PROC_UPTIME_PATH.into(),
            data: Arc::new(proc_uptime_content()),
            offset: 0,
        })
    })
}

pub(super) fn proc_uptime_path_entry(path: &str) -> Option<FdEntry> {
    (normalize_path("/", path).as_deref() == Some(PROC_UPTIME_PATH)).then(|| {
        FdEntry::Path(PathEntry::synthetic_file(
            PROC_UPTIME_PATH,
            proc_uptime_content().len(),
        ))
    })
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
# ArceOS synthetic kernel config for implemented Linux ABI surfaces.
# Entries below are exposed only when backed by syscall/file-descriptor code.
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

const PROC_SYSVIPC_SEM_PATH: &str = "/proc/sysvipc/sem";
const PROC_SYSVIPC_SHM_PATH: &str = "/proc/sysvipc/shm";
const PROC_SYSVIPC_MSG_PATH: &str = "/proc/sysvipc/msg";

pub(super) fn proc_sysvipc_msg_fd_entry(path: &str) -> Option<FdEntry> {
    let normalized = normalize_path("/", path)?;
    if normalized != PROC_SYSVIPC_MSG_PATH {
        return None;
    }
    Some(FdEntry::MemoryFile(MemoryFileEntry {
        path: normalized,
        data: Arc::new(sysv_msg::proc_sysvipc_msg_content()),
        offset: 0,
    }))
}

pub(super) fn proc_sysvipc_msg_path_entry(path: &str) -> Option<FdEntry> {
    let normalized = normalize_path("/", path)?;
    if normalized != PROC_SYSVIPC_MSG_PATH {
        return None;
    }
    let size = sysv_msg::proc_sysvipc_msg_content().len();
    Some(FdEntry::Path(PathEntry::synthetic_file(
        normalized.as_str(),
        size,
    )))
}

pub(super) fn proc_sysvipc_sem_fd_entry(path: &str) -> Option<FdEntry> {
    let normalized = normalize_path("/", path)?;
    if normalized != PROC_SYSVIPC_SEM_PATH {
        return None;
    }
    Some(FdEntry::MemoryFile(MemoryFileEntry {
        path: normalized,
        data: Arc::new(sysv_sem::proc_sysvipc_sem_content()),
        offset: 0,
    }))
}

pub(super) fn proc_sysvipc_sem_path_entry(path: &str) -> Option<FdEntry> {
    let normalized = normalize_path("/", path)?;
    if normalized != PROC_SYSVIPC_SEM_PATH {
        return None;
    }
    let size = sysv_sem::proc_sysvipc_sem_content().len();
    Some(FdEntry::Path(PathEntry::synthetic_file(
        normalized.as_str(),
        size,
    )))
}

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
const PROC_SYS_KERNEL_DOMAINNAME_PATH: &str = "/proc/sys/kernel/domainname";
const PROC_SYS_KERNEL_HOSTNAME_PATH: &str = "/proc/sys/kernel/hostname";
const PROC_SYS_KERNEL_PID_MAX_PATH: &str = "/proc/sys/kernel/pid_max";
const PROC_SYS_KERNEL_PID_MAX_CONTENT: &[u8] = b"4194304\n";
const PROC_SYS_KERNEL_PRINTK_PATH: &str = "/proc/sys/kernel/printk";
const PROC_SYS_KERNEL_SHMALL_PATH: &str = "/proc/sys/kernel/shmall";
const PROC_SYS_KERNEL_SHMMAX_PATH: &str = "/proc/sys/kernel/shmmax";
const PROC_SYS_KERNEL_SHMMNI_PATH: &str = "/proc/sys/kernel/shmmni";
const PROC_SYS_KERNEL_TAINTED_PATH: &str = "/proc/sys/kernel/tainted";
const PROC_SYS_FS_PIPE_MAX_SIZE_PATH: &str = "/proc/sys/fs/pipe-max-size";
const PROC_SYS_FS_PIPE_USER_PAGES_HARD_PATH: &str = "/proc/sys/fs/pipe-user-pages-hard";
const PROC_SYS_FS_PIPE_USER_PAGES_SOFT_PATH: &str = "/proc/sys/fs/pipe-user-pages-soft";
const PROC_SYS_VM_MAX_MAP_COUNT_PATH: &str = "/proc/sys/vm/max_map_count";
const PROC_SYS_VM_MIN_FREE_KBYTES_PATH: &str = "/proc/sys/vm/min_free_kbytes";
const PROC_SYS_VM_OVERCOMMIT_MEMORY_PATH: &str = "/proc/sys/vm/overcommit_memory";
const PROC_SYS_VM_OVERCOMMIT_RATIO_PATH: &str = "/proc/sys/vm/overcommit_ratio";
const PROC_SYS_VM_PANIC_ON_OOM_PATH: &str = "/proc/sys/vm/panic_on_oom";
const PROC_SYS_KERNEL_NGROUPS_MAX_PATH: &str = "/proc/sys/kernel/ngroups_max";
const PROC_SYS_DEFAULT_DOMAINNAME: &[u8] = b"localdomain";
const PROC_SYS_DEFAULT_HOSTNAME: &[u8] = b"arceos";
const PROC_SYS_KERNEL_NGROUPS_MAX_CONTENT: &[u8] = b"65536\n";
const PROC_SYS_DEFAULT_PIPE_MAX_SIZE: usize = 65_536;
const PROC_SYS_DEFAULT_PIPE_USER_PAGES_HARD: usize = 0;
const PROC_SYS_DEFAULT_PIPE_USER_PAGES_SOFT: usize = 512;
const PROC_SYS_DEFAULT_MAX_MAP_COUNT: usize = 65_530;
const PROC_SYS_DEFAULT_MIN_FREE_KBYTES: usize = 4_096;
const PROC_SYS_DEFAULT_OVERCOMMIT_RATIO: usize = 50;
const PROC_SYS_DEFAULT_SHMALL: usize = SYSV_SHM_MAX_SIZE / PAGE_SIZE_4K;
const PROC_SYS_MAX_STRING_LEN: usize = 64;

static PROC_SYS_PIPE_MAX_SIZE: AtomicUsize = AtomicUsize::new(PROC_SYS_DEFAULT_PIPE_MAX_SIZE);
static PROC_SYS_PIPE_USER_PAGES_HARD: AtomicUsize =
    AtomicUsize::new(PROC_SYS_DEFAULT_PIPE_USER_PAGES_HARD);
static PROC_SYS_PIPE_USER_PAGES_SOFT: AtomicUsize =
    AtomicUsize::new(PROC_SYS_DEFAULT_PIPE_USER_PAGES_SOFT);
static PROC_SYS_KERNEL_SHMALL: AtomicUsize = AtomicUsize::new(PROC_SYS_DEFAULT_SHMALL);
static PROC_SYS_KERNEL_SHMMAX: AtomicUsize = AtomicUsize::new(SYSV_SHM_MAX_SIZE);
static PROC_SYS_KERNEL_SHMMNI: AtomicUsize = AtomicUsize::new(SYSV_SHM_MAX_SEGMENTS);
static PROC_SYS_KERNEL_TAINTED: AtomicUsize = AtomicUsize::new(0);
static PROC_SYS_VM_MAX_MAP_COUNT: AtomicUsize = AtomicUsize::new(PROC_SYS_DEFAULT_MAX_MAP_COUNT);
static PROC_SYS_VM_MIN_FREE_KBYTES: AtomicUsize =
    AtomicUsize::new(PROC_SYS_DEFAULT_MIN_FREE_KBYTES);
static PROC_SYS_VM_OVERCOMMIT_MEMORY: AtomicUsize = AtomicUsize::new(0);
static PROC_SYS_VM_OVERCOMMIT_RATIO: AtomicUsize =
    AtomicUsize::new(PROC_SYS_DEFAULT_OVERCOMMIT_RATIO);
static PROC_SYS_VM_PANIC_ON_OOM: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy)]
pub(super) enum ProcSysFileKind {
    Domainname,
    Hostname,
    KernelPrintk,
    KernelShmall,
    KernelShmmax,
    KernelShmmni,
    KernelTainted,
    PipeMaxSize,
    PipeUserPagesHard,
    PipeUserPagesSoft,
    VmMaxMapCount,
    VmMinFreeKbytes,
    VmOvercommitMemory,
    VmOvercommitRatio,
    VmPanicOnOom,
}

#[derive(Clone)]
pub(super) struct ProcSysFileEntry {
    path: String,
    kind: ProcSysFileKind,
    offset: usize,
    status_flags: u32,
}

fn proc_sys_file_kind(path: &str) -> Option<(String, ProcSysFileKind)> {
    let normalized = normalize_path("/", path)?;
    let kind = match normalized.as_str() {
        PROC_SYS_KERNEL_DOMAINNAME_PATH => ProcSysFileKind::Domainname,
        PROC_SYS_KERNEL_HOSTNAME_PATH => ProcSysFileKind::Hostname,
        PROC_SYS_KERNEL_PRINTK_PATH => ProcSysFileKind::KernelPrintk,
        PROC_SYS_KERNEL_SHMALL_PATH => ProcSysFileKind::KernelShmall,
        PROC_SYS_KERNEL_SHMMAX_PATH => ProcSysFileKind::KernelShmmax,
        PROC_SYS_KERNEL_SHMMNI_PATH => ProcSysFileKind::KernelShmmni,
        PROC_SYS_KERNEL_TAINTED_PATH => ProcSysFileKind::KernelTainted,
        PROC_SYS_FS_PIPE_MAX_SIZE_PATH => ProcSysFileKind::PipeMaxSize,
        PROC_SYS_FS_PIPE_USER_PAGES_HARD_PATH => ProcSysFileKind::PipeUserPagesHard,
        PROC_SYS_FS_PIPE_USER_PAGES_SOFT_PATH => ProcSysFileKind::PipeUserPagesSoft,
        PROC_SYS_VM_MAX_MAP_COUNT_PATH => ProcSysFileKind::VmMaxMapCount,
        PROC_SYS_VM_MIN_FREE_KBYTES_PATH => ProcSysFileKind::VmMinFreeKbytes,
        PROC_SYS_VM_OVERCOMMIT_MEMORY_PATH => ProcSysFileKind::VmOvercommitMemory,
        PROC_SYS_VM_OVERCOMMIT_RATIO_PATH => ProcSysFileKind::VmOvercommitRatio,
        PROC_SYS_VM_PANIC_ON_OOM_PATH => ProcSysFileKind::VmPanicOnOom,
        _ => return None,
    };
    Some((normalized, kind))
}

fn proc_sys_string_state(kind: ProcSysFileKind) -> &'static Mutex<Vec<u8>> {
    static DOMAINNAME: LazyInit<Mutex<Vec<u8>>> = LazyInit::new();
    static HOSTNAME: LazyInit<Mutex<Vec<u8>>> = LazyInit::new();
    static PRINTK: LazyInit<Mutex<Vec<u8>>> = LazyInit::new();

    match kind {
        ProcSysFileKind::Domainname => {
            let _ = DOMAINNAME.call_once(|| Mutex::new(PROC_SYS_DEFAULT_DOMAINNAME.to_vec()));
            &DOMAINNAME
        }
        ProcSysFileKind::Hostname => {
            let _ = HOSTNAME.call_once(|| Mutex::new(PROC_SYS_DEFAULT_HOSTNAME.to_vec()));
            &HOSTNAME
        }
        ProcSysFileKind::KernelPrintk => {
            let _ = PRINTK.call_once(|| Mutex::new(b"7 4 1 7".to_vec()));
            &PRINTK
        }
        ProcSysFileKind::KernelShmall
        | ProcSysFileKind::KernelShmmax
        | ProcSysFileKind::KernelShmmni
        | ProcSysFileKind::KernelTainted
        | ProcSysFileKind::PipeUserPagesHard
        | ProcSysFileKind::PipeUserPagesSoft
        | ProcSysFileKind::PipeMaxSize
        | ProcSysFileKind::VmMaxMapCount
        | ProcSysFileKind::VmMinFreeKbytes
        | ProcSysFileKind::VmOvercommitMemory
        | ProcSysFileKind::VmOvercommitRatio
        | ProcSysFileKind::VmPanicOnOom => {
            unreachable!("numeric proc-sys file is not string-backed")
        }
    }
}

fn proc_sys_file_content(kind: ProcSysFileKind) -> Vec<u8> {
    match kind {
        ProcSysFileKind::Domainname | ProcSysFileKind::Hostname | ProcSysFileKind::KernelPrintk => {
            let state = proc_sys_string_state(kind).lock();
            let mut content = state.clone();
            content.push(b'\n');
            content
        }
        ProcSysFileKind::KernelShmall => {
            format!("{}\n", PROC_SYS_KERNEL_SHMALL.load(Ordering::Acquire)).into_bytes()
        }
        ProcSysFileKind::KernelShmmax => {
            format!("{}\n", PROC_SYS_KERNEL_SHMMAX.load(Ordering::Acquire)).into_bytes()
        }
        ProcSysFileKind::KernelShmmni => {
            format!("{}\n", PROC_SYS_KERNEL_SHMMNI.load(Ordering::Acquire)).into_bytes()
        }
        ProcSysFileKind::KernelTainted => {
            format!("{}\n", PROC_SYS_KERNEL_TAINTED.load(Ordering::Acquire)).into_bytes()
        }
        ProcSysFileKind::PipeMaxSize => {
            format!("{}\n", PROC_SYS_PIPE_MAX_SIZE.load(Ordering::Acquire)).into_bytes()
        }
        ProcSysFileKind::PipeUserPagesHard => format!(
            "{}\n",
            PROC_SYS_PIPE_USER_PAGES_HARD.load(Ordering::Acquire)
        )
        .into_bytes(),
        ProcSysFileKind::PipeUserPagesSoft => format!(
            "{}\n",
            PROC_SYS_PIPE_USER_PAGES_SOFT.load(Ordering::Acquire)
        )
        .into_bytes(),
        ProcSysFileKind::VmMaxMapCount => {
            format!("{}\n", PROC_SYS_VM_MAX_MAP_COUNT.load(Ordering::Acquire)).into_bytes()
        }
        ProcSysFileKind::VmMinFreeKbytes => {
            format!("{}\n", PROC_SYS_VM_MIN_FREE_KBYTES.load(Ordering::Acquire)).into_bytes()
        }
        ProcSysFileKind::VmOvercommitMemory => format!(
            "{}\n",
            PROC_SYS_VM_OVERCOMMIT_MEMORY.load(Ordering::Acquire)
        )
        .into_bytes(),
        ProcSysFileKind::VmOvercommitRatio => {
            format!("{}\n", PROC_SYS_VM_OVERCOMMIT_RATIO.load(Ordering::Acquire)).into_bytes()
        }
        ProcSysFileKind::VmPanicOnOom => {
            format!("{}\n", PROC_SYS_VM_PANIC_ON_OOM.load(Ordering::Acquire)).into_bytes()
        }
    }
}

pub(super) fn proc_sys_vm_max_map_count() -> usize {
    PROC_SYS_VM_MAX_MAP_COUNT.load(Ordering::Acquire)
}

pub(super) fn proc_sys_kernel_shmmax() -> usize {
    PROC_SYS_KERNEL_SHMMAX.load(Ordering::Acquire)
}

pub(super) fn proc_sys_kernel_shmmni() -> usize {
    PROC_SYS_KERNEL_SHMMNI.load(Ordering::Acquire)
}

pub(super) fn proc_sys_kernel_shmall() -> usize {
    PROC_SYS_KERNEL_SHMALL.load(Ordering::Acquire)
}

fn file_is_readable(status_flags: u32) -> bool {
    !matches!(status_flags & general::O_ACCMODE, general::O_WRONLY)
}

fn file_is_writable(status_flags: u32) -> bool {
    matches!(
        status_flags & general::O_ACCMODE,
        general::O_WRONLY | general::O_RDWR
    )
}

fn write_proc_sys_string(kind: ProcSysFileKind, src: &[u8]) -> Result<(), LinuxError> {
    let text = core::str::from_utf8(src).map_err(|_| LinuxError::EINVAL)?;
    let value = text
        .trim_end_matches(|c| matches!(c, '\n' | '\r' | '\0'))
        .as_bytes();
    if value.len() > PROC_SYS_MAX_STRING_LEN {
        return Err(LinuxError::EINVAL);
    }
    let mut state = proc_sys_string_state(kind).lock();
    state.clear();
    state.extend_from_slice(value);
    Ok(())
}

fn write_proc_sys_pipe_max_size(src: &[u8]) -> Result<(), LinuxError> {
    let text = core::str::from_utf8(src).map_err(|_| LinuxError::EINVAL)?;
    let value_text = text.split_whitespace().next().ok_or(LinuxError::EINVAL)?;
    let value = value_text
        .parse::<usize>()
        .map_err(|_| LinuxError::EINVAL)?;
    if value == 0 {
        return Err(LinuxError::EINVAL);
    }
    PROC_SYS_PIPE_MAX_SIZE.store(value, Ordering::Release);
    Ok(())
}

fn write_proc_sys_usize(
    src: &[u8],
    min: usize,
    max: Option<usize>,
    target: &AtomicUsize,
) -> Result<(), LinuxError> {
    let text = core::str::from_utf8(src).map_err(|_| LinuxError::EINVAL)?;
    let value_text = text.split_whitespace().next().ok_or(LinuxError::EINVAL)?;
    let value = value_text
        .parse::<usize>()
        .map_err(|_| LinuxError::EINVAL)?;
    if value < min || max.is_some_and(|max| value > max) {
        return Err(LinuxError::EINVAL);
    }
    target.store(value, Ordering::Release);
    Ok(())
}

impl ProcSysFileEntry {
    fn new(path: String, kind: ProcSysFileKind, status_flags: u32) -> Self {
        Self {
            path,
            kind,
            offset: 0,
            status_flags,
        }
    }

    pub(super) fn path(&self) -> &str {
        self.path.as_str()
    }

    pub(super) fn status_flags(&self) -> u32 {
        self.status_flags
    }

    pub(super) fn set_status_flags(&mut self, flags: u32) {
        self.status_flags =
            (self.status_flags & general::O_ACCMODE) | (flags & general::O_NONBLOCK);
    }

    pub(super) fn stat(&self) -> general::stat {
        PathEntry::synthetic_file_with_mode(
            self.path.as_str(),
            proc_sys_file_content(self.kind).len(),
            0o644,
        )
        .stat()
    }

    pub(super) fn read(&mut self, dst: &mut [u8]) -> Result<usize, LinuxError> {
        if !file_is_readable(self.status_flags) {
            return Err(LinuxError::EBADF);
        }
        let data = proc_sys_file_content(self.kind);
        let start = self.offset.min(data.len());
        let end = core::cmp::min(start + dst.len(), data.len());
        let len = end.saturating_sub(start);
        dst[..len].copy_from_slice(&data[start..end]);
        self.offset = end;
        Ok(len)
    }

    pub(super) fn write(&mut self, src: &[u8]) -> Result<usize, LinuxError> {
        if !file_is_writable(self.status_flags) {
            return Err(LinuxError::EBADF);
        }
        if src.is_empty() {
            return Ok(0);
        }
        match self.kind {
            ProcSysFileKind::Domainname
            | ProcSysFileKind::Hostname
            | ProcSysFileKind::KernelPrintk => write_proc_sys_string(self.kind, src)?,
            ProcSysFileKind::KernelShmall => {
                write_proc_sys_usize(src, 1, None, &PROC_SYS_KERNEL_SHMALL)?
            }
            ProcSysFileKind::KernelShmmax => {
                write_proc_sys_usize(src, 1, None, &PROC_SYS_KERNEL_SHMMAX)?
            }
            ProcSysFileKind::KernelShmmni => {
                write_proc_sys_usize(src, 1, None, &PROC_SYS_KERNEL_SHMMNI)?
            }
            ProcSysFileKind::KernelTainted => {
                write_proc_sys_usize(src, 0, None, &PROC_SYS_KERNEL_TAINTED)?
            }
            ProcSysFileKind::PipeMaxSize => write_proc_sys_pipe_max_size(src)?,
            ProcSysFileKind::PipeUserPagesHard => {
                write_proc_sys_usize(src, 0, None, &PROC_SYS_PIPE_USER_PAGES_HARD)?
            }
            ProcSysFileKind::PipeUserPagesSoft => {
                write_proc_sys_usize(src, 0, None, &PROC_SYS_PIPE_USER_PAGES_SOFT)?
            }
            ProcSysFileKind::VmMaxMapCount => {
                write_proc_sys_usize(src, 1, None, &PROC_SYS_VM_MAX_MAP_COUNT)?
            }
            ProcSysFileKind::VmMinFreeKbytes => {
                write_proc_sys_usize(src, 0, None, &PROC_SYS_VM_MIN_FREE_KBYTES)?
            }
            ProcSysFileKind::VmOvercommitMemory => {
                write_proc_sys_usize(src, 0, Some(2), &PROC_SYS_VM_OVERCOMMIT_MEMORY)?
            }
            ProcSysFileKind::VmOvercommitRatio => {
                write_proc_sys_usize(src, 0, None, &PROC_SYS_VM_OVERCOMMIT_RATIO)?
            }
            ProcSysFileKind::VmPanicOnOom => {
                write_proc_sys_usize(src, 0, Some(1), &PROC_SYS_VM_PANIC_ON_OOM)?
            }
        }
        self.offset = self.offset.saturating_add(src.len());
        Ok(src.len())
    }

    pub(super) fn seek(&mut self, pos: axio::SeekFrom) -> Result<u64, LinuxError> {
        let size = proc_sys_file_content(self.kind).len() as i64;
        let next = match pos {
            axio::SeekFrom::Start(offset) => offset as i64,
            axio::SeekFrom::Current(offset) => self.offset as i64 + offset,
            axio::SeekFrom::End(offset) => size + offset,
        };
        if next < 0 {
            return Err(LinuxError::EINVAL);
        }
        self.offset = next as usize;
        Ok(self.offset as u64)
    }
}

pub(super) fn proc_sys_file_fd_entry(path: &str, status_flags: u32) -> Option<FdEntry> {
    proc_sys_file_kind(path)
        .map(|(path, kind)| FdEntry::ProcSysFile(ProcSysFileEntry::new(path, kind, status_flags)))
}

pub(super) fn proc_sys_file_path_entry(path: &str) -> Option<FdEntry> {
    proc_sys_file_kind(path).map(|(path, kind)| {
        FdEntry::Path(PathEntry::synthetic_file_with_mode(
            path.as_str(),
            proc_sys_file_content(kind).len(),
            0o644,
        ))
    })
}

pub(super) fn synthetic_proc_sys_content(path: &str) -> Option<(&'static str, &'static [u8])> {
    if let Some(content) = sysv_msg::proc_sys_kernel_msg_content(path) {
        return Some(content);
    }
    if let Some(content) = sysv_sem::proc_sys_kernel_sem_content(path) {
        return Some(content);
    }
    match normalize_path("/", path).as_deref() {
        Some(PROC_SYS_KERNEL_CORE_PATTERN_PATH) => Some((
            PROC_SYS_KERNEL_CORE_PATTERN_PATH,
            PROC_SYS_KERNEL_CORE_PATTERN_CONTENT,
        )),
        Some(PROC_SYS_KERNEL_NGROUPS_MAX_PATH) => Some((
            PROC_SYS_KERNEL_NGROUPS_MAX_PATH,
            PROC_SYS_KERNEL_NGROUPS_MAX_CONTENT,
        )),
        Some(PROC_SYS_KERNEL_PID_MAX_PATH) => Some((
            PROC_SYS_KERNEL_PID_MAX_PATH,
            PROC_SYS_KERNEL_PID_MAX_CONTENT,
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
