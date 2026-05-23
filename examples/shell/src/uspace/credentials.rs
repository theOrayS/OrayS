use core::mem::size_of;
use core::sync::atomic::Ordering;

use axerrno::LinuxError;
use linux_raw_sys::general;
use std::string::String;
use std::vec::Vec;

use super::linux_abi::{ACCESS_R_OK, ACCESS_W_OK, ACCESS_X_OK, CHOWN_ID_UNCHANGED};
use super::user_memory::{read_user_value, write_user_value};
use super::{UserProcess, neg_errno};

const NGROUPS_MAX: usize = 65_536;

impl UserProcess {
    pub(super) fn real_uid(&self) -> u32 {
        self.real_uid.load(Ordering::Acquire)
    }

    pub(super) fn uid(&self) -> u32 {
        self.uid.load(Ordering::Acquire)
    }

    pub(super) fn saved_uid(&self) -> u32 {
        self.saved_uid.load(Ordering::Acquire)
    }

    pub(super) fn fs_uid(&self) -> u32 {
        self.fs_uid.load(Ordering::Acquire)
    }

    pub(super) fn real_gid(&self) -> u32 {
        self.real_gid.load(Ordering::Acquire)
    }

    pub(super) fn gid(&self) -> u32 {
        self.gid.load(Ordering::Acquire)
    }

    pub(super) fn saved_gid(&self) -> u32 {
        self.saved_gid.load(Ordering::Acquire)
    }

    pub(super) fn fs_gid(&self) -> u32 {
        self.fs_gid.load(Ordering::Acquire)
    }

    pub(super) fn set_uid(&self, uid: u32) {
        self.real_uid.store(uid, Ordering::Release);
        self.uid.store(uid, Ordering::Release);
        self.saved_uid.store(uid, Ordering::Release);
        self.fs_uid.store(uid, Ordering::Release);
        self.mark_credentials_changed();
    }

    pub(super) fn set_gid(&self, gid: u32) {
        self.real_gid.store(gid, Ordering::Release);
        self.gid.store(gid, Ordering::Release);
        self.saved_gid.store(gid, Ordering::Release);
        self.fs_gid.store(gid, Ordering::Release);
        self.mark_credentials_changed();
    }

    pub(super) fn set_user_ids(
        &self,
        real: Option<u32>,
        effective: Option<u32>,
        saved: Option<u32>,
    ) {
        if let Some(uid) = real {
            self.real_uid.store(uid, Ordering::Release);
        }
        if let Some(uid) = effective {
            self.uid.store(uid, Ordering::Release);
            self.fs_uid.store(uid, Ordering::Release);
        }
        if let Some(uid) = saved {
            self.saved_uid.store(uid, Ordering::Release);
        }
        if real.is_some() || effective.is_some() || saved.is_some() {
            self.mark_credentials_changed();
        }
    }

    pub(super) fn set_group_ids(
        &self,
        real: Option<u32>,
        effective: Option<u32>,
        saved: Option<u32>,
    ) {
        if let Some(gid) = real {
            self.real_gid.store(gid, Ordering::Release);
        }
        if let Some(gid) = effective {
            self.gid.store(gid, Ordering::Release);
            self.fs_gid.store(gid, Ordering::Release);
        }
        if let Some(gid) = saved {
            self.saved_gid.store(gid, Ordering::Release);
        }
        if real.is_some() || effective.is_some() || saved.is_some() {
            self.mark_credentials_changed();
        }
    }

    pub(super) fn groups(&self) -> Vec<u32> {
        self.groups.lock().clone()
    }

    pub(super) fn set_groups(&self, groups: Vec<u32>) {
        *self.groups.lock() = groups;
        self.mark_credentials_changed();
    }

    pub(super) fn has_group(&self, gid: u32) -> bool {
        self.gid() == gid || self.groups.lock().contains(&gid)
    }

