#![allow(dead_code)]
#![allow(unused_macros)]

use axerrno::{LinuxError, LinuxResult};
use core::ffi::{c_char, c_void, CStr};
use core::mem::{align_of, size_of};
use core::ptr::NonNull;

pub fn char_ptr_to_str<'a>(str: *const c_char) -> LinuxResult<&'a str> {
    if str.is_null() {
        Err(LinuxError::EFAULT)
    } else {
        unsafe { CStr::from_ptr(str) }
            .to_str()
            .map_err(|_| LinuxError::EINVAL)
    }
}

pub fn check_null_ptr<T>(ptr: *const T) -> LinuxResult {
    if ptr.is_null() {
        Err(LinuxError::EFAULT)
    } else {
        Ok(())
    }
}

pub fn check_null_mut_ptr<T>(ptr: *mut T) -> LinuxResult {
    if ptr.is_null() {
        Err(LinuxError::EFAULT)
    } else {
        Ok(())
    }
}

fn validate_user_range<T>(ptr: *const T, len: usize, require_align: bool) -> LinuxResult<*const T> {
    let elem_size = size_of::<T>();
    let bytes = len.checked_mul(elem_size).ok_or(LinuxError::EFAULT)?;
    if bytes == 0 {
        return Ok(NonNull::<T>::dangling().as_ptr());
    }
    if ptr.is_null() {
        return Err(LinuxError::EFAULT);
    }
    let addr = ptr as usize;
    addr.checked_add(bytes - 1).ok_or(LinuxError::EFAULT)?;
    if require_align {
        let align = align_of::<T>();
        if align > 1 && addr % align != 0 {
            return Err(LinuxError::EFAULT);
        }
    }
    Ok(ptr)
}

/// Read one ABI value from a user-provided pointer after the shared null check.
///
/// This helper does not make arbitrary user memory trusted; it only centralizes
/// the syscall boundary contract used by the current single-address-space
/// runtime. Callers still need to validate lengths and semantic ranges before
/// acting on the copied value.
///
/// # Safety
///
/// `ptr` must be readable for one `T` when non-null.
pub unsafe fn read_user_value<T: Copy>(ptr: *const T) -> LinuxResult<T> {
    let ptr = validate_user_range(ptr, 1, false)?;
    Ok(unsafe { core::ptr::read_unaligned(ptr) })
}

/// Write one ABI value to a user-provided pointer after the shared null check.
///
/// # Safety
///
/// `ptr` must be writable for one `T` when non-null.
pub unsafe fn write_user_value<T>(ptr: *mut T, value: T) -> LinuxResult {
    let ptr = validate_user_range(ptr.cast_const(), 1, false)?.cast_mut();
    unsafe { core::ptr::write_unaligned(ptr, value) };
    Ok(())
}

/// Borrow one aligned readable user object in-place.
///
/// This is only for ABI objects whose state must remain in user memory (for
/// example pthread mutexes).  Ordinary syscall structs should prefer
/// `read_user_value`/`write_user_value` copy-in/copy-out.
///
/// # Safety
///
/// `ptr` must point to a live, readable, aligned `T`.
pub unsafe fn user_ref<'a, T>(ptr: *const T) -> LinuxResult<&'a T> {
    let ptr = validate_user_range(ptr, 1, true)?;
    Ok(unsafe { &*ptr })
}

/// Borrow one aligned writable user object in-place.
///
/// # Safety
///
/// `ptr` must point to a live, writable, aligned `T`.
pub unsafe fn user_mut_ref<'a, T>(ptr: *mut T) -> LinuxResult<&'a mut T> {
    let ptr = validate_user_range(ptr.cast_const(), 1, true)?.cast_mut();
    Ok(unsafe { &mut *ptr })
}

/// Borrow a readable user buffer, accepting `NULL` only for zero-length I/O.
///
/// # Safety
///
/// `buf` must either be null with `len == 0`, or readable for `len` bytes.
pub unsafe fn readable_user_buffer<'a>(buf: *const c_void, len: usize) -> LinuxResult<&'a [u8]> {
    unsafe { readable_user_slice(buf.cast::<u8>(), len) }
}

/// Borrow a readable user array, accepting `NULL` only for zero-length arrays.
///
/// # Safety
///
/// `ptr` must either be null with `len == 0`, or readable for `len` elements
/// of `T` and satisfy `T` alignment.
pub unsafe fn readable_user_slice<'a, T>(ptr: *const T, len: usize) -> LinuxResult<&'a [T]> {
    let ptr = validate_user_range(ptr, len, true)?;
    Ok(unsafe { core::slice::from_raw_parts(ptr, len) })
}

/// Borrow a writable user buffer, accepting `NULL` only for zero-length I/O.
///
/// # Safety
///
/// `buf` must either be null with `len == 0`, or writable for `len` bytes.
pub unsafe fn writable_user_buffer<'a>(buf: *mut c_void, len: usize) -> LinuxResult<&'a mut [u8]> {
    unsafe { writable_user_slice(buf.cast::<u8>(), len) }
}

/// Borrow a writable user array, accepting `NULL` only for zero-length arrays.
///
/// # Safety
///
/// `ptr` must either be null with `len == 0`, or writable for `len` elements
/// of `T` and satisfy `T` alignment.
pub unsafe fn writable_user_slice<'a, T>(ptr: *mut T, len: usize) -> LinuxResult<&'a mut [T]> {
    let ptr = validate_user_range(ptr.cast_const(), len, true)?.cast_mut();
    Ok(unsafe { core::slice::from_raw_parts_mut(ptr, len) })
}

macro_rules! syscall_body {
    ($fn: ident, debug_errors: [$($debug_err:path),+ $(,)?], $($stmt: tt)*) => {{
        #[allow(clippy::redundant_closure_call)]
        let res = (|| -> axerrno::LinuxResult<_> { $($stmt)* })();
        match res {
            Ok(_) | Err(axerrno::LinuxError::EAGAIN $(| $debug_err)+) => {
                debug!(concat!(stringify!($fn), " => {:?}"),  res)
            }
            Err(_) => info!(concat!(stringify!($fn), " => {:?}"), res),
        }
        match res {
            Ok(v) => v as _,
            Err(e) => {
                -e.code() as _
            }
        }
    }};
    ($fn: ident, $($stmt: tt)*) => {{
        #[allow(clippy::redundant_closure_call)]
        let res = (|| -> axerrno::LinuxResult<_> { $($stmt)* })();
        match res {
            Ok(_) | Err(axerrno::LinuxError::EAGAIN) => debug!(concat!(stringify!($fn), " => {:?}"),  res),
            Err(_) => info!(concat!(stringify!($fn), " => {:?}"), res),
        }
        match res {
            Ok(v) => v as _,
            Err(e) => {
                -e.code() as _
            }
        }
    }};
}

macro_rules! syscall_body_no_debug {
    ($($stmt: tt)*) => {{
        #[allow(clippy::redundant_closure_call)]
        let res = (|| -> axerrno::LinuxResult<_> { $($stmt)* })();
        match res {
            Ok(v) => v as _,
            Err(e) => {
                -e.code() as _
            }
        }
    }};
}
