use core::cmp;

use axalloc::global_allocator;
use axerrno::LinuxError;
use axfs::fops::{FileAttr, FileType};
use linux_raw_sys::general;
use std::string::{String, ToString};

use super::UserProcess;
use super::credentials::{access_allowed, apply_chown_metadata, chown_ids};
use super::fd_table::{FdEntry, resolve_dirfd_path};
use super::linux_abi::{
    ACCESS_MODE_MASK, DEVFS_MAGIC, FILE_MODE_PERMISSION_MASK, LINUX_EACCES, PIPEFS_MAGIC,
    PROC_SUPER_MAGIC, ST_MODE_CHR, ST_MODE_DIR, ST_MODE_FILE, STATFS_BLOCK_SIZE, STATFS_NAME_MAX,
    SYSFS_MAGIC, TMPFS_MAGIC, neg_errno, neg_errno_code,
};
use super::runtime_paths::normalize_path;
use super::synthetic_fs::{dev_shm_host_path, proc_exe_link_target};
use super::user_memory::{read_cstr, write_user_bytes, write_user_value};

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

pub(super) fn sys_fchmod(process: &UserProcess, fd: usize, mode: usize) -> isize {
    let path = match process.fds.lock().entry(fd as i32) {
        Ok(entry) => fd_entry_path(entry).map(ToString::to_string),
        Err(err) => return neg_errno(err),
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
    match fds.path_stat(process, dirfd as i32, path.as_str()) {
        Ok((resolved_path, _)) => {
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

pub(super) fn stdio_stat(readable: bool) -> general::stat {
    let perm = if readable { 0o440 } else { 0o220 };
    let mut st: general::stat = unsafe { core::mem::zeroed() };
    st.st_ino = 1;
    st.st_mode = ST_MODE_CHR | perm;
    st.st_nlink = 1;
    st.st_blksize = 512;
    st
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
    _flags: usize,
) -> isize {
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let st = match process
        .fds
        .lock()
        .stat_path(process, dirfd as i32, path.as_str())
    {
        Ok(st) => st,
        Err(err) => return neg_errno(err),
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
    let supported_flags = general::AT_SYMLINK_NOFOLLOW | general::AT_EMPTY_PATH;
    if flags & !supported_flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }

    let st = if pathname == 0 {
        if flags & general::AT_EMPTY_PATH == 0 {
            return neg_errno(LinuxError::EFAULT);
        }
        match process
            .fds
            .lock()
            .stat_with_recorded_path(process, dirfd as i32)
        {
            Ok((_, st)) => st,
            Err(err) => return neg_errno(err),
        }
    } else {
        let path = match read_cstr(process, pathname) {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        };
        if path.is_empty() && flags & general::AT_EMPTY_PATH != 0 {
            match process
                .fds
                .lock()
                .stat_with_recorded_path(process, dirfd as i32)
            {
                Ok((_, st)) => st,
                Err(err) => return neg_errno(err),
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