    fn mark_credentials_changed(&self) {
        self.credential_generation.fetch_add(1, Ordering::AcqRel);
    }
}

fn privileged(process: &UserProcess) -> bool {
    process.uid() == 0
}

fn uid_is_current(process: &UserProcess, uid: u32) -> bool {
    uid == process.real_uid() || uid == process.uid() || uid == process.saved_uid()
}

fn gid_is_current(process: &UserProcess, gid: u32) -> bool {
    gid == process.real_gid() || gid == process.gid() || gid == process.saved_gid()
}

fn uid_is_current_or_fs(process: &UserProcess, uid: u32) -> bool {
    uid_is_current(process, uid) || uid == process.fs_uid()
}

fn gid_is_current_or_fs(process: &UserProcess, gid: u32) -> bool {
    gid_is_current(process, gid) || gid == process.fs_gid()
}

fn requested_ids_allowed<const N: usize>(
    ids: &[Option<u32>; N],
    mut allowed: impl FnMut(u32) -> bool,
) -> bool {
    ids.iter().flatten().copied().all(|id| allowed(id))
}

fn set_single_id(id: usize) -> Result<u32, LinuxError> {
    u32::try_from(id).map_err(|_| LinuxError::EINVAL)
}

pub(super) fn id_arg_optional(id: usize) -> Result<Option<u32>, LinuxError> {
    if id == usize::MAX || id == CHOWN_ID_UNCHANGED as usize {
        return Ok(None);
    }
    u32::try_from(id).map(Some).map_err(|_| LinuxError::EINVAL)
}

fn parse_id_args<const N: usize>(ids: [usize; N]) -> Result<[Option<u32>; N], LinuxError> {
    let mut parsed = [None; N];
    for (dst, id) in parsed.iter_mut().zip(ids) {
        *dst = id_arg_optional(id)?;
    }
    Ok(parsed)
}

fn parse_re_ids(real: usize, effective: usize) -> Result<[Option<u32>; 2], LinuxError> {
    parse_id_args([real, effective])
}

fn parse_res_ids(
    real: usize,
    effective: usize,
    saved: usize,
) -> Result<[Option<u32>; 3], LinuxError> {
    parse_id_args([real, effective, saved])
}

pub(super) fn set_fs_id<F>(old: u32, id: usize, allow: bool, apply: F) -> isize
where
    F: FnOnce(u32),
{
    if let Ok(Some(id)) = id_arg_optional(id) {
        if allow {
            apply(id);
        }
    }
    old as isize
}

pub(super) fn write_id_triplet(process: &UserProcess, ptrs: [usize; 3], values: [u32; 3]) -> isize {
    for (ptr, value) in ptrs.into_iter().zip(values.into_iter()) {
        let ret = write_user_value(process, ptr, &value);
        if ret != 0 {
            return ret;
        }
    }
    0
}

pub(super) fn write_getgroups_response(
    process: &UserProcess,
    size: usize,
    list: usize,
    groups: &[u32],
) -> isize {
    if size > NGROUPS_MAX {
        return neg_errno(LinuxError::EINVAL);
    }
    if size == 0 {
        return groups.len() as isize;
    }
    if size < groups.len() {
        return neg_errno(LinuxError::EINVAL);
    }
    write_group_list(process, list, groups)
}

fn write_group_list(process: &UserProcess, list: usize, groups: &[u32]) -> isize {
    for (idx, gid) in groups.iter().enumerate() {
        let ret = write_user_value(process, list + idx * size_of::<u32>(), gid);
        if ret != 0 {
            return ret;
        }
    }
    groups.len() as isize
}

pub(super) fn read_group_list(
    process: &UserProcess,
    size: usize,
    list: usize,
) -> Result<Vec<u32>, LinuxError> {
    let mut groups = Vec::new();
    for idx in 0..size {
        groups.push(read_user_value::<u32>(
            process,
            list + idx * size_of::<u32>(),
        )?);
    }
    Ok(groups)
}

