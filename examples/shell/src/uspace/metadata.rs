use core::cmp;
use core::sync::atomic::Ordering;

use axalloc::global_allocator;
use axerrno::LinuxError;
use axfs::fops::{FileAttr, FileType, OpenOptions};
use linux_raw_sys::general;
use std::string::{String, ToString};
use std::vec::Vec;

use super::credentials::{access_allowed, apply_chown_metadata, chown_ids};
use super::fd_table::{resolve_dirfd_path, FdEntry};
use super::linux_abi::{
    neg_errno, neg_errno_code, ACCESS_MODE_MASK, ACCESS_W_OK, DEVFS_MAGIC, FILE_MODE_GROUP_EXECUTE,
    FILE_MODE_PERMISSION_MASK, FILE_MODE_SET_GID, FILE_MODE_SET_UID, LINUX_EACCES,
    MAX_IN_MEMORY_FILE_SIZE, PIPEFS_MAGIC, PROC_SUPER_MAGIC, RLIMIT_FSIZE_RESOURCE,
    STATFS_BLOCK_SIZE, STATFS_NAME_MAX, ST_MODE_BLK, ST_MODE_CHR, ST_MODE_DIR, ST_MODE_FIFO,
    ST_MODE_FILE, ST_MODE_LNK, ST_MODE_SOCKET, ST_MODE_TYPE_MASK, SYSFS_MAGIC, TMPFS_MAGIC,
};
use super::runtime_paths::normalize_path;
use super::synthetic_fs::{dev_shm_host_path, proc_exe_link_target};
use super::user_memory::{read_cstr, read_user_bytes, write_user_bytes, write_user_value};
use super::UserProcess;

const DEV_NULL_RDEV: u64 = 259; // Linux makedev(1, 3).
const DEV_VDA_RDEV: u64 = 65_024; // Linux makedev(254, 0), virtio block.
const DEV_SDA_RDEV: u64 = 2_048; // Linux makedev(8, 0).
const DEV_XVDA_RDEV: u64 = 51_712; // Linux makedev(202, 0).
const LINUX_PATH_MAX: usize = 4096;
const XATTR_CREATE: usize = 0x1;
const XATTR_REPLACE: usize = 0x2;

pub(super) fn file_attr_to_stat(attr: &FileAttr, path: Option<&str>) -> general::stat {
    let st_mode = file_type_mode(attr.file_type()) | attr.perm().bits() as u32;
    let mut st: general::stat = unsafe { core::mem::zeroed() };
    st.st_dev = 1;
    st.st_ino = path_inode(path);
    st.st_mode = st_mode;
    st.st_nlink = 1;
    st.st_size = attr.size() as _;
    st.st_blksize = 512;
    st.st_blocks = attr.blocks() as _;
    st
}

pub(super) fn normalize_file_mode(mode: u32) -> u32 {
    mode & FILE_MODE_PERMISSION_MASK
}

impl UserProcess {
    pub(super) fn set_path_mode(&self, path: String, mode: u32) {
        self.path_modes
            .lock()
            .insert(path, normalize_file_mode(mode));
    }

    pub(super) fn path_mode(&self, path: &str) -> Option<u32> {
        self.path_modes.lock().get(path).copied()
    }

    pub(super) fn set_path_special_mode(&self, path: String, ty: u32) {
        self.path_special_modes
            .lock()
            .insert(path, ty & ST_MODE_TYPE_MASK);
    }

    pub(super) fn remove_path_special_mode(&self, path: &str) {
        self.path_special_modes.lock().remove(path);
    }

    pub(super) fn path_special_mode(&self, path: &str) -> Option<u32> {
        self.path_special_modes.lock().get(path).copied()
    }

    pub(super) fn set_path_rdev(&self, path: String, rdev: u64) {
        self.path_rdevs.lock().insert(path, rdev);
    }

    pub(super) fn remove_path_rdev(&self, path: &str) {
        self.path_rdevs.lock().remove(path);
    }

    pub(super) fn path_rdev(&self, path: &str) -> Option<u64> {
        self.path_rdevs.lock().get(path).copied()
    }

    pub(super) fn apply_umask(&self, mode: u32) -> u32 {
        mode & !self.umask.load(Ordering::Acquire)
    }

    pub(super) fn replace_umask(&self, mask: u32) -> u32 {
        self.umask
            .swap(mask & FILE_MODE_PERMISSION_MASK, Ordering::AcqRel)
            & FILE_MODE_PERMISSION_MASK
    }

    pub(super) fn set_path_owner(&self, path: String, owner: Option<u32>, group: Option<u32>) {
        let mut path_owners = self.path_owners.lock();
        let (current_owner, current_group) =
            path_owners.get(path.as_str()).copied().unwrap_or((0, 0));
        path_owners.insert(
            path,
            (
                owner.unwrap_or(current_owner),
                group.unwrap_or(current_group),
            ),
        );
    }

    pub(super) fn path_owner(&self, path: &str) -> Option<(u32, u32)> {
        self.path_owners.lock().get(path).copied()
    }

    pub(super) fn set_path_symlink(&self, path: String, target: String) {
        self.path_symlinks.lock().insert(path, target);
    }

    pub(super) fn remove_path_symlink(&self, path: &str) -> bool {
        self.path_symlinks.lock().remove(path).is_some()
    }

    pub(super) fn path_symlink(&self, path: &str) -> Option<String> {
        self.path_symlinks.lock().get(path).cloned()
    }

    pub(super) fn path_symlink_names_in_dir(&self, dir: &str) -> Vec<String> {
        let prefix = if dir == "/" {
            String::from("/")
        } else {
            let mut prefix = dir.trim_end_matches('/').to_string();
            prefix.push('/');
            prefix
        };
        self.path_symlinks
            .lock()
            .keys()
            .filter_map(|path| {
                let name = path.strip_prefix(prefix.as_str())?;
                if name.is_empty() || name.contains('/') {
                    None
                } else {
                    Some(name.to_string())
                }
            })
            .collect()
    }

    pub(super) fn resolve_path_symlink(&self, path: &str) -> Result<Option<String>, LinuxError> {
        let mut current = path.to_string();
        for _ in 0..40 {
            let Some(target) = self.path_symlink(current.as_str()) else {
                return Ok((current != path).then_some(current));
            };
            current = normalize_symlink_target(current.as_str(), target.as_str())
                .ok_or(LinuxError::EINVAL)?;
        }
        Err(LinuxError::ELOOP)
    }

