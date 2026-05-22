use core::mem::size_of;

use axerrno::LinuxError;

use super::user_memory::{clear_user_bytes, validate_user_read, write_user_value};
use super::{UserProcess, neg_errno};

fn nodemask_len(maxnode: usize) -> usize {
    if maxnode == 0 {
        0
    } else {
        maxnode.div_ceil(usize::BITS as usize) * size_of::<usize>()
    }
}

fn validate_mempolicy_nodemask(
    process: &UserProcess,
    nodemask: usize,
    maxnode: usize,
) -> Result<(), LinuxError> {
    let mask_len = nodemask_len(maxnode);
    if nodemask != 0 && mask_len != 0 {
        validate_user_read(process, nodemask, mask_len)?;
    }
    Ok(())
}

pub(super) fn validate_mempolicy_request(
    process: &UserProcess,
    nodemask: usize,
    maxnode: usize,
) -> isize {
    match validate_mempolicy_nodemask(process, nodemask, maxnode) {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn write_default_mempolicy(
    process: &UserProcess,
    mode: usize,
    nodemask: usize,
    maxnode: usize,
) -> isize {
    if mode != 0 {
        let default_mode = 0i32;
        let ret = write_user_value(process, mode, &default_mode);
        if ret != 0 {
            return ret;
        }
    }
    let mask_len = nodemask_len(maxnode);
    if nodemask != 0 && mask_len != 0 {
        if let Err(err) = clear_user_bytes(process, nodemask, mask_len) {
            return neg_errno(err);
        }
    }
    0
}

pub(super) fn sys_mbind(
    process: &UserProcess,
    start: usize,
    len: usize,
    mode: usize,
    nodemask: usize,
    maxnode: usize,
) -> isize {
    let _ = (start, len, mode);
    validate_mempolicy_request(process, nodemask, maxnode)
}

pub(super) fn sys_get_mempolicy(
    process: &UserProcess,
    mode: usize,
    nodemask: usize,
    maxnode: usize,
    _addr: usize,
    _flags: usize,
) -> isize {
    write_default_mempolicy(process, mode, nodemask, maxnode)
}

pub(super) fn sys_set_mempolicy(
    process: &UserProcess,
    mode: usize,
    nodemask: usize,
    maxnode: usize,
) -> isize {
    let _ = mode;
    validate_mempolicy_request(process, nodemask, maxnode)
}
