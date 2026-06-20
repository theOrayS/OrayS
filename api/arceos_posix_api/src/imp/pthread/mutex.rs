use crate::{
    ctypes,
    utils::{read_user_value, user_ref, write_user_value},
};

use axerrno::{LinuxError, LinuxResult};

use core::ffi::c_int;
use core::mem::size_of;
use core::sync::atomic::{AtomicU64, Ordering};

static_assertions::const_assert_eq!(
    size_of::<ctypes::pthread_mutex_t>(),
    size_of::<PthreadMutex>()
);

#[repr(C)]
pub struct PthreadMutex {
    owner: AtomicU64,
    lock_count: AtomicU64,
    kind: AtomicU64,
    _reserved: [u64; 2],
}

impl PthreadMutex {
    const NORMAL: u64 = 0;
    const RECURSIVE: u64 = 1;
    const ERRORCHECK: u64 = 2;

    const fn new(kind: u64) -> Self {
        Self {
            owner: AtomicU64::new(0),
            lock_count: AtomicU64::new(0),
            kind: AtomicU64::new(kind),
            _reserved: [0; 2],
        }
    }

    fn from_attr(attr: *const ctypes::pthread_mutexattr_t) -> LinuxResult<Self> {
        if attr.is_null() {
            return Ok(Self::new(Self::NORMAL));
        }

        let raw = unsafe { read_user_value(attr)? }.__attr;
        let kind = (raw & 0b11) as u64;
        let unsupported = raw & !0b11;
        if unsupported != 0 {
            return Err(LinuxError::EOPNOTSUPP);
        }
        match kind {
            Self::NORMAL | Self::RECURSIVE | Self::ERRORCHECK => Ok(Self::new(kind)),
            _ => Err(LinuxError::EINVAL),
        }
    }

    unsafe fn from_user<'a>(mutex: *mut ctypes::pthread_mutex_t) -> LinuxResult<&'a Self> {
        unsafe { user_ref(mutex.cast::<Self>()) }
    }

    fn lock_recursive(&self) -> LinuxResult {
        self.lock_count
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| {
                count.checked_add(1)
            })
            .map(|_| ())
            .map_err(|_| LinuxError::EAGAIN)
    }

    fn lock(&self) -> LinuxResult {
        let current_id = axtask::current().id().as_u64();
        loop {
            match self.owner.compare_exchange_weak(
                0,
                current_id,
                Ordering::Acquire,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    self.lock_count.store(1, Ordering::Release);
                    return Ok(());
                }
                Err(owner_id) => {
                    if owner_id == current_id {
                        return match self.kind.load(Ordering::Acquire) {
                            Self::RECURSIVE => self.lock_recursive(),
                            Self::NORMAL | Self::ERRORCHECK => Err(LinuxError::EDEADLK),
                            _ => Err(LinuxError::EINVAL),
                        };
                    }
                    axtask::yield_now();
                }
            }
        }
    }

    fn try_lock(&self) -> LinuxResult {
        let current_id = axtask::current().id().as_u64();
        match self
            .owner
            .compare_exchange(0, current_id, Ordering::Acquire, Ordering::Relaxed)
        {
            Ok(_) => {
                self.lock_count.store(1, Ordering::Release);
                Ok(())
            }
            Err(owner_id) if owner_id == current_id => match self.kind.load(Ordering::Acquire) {
                Self::RECURSIVE => self.lock_recursive(),
                Self::NORMAL | Self::ERRORCHECK => Err(LinuxError::EBUSY),
                _ => Err(LinuxError::EINVAL),
            },
            Err(_) => Err(LinuxError::EBUSY),
        }
    }

    fn unlock(&self) -> LinuxResult {
        let current_id = axtask::current().id().as_u64();
        if self.owner.load(Ordering::Acquire) == current_id
            && self.kind.load(Ordering::Acquire) == Self::RECURSIVE
        {
            let count = self.lock_count.load(Ordering::Acquire);
            if count > 1 {
                self.lock_count.fetch_sub(1, Ordering::AcqRel);
                return Ok(());
            }
        }
        match self
            .owner
            .compare_exchange(current_id, 0, Ordering::Release, Ordering::Relaxed)
        {
            Ok(_) => {
                self.lock_count.store(0, Ordering::Release);
                Ok(())
            }
            Err(owner_id) => {
                if owner_id == 0 {
                    Err(LinuxError::EINVAL)
                } else {
                    Err(LinuxError::EPERM)
                }
            }
        }
    }
}

/// Initialize a mutex.
///
/// # Safety
///
/// `mutex` must point to writable storage for a `pthread_mutex_t`.
pub unsafe fn sys_pthread_mutex_init(
    mutex: *mut ctypes::pthread_mutex_t,
    _attr: *const ctypes::pthread_mutexattr_t,
) -> c_int {
    debug!("sys_pthread_mutex_init <= {:#x}", mutex as usize);
    syscall_body!(sys_pthread_mutex_init, {
        let value = PthreadMutex::from_attr(_attr)?;
        unsafe { write_user_value(mutex.cast::<PthreadMutex>(), value)? };
        Ok(0)
    })
}

/// Lock the given mutex.
///
/// # Safety
///
/// `mutex` must point to a valid initialized `pthread_mutex_t`.
pub unsafe fn sys_pthread_mutex_lock(mutex: *mut ctypes::pthread_mutex_t) -> c_int {
    debug!("sys_pthread_mutex_lock <= {:#x}", mutex as usize);
    syscall_body!(sys_pthread_mutex_lock, {
        unsafe { PthreadMutex::from_user(mutex)?.lock()? };
        Ok(0)
    })
}

/// Try to lock the given mutex without blocking.
///
/// # Safety
///
/// `mutex` must point to a valid initialized `pthread_mutex_t`.
pub unsafe fn sys_pthread_mutex_trylock(mutex: *mut ctypes::pthread_mutex_t) -> c_int {
    debug!("sys_pthread_mutex_trylock <= {:#x}", mutex as usize);
    syscall_body!(sys_pthread_mutex_trylock, {
        unsafe { PthreadMutex::from_user(mutex)?.try_lock()? };
        Ok(0)
    })
}

/// Unlock the given mutex.
///
/// # Safety
///
/// `mutex` must point to a valid initialized `pthread_mutex_t` owned by the current task.
pub unsafe fn sys_pthread_mutex_unlock(mutex: *mut ctypes::pthread_mutex_t) -> c_int {
    debug!("sys_pthread_mutex_unlock <= {:#x}", mutex as usize);
    syscall_body!(sys_pthread_mutex_unlock, {
        unsafe { PthreadMutex::from_user(mutex)?.unlock()? };
        Ok(0)
    })
}