    pub(super) fn path_symlink_stat(&self, path: &str) -> Option<general::stat> {
        let target = self.path_symlink(path)?;
        let mut st: general::stat = unsafe { core::mem::zeroed() };
        st.st_dev = 1;
        st.st_ino = path_inode(Some(path));
        st.st_mode = ST_MODE_LNK | 0o777;
        st.st_nlink = 1;
        st.st_size = target.len() as _;
        st.st_blksize = 512;
        st.st_blocks = 0;
        if let Some((uid, gid)) = self.path_owner(path) {
            st.st_uid = uid;
            st.st_gid = gid;
        }
        Some(st)
    }

    fn set_path_xattr(
        &self,
        path: String,
        name: String,
        value: Vec<u8>,
        flags: usize,
    ) -> Result<(), LinuxError> {
        if flags & !(XATTR_CREATE | XATTR_REPLACE) != 0 {
            return Err(LinuxError::EINVAL);
        }
        let mut all_xattrs = self.path_xattrs.lock();
        let attrs = all_xattrs.entry(path).or_default();
        let exists = attrs.contains_key(name.as_str());
        if flags & XATTR_CREATE != 0 && exists {
            return Err(LinuxError::EEXIST);
        }
        if flags & XATTR_REPLACE != 0 && !exists {
            return Err(LinuxError::ENODATA);
        }
        attrs.insert(name, value);
        Ok(())
    }

    fn get_path_xattr(&self, path: &str, name: &str) -> Result<Vec<u8>, LinuxError> {
        self.path_xattrs
            .lock()
            .get(path)
            .and_then(|attrs| attrs.get(name).cloned())
            .ok_or(LinuxError::ENODATA)
    }

    fn list_path_xattrs(&self, path: &str) -> Vec<u8> {
        let mut out = Vec::new();
        if let Some(attrs) = self.path_xattrs.lock().get(path) {
            for name in attrs.keys() {
                out.extend_from_slice(name.as_bytes());
                out.push(0);
            }
        }
        out
    }

    fn remove_path_xattr(&self, path: &str, name: &str) -> Result<(), LinuxError> {
        let mut all_xattrs = self.path_xattrs.lock();
        let attrs = all_xattrs.get_mut(path).ok_or(LinuxError::ENODATA)?;
        if attrs.remove(name).is_none() {
            return Err(LinuxError::ENODATA);
        }
        if attrs.is_empty() {
            all_xattrs.remove(path);
        }
        Ok(())
    }

    pub(super) fn set_path_sparse_size(&self, path: String, size: u64) {
        self.path_sparse_sizes.lock().insert(path, size);
    }

    pub(super) fn path_sparse_size(&self, path: &str) -> Option<u64> {
        self.path_sparse_sizes.lock().get(path).copied()
    }

    pub(super) fn clear_path_sparse_file(&self, path: &str) {
        self.path_sparse_sizes.lock().remove(path);
        self.path_sparse_data.lock().remove(path);
    }

    pub(super) fn move_path_sparse_file(&self, old_path: &str, new_path: String) {
        if let Some(size) = self.path_sparse_sizes.lock().remove(old_path) {
            self.path_sparse_sizes.lock().insert(new_path.clone(), size);
        }
        if let Some(data) = self.path_sparse_data.lock().remove(old_path) {
            self.path_sparse_data.lock().insert(new_path, data);
        }
    }

    pub(super) fn truncate_path_sparse_file(&self, path: String, size: u64) {
        self.path_sparse_sizes.lock().insert(path.clone(), size);

        let mut all_data = self.path_sparse_data.lock();
        let remove_empty = if let Some(extents) = all_data.get_mut(path.as_str()) {
            let mut retained = Vec::new();
            for (offset, data) in extents.drain(..) {
                if offset >= size {
                    continue;
                }
                let keep = cmp::min(data.len(), size.saturating_sub(offset) as usize);
                if keep > 0 {
                    retained.push((offset, data[..keep].to_vec()));
                }
            }
            if retained.is_empty() {
                true
            } else {
                *extents = retained;
                false
            }
        } else {
            false
        };
        if remove_empty {
            all_data.remove(path.as_str());
        }
    }

    pub(super) fn write_path_sparse_data(&self, path: String, offset: u64, data: &[u8]) {
        let end = offset.saturating_add(data.len() as u64);
        let logical_size = self.path_sparse_size(path.as_str()).unwrap_or(0).max(end);
        self.set_path_sparse_size(path.clone(), logical_size);
        if data.is_empty() {
            return;
        }

        let mut all_data = self.path_sparse_data.lock();
        let extents = all_data.entry(path).or_default();
        let mut retained = Vec::new();
        for (existing_offset, existing_data) in extents.drain(..) {
            let existing_end = existing_offset.saturating_add(existing_data.len() as u64);
            if existing_end <= offset || existing_offset >= end {
                retained.push((existing_offset, existing_data));
                continue;
            }
            if existing_offset < offset {
                let keep = offset.saturating_sub(existing_offset) as usize;
                retained.push((existing_offset, existing_data[..keep].to_vec()));
            }
            if existing_end > end {
                let skip = end.saturating_sub(existing_offset) as usize;
                retained.push((end, existing_data[skip..].to_vec()));
            }
        }
        retained.push((offset, data.to_vec()));
        retained.sort_by_key(|(extent_offset, _)| *extent_offset);
        *extents = retained;
    }

    pub(super) fn copy_path_sparse_data(&self, path: &str, offset: u64, dst: &mut [u8]) {
        if dst.is_empty() {
            return;
        }
        let end = offset.saturating_add(dst.len() as u64);
        let all_data = self.path_sparse_data.lock();
        let Some(extents) = all_data.get(path) else {
            return;
        };
        for (extent_offset, data) in extents {
            let extent_end = extent_offset.saturating_add(data.len() as u64);
            if extent_end <= offset || *extent_offset >= end {
                continue;
            }
            let copy_start = (*extent_offset).max(offset);
            let copy_end = extent_end.min(end);
            let dst_start = copy_start.saturating_sub(offset) as usize;
            let src_start = copy_start.saturating_sub(*extent_offset) as usize;
            let len = copy_end.saturating_sub(copy_start) as usize;
            dst[dst_start..dst_start + len].copy_from_slice(&data[src_start..src_start + len]);
        }
    }

    pub(super) fn clear_path_chown_special_bits(&self, path: &str, current_mode: u32) {
        let mode = self
            .path_mode(path)
            .unwrap_or(current_mode & FILE_MODE_PERMISSION_MASK);
        let mut updated_mode = mode & !FILE_MODE_SET_UID;
        if mode & FILE_MODE_GROUP_EXECUTE != 0 {
            updated_mode &= !FILE_MODE_SET_GID;
        }
        self.set_path_mode(path.to_string(), updated_mode);
    }
}

