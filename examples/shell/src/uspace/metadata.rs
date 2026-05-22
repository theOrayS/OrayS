use core::cmp;
use core::sync::atomic::Ordering;

use axalloc::global_allocator;
use axerrno::LinuxError;
use axfs::fops::{FileAttr, FileType};
use linux_raw_sys::general;
use std::string::{String, ToString};

use super::credentials::{access_allowed, apply_chown_metadata, chown_ids};
use super::fd_table::{resolve_dirfd_path, FdEntry};
use super::linux_abi::{
    neg_errno, neg_errno_code, ACCESS_MODE_MASK, DEVFS_MAGIC, FILE_MODE_GROUP_EXECUTE,
    FILE_MODE_PERMISSION_MASK, FILE_MODE_SET_GID, FILE_MODE_SET_UID, LINUX_EACCES, PIPEFS_MAGIC,
    PROC_SUPER_MAGIC, STATFS_BLOCK_SIZE, STATFS_NAME_MAX, ST_MODE_CHR, ST_MODE_DIR, ST_MODE_FILE,
    ST_MODE_LNK, SYSFS_MAGIC, TMPFS_MAGIC,
};
use super::runtime_paths::normalize_path;
use super::synthetic_fs::{dev_shm_host_path, proc_exe_link_target};
use super::user_memory::{read_cstr, write_user_bytes, write_user_value};
use super::UserProcess;

const DEV_NULL_RDEV: u64 = 259; // Linux makedev(1, 3).
const LINUX_PATH_MAX: usize = 4096;

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
        Some(st)
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
    if let Some(mode) = process.path_mode(path) {
        st.st_mode = (st.st_mode & !FILE_MODE_PERMISSION_MASK) | mode;
    }
    if let Some((uid, gid)) = process.path_owner(path) {
        st.st_uid = uid;
        st.st_gid = gid;
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
        FdEntry::Path(path) => Some(path.path.as_str()),
        FdEntry::MemoryFile(file) => Some(file.path.as_str()),
        _ => None,
    }
}

pub(super) fn sys_faccessat(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    mode: usize,
    _flags: usize,
) -> isize {
    if mode & !ACCESS_MODE_MASK != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let mut fds = process.fds.lock();
    let (resolved_path, stat) = match fds.path_stat(process, dirfd as i32, path.as_str()) {
        Ok(result) => result,
        Err(err) => return neg_errno(err),
    };
    let uid = process.uid();
    let gid = process.gid();
    let parents_searchable =
        match fds.parent_dirs_searchable(process, resolved_path.as_str(), uid, gid) {
            Ok(searchable) => searchable,
            Err(err) => return neg_errno(err),
        };
    if parents_searchable && access_allowed(&stat, mode, uid, gid) {
        0
    } else {
        neg_errno_code(LINUX_EACCES)
    }
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
    let path = match process.fds.lock().entry(fd as i32) {
        Ok(entry) => fd_entry_path(entry).map(ToString::to_string),
        Err(err) => {
            return neg_errno(err);
        }
    };
    if let Some(path) = path {
        process.set_path_mode(path, mode as u32);
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
    let mode = mode as u32;
    if path.is_empty() {
        if flags & general::AT_EMPTY_PATH == 0 {
            return neg_errno(LinuxError::ENOENT);
        }
        if dirfd as i32 == general::AT_FDCWD {
            let cwd = process.cwd();
            return match axfs::api::metadata(cwd.as_str()) {
                Ok(_) => {
                    process.set_path_mode(cwd, mode);
                    0
                }
                Err(err) => neg_errno(LinuxError::from(err)),
            };
        }
        return match process.fds.lock().entry(dirfd as i32) {
            Ok(entry) => {
                if let Some(path) = fd_entry_path(entry) {
                    process.set_path_mode(path.to_string(), mode);
                }
                0
            }
            Err(err) => neg_errno(err),
        };
    }

    let mut fds = process.fds.lock();
    let resolved_path = match fds.resolve_path(process, dirfd as i32, path.as_str()) {
        Ok(path) => path,
        Err(err) => {
            return neg_errno(err);
        }
    };
    if axfs::api::metadata(resolved_path.as_str()).is_ok() {
        process.set_path_mode(resolved_path, mode);
        return 0;
    }
    match fds.stat_path(process, dirfd as i32, path.as_str()) {
        Ok(_) => {
            process.set_path_mode(resolved_path, mode);
            0
        }
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_fchown(process: &UserProcess, fd: usize, owner: usize, group: usize) -> isize {
    let (owner, group) = match chown_ids(owner, group) {
        Ok(ids) => ids,
        Err(err) => return neg_errno(err),
    };
    let (path, st) = match process
        .fds
        .lock()
        .stat_with_recorded_path(process, fd as i32)
    {
        Ok((path, st)) => (path, st),
        Err(err) => return neg_errno(err),
    };
    apply_chown_metadata(process, path, &st, owner, group)
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
    let (record_path, st) = if path.is_empty() {
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
        let (resolved_path, st) = match fds.path_stat(process, dirfd as i32, path.as_str()) {
            Ok(result) => result,
            Err(err) => return neg_errno(err),
        };
        (Some(resolved_path), st)
    };
    apply_chown_metadata(process, record_path, &st, owner, group)
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
    let available_pages = alloc.available_pages() as i64;
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
    let resolved_path = {
        let table = process.fds.lock();
        match resolve_dirfd_path(process, &table, dirfd as i32, path.as_str()) {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        }
    };
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
