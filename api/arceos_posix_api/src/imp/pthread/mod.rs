use alloc::{collections::BTreeMap, sync::Arc};
use core::cell::UnsafeCell;
use core::ffi::{c_int, c_void};
use core::sync::atomic::{AtomicBool, Ordering};

use axerrno::{LinuxError, LinuxResult};
use axtask::AxTaskRef;
use spin::RwLock;

use crate::{ctypes, utils::write_user_value};

pub mod mutex;

lazy_static::lazy_static! {
    static ref TID_TO_PTHREAD: RwLock<BTreeMap<u64, Arc<Pthread>>> = {
        let mut map = BTreeMap::new();
        let main_task = axtask::current();
        let main_tid = main_task.id().as_u64();
        let main_thread = Arc::new(Pthread {
            inner: main_task.as_task_ref().clone(),
            retval: Arc::new(Packet {
                result: UnsafeCell::new(core::ptr::null_mut()),
            }),
            join_in_progress: AtomicBool::new(false),
        });
        map.insert(main_tid, main_thread);
        RwLock::new(map)
    };
}

struct Packet<T> {
    result: UnsafeCell<T>,
}

unsafe impl<T> Send for Packet<T> {}
unsafe impl<T> Sync for Packet<T> {}

pub struct Pthread {
    inner: AxTaskRef,
    retval: Arc<Packet<*mut c_void>>,
    join_in_progress: AtomicBool,
}

impl Pthread {
    fn create(
        _attr: *const ctypes::pthread_attr_t,
        start_routine: extern "C" fn(arg: *mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> LinuxResult<ctypes::pthread_t> {
        let arg_wrapper = ForceSendSync(arg);

        let my_packet: Arc<Packet<*mut c_void>> = Arc::new(Packet {
            result: UnsafeCell::new(core::ptr::null_mut()),
        });
        let their_packet = my_packet.clone();
        let registration_ready = Arc::new(AtomicBool::new(false));
        let child_registration_ready = registration_ready.clone();

        let main = move || {
            while !child_registration_ready.load(Ordering::Acquire) {
                axtask::yield_now();
            }
            let arg = arg_wrapper;
            let ret = start_routine(arg.0);
            unsafe { *their_packet.result.get() = ret };
            drop(their_packet);
        };

        let task_inner = axtask::spawn(main);
        let tid = task_inner.id().as_u64();
        let thread = Pthread {
            inner: task_inner,
            retval: my_packet,
            join_in_progress: AtomicBool::new(false),
        };
        let thread = Arc::new(thread);
        let ptr = Arc::as_ptr(&thread) as *mut c_void;
        TID_TO_PTHREAD.write().insert(tid, thread);
        registration_ready.store(true, Ordering::Release);
        Ok(ptr)
    }

    fn current_ptr() -> *mut Pthread {
        let tid = axtask::current().id().as_u64();
        match TID_TO_PTHREAD.read().get(&tid) {
            None => {
                error!("pthread_self: missing pthread registration for tid={tid}");
                core::ptr::null_mut()
            }
            Some(thread) => Arc::as_ptr(thread) as *mut Pthread,
        }
    }

    fn exit_current(retval: *mut c_void) -> ! {
        let tid = axtask::current().id().as_u64();
        {
            let threads = TID_TO_PTHREAD.read();
            if let Some(thread) = threads.get(&tid) {
                unsafe { *thread.retval.result.get() = retval };
            } else {
                error!("pthread_exit: missing pthread registration for tid={tid}");
            }
        }
        axtask::exit(0);
    }

    fn join(ptr: ctypes::pthread_t) -> LinuxResult<*mut c_void> {
        if ptr.is_null() {
            return Err(LinuxError::ESRCH);
        }
        let current_tid = axtask::current().id().as_u64();

        let (tid, thread) = {
            let threads = TID_TO_PTHREAD.read();
            let Some((tid, thread)) = threads
                .iter()
                .find(|(_, stored)| Arc::as_ptr(stored) as ctypes::pthread_t == ptr)
            else {
                return Err(LinuxError::ESRCH);
            };
            if *tid == current_tid {
                return Err(LinuxError::EDEADLK);
            }
            if thread.join_in_progress.swap(true, Ordering::AcqRel) {
                return Err(LinuxError::EINVAL);
            }
            (*tid, thread.clone())
        };

        thread.inner.join();
        let retval = unsafe { *thread.retval.result.get() };
        let mut threads = TID_TO_PTHREAD.write();
        if threads
            .get(&tid)
            .is_some_and(|stored| Arc::ptr_eq(stored, &thread))
        {
            threads.remove(&tid);
        }
        Ok(retval)
    }
}

unsafe fn write_pthread_output<T>(dst: *mut T, value: T) -> LinuxResult {
    unsafe { write_user_value(dst, value) }
}

/// Returns the `pthread` struct of current thread.
pub fn sys_pthread_self() -> ctypes::pthread_t {
    Pthread::current_ptr() as _
}

/// Create a new thread with the given entry point and argument.
///
/// If successful, it stores the pointer to the newly created `struct __pthread`
/// in `res` and returns 0.
///
/// # Safety
///
/// `res` must point to writable storage for one `pthread_t`. `attr` must be
/// either null or point to a valid `pthread_attr_t`.
pub unsafe fn sys_pthread_create(
    res: *mut ctypes::pthread_t,
    attr: *const ctypes::pthread_attr_t,
    start_routine: extern "C" fn(arg: *mut c_void) -> *mut c_void,
    arg: *mut c_void,
) -> c_int {
    debug!(
        "sys_pthread_create <= {:#x}, {:#x}",
        start_routine as usize, arg as usize
    );
    syscall_body!(sys_pthread_create, {
        if res.is_null() {
            return Err(LinuxError::EFAULT);
        }
        let ptr = Pthread::create(attr, start_routine, arg)?;
        unsafe { write_pthread_output(res, ptr)? };
        Ok(0)
    })
}

/// Exits the current thread. The value `retval` will be returned to the joiner.
pub fn sys_pthread_exit(retval: *mut c_void) -> ! {
    debug!("sys_pthread_exit <= {:#x}", retval as usize);
    Pthread::exit_current(retval);
}

/// Waits for the given thread to exit, and stores the return value in `retval`.
///
/// # Safety
///
/// If `retval` is non-null, it must point to writable storage for one thread
/// return pointer.
pub unsafe fn sys_pthread_join(thread: ctypes::pthread_t, retval: *mut *mut c_void) -> c_int {
    debug!("sys_pthread_join <= {:#x}", retval as usize);
    syscall_body!(sys_pthread_join, {
        let ret = Pthread::join(thread)?;
        if !retval.is_null() {
            unsafe { write_pthread_output(retval, ret)? };
        }
        Ok(0)
    })
}

#[derive(Clone, Copy)]
struct ForceSendSync<T>(T);

unsafe impl<T> Send for ForceSendSync<T> {}
unsafe impl<T> Sync for ForceSendSync<T> {}