fn normalize_symlink_target(link_path: &str, target: &str) -> Option<String> {
    if target.starts_with('/') {
        normalize_path("/", target)
    } else {
        let parent = link_path
            .rsplit_once('/')
            .map(|(parent, _)| parent)
            .unwrap_or("/");
        let parent = if parent.is_empty() { "/" } else { parent };
        normalize_path(parent, target)
    }
}

pub(super) fn apply_recorded_path_metadata(
    process: &UserProcess,
    path: &str,
    mut st: general::stat,
) -> general::stat {
    if let Some(ty) = process.path_special_mode(path) {
        st.st_mode = (st.st_mode & !ST_MODE_TYPE_MASK) | ty;
        if matches!(ty, ST_MODE_CHR | ST_MODE_BLK) {
            st.st_rdev = process.path_rdev(path).unwrap_or(0) as _;
        }
    }
    if let Some(mode) = process.path_mode(path) {
        st.st_mode = (st.st_mode & !FILE_MODE_PERMISSION_MASK) | mode;
    }
    if let Some((uid, gid)) = process.path_owner(path) {
        st.st_uid = uid;
        st.st_gid = gid;
    }
    if let Some(size) = process.path_sparse_size(path) {
        st.st_size = size.min(i64::MAX as u64) as _;
    }
    st
}

pub(super) fn canonical_permission_path(path: String) -> String {
    dev_shm_host_path(path.as_str()).unwrap_or(path)
}

pub(super) fn fd_entry_path(entry: &FdEntry) -> Option<&str> {
    match entry {
        FdEntry::BlockDevice(dev) => Some(dev.path.as_str()),
        FdEntry::File(file) => Some(file.path.as_str()),
        FdEntry::Directory(dir) => Some(dir.path.as_str()),
        FdEntry::ProcFdDir(dir) => Some(dir.path.as_str()),
        FdEntry::Path(path) => Some(path.path.as_str()),
        FdEntry::MemoryFile(file) => Some(file.path.as_str()),
        FdEntry::ProcPagemap(file) => Some(file.path.as_str()),
        _ => None,
    }
}

pub(super) fn sys_faccessat(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    mode: usize,
    flags: usize,
) -> isize {
    if mode & !ACCESS_MODE_MASK != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let supported_flags =
        (general::AT_EACCESS | general::AT_SYMLINK_NOFOLLOW | general::AT_EMPTY_PATH) as usize;
    if flags & !supported_flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let mut fds = process.fds.lock();
    let (resolved_path, stat, parents_already_reached) = if path.is_empty() {
        if flags & general::AT_EMPTY_PATH as usize == 0 {
            return neg_errno(LinuxError::ENOENT);
        }
        if dirfd as i32 == general::AT_FDCWD {
            match fds.path_stat(process, general::AT_FDCWD, ".") {
                Ok((path, stat)) => (path, stat, false),
                Err(err) => return neg_errno(err),
            }
        } else {
            match fds.stat_with_recorded_path(process, dirfd as i32) {
                Ok((path, stat)) => (path.unwrap_or_default(), stat, true),
                Err(err) => return neg_errno(err),
            }
        }
    } else {
        match fds.path_stat(process, dirfd as i32, path.as_str()) {
            Ok((path, stat)) => (path, stat, false),
            Err(err) => return neg_errno(err),
        }
    };
    let use_effective_ids = flags & general::AT_EACCESS as usize != 0;
    let uid = if use_effective_ids {
        process.uid()
    } else {
        process.real_uid()
    };
    let gid = if use_effective_ids {
        process.gid()
    } else {
        process.real_gid()
    };
    let parents_searchable = if parents_already_reached || resolved_path.is_empty() {
        true
    } else {
        match fds.parent_dirs_searchable(process, resolved_path.as_str(), uid, gid) {
            Ok(searchable) => searchable,
            Err(err) => return neg_errno(err),
        }
    };
    if !parents_searchable {
        return neg_errno_code(LINUX_EACCES);
    }
    if mode & ACCESS_W_OK != 0 && process.path_on_readonly_mount(resolved_path.as_str()) {
        return neg_errno(LinuxError::EROFS);
    }
    if access_allowed(&stat, mode, uid, gid) {
        0
    } else {
        neg_errno_code(LINUX_EACCES)
    }
}

pub(super) fn sys_truncate(process: &UserProcess, pathname: usize, length: usize) -> isize {
    let length = length as isize;
    if length < 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let length = length as u64;
    if length > process.get_rlimit(RLIMIT_FSIZE_RESOURCE).current() {
        return neg_errno(LinuxError::EFBIG);
    }
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    if path.is_empty() {
        return neg_errno(LinuxError::ENOENT);
    }
    if path.len() >= LINUX_PATH_MAX {
        return neg_errno(LinuxError::ENAMETOOLONG);
    }

    let resolved_path = {
        let table = process.fds.lock();
        match resolve_dirfd_path(process, &table, general::AT_FDCWD, path.as_str()) {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        }
    };
    let target_path = match process.resolve_path_symlink(resolved_path.as_str()) {
        Ok(Some(target)) => target,
        Ok(None) => resolved_path,
        Err(err) => return neg_errno(err),
    };
    let target_path = dev_shm_host_path(target_path.as_str()).unwrap_or(target_path);
    let st = match {
        let mut fds = process.fds.lock();
        fds.stat_path(process, general::AT_FDCWD, target_path.as_str())
    } {
        Ok(st) => st,
        Err(err) => return neg_errno(err),
    };
    if st.st_mode & ST_MODE_TYPE_MASK == ST_MODE_DIR {
        return neg_errno(LinuxError::EISDIR);
    }
    if process.path_on_readonly_mount(target_path.as_str()) {
        return neg_errno(LinuxError::EROFS);
    }
    if !access_allowed(&st, ACCESS_W_OK, process.fs_uid(), process.fs_gid()) {
        return neg_errno(LinuxError::EACCES);
    }

    let mut opts = OpenOptions::new();
    opts.write(true);
    let file = match axfs::fops::File::open(target_path.as_str(), &opts) {
        Ok(file) => file,
        Err(err) => return neg_errno(LinuxError::from(err)),
    };
    if length <= MAX_IN_MEMORY_FILE_SIZE {
        if let Err(err) = file.truncate(length) {
            return neg_errno(LinuxError::from(err));
        }
    }
    process.truncate_path_sparse_file(target_path, length);
    0
}

