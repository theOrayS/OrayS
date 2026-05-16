use core::sync::atomic::{AtomicBool, Ordering};

use axmm::AddrSpace;
use axsync::Mutex;
use std::vec::Vec;

use super::{ChildTask, FdTable};

pub(super) struct ProcessTeardown {
    done: AtomicBool,
}

impl ProcessTeardown {
    pub(super) fn new() -> Self {
        Self {
            done: AtomicBool::new(false),
        }
    }

    pub(super) fn run(
        &self,
        aspace: &Mutex<AddrSpace>,
        fds: &Mutex<FdTable>,
        children: &Mutex<Vec<ChildTask>>,
    ) {
        if self.done.swap(true, Ordering::AcqRel) {
            return;
        }

        aspace.lock().clear();
        {
            let mut fds = fds.lock();
            fds.close_all();
            *fds = FdTable::new();
        }
        children.lock().clear();
    }
}
