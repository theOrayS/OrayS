use core::mem::size_of;

use axerrno::LinuxError;

use super::user_memory::{clear_user_bytes, read_user_bytes, validate_user_read, write_user_value};
use super::{UserProcess, neg_errno};

const MPOL_DEFAULT: usize = 0;
const MEMBIND_SUPPORTED_FLAGS: usize = 0;
const MAX_NODEMASK_BYTES: usize = 4096;

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
    if mask_len > MAX_NODEMASK_BYTES {
        return Err(LinuxError::EINVAL);
    }
    if nodemask != 0 && mask_len != 0 {
        validate_user_read(process, nodemask, mask_len)?;
    }
    Ok(())
}

fn nodemask_is_empty(
    process: &UserProcess,
    nodemask: usize,
    maxnode: usize,
) -> Result<bool, LinuxError> {
    validate_mempolicy_nodemask(process, nodemask, maxnode)?;
    let mask_len = nodemask_len(maxnode);
    if nodemask == 0 || mask_len == 0 {
        return Ok(true);
    }
    let bytes = read_user_bytes(process, nodemask, mask_len)?;
    Ok(bytes.iter().all(|byte| *byte == 0))
}

fn default_policy_only(
    process: &UserProcess,
    mode: usize,
    nodemask: usize,
    maxnode: usize,
) -> Result<(), LinuxError> {
    let empty_mask = nodemask_is_empty(process, nodemask, maxnode)?;
    if mode == MPOL_DEFAULT && empty_mask {
        Ok(())
    } else if mode == MPOL_DEFAULT {
        Err(LinuxError::EINVAL)
    } else {
        Err(LinuxError::EOPNOTSUPP)
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
    flags: usize,
) -> isize {
    if start.checked_add(len).is_none() {
        return neg_errno(LinuxError::EINVAL);
    }
    if flags != MEMBIND_SUPPORTED_FLAGS {
        return neg_errno(LinuxError::EOPNOTSUPP);
    }
    match default_policy_only(process, mode, nodemask, maxnode) {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_get_mempolicy(
    process: &UserProcess,
    mode: usize,
    nodemask: usize,
    maxnode: usize,
    _addr: usize,
    flags: usize,
) -> isize {
    if flags != 0 {
        return neg_errno(LinuxError::EOPNOTSUPP);
    }
    write_default_mempolicy(process, mode, nodemask, maxnode)
}

pub(super) fn sys_set_mempolicy(
    process: &UserProcess,
    mode: usize,
    nodemask: usize,
    maxnode: usize,
) -> isize {
    match default_policy_only(process, mode, nodemask, maxnode) {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}