pub(super) fn sys_symlinkat(
    process: &UserProcess,
    target: usize,
    newdirfd: usize,
    linkpath: usize,
) -> isize {
    let target = match read_cstr(process, target) {
        Ok(target) => target,
        Err(err) => return neg_errno(err),
    };
    let linkpath = match read_cstr(process, linkpath) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    if target.is_empty() || linkpath.is_empty() {
        return neg_errno(LinuxError::ENOENT);
    }
    if target.len() >= LINUX_PATH_MAX || linkpath.len() >= LINUX_PATH_MAX {
        return neg_errno(LinuxError::ENAMETOOLONG);
    }
    let resolved_path = {
        let table = process.fds.lock();
        match resolve_dirfd_path(process, &table, newdirfd as i32, linkpath.as_str()) {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        }
    };
    if process.path_symlink(resolved_path.as_str()).is_some()
        || axfs::api::metadata(resolved_path.as_str()).is_ok()
    {
        return neg_errno(LinuxError::EEXIST);
    }
    process.set_path_symlink(resolved_path, target);
    0
}

pub(super) fn sys_umask(process: &UserProcess, mask: usize) -> isize {
    process.replace_umask(mask as u32) as isize
}

pub(super) fn sys_fchmod(process: &UserProcess, fd: usize, mode: usize) -> isize {
    let (path, st) = {
        let mut fds = process.fds.lock();
        if matches!(fds.entry(fd as i32), Ok(FdEntry::Path(_))) {
            return neg_errno(LinuxError::EBADF);
        }
        match fds.stat_with_recorded_path(process, fd as i32) {
            Ok((path, st)) => (path, st),
            Err(err) => return neg_errno(err),
        }
    };
    if let Some(path) = path.as_deref() {
        if process.path_on_readonly_mount(path) {
            return neg_errno(LinuxError::EROFS);
        }
    }
    if !chmod_permission_allowed(process, &st) {
        return neg_errno(LinuxError::EPERM);
    }
    if let Some(path) = path {
        process.set_path_mode(path, chmod_effective_mode(process, &st, mode as u32));
    }
    0
}

pub(super) fn sys_fchmodat(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    mode: usize,
    flags: usize,
) -> isize {
    let flags = flags as u32;
    let supported_flags = general::AT_SYMLINK_NOFOLLOW | general::AT_EMPTY_PATH;
    if flags & !supported_flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }

    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    if path.len() >= LINUX_PATH_MAX {
        return neg_errno(LinuxError::ENAMETOOLONG);
    }
    let mode = mode as u32;
    if let Some(fd) = proc_self_fd_number(path.as_str()) {
        let mut fds = process.fds.lock();
        if matches!(fds.entry(fd), Ok(FdEntry::Path(_))) {
            return neg_errno(LinuxError::EBADF);
        }
        let (path, st) = match fds.stat_with_recorded_path(process, fd) {
            Ok((path, st)) => (path, st),
            Err(err) => return neg_errno(err),
        };
        if let Some(path) = path.as_deref() {
            if process.path_on_readonly_mount(path) {
                return neg_errno(LinuxError::EROFS);
            }
        }
        if !chmod_permission_allowed(process, &st) {
            return neg_errno(LinuxError::EPERM);
        }
        if let Some(path) = path {
            process.set_path_mode(path, chmod_effective_mode(process, &st, mode));
        }
        return 0;
    }
    if path.is_empty() {
        if matches!(process.fds.lock().entry(dirfd as i32), Ok(FdEntry::Path(_))) {
            return neg_errno(LinuxError::EBADF);
        }
        if flags & general::AT_EMPTY_PATH == 0 {
            return neg_errno(LinuxError::ENOENT);
        }
        if dirfd as i32 == general::AT_FDCWD {
            let cwd = process.cwd();
            return match axfs::api::metadata(cwd.as_str()) {
                Ok(_) => {
                    let mut fds = process.fds.lock();
                    let st = match fds.stat_path(process, general::AT_FDCWD, ".") {
                        Ok(st) => st,
                        Err(err) => return neg_errno(err),
                    };
                    if process.path_on_readonly_mount(cwd.as_str()) {
                        return neg_errno(LinuxError::EROFS);
                    }
                    if !chmod_permission_allowed(process, &st) {
                        return neg_errno(LinuxError::EPERM);
                    }
                    process.set_path_mode(cwd, chmod_effective_mode(process, &st, mode));
                    0
                }
                Err(err) => neg_errno(LinuxError::from(err)),
            };
        }
        let (path, st) = {
            let mut fds = process.fds.lock();
            if matches!(fds.entry(dirfd as i32), Ok(FdEntry::Path(_))) {
                return neg_errno(LinuxError::EBADF);
            }
            match fds.stat_with_recorded_path(process, dirfd as i32) {
                Ok((path, st)) => (path, st),
                Err(err) => return neg_errno(err),
            }
        };
        if let Some(path) = path.as_deref() {
            if process.path_on_readonly_mount(path) {
                return neg_errno(LinuxError::EROFS);
            }
        }
        if !chmod_permission_allowed(process, &st) {
            return neg_errno(LinuxError::EPERM);
        }
        if let Some(path) = path {
            process.set_path_mode(path, chmod_effective_mode(process, &st, mode));
        }
        return 0;
    }

    let mut fds = process.fds.lock();
    let resolved_path = match fds.resolve_path(process, dirfd as i32, path.as_str()) {
        Ok(path) => path,
        Err(err) => {
            return neg_errno(err);
        }
    };
    match fds.parent_dirs_searchable(
        process,
        resolved_path.as_str(),
        process.fs_uid(),
        process.fs_gid(),
    ) {
        Ok(true) => {}
        Ok(false) => return neg_errno(LinuxError::EACCES),
        Err(err) => return neg_errno(err),
    }
    if axfs::api::metadata(resolved_path.as_str()).is_ok() {
        let st = match fds.stat_path(process, dirfd as i32, path.as_str()) {
            Ok(st) => st,
            Err(err) => return neg_errno(err),
        };
        if process.path_on_readonly_mount(resolved_path.as_str()) {
            return neg_errno(LinuxError::EROFS);
        }
        if !chmod_permission_allowed(process, &st) {
            return neg_errno(LinuxError::EPERM);
        }
        process.set_path_mode(resolved_path, chmod_effective_mode(process, &st, mode));
        return 0;
    }
    match fds.stat_path(process, dirfd as i32, path.as_str()) {
        Ok(st) => {
            if process.path_on_readonly_mount(resolved_path.as_str()) {
                return neg_errno(LinuxError::EROFS);
            }
            if !chmod_permission_allowed(process, &st) {
                return neg_errno(LinuxError::EPERM);
            }
            process.set_path_mode(resolved_path, chmod_effective_mode(process, &st, mode));
            0
        }
        Err(err) => neg_errno(err),
    }
}

fn chmod_permission_allowed(process: &UserProcess, st: &general::stat) -> bool {
    process.uid() == 0 || st.st_uid as u32 == process.uid()
}

