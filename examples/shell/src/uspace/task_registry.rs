use core::sync::atomic::Ordering;

use axsync::Mutex;
use axtask::AxTaskRef;
use lazyinit::LazyInit;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::vec::Vec;

use super::UserProcess;

#[derive(Clone)]
pub(super) struct UserThreadEntry {
    pub(super) task: AxTaskRef,
    pub(super) process: Arc<UserProcess>,
}

fn user_thread_table() -> &'static Mutex<BTreeMap<i32, UserThreadEntry>> {
    static USER_THREADS: LazyInit<Mutex<BTreeMap<i32, UserThreadEntry>>> = LazyInit::new();
    let _ = USER_THREADS.call_once(|| Mutex::new(BTreeMap::new()));
    &USER_THREADS
}

pub(super) fn register_user_task(task: AxTaskRef, process: Arc<UserProcess>) {
    let tid = task.id().as_u64() as i32;
    user_thread_table()
        .lock()
        .insert(tid, UserThreadEntry { task, process });
}

pub(super) fn live_user_thread_count() -> usize {
    user_thread_table().lock().len()
}

#[cfg(feature = "auto-run-tests")]
pub(super) fn live_user_thread_entries() -> Vec<UserThreadEntry> {
    user_thread_table()
        .lock()
        .values()
        .filter(|entry| entry.process.live_threads.load(Ordering::Acquire) != 0)
        .cloned()
        .collect()
}

pub(super) fn unregister_user_task(tid: i32) {
    user_thread_table().lock().remove(&tid);
}

pub(super) fn user_thread_entry_by_tid(tid: i32) -> Option<UserThreadEntry> {
    user_thread_table().lock().get(&tid).cloned()
}

pub(super) fn user_thread_entry_by_process_pid(pid: i32) -> Option<UserThreadEntry> {
    let table = user_thread_table().lock();
    table.get(&pid).cloned().or_else(|| {
        table
            .values()
            .find(|entry| {
                entry.process.pid() == pid
                    && entry.process.live_threads.load(Ordering::Acquire) != 0
            })
            .cloned()
    })
}

pub(super) fn user_thread_entries_by_process_pid(pid: i32) -> Vec<UserThreadEntry> {
    let table = user_thread_table().lock();
    table
        .values()
        .filter(|entry| {
            entry.process.pid() == pid && entry.process.live_threads.load(Ordering::Acquire) != 0
        })
        .cloned()
        .collect()
}

pub(super) fn user_thread_entries_by_process_group(pgid: i32) -> Vec<UserThreadEntry> {
    let table = user_thread_table().lock();
    let mut entries = Vec::new();
    let mut pids = Vec::new();
    for entry in table.values() {
        let pid = entry.process.pid();
        if entry.process.pgid() == pgid
            && entry.process.live_threads.load(Ordering::Acquire) != 0
            && !pids.contains(&pid)
        {
            pids.push(pid);
            entries.push(entry.clone());
        }
    }
    entries
}

pub(super) fn live_user_process_entries() -> Vec<UserThreadEntry> {
    let table = user_thread_table().lock();
    let mut entries = Vec::new();
    let mut pids = Vec::new();
    for entry in table.values() {
        let pid = entry.process.pid();
        if entry.process.live_threads.load(Ordering::Acquire) != 0 && !pids.contains(&pid) {
            pids.push(pid);
            entries.push(entry.clone());
        }
    }
    entries
}

pub(super) fn user_thread_entry_for_process(process: &UserProcess) -> Option<UserThreadEntry> {
    let pid = process.pid();
    let table = user_thread_table().lock();
    table.get(&pid).cloned().or_else(|| {
        table
            .values()
            .find(|entry| entry.process.pid() == pid)
            .cloned()
    })
}