pub(super) fn sys_setuid(process: &UserProcess, uid: usize) -> isize {
    let uid = match set_single_id(uid) {
        Ok(uid) => uid,
        Err(err) => return neg_errno(err),
    };
    if privileged(process) {
        process.set_uid(uid);
    } else if uid_is_current(process, uid) {
        process.set_user_ids(None, Some(uid), None);
    } else {
        return neg_errno(LinuxError::EPERM);
    }
    0
}

pub(super) fn sys_setgid(process: &UserProcess, gid: usize) -> isize {
    let gid = match set_single_id(gid) {
        Ok(gid) => gid,
        Err(err) => return neg_errno(err),
    };
    if privileged(process) {
        process.set_gid(gid);
    } else if gid_is_current(process, gid) {
        process.set_group_ids(None, Some(gid), None);
    } else {
        return neg_errno(LinuxError::EPERM);
    }
    0
}

pub(super) fn sys_setreuid(process: &UserProcess, ruid: usize, euid: usize) -> isize {
    let [ruid, euid] = match parse_re_ids(ruid, euid) {
        Ok(ids) => ids,
        Err(err) => return neg_errno(err),
    };
    if !privileged(process)
        && !requested_ids_allowed(&[ruid, euid], |uid| uid_is_current(process, uid))
    {
        return neg_errno(LinuxError::EPERM);
    }

    let old_ruid = process.real_uid();
    let new_euid = euid.unwrap_or_else(|| process.uid());
    let saved = if ruid.is_some() || euid.is_some_and(|uid| uid != old_ruid) {
        Some(new_euid)
    } else {
        None
    };
    process.set_user_ids(ruid, euid, saved);
    0
}

pub(super) fn sys_setregid(process: &UserProcess, rgid: usize, egid: usize) -> isize {
    let [rgid, egid] = match parse_re_ids(rgid, egid) {
        Ok(ids) => ids,
        Err(err) => return neg_errno(err),
    };
    if !privileged(process)
        && !requested_ids_allowed(&[rgid, egid], |gid| gid_is_current(process, gid))
    {
        return neg_errno(LinuxError::EPERM);
    }

    let old_rgid = process.real_gid();
    let new_egid = egid.unwrap_or_else(|| process.gid());
    let saved = if rgid.is_some() || egid.is_some_and(|gid| gid != old_rgid) {
        Some(new_egid)
    } else {
        None
    };
    process.set_group_ids(rgid, egid, saved);
    0
}

pub(super) fn sys_setresuid(process: &UserProcess, ruid: usize, euid: usize, suid: usize) -> isize {
    let [ruid, euid, suid] = match parse_res_ids(ruid, euid, suid) {
        Ok(ids) => ids,
        Err(err) => return neg_errno(err),
    };
    if !privileged(process)
        && !requested_ids_allowed(&[ruid, euid, suid], |uid| uid_is_current(process, uid))
    {
        return neg_errno(LinuxError::EPERM);
    }
    process.set_user_ids(ruid, euid, suid);
    0
}

pub(super) fn sys_setresgid(process: &UserProcess, rgid: usize, egid: usize, sgid: usize) -> isize {
    let [rgid, egid, sgid] = match parse_res_ids(rgid, egid, sgid) {
        Ok(ids) => ids,
        Err(err) => return neg_errno(err),
    };
    if !privileged(process)
        && !requested_ids_allowed(&[rgid, egid, sgid], |gid| gid_is_current(process, gid))
    {
        return neg_errno(LinuxError::EPERM);
    }
    process.set_group_ids(rgid, egid, sgid);
    0
}

pub(super) fn sys_getresuid(process: &UserProcess, ruid: usize, euid: usize, suid: usize) -> isize {
    write_id_triplet(
        process,
        [ruid, euid, suid],
        [process.real_uid(), process.uid(), process.saved_uid()],
    )
}