fn chmod_effective_mode(process: &UserProcess, st: &general::stat, mode: u32) -> u32 {
    let mut mode = mode;
    if process.uid() != 0
        && st.st_mode & ST_MODE_DIR != 0
        && mode & FILE_MODE_SET_GID != 0
        && !process.has_group(st.st_gid)
    {
        mode &= !FILE_MODE_SET_GID;
    }
    mode
}

pub(super) fn sys_fchown(process: &UserProcess, fd: usize, owner: usize, group: usize) -> isize {
    let (owner, group) = match chown_ids(owner, group) {
        Ok(ids) => ids,
        Err(err) => return neg_errno(err),
    };
    let (path, st) = {
        let mut fds = process.fds.lock();
        if matches!(fds.entry(fd as i32), Ok(FdEntry::Path(_))) {
            return neg_errno(LinuxError::EBADF);
        }
        match fds.stat_with_recorded_path(process, fd as i32) {
            Ok((path, st)) => (path, st),
            Err(err) => return neg_errno(err),
        }
    };
    apply_chown_metadata(process, path, &st, owner, group)
}

fn resolve_xattr_path(
    process: &UserProcess,
    dirfd: i32,
    path: &str,
    follow_symlink: bool,
) -> Result<String, LinuxError> {
    if path.is_empty() {
        return Err(LinuxError::ENOENT);
    }
    if path.len() >= LINUX_PATH_MAX {
        return Err(LinuxError::ENAMETOOLONG);
    }
    let mut fds = process.fds.lock();
    let resolved_path = fds.resolve_path(process, dirfd, path)?;
    if !follow_symlink && process.path_symlink_stat(resolved_path.as_str()).is_some() {
        return Ok(resolved_path);
    }
    let (record_path, _) = fds.path_stat(process, dirfd, path)?;
    Ok(record_path)
}

fn resolve_fd_xattr_path(process: &UserProcess, fd: i32) -> Result<String, LinuxError> {
    let mut fds = process.fds.lock();
    if matches!(fds.entry(fd), Ok(FdEntry::Path(_))) {
        return Err(LinuxError::EBADF);
    }
    let (path, _) = fds.stat_with_recorded_path(process, fd)?;
    path.ok_or(LinuxError::ENODATA)
}

fn read_xattr_name(process: &UserProcess, name: usize) -> Result<String, LinuxError> {
    if name == 0 {
        return Err(LinuxError::EFAULT);
    }
    let name = read_cstr(process, name)?;
    if name.is_empty() {
        return Err(LinuxError::ERANGE);
    }
    Ok(name)
}

fn sys_setxattr_for_path(
    process: &UserProcess,
    path: String,
    name: usize,
    value: usize,
    size: usize,
    flags: usize,
) -> isize {
    let name = match read_xattr_name(process, name) {
        Ok(name) => name,
        Err(err) => return neg_errno(err),
    };
    let value = match read_user_bytes(process, value, size) {
        Ok(value) => value,
        Err(err) => return neg_errno(err),
    };
    process
        .set_path_xattr(path, name, value, flags)
        .map_or_else(neg_errno, |_| 0)
}

fn sys_getxattr_for_path(
    process: &UserProcess,
    path: String,
    name: usize,
    value: usize,
    size: usize,
) -> isize {
    let name = match read_xattr_name(process, name) {
        Ok(name) => name,
        Err(err) => return neg_errno(err),
    };
    let bytes = match process.get_path_xattr(path.as_str(), name.as_str()) {
        Ok(bytes) => bytes,
        Err(err) => return neg_errno(err),
    };
    if value == 0 || size == 0 {
        return bytes.len() as isize;
    }
    if size < bytes.len() {
        return neg_errno(LinuxError::ERANGE);
    }
    write_user_bytes(process, value, bytes.as_slice())
        .map_or_else(neg_errno, |_| bytes.len() as isize)
}

fn sys_listxattr_for_path(process: &UserProcess, path: String, list: usize, size: usize) -> isize {
    let bytes = process.list_path_xattrs(path.as_str());
    if list == 0 || size == 0 {
        return bytes.len() as isize;
    }
    if size < bytes.len() {
        return neg_errno(LinuxError::ERANGE);
    }
    write_user_bytes(process, list, bytes.as_slice())
        .map_or_else(neg_errno, |_| bytes.len() as isize)
}

fn sys_removexattr_for_path(process: &UserProcess, path: String, name: usize) -> isize {
    let name = match read_xattr_name(process, name) {
        Ok(name) => name,
        Err(err) => return neg_errno(err),
    };
    process
        .remove_path_xattr(path.as_str(), name.as_str())
        .map_or_else(neg_errno, |_| 0)
}

pub(super) fn sys_setxattr(
    process: &UserProcess,
    pathname: usize,
    name: usize,
    value: usize,
    size: usize,
    flags: usize,
) -> isize {
    let pathname = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let path = match resolve_xattr_path(process, general::AT_FDCWD, pathname.as_str(), true) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_setxattr_for_path(process, path, name, value, size, flags)
}

pub(super) fn sys_lsetxattr(
    process: &UserProcess,
    pathname: usize,
    name: usize,
    value: usize,
    size: usize,
    flags: usize,
) -> isize {
    let pathname = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let path = match resolve_xattr_path(process, general::AT_FDCWD, pathname.as_str(), false) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_setxattr_for_path(process, path, name, value, size, flags)
}

pub(super) fn sys_fsetxattr(
    process: &UserProcess,
    fd: usize,
    name: usize,
    value: usize,
    size: usize,
    flags: usize,
) -> isize {
    let path = match resolve_fd_xattr_path(process, fd as i32) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_setxattr_for_path(process, path, name, value, size, flags)
}

pub(super) fn sys_getxattr(
    process: &UserProcess,
    pathname: usize,
    name: usize,
    value: usize,
    size: usize,
) -> isize {
    let pathname = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let path = match resolve_xattr_path(process, general::AT_FDCWD, pathname.as_str(), true) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_getxattr_for_path(process, path, name, value, size)
}

pub(super) fn sys_lgetxattr(
    process: &UserProcess,
    pathname: usize,
    name: usize,
    value: usize,
    size: usize,
) -> isize {
    let pathname = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let path = match resolve_xattr_path(process, general::AT_FDCWD, pathname.as_str(), false) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_getxattr_for_path(process, path, name, value, size)
}

pub(super) fn sys_fgetxattr(
    process: &UserProcess,
    fd: usize,
    name: usize,
    value: usize,
    size: usize,
) -> isize {
    let path = match resolve_fd_xattr_path(process, fd as i32) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_getxattr_for_path(process, path, name, value, size)
}

pub(super) fn sys_listxattr(
    process: &UserProcess,
    pathname: usize,
    list: usize,
    size: usize,
) -> isize {
    let pathname = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let path = match resolve_xattr_path(process, general::AT_FDCWD, pathname.as_str(), true) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_listxattr_for_path(process, path, list, size)
}

pub(super) fn sys_llistxattr(
    process: &UserProcess,
    pathname: usize,
    list: usize,
    size: usize,
) -> isize {
    let pathname = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let path = match resolve_xattr_path(process, general::AT_FDCWD, pathname.as_str(), false) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_listxattr_for_path(process, path, list, size)
}

pub(super) fn sys_flistxattr(process: &UserProcess, fd: usize, list: usize, size: usize) -> isize {
    let path = match resolve_fd_xattr_path(process, fd as i32) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_listxattr_for_path(process, path, list, size)
}

pub(super) fn sys_removexattr(process: &UserProcess, pathname: usize, name: usize) -> isize {
    let pathname = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let path = match resolve_xattr_path(process, general::AT_FDCWD, pathname.as_str(), true) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_removexattr_for_path(process, path, name)
}

pub(super) fn sys_lremovexattr(process: &UserProcess, pathname: usize, name: usize) -> isize {
    let pathname = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let path = match resolve_xattr_path(process, general::AT_FDCWD, pathname.as_str(), false) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_removexattr_for_path(process, path, name)
}

pub(super) fn sys_fremovexattr(process: &UserProcess, fd: usize, name: usize) -> isize {
    let path = match resolve_fd_xattr_path(process, fd as i32) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    sys_removexattr_for_path(process, path, name)
}

pub(super) fn sys_fchownat(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    owner: usize,
    group: usize,
    flags: usize,
) -> isize {
    let flags = flags as u32;
    let supported_flags = general::AT_SYMLINK_NOFOLLOW
        | general::AT_NO_AUTOMOUNT
        | general::AT_EMPTY_PATH
        | general::AT_STATX_SYNC_TYPE;
    if flags & !supported_flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let (owner, group) = match chown_ids(owner, group) {
        Ok(ids) => ids,
        Err(err) => return neg_errno(err),
    };
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    if let Some(fd) = proc_self_fd_number(path.as_str()) {
        let mut fds = process.fds.lock();
        if matches!(fds.entry(fd), Ok(FdEntry::Path(_))) {
            return neg_errno(LinuxError::EBADF);
        }
        let (path, st) = match fds.stat_with_recorded_path(process, fd) {
            Ok((path, st)) => (path, st),
            Err(err) => return neg_errno(err),
        };
        return apply_chown_metadata(process, path, &st, owner, group);
    }
    let (record_path, st) = if path.is_empty() {
        if matches!(process.fds.lock().entry(dirfd as i32), Ok(FdEntry::Path(_))) {
            return neg_errno(LinuxError::EBADF);
        }
        if flags & general::AT_EMPTY_PATH == 0 {
            return neg_errno(LinuxError::ENOENT);
        }
        if dirfd as i32 == general::AT_FDCWD {
            let cwd = process.cwd();
            let st = match process
                .fds
                .lock()
                .stat_path(process, general::AT_FDCWD, ".")
            {
                Ok(st) => st,
                Err(err) => return neg_errno(err),
            };
            (Some(cwd), st)
        } else {
            match process
                .fds
                .lock()
                .stat_with_recorded_path(process, dirfd as i32)
            {
                Ok((path, st)) => (path, st),
                Err(err) => return neg_errno(err),
            }
        }
    } else {
        let mut fds = process.fds.lock();
        let (resolved_path, st) = if flags & general::AT_SYMLINK_NOFOLLOW != 0 {
            let resolved_path = match resolve_dirfd_path(process, &fds, dirfd as i32, path.as_str())
            {
                Ok(path) => path,
                Err(err) => return neg_errno(err),
            };
            let st = match process.path_symlink_stat(resolved_path.as_str()) {
                Some(st) => st,
                None => match fds.stat_path(process, dirfd as i32, path.as_str()) {
                    Ok(st) => st,
                    Err(err) => return neg_errno(err),
                },
            };
            (resolved_path, st)
        } else {
            match fds.path_stat(process, dirfd as i32, path.as_str()) {
                Ok(result) => result,
                Err(err) => return neg_errno(err),
            }
        };
        match fds.parent_dirs_searchable(
            process,
            resolved_path.as_str(),
            process.fs_uid(),
            process.fs_gid(),
        ) {
            Ok(true) => {}
            Ok(false) => return neg_errno(LinuxError::EACCES),
            Err(err) => return neg_errno(err),
        }
        (Some(resolved_path), st)
    };
    apply_chown_metadata(process, record_path, &st, owner, group)
}

fn proc_self_fd_number(path: &str) -> Option<i32> {
    let rest = path
        .strip_prefix("/proc/self/fd/")
        .or_else(|| path.strip_prefix("/dev/fd/"))?;
    if rest.is_empty() || rest.contains('/') {
        return None;
    }
    rest.parse().ok()
}

pub(super) fn fd_entry_statfs_path(entry: &FdEntry) -> Option<&str> {
    match entry {
        FdEntry::DevNull => Some("/dev/null"),
        FdEntry::Rtc => Some("/dev/misc/rtc"),
        FdEntry::Pipe(_) => Some("pipe:"),
        FdEntry::Socket(_) | FdEntry::LocalSocket(_) => Some("socket:"),
        _ => fd_entry_path(entry),
    }
}

fn statfs_type_for_path(path: Option<&str>) -> i64 {
    match path {
        Some(path) if path == "/proc" || path.starts_with("/proc/") => PROC_SUPER_MAGIC,
        Some(path) if path == "/sys" || path.starts_with("/sys/") => SYSFS_MAGIC,
        Some(path) if path == "/dev" || path.starts_with("/dev/") => DEVFS_MAGIC,
        Some(path) if path.starts_with("pipe:") => PIPEFS_MAGIC,
        _ => TMPFS_MAGIC,
    }
}

pub(super) fn generic_statfs(path: Option<&str>) -> general::statfs {
    let alloc = global_allocator();
    let reported_file_blocks = (MAX_IN_MEMORY_FILE_SIZE / STATFS_BLOCK_SIZE as u64).max(1) as i64;
    let available_pages = (alloc.available_pages() as i64).min(reported_file_blocks);
    let total_pages = (alloc.used_pages() as i64 + available_pages).max(1);
    let fs_type = statfs_type_for_path(path);
    general::statfs {
        f_type: fs_type as _,
        f_bsize: STATFS_BLOCK_SIZE as _,
        f_blocks: total_pages as _,
        f_bfree: available_pages as _,
        f_bavail: available_pages as _,
        f_files: total_pages as _,
        f_ffree: available_pages as _,
        f_fsid: general::__kernel_fsid_t {
            val: [fs_type as i32, 0],
        },
        f_namelen: STATFS_NAME_MAX as _,
        f_frsize: STATFS_BLOCK_SIZE as _,
        f_flags: 0,
        f_spare: [0; 4],
    }
}