pub(super) fn sys_getresgid(process: &UserProcess, rgid: usize, egid: usize, sgid: usize) -> isize {
    write_id_triplet(
        process,
        [rgid, egid, sgid],
        [process.real_gid(), process.gid(), process.saved_gid()],
    )
}

pub(super) fn sys_setfsuid(process: &UserProcess, uid: usize) -> isize {
    let old = process.fs_uid();
    let allow = id_arg_optional(uid)
        .ok()
        .flatten()
        .is_some_and(|uid| privileged(process) || uid_is_current_or_fs(process, uid));
    set_fs_id(old, uid, allow, |uid| {
        process.fs_uid.store(uid, Ordering::Release);
        process.mark_credentials_changed();
    })
}

pub(super) fn sys_setfsgid(process: &UserProcess, gid: usize) -> isize {
    let old = process.fs_gid();
    let allow = id_arg_optional(gid)
        .ok()
        .flatten()
        .is_some_and(|gid| privileged(process) || gid_is_current_or_fs(process, gid));
    set_fs_id(old, gid, allow, |gid| {
        process.fs_gid.store(gid, Ordering::Release);
        process.mark_credentials_changed();
    })
}

pub(super) fn sys_getgroups(process: &UserProcess, size: usize, list: usize) -> isize {
    let groups = process.groups();
    write_getgroups_response(process, size, list, &groups)
}

pub(super) fn sys_setgroups(process: &UserProcess, size: usize, list: usize) -> isize {
    if process.uid() != 0 {
        return neg_errno(LinuxError::EPERM);
    }
    if size > NGROUPS_MAX {
        return neg_errno(LinuxError::EINVAL);
    }
    let groups = match read_group_list(process, size, list) {
        Ok(groups) => groups,
        Err(err) => return neg_errno(err),
    };
    process.set_groups(groups);
    0
}

pub(super) fn access_allowed(st: &general::stat, mode: usize, uid: u32, gid: u32) -> bool {
    if mode == 0 {
        return true;
    }

    let permissions = (st.st_mode & 0o777) as u32;
    if uid == 0 {
        return (mode & ACCESS_X_OK == 0) || (permissions & 0o111 != 0);
    }

    let bits = if uid == st.st_uid as u32 {
        (permissions >> 6) & 0o7
    } else if gid == st.st_gid as u32 {
        (permissions >> 3) & 0o7
    } else {
        permissions & 0o7
    };

    if mode & ACCESS_R_OK != 0 && bits & 0o4 == 0 {
        return false;
    }
    if mode & ACCESS_W_OK != 0 && bits & 0o2 == 0 {
        return false;
    }
    if mode & ACCESS_X_OK != 0 && bits & 0o1 == 0 {
        return false;
    }
    true
}

pub(super) fn chown_ids(
    owner: usize,
    group: usize,
) -> Result<(Option<u32>, Option<u32>), LinuxError> {
    parse_id_args([owner, group]).map(|[owner, group]| (owner, group))
}

pub(super) fn apply_chown_metadata(
    process: &UserProcess,
    path: Option<String>,
    st: &general::stat,
    owner: Option<u32>,
    group: Option<u32>,
) -> isize {
    if !chown_allowed(process, st, owner, group) {
        return neg_errno(LinuxError::EPERM);
    }
    if let Some(path) = path {
        process.set_path_owner(path.clone(), owner, group);
        if owner.is_some() || group.is_some() {
            process.clear_path_chown_special_bits(path.as_str(), st.st_mode);
        }
    }
    0
}

fn chown_allowed(
    process: &UserProcess,
    st: &general::stat,
    owner: Option<u32>,
    group: Option<u32>,
) -> bool {
    if process.uid() == 0 {
        return true;
    }
    if let Some(owner) = owner {
        if owner != st.st_uid || owner != process.uid() {
            return false;
        }
    }
    if let Some(group) = group {
        if group != st.st_gid && !process.has_group(group) {
            return false;
        }
    }
    true
}