pub(super) fn dirent_type(ty: FileType) -> u32 {
    match ty {
        FileType::Dir => general::DT_DIR,
        FileType::CharDevice => general::DT_CHR,
        FileType::BlockDevice => general::DT_BLK,
        FileType::Fifo => general::DT_FIFO,
        FileType::Socket => general::DT_SOCK,
        FileType::SymLink => general::DT_LNK,
        _ => general::DT_REG,
    }
}

fn synthetic_char_stat(ino: u64, mode: u32, rdev: u64) -> general::stat {
    let mut st: general::stat = unsafe { core::mem::zeroed() };
    st.st_ino = ino;
    st.st_mode = mode;
    st.st_nlink = 1;
    st.st_rdev = rdev as _;
    st.st_blksize = 512;
    st
}

pub(super) fn stdio_stat(readable: bool) -> general::stat {
    let perm = if readable { 0o440 } else { 0o220 };
    synthetic_char_stat(1, ST_MODE_CHR | perm, 0)
}

pub(super) fn synthetic_char_stat_for_path(path: &str, mode: u32) -> general::stat {
    let rdev = match path {
        "/dev/null" => DEV_NULL_RDEV,
        _ => 0,
    };
    synthetic_char_stat(path_inode(Some(path)), mode, rdev)
}

pub(super) fn synthetic_block_stat_for_path(path: &str, mode: u32) -> general::stat {
    let rdev = match normalize_path("/", path).as_deref() {
        Some("/dev/sda") => DEV_SDA_RDEV,
        Some("/dev/xvda") => DEV_XVDA_RDEV,
        Some("/dev/vda") => DEV_VDA_RDEV,
        _ => 0,
    };
    synthetic_char_stat(path_inode(Some(path)), mode, rdev)
}

pub(super) fn dev_null_stat() -> general::stat {
    synthetic_char_stat_for_path("/dev/null", ST_MODE_CHR | 0o220)
}

pub(super) fn path_inode(path: Option<&str>) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
    let Some(path) = path else {
        return 1;
    };
    let mut hash = FNV_OFFSET;
    for &byte in path.as_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash.max(1)
}

pub(super) fn file_type_mode(ty: FileType) -> u32 {
    match ty {
        FileType::Dir => ST_MODE_DIR,
        FileType::CharDevice => ST_MODE_CHR,
        FileType::Fifo => ST_MODE_FIFO,
        FileType::Socket => ST_MODE_SOCKET,
        _ => ST_MODE_FILE,
    }
}

pub(super) fn sys_newfstatat(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    statbuf: usize,
    flags: usize,
) -> isize {
    let flags = flags as u32;
    let supported_flags = general::AT_SYMLINK_NOFOLLOW
        | general::AT_EMPTY_PATH
        | general::AT_NO_AUTOMOUNT
        | general::AT_STATX_SYNC_TYPE;
    if flags & !supported_flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    if pathname == 0 {
        if flags & general::AT_EMPTY_PATH == 0 {
            return neg_errno(LinuxError::EFAULT);
        }
        let st = match stat_empty_path(process, dirfd as i32) {
            Ok(st) => st,
            Err(err) => return neg_errno(err),
        };
        return write_user_value(process, statbuf, &st);
    }
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let st = if path.is_empty() && flags & general::AT_EMPTY_PATH != 0 {
        match stat_empty_path(process, dirfd as i32) {
            Ok(st) => st,
            Err(err) => return neg_errno(err),
        }
    } else {
        if flags & general::AT_SYMLINK_NOFOLLOW != 0 {
            let resolved_path = {
                let table = process.fds.lock();
                match resolve_dirfd_path(process, &table, dirfd as i32, path.as_str()) {
                    Ok(path) => path,
                    Err(err) => return neg_errno(err),
                }
            };
            if let Some(st) = process.path_symlink_stat(resolved_path.as_str()) {
                st
            } else {
                match process
                    .fds
                    .lock()
                    .stat_path(process, dirfd as i32, path.as_str())
                {
                    Ok(st) => st,
                    Err(err) => return neg_errno(err),
                }
            }
        } else {
            match process
                .fds
                .lock()
                .stat_path(process, dirfd as i32, path.as_str())
            {
                Ok(st) => st,
                Err(err) => return neg_errno(err),
            }
        }
    };
    write_user_value(process, statbuf, &st)
}

pub(super) fn sys_fstat(process: &UserProcess, fd: usize, statbuf: usize) -> isize {
    let st = match process
        .fds
        .lock()
        .stat_with_recorded_path(process, fd as i32)
    {
        Ok((_, st)) => st,
        Err(err) => return neg_errno(err),
    };
    write_user_value(process, statbuf, &st)
}

fn stat_to_statx(st: general::stat) -> general::statx {
    let mut stx: general::statx = unsafe { core::mem::zeroed() };
    stx.stx_mask = general::STATX_BASIC_STATS;
    stx.stx_blksize = st.st_blksize as _;
    stx.stx_nlink = st.st_nlink as _;
    stx.stx_uid = st.st_uid as _;
    stx.stx_gid = st.st_gid as _;
    stx.stx_mode = st.st_mode as _;
    stx.stx_ino = st.st_ino as _;
    stx.stx_size = st.st_size as _;
    stx.stx_blocks = st.st_blocks as _;
    stx.stx_attributes_mask = 0;
    stx.stx_dev_major = ((st.st_dev as u64) >> 8) as _;
    stx.stx_dev_minor = ((st.st_dev as u64) & 0xff) as _;
    stx.stx_rdev_major = ((st.st_rdev as u64) >> 8) as _;
    stx.stx_rdev_minor = ((st.st_rdev as u64) & 0xff) as _;
    stx
}

pub(super) fn sys_statx(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    flags: usize,
    mask: usize,
    statxbuf: usize,
) -> isize {
    if statxbuf == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    let flags = flags as u32;
    let supported_flags = general::AT_SYMLINK_NOFOLLOW
        | general::AT_EMPTY_PATH
        | general::AT_NO_AUTOMOUNT
        | general::AT_STATX_SYNC_TYPE;
    if flags & !supported_flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let mask = mask as u32;
    let supported_mask = general::STATX_BASIC_STATS
        | general::STATX_BTIME
        | general::STATX_MNT_ID
        | general::STATX_DIOALIGN
        | general::STATX_MNT_ID_UNIQUE
        | general::STATX_SUBVOL
        | general::STATX_WRITE_ATOMIC
        | general::STATX_DIO_READ_ALIGN;
    if mask & !supported_mask != 0 {
        return neg_errno(LinuxError::EINVAL);
    }

    let st = if pathname == 0 {
        if flags & general::AT_EMPTY_PATH == 0 {
            return neg_errno(LinuxError::EFAULT);
        }
        match stat_empty_path(process, dirfd as i32) {
            Ok(st) => st,
            Err(err) => return neg_errno(err),
        }
    } else {
        let path = match read_cstr(process, pathname) {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        };
        if path.is_empty() && flags & general::AT_EMPTY_PATH != 0 {
            match stat_empty_path(process, dirfd as i32) {
                Ok(st) => st,
                Err(err) => return neg_errno(err),
            }
        } else if path.is_empty() {
            return neg_errno(LinuxError::ENOENT);
        } else if flags & general::AT_SYMLINK_NOFOLLOW != 0 {
            let resolved_path = {
                let table = process.fds.lock();
                match resolve_dirfd_path(process, &table, dirfd as i32, path.as_str()) {
                    Ok(path) => path,
                    Err(err) => return neg_errno(err),
                }
            };
            if let Some(st) = process.path_symlink_stat(resolved_path.as_str()) {
                st
            } else {
                match process
                    .fds
                    .lock()
                    .stat_path(process, dirfd as i32, path.as_str())
                {
                    Ok(st) => st,
                    Err(err) => return neg_errno(err),
                }
            }
        } else {
            match process
                .fds
                .lock()
                .stat_path(process, dirfd as i32, path.as_str())
            {
                Ok(st) => st,
                Err(err) => return neg_errno(err),
            }
        }
    };
    write_user_value(process, statxbuf, &stat_to_statx(st))
}

fn stat_empty_path(process: &UserProcess, dirfd: i32) -> Result<general::stat, LinuxError> {
    if dirfd == general::AT_FDCWD {
        process
            .fds
            .lock()
            .stat_path(process, general::AT_FDCWD, ".")
    } else {
        process
            .fds
            .lock()
            .stat_with_recorded_path(process, dirfd)
            .map(|(_, st)| st)
    }
}

pub(super) fn sys_readlinkat(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    buf: usize,
    bufsiz: usize,
) -> isize {
    if bufsiz == 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    if path.len() >= LINUX_PATH_MAX {
        return neg_errno(LinuxError::ENAMETOOLONG);
    }
    let resolved_path = if path.is_empty() {
        if dirfd as i32 == general::AT_FDCWD {
            return neg_errno(LinuxError::ENOENT);
        }
        let table = process.fds.lock();
        match table.entry(dirfd as i32).and_then(|entry| {
            fd_entry_path(entry)
                .map(ToString::to_string)
                .ok_or(LinuxError::EINVAL)
        }) {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        }
    } else {
        let table = process.fds.lock();
        match resolve_dirfd_path(process, &table, dirfd as i32, path.as_str()) {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        }
    };
    if process.fs_uid() != 0 {
        let mut table = process.fds.lock();
        match table.parent_dirs_searchable(
            process,
            resolved_path.as_str(),
            process.fs_uid(),
            process.fs_gid(),
        ) {
            Ok(true) => {}
            Ok(false) => return neg_errno(LinuxError::EACCES),
            Err(err) => return neg_errno(err),
        }
    }
    if let Some(fd) = proc_self_fd_number(resolved_path.as_str()) {
        let target = {
            let table = process.fds.lock();
            match table.entry(fd) {
                Ok(entry) => match fd_entry_path(entry) {
                    Some(path) => path.to_string(),
                    None => return neg_errno(LinuxError::EINVAL),
                },
                Err(err) => return neg_errno(err),
            }
        };
        let bytes = target.as_bytes();
        let copy_len = cmp::min(bytes.len(), bufsiz);
        return write_user_bytes(process, buf, &bytes[..copy_len])
            .map_or_else(|err| neg_errno(err), |_| copy_len as isize);
    }
    if let Some(target) = proc_exe_link_target(process, resolved_path.as_str()) {
        let bytes = target.as_bytes();
        let copy_len = cmp::min(bytes.len(), bufsiz);
        return write_user_bytes(process, buf, &bytes[..copy_len])
            .map_or_else(|err| neg_errno(err), |_| copy_len as isize);
    }
    if let Some(target) = process.path_symlink(resolved_path.as_str()) {
        let bytes = target.as_bytes();
        let copy_len = cmp::min(bytes.len(), bufsiz);
        return write_user_bytes(process, buf, &bytes[..copy_len])
            .map_or_else(|err| neg_errno(err), |_| copy_len as isize);
    }
    match axfs::api::metadata(resolved_path.as_str()) {
        Ok(_) => neg_errno(LinuxError::EINVAL),
        Err(err) => neg_errno(LinuxError::from(err)),
    }
}

pub(super) fn sys_utimensat(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    _times: usize,
    _flags: usize,
) -> isize {
    if pathname == 0 {
        let table = process.fds.lock();
        return if table.entry(dirfd as i32).is_ok() {
            0
        } else {
            neg_errno(LinuxError::EBADF)
        };
    }
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let abs_path = {
        let table = process.fds.lock();
        match resolve_dirfd_path(process, &table, dirfd as i32, path.as_str()) {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        }
    };
    match axfs::api::metadata(abs_path.as_str()) {
        Ok(_) => 0,
        Err(err) => neg_errno(LinuxError::from(err)),
    }
}

pub(super) fn sys_statfs(process: &UserProcess, pathname: usize, statfsbuf: usize) -> isize {
    if statfsbuf == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let cwd = process.cwd();
    let Some(abs_path) = normalize_path(cwd.as_str(), path.as_str()) else {
        return neg_errno(LinuxError::EINVAL);
    };
    let st = match process
        .fds
        .lock()
        .statfs_path(process, general::AT_FDCWD, abs_path.as_str())
    {
        Ok(st) => st,
        Err(err) => return neg_errno(err),
    };
    write_user_value(process, statfsbuf, &st)
}

pub(super) fn sys_fstatfs(process: &UserProcess, fd: usize, statfsbuf: usize) -> isize {
    if statfsbuf == 0 {
        return neg_errno(LinuxError::EFAULT);
    }
    let st = match process.fds.lock().statfs(fd as i32) {
        Ok(st) => st,
        Err(err) => return neg_errno(err),
    };
    write_user_value(process, statfsbuf, &st)
}
