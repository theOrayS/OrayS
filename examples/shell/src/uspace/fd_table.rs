use core::cmp;
use core::mem::{offset_of, size_of};
use core::ptr;

use axerrno::LinuxError;
use axfs::fops::{self, Directory, File, FileAttr, OpenOptions};
use axio::SeekFrom;
use lazyinit::LazyInit;
use linux_raw_sys::{general, ioctl};
use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::sync::{Arc, Mutex};
use std::vec::Vec;

use super::credentials::access_allowed;
use super::fd_pipe::PipeEndpoint;
use super::fd_socket::{recv_socket_data_to_user, socket_entry, LocalSocketEntry, SocketEntry};
use super::linux_abi::{
    fd_cloexec_flag, neg_errno, posix_ret_i32, ACCESS_R_OK, ACCESS_W_OK, ACCESS_X_OK,
    DEFAULT_NOFILE_LIMIT, FILE_MODE_STICKY, MAX_IN_MEMORY_FILE_SIZE, O_NOFOLLOW_FLAG, O_PATH_FLAG,
    RLIMIT_FSIZE_RESOURCE, RTC_RD_TIME, ST_MODE_BLK, ST_MODE_CHR, ST_MODE_DIR, ST_MODE_FIFO,
    ST_MODE_FILE, ST_MODE_LNK, ST_MODE_TYPE_MASK,
};
use super::memory_map::align_up;
use super::metadata::{
    apply_recorded_path_metadata, canonical_permission_path, dev_null_stat, dirent_type,
    fd_entry_path, fd_entry_statfs_path, file_attr_to_stat, file_type_mode, generic_statfs,
    path_inode, stdio_stat, synthetic_char_stat_for_path,
};
use super::runtime_paths::{
    busybox_applet_target_path, normalize_path, push_runtime_candidate,
    runtime_absolute_path_candidates, runtime_library_name_candidates,
};
use super::select_fdset::SelectMode;
use super::synthetic_fs::{
    dev_shm_host_path, ensure_dev_shm_dir, is_proc_self_maps_path, proc_comm_fd_entry,
    proc_comm_path_entry, proc_pagemap_fd_entry, proc_pagemap_path_entry, proc_pid_stat_fd_entry,
    proc_pid_stat_path_entry, proc_pid_status_fd_entry, proc_pid_status_path_entry,
    proc_self_maps_fd_entry, proc_self_maps_is_writable_open, proc_self_maps_path_entry,
    synthetic_file_is_writable_open, synthetic_userdb_content, synthetic_userdb_fd_entry,
    synthetic_userdb_path_entry,
};
use super::system_info::write_default_winsize;
use super::task_registry::user_thread_entry_by_process_pid;
use super::time_abi::rtc_time_from_wall_time;
use super::user_memory::{
    read_cstr, read_iovec_entries, read_user_bytes, read_user_value, user_io_buffer,
    validate_user_read, validate_user_write, with_readable_user_buffer, with_writable_user_buffer,
    write_user_bytes, write_user_value, MAX_USER_IO_CHUNK,
};
use super::UserProcess;

pub(super) struct FdTable {
    pub(super) entries: Vec<Option<FdEntry>>,
    pub(super) fd_flags: Vec<u32>,
}

const FD_TABLE_LIMIT: usize = DEFAULT_NOFILE_LIMIT as usize;
const LINUX_PATH_MAX: usize = 4096;
const LINUX_NAME_MAX: usize = 255;

pub(super) enum FdEntry {
    Stdin,
    Stdout,
    Stderr,
    DevNull,
    BlockDevice(BlockDeviceEntry),
    Rtc,
    File(FileEntry),
    Directory(DirectoryEntry),
    Path(PathEntry),
    MemoryFile(MemoryFileEntry),
    ProcPagemap(ProcPagemapEntry),
    Pipe(PipeEndpoint),
    Socket(SocketEntry),
    LocalSocket(LocalSocketEntry),
}

#[derive(Clone)]
pub(super) struct FileEntry {
    pub(super) file: File,
    pub(super) path: String,
    pub(super) status_flags: u32,
    offset: Arc<Mutex<u64>>,
    lease_type: Arc<Mutex<u32>>,
}

#[derive(Clone)]
pub(super) struct DirectoryEntry {
    pub(super) dir: Directory,
    pub(super) attr: FileAttr,
    pub(super) path: String,
    next_dirent_cookie: u64,
}

#[derive(Clone)]
pub(super) struct BlockDeviceEntry {
    pub(super) path: String,
}

#[derive(Clone)]
pub(super) struct PathEntry {
    pub(super) path: String,
    pub(super) mode: u32,
    pub(super) size: u64,
    pub(super) blocks: u64,
}

#[derive(Clone)]
pub(super) struct MemoryFileEntry {
    pub(super) path: String,
    pub(super) data: Arc<Vec<u8>>,
    pub(super) offset: usize,
}

#[derive(Clone)]
pub(super) struct ProcPagemapEntry {
    pub(super) path: String,
    pub(super) present_ranges: Arc<Vec<(u64, u64)>>,
    pub(super) offset: u64,
    pub(super) size: u64,
}

pub(super) fn read_file_at_into(
    file: &File,
    offset: u64,
    dst: &mut [u8],
) -> Result<usize, LinuxError> {
    let mut filled = 0usize;
    while filled < dst.len() {
        let read = file
            .read_at(offset + filled as u64, &mut dst[filled..])
            .map_err(LinuxError::from)?;
        if read == 0 {
            break;
        }
        filled += read;
    }
    Ok(filled)
}

pub(super) fn sys_openat(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    flags: usize,
    mode: usize,
) -> isize {
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    match process.fds.lock().open(
        process,
        dirfd as i32,
        path.as_str(),
        flags as u32,
        mode as u32,
    ) {
        Ok(fd) => fd as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_ftruncate(process: &UserProcess, fd: usize, length: usize) -> isize {
    let length = length as isize;
    if length < 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let length = length as u64;
    let file_size_limit = process.get_rlimit(RLIMIT_FSIZE_RESOURCE).current();
    if length > file_size_limit {
        return neg_errno(LinuxError::EFBIG);
    }
    match process.fds.lock().truncate(fd as i32, length) {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_fallocate(
    process: &UserProcess,
    fd: usize,
    mode: usize,
    offset: usize,
    len: usize,
) -> isize {
    let offset = offset as isize;
    let len = len as isize;
    if offset < 0 || len <= 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    if mode != 0 {
        return neg_errno(LinuxError::EOPNOTSUPP);
    }
    let Some(end) = (offset as u64).checked_add(len as u64) else {
        return neg_errno(LinuxError::EFBIG);
    };
    let file_size_limit = process.get_rlimit(RLIMIT_FSIZE_RESOURCE).current();
    if end > file_size_limit {
        return neg_errno(LinuxError::EFBIG);
    }
    match process.fds.lock().truncate(fd as i32, end) {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_close(process: &UserProcess, fd: usize) -> isize {
    match process.fds.lock().close_for_process(process, fd as i32) {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_read(process: &UserProcess, fd: usize, buf: usize, count: usize) -> isize {
    if let Ok(socket) = socket_entry(process, fd) {
        return recv_socket_data_to_user(process, socket.posix_fd, buf, count, 0);
    }
    with_writable_user_buffer(process, buf, count, |dst| {
        process.fds.lock().read(fd as i32, dst)
    })
}

pub(super) fn sys_pread64(
    process: &UserProcess,
    fd: usize,
    buf: usize,
    count: usize,
    offset: usize,
) -> isize {
    let offset = offset as isize;
    if offset < 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    with_writable_user_buffer(process, buf, count, |dst| {
        process
            .fds
            .lock()
            .read_file_at_into_fd(fd as i32, offset as u64, dst)
    })
}

pub(super) fn sys_write(process: &UserProcess, fd: usize, buf: usize, count: usize) -> isize {
    let file_size_limit = process.get_rlimit(RLIMIT_FSIZE_RESOURCE).current();
    with_readable_user_buffer(process, buf, count, |src| {
        process
            .fds
            .lock()
            .write(fd as i32, src, Some(file_size_limit))
    })
}

pub(super) fn sys_pwrite64(
    process: &UserProcess,
    fd: usize,
    buf: usize,
    count: usize,
    offset: usize,
) -> isize {
    let offset = offset as isize;
    if offset < 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    with_readable_user_buffer(process, buf, count, |src| {
        process
            .fds
            .lock()
            .write_file_at(fd as i32, offset as u64, src)
    })
}

pub(super) fn sys_writev(process: &UserProcess, fd: usize, iov: usize, iovcnt: usize) -> isize {
    let iov_entries = match read_iovec_entries(process, iov, iovcnt) {
        Ok(iov_entries) => iov_entries,
        Err(err) => return neg_errno(err),
    };
    let mut written = 0isize;
    for entry in iov_entries {
        let mut base = entry.iov_base as usize;
        let mut remaining = entry.iov_len as usize;
        if let Err(err) = validate_user_read(process, base, remaining) {
            return if written > 0 { written } else { neg_errno(err) };
        }
        while remaining > 0 {
            let len = remaining.min(MAX_USER_IO_CHUNK);
            let src = match read_user_bytes(process, base, len) {
                Ok(bytes) => bytes,
                Err(err) => return if written > 0 { written } else { neg_errno(err) },
            };
            let file_size_limit = process.get_rlimit(RLIMIT_FSIZE_RESOURCE).current();
            let n = match process
                .fds
                .lock()
                .write(fd as i32, &src, Some(file_size_limit))
            {
                Ok(v) => v,
                Err(err) => return if written > 0 { written } else { neg_errno(err) },
            };
            written += n as isize;
            if n < len {
                return written;
            }
            base = base.saturating_add(n);
            remaining -= n;
        }
    }
    written
}

pub(super) fn sys_readv(process: &UserProcess, fd: usize, iov: usize, iovcnt: usize) -> isize {
    let iov_entries = match read_iovec_entries(process, iov, iovcnt) {
        Ok(iov_entries) => iov_entries,
        Err(err) => return neg_errno(err),
    };
    let mut total = 0isize;
    for entry in iov_entries {
        let mut base = entry.iov_base as usize;
        let mut remaining = entry.iov_len as usize;
        if let Err(err) = validate_user_write(process, base, remaining) {
            return if total > 0 { total } else { neg_errno(err) };
        }
        while remaining > 0 {
            let len = remaining.min(MAX_USER_IO_CHUNK);
            let mut bytes = match user_io_buffer(len) {
                Ok(bytes) => bytes,
                Err(err) => return if total > 0 { total } else { neg_errno(err) },
            };
            let n = match process.fds.lock().read(fd as i32, &mut bytes) {
                Ok(v) => v,
                Err(err) => return if total > 0 { total } else { neg_errno(err) },
            };
            if n > len {
                return if total > 0 {
                    total
                } else {
                    neg_errno(LinuxError::EINVAL)
                };
            }
            if let Err(err) = write_user_bytes(process, base, &bytes[..n]) {
                return if total > 0 { total } else { neg_errno(err) };
            }
            total += n as isize;
            if n < len {
                return total;
            }
            base = base.saturating_add(n);
            remaining -= n;
        }
    }
    total
}

pub(super) fn sys_preadv(
    process: &UserProcess,
    fd: usize,
    iov: usize,
    iovcnt: usize,
    offset: usize,
) -> isize {
    let offset = offset as isize;
    if offset < 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let iov_entries = match read_iovec_entries(process, iov, iovcnt) {
        Ok(iov_entries) => iov_entries,
        Err(err) => return neg_errno(err),
    };
    let mut total = 0isize;
    let mut next_offset = offset as u64;
    for entry in iov_entries {
        let mut base = entry.iov_base as usize;
        let mut remaining = entry.iov_len as usize;
        if let Err(err) = validate_user_write(process, base, remaining) {
            return if total > 0 { total } else { neg_errno(err) };
        }
        while remaining > 0 {
            let len = remaining.min(MAX_USER_IO_CHUNK);
            let mut bytes = match user_io_buffer(len) {
                Ok(bytes) => bytes,
                Err(err) => return if total > 0 { total } else { neg_errno(err) },
            };
            let n =
                match process
                    .fds
                    .lock()
                    .read_file_at_into_fd(fd as i32, next_offset, &mut bytes)
                {
                    Ok(v) => v,
                    Err(err) => return if total > 0 { total } else { neg_errno(err) },
                };
            if let Err(err) = write_user_bytes(process, base, &bytes[..n]) {
                return if total > 0 { total } else { neg_errno(err) };
            }
            total += n as isize;
            next_offset = next_offset.saturating_add(n as u64);
            if n < len {
                return total;
            }
            base = base.saturating_add(n);
            remaining -= n;
        }
    }
    total
}

fn split_offset_arg(pos_l: usize, pos_h: usize) -> i64 {
    let low = pos_l as u32 as u64;
    let high = pos_h as u32 as u64;
    ((high << 32) | low) as i64
}

pub(super) fn sys_preadv2(
    process: &UserProcess,
    fd: usize,
    iov: usize,
    iovcnt: usize,
    pos_l: usize,
    pos_h: usize,
    flags: usize,
) -> isize {
    if flags != 0 {
        return neg_errno(LinuxError::EOPNOTSUPP);
    }
    match split_offset_arg(pos_l, pos_h) {
        -1 => sys_readv(process, fd, iov, iovcnt),
        offset if offset < -1 => neg_errno(LinuxError::EINVAL),
        offset => sys_preadv(process, fd, iov, iovcnt, offset as usize),
    }
}

pub(super) fn sys_pwritev(
    process: &UserProcess,
    fd: usize,
    iov: usize,
    iovcnt: usize,
    offset: usize,
) -> isize {
    let offset = offset as isize;
    if offset < 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let iov_entries = match read_iovec_entries(process, iov, iovcnt) {
        Ok(iov_entries) => iov_entries,
        Err(err) => return neg_errno(err),
    };
    let mut total = 0isize;
    let mut next_offset = offset as u64;
    for entry in iov_entries {
        let mut base = entry.iov_base as usize;
        let mut remaining = entry.iov_len as usize;
        if let Err(err) = validate_user_read(process, base, remaining) {
            return if total > 0 { total } else { neg_errno(err) };
        }
        while remaining > 0 {
            let len = remaining.min(MAX_USER_IO_CHUNK);
            let src = match read_user_bytes(process, base, len) {
                Ok(bytes) => bytes,
                Err(err) => return if total > 0 { total } else { neg_errno(err) },
            };
            let n = match process
                .fds
                .lock()
                .write_file_at(fd as i32, next_offset, &src)
            {
                Ok(v) => v,
                Err(err) => return if total > 0 { total } else { neg_errno(err) },
            };
            total += n as isize;
            next_offset = next_offset.saturating_add(n as u64);
            if n < len {
                return total;
            }
            base = base.saturating_add(n);
            remaining -= n;
        }
    }
    total
}

pub(super) fn sys_pwritev2(
    process: &UserProcess,
    fd: usize,
    iov: usize,
    iovcnt: usize,
    pos_l: usize,
    pos_h: usize,
    flags: usize,
) -> isize {
    if flags != 0 {
        return neg_errno(LinuxError::EOPNOTSUPP);
    }
    match split_offset_arg(pos_l, pos_h) {
        -1 => sys_writev(process, fd, iov, iovcnt),
        offset if offset < -1 => neg_errno(LinuxError::EINVAL),
        offset => sys_pwritev(process, fd, iov, iovcnt, offset as usize),
    }
}

pub(super) fn sys_sendfile(
    process: &UserProcess,
    out_fd: usize,
    in_fd: usize,
    offset_ptr: usize,
    count: usize,
) -> isize {
    let mut offset = if offset_ptr == 0 {
        None
    } else {
        if let Err(err) = validate_user_write(process, offset_ptr, size_of::<i64>()) {
            return neg_errno(err);
        }
        match read_user_value::<i64>(process, offset_ptr) {
            Ok(value) if value >= 0 => Some(value as u64),
            Ok(_) => return neg_errno(LinuxError::EINVAL),
            Err(err) => return neg_errno(err),
        }
    };
    let file_size_limit = process.get_rlimit(RLIMIT_FSIZE_RESOURCE).current();
    {
        let mut table = process.fds.lock();
        let input_check = match offset {
            Some(pos) => table.read_file_at_into_fd(in_fd as i32, pos, &mut []),
            None => table
                .read_file_at_current_offset_into_fd(in_fd as i32, &mut [])
                .map(|(_, read)| read),
        };
        if let Err(err) = input_check {
            return neg_errno(err);
        }
        if let Err(err) = table.write(out_fd as i32, &[], Some(file_size_limit)) {
            return neg_errno(err);
        }
    }

    let mut copied = 0usize;
    while copied < count {
        let chunk_len = (count - copied).min(MAX_USER_IO_CHUNK);
        let mut buf = match user_io_buffer(chunk_len) {
            Ok(buf) => buf,
            Err(err) => {
                return if copied > 0 {
                    copied as isize
                } else {
                    neg_errno(err)
                };
            }
        };
        let read = {
            let mut table = process.fds.lock();
            match offset {
                Some(pos) => table.read_file_at_into_fd(in_fd as i32, pos, &mut buf),
                None => table
                    .read_file_at_current_offset_into_fd(in_fd as i32, &mut buf)
                    .map(|(_, read)| read),
            }
        };
        let read = match read {
            Ok(0) => break,
            Ok(n) => n,
            Err(err) => {
                return if copied > 0 {
                    copied as isize
                } else {
                    neg_errno(err)
                };
            }
        };
        let written =
            match process
                .fds
                .lock()
                .write(out_fd as i32, &buf[..read], Some(file_size_limit))
            {
                Ok(n) => n,
                Err(err) => {
                    return if copied > 0 {
                        copied as isize
                    } else {
                        neg_errno(err)
                    };
                }
            };
        if let Some(pos) = offset.as_mut() {
            *pos = pos.saturating_add(written as u64);
        } else if let Err(err) = process
            .fds
            .lock()
            .advance_file_offset_fd(in_fd as i32, written)
        {
            return if copied > 0 {
                copied as isize
            } else {
                neg_errno(err)
            };
        }
        copied += written;
        if written < read {
            break;
        }
    }
    if let Some(pos) = offset {
        let out: i64 = match pos.try_into() {
            Ok(value) => value,
            Err(_) => return neg_errno(LinuxError::EOVERFLOW),
        };
        let ret = write_user_value(process, offset_ptr, &out);
        if ret < 0 {
            return if copied > 0 { copied as isize } else { ret };
        }
    }
    copied as isize
}

pub(super) fn sys_getdents64(process: &UserProcess, fd: usize, dirp: usize, count: usize) -> isize {
    if let Err(err) = validate_user_write(process, dirp, count) {
        return neg_errno(err);
    }
    let bytes = match process.fds.lock().getdents64(process, fd as i32, count) {
        Ok(bytes) => bytes,
        Err(err) => return neg_errno(err),
    };
    if let Err(err) = write_user_bytes(process, dirp, &bytes) {
        return neg_errno(err);
    }
    bytes.len() as isize
}

pub(super) fn sys_lseek(process: &UserProcess, fd: usize, offset: usize, whence: usize) -> isize {
    match process
        .fds
        .lock()
        .lseek(fd as i32, offset as isize as i64, whence as u32)
    {
        Ok(v) => v as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_dup(process: &UserProcess, fd: usize) -> isize {
    match process.fds.lock().dup(fd as i32) {
        Ok(new_fd) => new_fd as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_dup3(process: &UserProcess, oldfd: usize, newfd: usize, flags: usize) -> isize {
    match process
        .fds
        .lock()
        .dup3(process, oldfd as i32, newfd as i32, flags as u32)
    {
        Ok(fd) => fd as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_fcntl(process: &UserProcess, fd: usize, cmd: usize, arg: usize) -> isize {
    match process
        .fds
        .lock()
        .fcntl(process, fd as i32, cmd as u32, arg)
    {
        Ok(v) => v as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_flock(process: &UserProcess, fd: usize, operation: usize) -> isize {
    match process.fds.lock().flock(fd as i32, operation as u32) {
        Ok(v) => v as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_fsync(process: &UserProcess, fd: usize) -> isize {
    match process.fds.lock().entry(fd as i32) {
        Ok(FdEntry::DevNull | FdEntry::BlockDevice(_) | FdEntry::Rtc) => {
            neg_errno(LinuxError::EINVAL)
        }
        Ok(_) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_renameat2(
    process: &UserProcess,
    olddirfd: usize,
    oldpath: usize,
    newdirfd: usize,
    newpath: usize,
    flags: usize,
) -> isize {
    if flags != 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    let old_path = match read_cstr(process, oldpath) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let new_path = match read_cstr(process, newpath) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let (old_abs_path, new_abs_path) = {
        let table = process.fds.lock();
        let old_abs = match resolve_dirfd_path(process, &table, olddirfd as i32, old_path.as_str())
        {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        };
        let new_abs = match resolve_dirfd_path(process, &table, newdirfd as i32, new_path.as_str())
        {
            Ok(path) => path,
            Err(err) => return neg_errno(err),
        };
        (old_abs, new_abs)
    };
    match axfs::api::rename(old_abs_path.as_str(), new_abs_path.as_str()) {
        Ok(()) => 0,
        Err(err) => neg_errno(LinuxError::from(err)),
    }
}

pub(super) fn sys_getcwd(process: &UserProcess, buf: usize, size: usize) -> isize {
    let cwd = process.cwd();
    let mut bytes = cwd.into_bytes();
    bytes.push(0);
    if bytes.len() > size {
        return neg_errno(LinuxError::ERANGE);
    }
    write_user_bytes(process, buf, &bytes)
        .map_or_else(|err| neg_errno(err), |_| bytes.len() as isize)
}

pub(super) fn sys_chdir(process: &UserProcess, pathname: usize) -> isize {
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    let visible_path = {
        let mut table = process.fds.lock();
        match table.resolve_path(process, general::AT_FDCWD, path.as_str()) {
            Ok(path) => {
                let stat = match table.stat_path(process, general::AT_FDCWD, path.as_str()) {
                    Ok(stat) => stat,
                    Err(err) => return neg_errno(err),
                };
                if stat.st_mode & ST_MODE_TYPE_MASK != ST_MODE_DIR {
                    return neg_errno(LinuxError::ENOTDIR);
                }
                let uid = process.fs_uid();
                let gid = process.fs_gid();
                let parents_searchable =
                    match table.parent_dirs_searchable(process, path.as_str(), uid, gid) {
                        Ok(searchable) => searchable,
                        Err(err) => return neg_errno(err),
                    };
                if uid != 0
                    && (!parents_searchable || !access_allowed(&stat, ACCESS_X_OK, uid, gid))
                {
                    return neg_errno(LinuxError::EACCES);
                }
                path
            }
            Err(err) => return neg_errno(err),
        }
    };
    let host_path = process.translate_mount_path(visible_path.as_str());
    if let Err(err) = open_dir_entry(host_path.as_str()) {
        return neg_errno(err);
    }
    process.set_cwd(visible_path);
    0
}

pub(super) fn sys_mkdirat(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    mode: usize,
) -> isize {
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    match process
        .fds
        .lock()
        .mkdirat(process, dirfd as i32, path.as_str(), mode as u32)
    {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_mknodat(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    mode: usize,
    _dev: usize,
) -> isize {
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    match process
        .fds
        .lock()
        .mknodat(process, dirfd as i32, path.as_str(), mode as u32)
    {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_unlinkat(
    process: &UserProcess,
    dirfd: usize,
    pathname: usize,
    flags: usize,
) -> isize {
    let path = match read_cstr(process, pathname) {
        Ok(path) => path,
        Err(err) => return neg_errno(err),
    };
    match process
        .fds
        .lock()
        .unlinkat(process, dirfd as i32, path.as_str(), flags as u32)
    {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_fchdir(process: &UserProcess, fd: usize) -> isize {
    let new_cwd = {
        let mut table = process.fds.lock();
        match table.entry(fd as i32) {
            Ok(FdEntry::Directory(dir)) => {
                let uid = process.fs_uid();
                let gid = process.fs_gid();
                let path = dir.path.clone();
                let stat = apply_recorded_path_metadata(
                    process,
                    path.as_str(),
                    file_attr_to_stat(&dir.attr, Some(path.as_str())),
                );
                let parents_searchable =
                    match table.parent_dirs_searchable(process, path.as_str(), uid, gid) {
                        Ok(searchable) => searchable,
                        Err(err) => return neg_errno(err),
                    };
                if uid != 0
                    && (!parents_searchable || !access_allowed(&stat, ACCESS_X_OK, uid, gid))
                {
                    return neg_errno(LinuxError::EACCES);
                }
                path
            }
            Ok(_) => return neg_errno(LinuxError::ENOTDIR),
            Err(err) => return neg_errno(err),
        }
    };
    process.set_cwd(new_cwd);
    0
}

pub(super) fn sys_ioctl(process: &UserProcess, fd: usize, req: usize, arg: usize) -> isize {
    const BLKGETSIZE64: u32 = 0x8008_1272;
    const FIONREAD: u32 = 0x541b;
    if req as u32 == BLKGETSIZE64 && process.fds.lock().is_block_device(fd as i32) {
        let size: u64 = 512 * 1024 * 1024;
        return write_user_value(process, arg, &size);
    }
    if req as u32 == FIONREAD {
        let available = match process.fds.lock().pipe_available_read(fd as i32) {
            Ok(available) => available as i32,
            Err(err) => return neg_errno(err),
        };
        return write_user_value(process, arg, &available);
    }
    if req as u32 == RTC_RD_TIME && process.fds.lock().is_rtc(fd as i32) {
        let rtc = rtc_time_from_wall_time();
        return write_user_value(process, arg, &rtc);
    }
    if req as u32 == ioctl::TIOCGWINSZ {
        if process.fds.lock().is_stdio(fd as i32) {
            return write_default_winsize(process, arg);
        }
    }
    neg_errno(LinuxError::ENOTTY)
}

impl FdTable {
    pub(super) fn new() -> Self {
        Self {
            entries: vec![
                Some(FdEntry::Stdin),
                Some(FdEntry::Stdout),
                Some(FdEntry::Stderr),
            ],
            fd_flags: vec![0, 0, 0],
        }
    }

    pub(super) fn fork_copy(&self) -> Result<Self, LinuxError> {
        let mut entries = Vec::with_capacity(self.entries.len());
        let mut fd_flags = Vec::with_capacity(self.entries.len());
        for (idx, entry) in self.entries.iter().enumerate() {
            entries.push(match entry {
                Some(entry) => Some(entry.duplicate_for_fork()?),
                None => None,
            });
            fd_flags.push(if entry.is_some() {
                self.fd_flags.get(idx).copied().unwrap_or(0)
            } else {
                0
            });
        }
        Ok(Self { entries, fd_flags })
    }

    pub(super) fn is_stdio(&self, fd: i32) -> bool {
        matches!(fd, 0..=2)
    }

    pub(super) fn is_rtc(&self, fd: i32) -> bool {
        matches!(self.entry(fd), Ok(FdEntry::Rtc))
    }

    pub(super) fn is_block_device(&self, fd: i32) -> bool {
        matches!(self.entry(fd), Ok(FdEntry::BlockDevice(_)))
    }

    pub(super) fn pipe_available_read(&self, fd: i32) -> Result<usize, LinuxError> {
        match self.entry(fd)? {
            FdEntry::Pipe(pipe) => Ok(pipe.available_read()),
            _ => Err(LinuxError::ENOTTY),
        }
    }

    pub(super) fn pipe_capacity(&self, fd: i32) -> Result<usize, LinuxError> {
        match self.entry(fd)? {
            FdEntry::Pipe(pipe) => Ok(pipe.capacity()),
            _ => Err(LinuxError::EBADF),
        }
    }

    pub(super) fn poll(&self, fd: i32, mode: SelectMode) -> bool {
        let Ok(entry) = self.entry(fd) else {
            return matches!(mode, SelectMode::Except);
        };
        match mode {
            SelectMode::Read => match entry {
                FdEntry::Stdin => false,
                FdEntry::Stdout | FdEntry::Stderr => false,
                FdEntry::DevNull
                | FdEntry::BlockDevice(_)
                | FdEntry::Rtc
                | FdEntry::File(_)
                | FdEntry::Directory(_)
                | FdEntry::MemoryFile(_)
                | FdEntry::ProcPagemap(_) => true,
                FdEntry::Path(_) => false,
                FdEntry::Pipe(pipe) => pipe.poll().readable,
                FdEntry::Socket(socket) => socket.poll(mode),
                FdEntry::LocalSocket(socket) => socket.poll(mode),
            },
            SelectMode::Write => match entry {
                FdEntry::Stdin => false,
                FdEntry::Stdout
                | FdEntry::Stderr
                | FdEntry::DevNull
                | FdEntry::BlockDevice(_)
                | FdEntry::Rtc => true,
                FdEntry::File(_) => true,
                FdEntry::Directory(_)
                | FdEntry::Path(_)
                | FdEntry::MemoryFile(_)
                | FdEntry::ProcPagemap(_) => false,
                FdEntry::Pipe(pipe) => pipe.poll().writable,
                FdEntry::Socket(socket) => socket.poll(mode),
                FdEntry::LocalSocket(socket) => socket.poll(mode),
            },
            SelectMode::Except => false,
        }
    }

    pub(super) fn read(&mut self, fd: i32, dst: &mut [u8]) -> Result<usize, LinuxError> {
        match self.entry_mut(fd)? {
            FdEntry::Stdin => Ok(0),
            FdEntry::DevNull => Ok(0),
            FdEntry::BlockDevice(_) => {
                dst.fill(0);
                Ok(dst.len())
            }
            FdEntry::Rtc => Ok(0),
            FdEntry::File(file) => {
                if !file_is_readable(file.status_flags) {
                    return Err(LinuxError::EBADF);
                }
                file_entry_read(file, dst)
            }
            FdEntry::MemoryFile(file) => Ok(file.read(dst)),
            FdEntry::ProcPagemap(file) => Ok(file.read(dst)),
            FdEntry::Directory(_) => Err(LinuxError::EISDIR),
            FdEntry::Pipe(pipe) => pipe.read(dst),
            FdEntry::Socket(socket) => socket.read(dst),
            FdEntry::LocalSocket(socket) => socket.read(dst),
            _ => Err(LinuxError::EBADF),
        }
    }

    pub(super) fn write(
        &mut self,
        fd: i32,
        src: &[u8],
        file_size_limit: Option<u64>,
    ) -> Result<usize, LinuxError> {
        match self.entry_mut(fd)? {
            FdEntry::Stdout | FdEntry::Stderr => {
                axhal::console::write_bytes(src);
                Ok(src.len())
            }
            FdEntry::DevNull => Ok(src.len()),
            FdEntry::BlockDevice(_) => Ok(src.len()),
            FdEntry::Rtc => Ok(src.len()),
            FdEntry::File(file) => {
                if !file_is_writable(file.status_flags) {
                    return Err(LinuxError::EBADF);
                }
                let src = limit_file_write_len(file, src, file_size_limit)?;
                file_entry_write(file, src)
            }
            FdEntry::Pipe(pipe) => pipe.write(src),
            FdEntry::Socket(socket) => socket.write(src),
            FdEntry::LocalSocket(socket) => socket.write(src),
            _ => Err(LinuxError::EBADF),
        }
    }

    pub(super) fn write_file_at(
        &mut self,
        fd: i32,
        offset: u64,
        src: &[u8],
    ) -> Result<usize, LinuxError> {
        let FdEntry::File(file) = self.entry_mut(fd)? else {
            return match self.entry(fd)? {
                FdEntry::Directory(_) => Err(LinuxError::EISDIR),
                FdEntry::Pipe(_) | FdEntry::Socket(_) | FdEntry::LocalSocket(_) => {
                    Err(LinuxError::ESPIPE)
                }
                _ => Err(LinuxError::EBADF),
            };
        };
        if !file_is_writable(file.status_flags) {
            return Err(LinuxError::EBADF);
        }
        let base_offset = if file.status_flags & general::O_APPEND != 0 {
            file.file.get_attr().map_err(LinuxError::from)?.size()
        } else {
            offset
        };
        let mut written = 0usize;
        while written < src.len() {
            let count = file
                .file
                .write_at(base_offset + written as u64, &src[written..])
                .map_err(LinuxError::from)?;
            if count == 0 {
                break;
            }
            written += count;
        }
        Ok(written)
    }

    fn close_slot(&mut self, idx: usize) -> Result<(), LinuxError> {
        if let Some(FdEntry::File(file)) = self.entries[idx].as_ref() {
            release_flock_on_last_close(file);
        }
        if let Some(FdEntry::Socket(socket)) = self.entries[idx].as_ref() {
            socket.close()?;
        }
        self.entries[idx] = None;
        if let Some(flags) = self.fd_flags.get_mut(idx) {
            *flags = 0;
        }
        Ok(())
    }

    pub(super) fn close(&mut self, fd: i32) -> Result<(), LinuxError> {
        if !(0..self.entries.len() as i32).contains(&fd) || self.entries[fd as usize].is_none() {
            return Err(LinuxError::EBADF);
        }
        self.close_slot(fd as usize)
    }

    pub(super) fn close_for_process(
        &mut self,
        process: &UserProcess,
        fd: i32,
    ) -> Result<(), LinuxError> {
        if !(0..self.entries.len() as i32).contains(&fd) || self.entries[fd as usize].is_none() {
            return Err(LinuxError::EBADF);
        }
        if let Some(FdEntry::File(file)) = self.entries[fd as usize].as_ref() {
            release_posix_record_locks_for_file_owner(record_lock_key(file), process.pid());
        }
        self.close_slot(fd as usize)
    }

    pub(super) fn close_all(&mut self) {
        for idx in 0..self.entries.len() {
            let _ = self.close_slot(idx);
        }
    }

    pub(super) fn close_cloexec(&mut self) {
        for idx in 0..self.entries.len() {
            if self.fd_flags.get(idx).copied().unwrap_or(0) & general::FD_CLOEXEC == 0 {
                continue;
            }
            let _ = self.close_slot(idx);
        }
    }

    pub(super) fn truncate(&mut self, fd: i32, size: u64) -> Result<(), LinuxError> {
        match self.entry_mut(fd)? {
            FdEntry::File(file) => {
                if !file_is_writable(file.status_flags) {
                    return Err(LinuxError::EINVAL);
                }
                if size > MAX_IN_MEMORY_FILE_SIZE {
                    return Err(LinuxError::ENOSPC);
                }
                file.file.truncate(size).map_err(LinuxError::from)
            }
            FdEntry::DevNull => Ok(()),
            FdEntry::BlockDevice(_) => Ok(()),
            FdEntry::Rtc => Ok(()),
            FdEntry::Path(_) | FdEntry::MemoryFile(_) | FdEntry::ProcPagemap(_) => {
                Err(LinuxError::EBADF)
            }
            _ => Err(LinuxError::EINVAL),
        }
    }

    pub(super) fn lseek(&mut self, fd: i32, offset: i64, whence: u32) -> Result<u64, LinuxError> {
        let pos = match whence {
            general::SEEK_SET => {
                if offset < 0 {
                    return Err(LinuxError::EINVAL);
                }
                SeekFrom::Start(offset as u64)
            }
            general::SEEK_CUR => SeekFrom::Current(offset),
            general::SEEK_END => SeekFrom::End(offset),
            _ => return Err(LinuxError::EINVAL),
        };
        match self.entry_mut(fd)? {
            FdEntry::File(file) => file_entry_seek(file, pos),
            FdEntry::DevNull => Ok(0),
            FdEntry::BlockDevice(_) => Ok(0),
            FdEntry::Rtc => Ok(0),
            FdEntry::Directory(_) => Err(LinuxError::EISDIR),
            FdEntry::Path(_) => Err(LinuxError::EBADF),
            FdEntry::MemoryFile(file) => file.seek(pos),
            FdEntry::ProcPagemap(file) => file.seek(pos),
            FdEntry::Pipe(_) => Err(LinuxError::ESPIPE),
            FdEntry::Socket(_) | FdEntry::LocalSocket(_) => Err(LinuxError::ESPIPE),
            _ => Err(LinuxError::ESPIPE),
        }
    }

    pub(super) fn dup(&mut self, fd: i32) -> Result<i32, LinuxError> {
        self.dup_min(fd, 0)
    }

    fn dup_min(&mut self, fd: i32, min_fd: i32) -> Result<i32, LinuxError> {
        self.dup_min_with_flags(fd, min_fd, 0)
    }

    pub(super) fn dup_min_with_flags(
        &mut self,
        fd: i32,
        min_fd: i32,
        fd_flags: u32,
    ) -> Result<i32, LinuxError> {
        if min_fd < 0 {
            return Err(LinuxError::EINVAL);
        }
        if min_fd as usize >= FD_TABLE_LIMIT {
            return Err(LinuxError::EINVAL);
        }
        let entry = self.entry(fd)?.duplicate_for_fork()?;
        self.insert_min_with_flags(entry, min_fd as usize, fd_flags & general::FD_CLOEXEC)
    }

    pub(super) fn dup3(
        &mut self,
        process: &UserProcess,
        oldfd: i32,
        newfd: i32,
        flags: u32,
    ) -> Result<i32, LinuxError> {
        if oldfd == newfd {
            return Err(LinuxError::EINVAL);
        }
        if flags & !general::O_CLOEXEC != 0 {
            return Err(LinuxError::EINVAL);
        }
        let entry = self.entry(oldfd)?.duplicate_for_fork()?;
        if newfd < 0 {
            return Err(LinuxError::EBADF);
        }
        let newfd = newfd as usize;
        if newfd >= FD_TABLE_LIMIT {
            return Err(LinuxError::EBADF);
        }
        if self.entries.len() <= newfd {
            self.entries.resize_with(newfd + 1, || None);
            self.fd_flags.resize(newfd + 1, 0);
        } else if self.entries[newfd].is_some() {
            let _ = self.close_for_process(process, newfd as i32);
        }
        if self.fd_flags.len() <= newfd {
            self.fd_flags.resize(newfd + 1, 0);
        }
        self.entries[newfd] = Some(entry);
        self.fd_flags[newfd] = fd_cloexec_flag(flags & general::O_CLOEXEC != 0);
        Ok(newfd as i32)
    }

    pub(super) fn getdents64(
        &mut self,
        process: &UserProcess,
        fd: i32,
        max_len: usize,
    ) -> Result<Vec<u8>, LinuxError> {
        let entry = self.entry_mut(fd)?;
        let FdEntry::Directory(dir) = entry else {
            return Err(LinuxError::ENOTDIR);
        };
        if axfs::api::metadata(dir.path.as_str()).is_err() {
            return Err(LinuxError::ENOENT);
        }
        let min_reclen = align_up(offset_of!(general::linux_dirent64, d_name) + 1, 8);
        if max_len < min_reclen {
            return Err(LinuxError::EINVAL);
        }
        let mut read_buf: [fops::DirEntry; 16] =
            core::array::from_fn(|_| fops::DirEntry::default());
        let count = dir.dir.read_dir(&mut read_buf).map_err(LinuxError::from)?;
        let mut out = Vec::new();
        let mut seen_names = Vec::new();
        for item in read_buf[..count].iter() {
            let name = item.name_as_bytes();
            let reclen = align_up(
                offset_of!(general::linux_dirent64, d_name) + name.len() + 1,
                8,
            );
            if out.len() + reclen > max_len {
                break;
            }
            let entry_path = core::str::from_utf8(name)
                .ok()
                .and_then(|name| normalize_path(dir.path.as_str(), name));
            if let Ok(name) = core::str::from_utf8(name) {
                seen_names.push(name.to_string());
            }
            dir.next_dirent_cookie = dir.next_dirent_cookie.saturating_add(1);
            let start = out.len();
            out.resize(start + reclen, 0);
            unsafe {
                let dirent = out[start..].as_mut_ptr() as *mut general::linux_dirent64;
                ptr::write_unaligned(
                    dirent,
                    general::linux_dirent64 {
                        d_ino: path_inode(entry_path.as_deref()) as _,
                        d_off: dir.next_dirent_cookie as _,
                        d_reclen: reclen as _,
                        d_type: dirent_type(item.entry_type()) as u8,
                        d_name: Default::default(),
                    },
                );
            }
            let name_start = start + offset_of!(general::linux_dirent64, d_name);
            out[name_start..name_start + name.len()].copy_from_slice(name);
        }
        for name in process.path_symlink_names_in_dir(dir.path.as_str()) {
            if seen_names.iter().any(|seen| seen == &name) {
                continue;
            }
            let name_bytes = name.as_bytes();
            let reclen = align_up(
                offset_of!(general::linux_dirent64, d_name) + name_bytes.len() + 1,
                8,
            );
            if out.len() + reclen > max_len {
                break;
            }
            let entry_path = normalize_path(dir.path.as_str(), name.as_str());
            dir.next_dirent_cookie = dir.next_dirent_cookie.saturating_add(1);
            let start = out.len();
            out.resize(start + reclen, 0);
            unsafe {
                let dirent = out[start..].as_mut_ptr() as *mut general::linux_dirent64;
                ptr::write_unaligned(
                    dirent,
                    general::linux_dirent64 {
                        d_ino: path_inode(entry_path.as_deref()) as _,
                        d_off: dir.next_dirent_cookie as _,
                        d_reclen: reclen as _,
                        d_type: general::DT_LNK as u8,
                        d_name: Default::default(),
                    },
                );
            }
            let name_start = start + offset_of!(general::linux_dirent64, d_name);
            out[name_start..name_start + name_bytes.len()].copy_from_slice(name_bytes);
        }
        Ok(out)
    }

    pub(super) fn read_file_at_into_fd(
        &mut self,
        fd: i32,
        offset: u64,
        dst: &mut [u8],
    ) -> Result<usize, LinuxError> {
        let FdEntry::File(file) = self.entry_mut(fd)? else {
            return match self.entry(fd)? {
                FdEntry::Directory(_) => Err(LinuxError::EISDIR),
                FdEntry::Pipe(_) | FdEntry::Socket(_) | FdEntry::LocalSocket(_) => {
                    Err(LinuxError::ESPIPE)
                }
                _ => Err(LinuxError::EBADF),
            };
        };
        if !file_is_readable(file.status_flags) {
            return Err(LinuxError::EBADF);
        }
        read_file_at_into(&file.file, offset, dst)
    }

    pub(super) fn read_file_at_current_offset_into_fd(
        &mut self,
        fd: i32,
        dst: &mut [u8],
    ) -> Result<(u64, usize), LinuxError> {
        let FdEntry::File(file) = self.entry_mut(fd)? else {
            return match self.entry(fd)? {
                FdEntry::Directory(_) => Err(LinuxError::EISDIR),
                FdEntry::Pipe(_) | FdEntry::Socket(_) | FdEntry::LocalSocket(_) => {
                    Err(LinuxError::ESPIPE)
                }
                _ => Err(LinuxError::EBADF),
            };
        };
        if !file_is_readable(file.status_flags) {
            return Err(LinuxError::EBADF);
        }
        let offset = *file.offset.lock();
        read_file_at_into(&file.file, offset, dst).map(|read| (offset, read))
    }

    pub(super) fn advance_file_offset_fd(
        &mut self,
        fd: i32,
        amount: usize,
    ) -> Result<(), LinuxError> {
        let FdEntry::File(file) = self.entry_mut(fd)? else {
            return match self.entry(fd)? {
                FdEntry::Directory(_) => Err(LinuxError::EISDIR),
                FdEntry::Pipe(_) | FdEntry::Socket(_) | FdEntry::LocalSocket(_) => {
                    Err(LinuxError::ESPIPE)
                }
                _ => Err(LinuxError::EBADF),
            };
        };
        let mut offset = file.offset.lock();
        *offset = offset.saturating_add(amount as u64);
        Ok(())
    }

    pub(super) fn mmap_read_file_at_into_fd(
        &mut self,
        fd: i32,
        offset: u64,
        dst: &mut [u8],
    ) -> Result<usize, LinuxError> {
        let FdEntry::File(file) = self.entry_mut(fd)? else {
            return match self.entry(fd)? {
                FdEntry::Directory(_) => Err(LinuxError::EISDIR),
                FdEntry::Pipe(_) | FdEntry::Socket(_) | FdEntry::LocalSocket(_) => {
                    Err(LinuxError::ESPIPE)
                }
                _ => Err(LinuxError::EBADF),
            };
        };
        if !file_is_readable(file.status_flags) {
            return Err(LinuxError::EACCES);
        }
        read_file_at_into(&file.file, offset, dst)
    }

    pub(super) fn insert_with_flags(
        &mut self,
        entry: FdEntry,
        fd_flags: u32,
    ) -> Result<i32, LinuxError> {
        self.insert_min_with_flags(entry, 0, fd_flags)
    }

    pub(super) fn insert_min_with_flags(
        &mut self,
        entry: FdEntry,
        min_fd: usize,
        fd_flags: u32,
    ) -> Result<i32, LinuxError> {
        if min_fd >= FD_TABLE_LIMIT {
            return Err(LinuxError::EMFILE);
        }
        if self.entries.len() < min_fd {
            self.entries.resize_with(min_fd, || None);
            self.fd_flags.resize(min_fd, 0);
        }
        if self.fd_flags.len() < self.entries.len() {
            self.fd_flags.resize(self.entries.len(), 0);
        }
        if let Some((idx, slot)) = self
            .entries
            .iter_mut()
            .enumerate()
            .take(FD_TABLE_LIMIT)
            .skip(min_fd)
            .find(|(_, slot)| slot.is_none())
        {
            *slot = Some(entry);
            self.fd_flags[idx] = fd_flags & general::FD_CLOEXEC;
            return Ok(idx as i32);
        }
        if self.entries.len() >= FD_TABLE_LIMIT {
            return Err(LinuxError::EMFILE);
        }
        self.entries.push(Some(entry));
        self.fd_flags.push(fd_flags & general::FD_CLOEXEC);
        Ok((self.entries.len() - 1) as i32)
    }

    pub(super) fn get_fd_flags(&self, fd: i32) -> Result<i32, LinuxError> {
        self.entry(fd)?;
        Ok(self.fd_flags.get(fd as usize).copied().unwrap_or(0) as i32)
    }

    pub(super) fn set_fd_flags(&mut self, fd: i32, flags: u32) -> Result<i32, LinuxError> {
        self.entry(fd)?;
        let idx = fd as usize;
        if self.fd_flags.len() <= idx {
            self.fd_flags.resize(idx + 1, 0);
        }
        self.fd_flags[idx] = flags & general::FD_CLOEXEC;
        Ok(0)
    }

    pub(super) fn entry(&self, fd: i32) -> Result<&FdEntry, LinuxError> {
        self.entries
            .get(fd as usize)
            .and_then(|entry| entry.as_ref())
            .ok_or(LinuxError::EBADF)
    }

    pub(super) fn entry_mut(&mut self, fd: i32) -> Result<&mut FdEntry, LinuxError> {
        self.entries
            .get_mut(fd as usize)
            .and_then(|entry| entry.as_mut())
            .ok_or(LinuxError::EBADF)
    }
}

fn limit_file_write_len<'a>(
    file: &mut FileEntry,
    src: &'a [u8],
    file_size_limit: Option<u64>,
) -> Result<&'a [u8], LinuxError> {
    let Some(limit) = file_size_limit else {
        return Ok(src);
    };
    if limit == u64::MAX {
        return Ok(src);
    }
    let offset = file
        .file
        .seek(SeekFrom::Current(0))
        .map_err(LinuxError::from)?;
    if offset >= limit {
        return Err(LinuxError::EFBIG);
    }
    let allowed = limit.saturating_sub(offset) as usize;
    Ok(&src[..src.len().min(allowed)])
}

impl FdTable {
    pub(super) fn open(
        &mut self,
        process: &UserProcess,
        dirfd: i32,
        path: &str,
        flags: u32,
        mode: u32,
    ) -> Result<i32, LinuxError> {
        let entry = open_fd_entry(process, self, dirfd, path, flags, mode)?;
        self.insert_with_flags(entry, fd_cloexec_flag(flags & general::O_CLOEXEC != 0))
    }

    pub(super) fn mkdirat(
        &mut self,
        process: &UserProcess,
        dirfd: i32,
        path: &str,
        mode: u32,
    ) -> Result<(), LinuxError> {
        if path.is_empty() {
            return Err(LinuxError::ENOENT);
        }
        if path_exceeds_linux_limits(path) {
            return Err(LinuxError::ENAMETOOLONG);
        }
        let abs_path = resolve_dirfd_path(process, self, dirfd, path)?;
        if axfs::api::metadata(abs_path.as_str()).is_ok() {
            return Err(LinuxError::EEXIST);
        }
        check_parent_write_search_permission(process, abs_path.as_str())?;
        directory_create_dir(abs_path.as_str())?;
        process.set_path_mode(abs_path.clone(), process.apply_umask(mode));
        process.set_path_owner(abs_path, Some(process.fs_uid()), Some(process.fs_gid()));
        Ok(())
    }

    pub(super) fn mknodat(
        &mut self,
        process: &UserProcess,
        dirfd: i32,
        path: &str,
        mode: u32,
    ) -> Result<(), LinuxError> {
        if path.is_empty() {
            return Err(LinuxError::ENOENT);
        }
        if path_exceeds_linux_limits(path) {
            return Err(LinuxError::ENAMETOOLONG);
        }
        let node_type = mode & ST_MODE_TYPE_MASK;
        let node_type = match node_type {
            0 | ST_MODE_FILE => ST_MODE_FILE,
            ST_MODE_FIFO => ST_MODE_FIFO,
            _ => return Err(LinuxError::EPERM),
        };
        let abs_path = resolve_dirfd_path(process, self, dirfd, path)?;
        if axfs::api::metadata(abs_path.as_str()).is_ok() {
            return Err(LinuxError::EEXIST);
        }
        check_parent_write_search_permission(process, abs_path.as_str())?;
        let mut opts = OpenOptions::new();
        opts.write(true);
        opts.create_new(true);
        File::open(abs_path.as_str(), &opts).map_err(LinuxError::from)?;
        process.set_path_mode(abs_path.clone(), process.apply_umask(mode));
        process.set_path_owner(
            abs_path.clone(),
            Some(process.fs_uid()),
            Some(process.fs_gid()),
        );
        if node_type == ST_MODE_FIFO {
            process.set_path_special_mode(abs_path, ST_MODE_FIFO);
        } else {
            process.remove_path_special_mode(abs_path.as_str());
        }
        Ok(())
    }

    pub(super) fn unlinkat(
        &mut self,
        process: &UserProcess,
        dirfd: i32,
        path: &str,
        flags: u32,
    ) -> Result<(), LinuxError> {
        let remove_dir = flags & general::AT_REMOVEDIR != 0;
        let supported_flags = general::AT_REMOVEDIR;
        if flags & !supported_flags != 0 {
            return Err(LinuxError::EINVAL);
        }
        if path.is_empty() {
            return Err(LinuxError::ENOENT);
        }
        if path_exceeds_linux_limits(path) {
            return Err(LinuxError::ENAMETOOLONG);
        }
        let abs_path = resolve_dirfd_path(process, self, dirfd, path)?;
        let parent_st = check_parent_write_search_permission(process, abs_path.as_str())?;
        let target_st = if let Some(st) = process.path_symlink_stat(abs_path.as_str()) {
            Some(apply_recorded_path_metadata(process, abs_path.as_str(), st))
        } else {
            match stat_absolute_path(process, abs_path.as_str()) {
                Ok(st) => Some(st),
                Err(LinuxError::ENOENT) if !remove_dir => None,
                Err(err) => return Err(err),
            }
        };
        if let Some(st) = target_st.as_ref() {
            check_sticky_parent_permission(process, &parent_st, st)?;
        }
        if process.path_symlink(abs_path.as_str()).is_some() {
            if remove_dir {
                return Err(LinuxError::ENOTDIR);
            }
            process.remove_path_symlink(abs_path.as_str());
            return Ok(());
        }
        let removed = if remove_dir {
            directory_remove_dir(abs_path.as_str())
        } else {
            directory_remove_file(abs_path.as_str())
        };
        if removed.is_ok() {
            process.remove_path_special_mode(abs_path.as_str());
        }
        removed
    }

    pub(super) fn stat(&mut self, fd: i32) -> Result<general::stat, LinuxError> {
        match self.entry_mut(fd)? {
            FdEntry::Stdin => Ok(stdio_stat(true)),
            FdEntry::Stdout | FdEntry::Stderr => Ok(stdio_stat(false)),
            FdEntry::DevNull => Ok(dev_null_stat()),
            FdEntry::BlockDevice(dev) => Ok(PathEntry::synthetic_block(dev.path.as_str()).stat()),
            FdEntry::Rtc => Ok(stdio_stat(false)),
            FdEntry::File(file) => Ok(file_attr_to_stat(
                &file.file.get_attr().map_err(LinuxError::from)?,
                Some(file.path.as_str()),
            )),
            FdEntry::Directory(dir) => Ok(file_attr_to_stat(&dir.attr, Some(dir.path.as_str()))),
            FdEntry::Path(path) => Ok(path.stat()),
            FdEntry::MemoryFile(file) => Ok(file.stat()),
            FdEntry::ProcPagemap(file) => Ok(file.stat()),
            FdEntry::Pipe(pipe) => Ok(pipe.stat()),
            FdEntry::Socket(socket) => Ok(socket.stat()),
            FdEntry::LocalSocket(socket) => Ok(socket.stat()),
        }
    }

    pub(super) fn stat_with_recorded_path(
        &mut self,
        process: &UserProcess,
        fd: i32,
    ) -> Result<(Option<String>, general::stat), LinuxError> {
        let path = fd_entry_path(self.entry(fd)?).map(ToString::to_string);
        let st = self.stat(fd)?;
        let st = match path.as_deref() {
            Some(path) => apply_recorded_path_metadata(process, path, st),
            None => st,
        };
        Ok((path, st))
    }

    pub(super) fn statfs(&self, fd: i32) -> Result<general::statfs, LinuxError> {
        Ok(generic_statfs(fd_entry_statfs_path(self.entry(fd)?)))
    }

    pub(super) fn stat_path(
        &mut self,
        process: &UserProcess,
        dirfd: i32,
        path: &str,
    ) -> Result<general::stat, LinuxError> {
        match open_fd_entry(process, self, dirfd, path, O_PATH_FLAG, 0) {
            Ok(FdEntry::DevNull) | Ok(FdEntry::Rtc) => Ok(stdio_stat(false)),
            Ok(FdEntry::BlockDevice(dev)) => {
                Ok(PathEntry::synthetic_block(dev.path.as_str()).stat())
            }
            Ok(FdEntry::File(file)) => Ok(apply_recorded_path_metadata(
                process,
                file.path.as_str(),
                file_attr_to_stat(
                    &file.file.get_attr().map_err(LinuxError::from)?,
                    Some(file.path.as_str()),
                ),
            )),
            Ok(FdEntry::Directory(dir)) => Ok(apply_recorded_path_metadata(
                process,
                dir.path.as_str(),
                file_attr_to_stat(&dir.attr, Some(dir.path.as_str())),
            )),
            Ok(FdEntry::Path(path)) => Ok(apply_recorded_path_metadata(
                process,
                path.path.as_str(),
                path.stat(),
            )),
            Ok(FdEntry::MemoryFile(file)) => Ok(apply_recorded_path_metadata(
                process,
                file.path.as_str(),
                file.stat(),
            )),
            Ok(FdEntry::ProcPagemap(file)) => Ok(apply_recorded_path_metadata(
                process,
                file.path.as_str(),
                file.stat(),
            )),
            Ok(_) => Err(LinuxError::EINVAL),
            Err(err) => Err(err),
        }
    }

    pub(super) fn path_stat(
        &mut self,
        process: &UserProcess,
        dirfd: i32,
        path: &str,
    ) -> Result<(String, general::stat), LinuxError> {
        let resolved_path = self.resolve_path(process, dirfd, path)?;
        let st = self.stat_path(process, dirfd, path)?;
        Ok((resolved_path, st))
    }

    pub(super) fn resolve_path(
        &self,
        process: &UserProcess,
        dirfd: i32,
        path: &str,
    ) -> Result<String, LinuxError> {
        if path.is_empty() {
            return Err(LinuxError::ENOENT);
        }
        let normalized = if path.starts_with('/') {
            normalize_path("/", path).ok_or(LinuxError::EINVAL)?
        } else if dirfd == general::AT_FDCWD {
            let cwd = process.cwd();
            normalize_path(cwd.as_str(), path).ok_or(LinuxError::EINVAL)?
        } else {
            let base = match self.entry(dirfd)? {
                FdEntry::Directory(dir) => dir.path.as_str(),
                FdEntry::Path(path_entry) if path_entry.mode & ST_MODE_TYPE_MASK == ST_MODE_DIR => {
                    path_entry.path.as_str()
                }
                _ => return Err(LinuxError::ENOTDIR),
            };
            normalize_path(base, path).ok_or(LinuxError::EINVAL)?
        };
        Ok(canonical_permission_path(normalized))
    }

    pub(super) fn parent_dirs_searchable(
        &mut self,
        process: &UserProcess,
        path: &str,
        uid: u32,
        gid: u32,
    ) -> Result<bool, LinuxError> {
        if uid == 0 {
            return Ok(true);
        }
        let components: Vec<&str> = path.split('/').filter(|part| !part.is_empty()).collect();
        if components.len() <= 1 {
            return Ok(true);
        }
        let mut parent = String::new();
        for component in &components[..components.len() - 1] {
            parent.push('/');
            parent.push_str(component);
            let st = self.stat_path(process, general::AT_FDCWD, parent.as_str())?;
            if !access_allowed(&st, ACCESS_X_OK, uid, gid) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub(super) fn statfs_path(
        &mut self,
        process: &UserProcess,
        dirfd: i32,
        path: &str,
    ) -> Result<general::statfs, LinuxError> {
        let entry = open_fd_entry(process, self, dirfd, path, O_PATH_FLAG, 0)?;
        let uid = process.fs_uid();
        if uid != 0 {
            let resolved_path = self.resolve_path(process, dirfd, path)?;
            if !self.parent_dirs_searchable(
                process,
                resolved_path.as_str(),
                uid,
                process.fs_gid(),
            )? {
                return Err(LinuxError::EACCES);
            }
        }
        Ok(generic_statfs(fd_entry_statfs_path(&entry)))
    }

    pub(super) fn fcntl(
        &mut self,
        process: &UserProcess,
        fd: i32,
        cmd: u32,
        arg: usize,
    ) -> Result<i32, LinuxError> {
        const F_SETPIPE_SZ: u32 = 1031;
        const F_GETPIPE_SZ: u32 = 1032;
        if matches!(self.entry(fd)?, FdEntry::Path(_)) && cmd == general::F_GETFL {
            return Ok(O_PATH_FLAG as i32);
        }
        if matches!(self.entry(fd)?, FdEntry::LocalSocket(_)) {
            return match cmd {
                general::F_DUPFD => self.dup_min_with_flags(fd, arg as i32, 0),
                general::F_DUPFD_CLOEXEC => self.insert_min_with_flags(
                    self.entry(fd)?.duplicate_for_fork()?,
                    arg,
                    general::FD_CLOEXEC,
                ),
                general::F_GETFD => self.get_fd_flags(fd),
                general::F_SETFD => self.set_fd_flags(fd, arg as u32),
                general::F_GETFL => match self.entry(fd)? {
                    FdEntry::LocalSocket(socket) => Ok(socket.status_flags()),
                    _ => unreachable!(),
                },
                general::F_SETFL => match self.entry_mut(fd)? {
                    FdEntry::LocalSocket(socket) => {
                        socket.set_status_flags(arg as i32);
                        Ok(0)
                    }
                    _ => unreachable!(),
                },
                _ => Ok(0),
            };
        }
        let socket = match self.entry(fd)? {
            FdEntry::Socket(socket) => Some(socket.clone()),
            _ => None,
        };
        if let Some(socket) = socket {
            return match cmd {
                general::F_DUPFD => {
                    self.insert_min_with_flags(FdEntry::Socket(socket.duplicate()?), arg, 0)
                }
                general::F_DUPFD_CLOEXEC => self.insert_min_with_flags(
                    FdEntry::Socket(socket.duplicate()?),
                    arg,
                    general::FD_CLOEXEC,
                ),
                general::F_GETFD => self.get_fd_flags(fd),
                general::F_SETFD => self.set_fd_flags(fd, arg as u32),
                general::F_GETFL | general::F_SETFL => posix_ret_i32(arceos_posix_api::sys_fcntl(
                    socket.posix_fd,
                    cmd as i32,
                    arg,
                )),
                _ => Ok(0),
            };
        }
        match cmd {
            general::F_DUPFD => self.dup_min_with_flags(fd, arg as i32, 0),
            general::F_DUPFD_CLOEXEC => {
                self.dup_min_with_flags(fd, arg as i32, general::FD_CLOEXEC)
            }
            general::F_GETFD => self.get_fd_flags(fd),
            general::F_SETFD => self.set_fd_flags(fd, arg as u32),
            general::F_GETFL => match self.entry(fd)? {
                FdEntry::File(file) => Ok(file.status_flags as i32),
                FdEntry::Pipe(pipe) => Ok(pipe.status_flags() as i32),
                _ => Ok(0),
            },
            F_GETPIPE_SZ => Ok(self.pipe_capacity(fd)? as i32),
            F_SETPIPE_SZ => match self.entry(fd)? {
                FdEntry::Pipe(pipe) => {
                    let capacity = pipe.capacity();
                    if arg <= capacity {
                        Ok(capacity as i32)
                    } else {
                        Err(LinuxError::EPERM)
                    }
                }
                _ => Err(LinuxError::EBADF),
            },
            general::F_SETFL => match self.entry_mut(fd)? {
                FdEntry::File(file) => {
                    file.status_flags =
                        (file.status_flags & general::O_ACCMODE) | fcntl_setfl_flags(arg as u32);
                    Ok(0)
                }
                FdEntry::Pipe(pipe) => {
                    pipe.set_status_flags(arg as u32);
                    Ok(0)
                }
                _ => Ok(0),
            },
            general::F_GETLK => self.fcntl_getlk(process, fd, arg),
            general::F_SETLK => self.fcntl_setlk(process, fd, arg, false),
            general::F_SETLKW => self.fcntl_setlk(process, fd, arg, true),
            general::F_GETLEASE => self.fcntl_getlease(fd),
            general::F_SETLEASE => self.fcntl_setlease(fd, arg as u32),
            _ => Err(LinuxError::EINVAL),
        }
    }

    pub(super) fn flock(&mut self, fd: i32, operation: u32) -> Result<i32, LinuxError> {
        let (key, owner) = match self.entry(fd)? {
            FdEntry::File(file) => (flock_key(file), flock_owner(file)),
            _ => return Err(LinuxError::EBADF),
        };
        apply_flock_operation(key, owner, operation)?;
        Ok(0)
    }

    fn fcntl_getlk(
        &mut self,
        process: &UserProcess,
        fd: i32,
        arg: usize,
    ) -> Result<i32, LinuxError> {
        let mut lock: general::flock = read_user_value(process, arg)?;
        validate_flock(&lock)?;
        let (key, request) = match self.entry(fd)? {
            FdEntry::File(file) => (
                record_lock_key(file),
                normalize_record_lock(file, process, &lock)?,
            ),
            _ => return Err(LinuxError::EBADF),
        };
        if let Some(conflict) = first_record_lock_conflict(key, &request) {
            lock.l_type = conflict.typ;
            lock.l_whence = general::SEEK_SET as _;
            lock.l_start = conflict.start as _;
            lock.l_len = conflict.len as _;
            lock.l_pid = conflict.owner_pid as _;
        } else {
            lock.l_type = general::F_UNLCK as _;
        }
        if write_user_value(process, arg, &lock) == 0 {
            Ok(0)
        } else {
            Err(LinuxError::EFAULT)
        }
    }

    fn fcntl_setlk(
        &mut self,
        process: &UserProcess,
        fd: i32,
        arg: usize,
        wait: bool,
    ) -> Result<i32, LinuxError> {
        let lock: general::flock = read_user_value(process, arg)?;
        validate_flock(&lock)?;
        let (key, request) = match self.entry(fd)? {
            FdEntry::File(file) => {
                if !record_lock_access_allowed(file, lock.l_type as u32) {
                    return Err(LinuxError::EBADF);
                }
                (
                    record_lock_key(file),
                    normalize_record_lock(file, process, &lock)?,
                )
            }
            _ => return Err(LinuxError::EBADF),
        };
        apply_record_lock(key, request, wait)?;
        Ok(0)
    }

    fn fcntl_getlease(&mut self, fd: i32) -> Result<i32, LinuxError> {
        match self.entry(fd)? {
            FdEntry::File(file) => Ok(*file.lease_type.lock() as i32),
            _ => Err(LinuxError::EINVAL),
        }
    }

    fn fcntl_setlease(&mut self, fd: i32, lease_type: u32) -> Result<i32, LinuxError> {
        match self.entry(fd)? {
            FdEntry::File(file) => match lease_type {
                general::F_RDLCK | general::F_WRLCK | general::F_UNLCK => {
                    *file.lease_type.lock() = lease_type;
                    Ok(())
                }
                _ => Err(LinuxError::EINVAL),
            },
            _ => return Err(LinuxError::EINVAL),
        }?;
        Ok(0)
    }
}

impl PathEntry {
    pub(super) fn symlink(path: &str) -> Self {
        Self {
            path: path.into(),
            mode: ST_MODE_LNK | 0o777,
            size: 0,
            blocks: 0,
        }
    }

    pub(super) fn from_attr(path: &str, attr: &FileAttr) -> Self {
        Self {
            path: path.into(),
            mode: file_type_mode(attr.file_type()) | attr.perm().bits() as u32,
            size: attr.size(),
            blocks: attr.blocks(),
        }
    }

    pub(super) fn fifo(path: &str, mode: u32) -> Self {
        Self {
            path: path.into(),
            mode: ST_MODE_FIFO | (mode & 0o7777),
            size: 0,
            blocks: 0,
        }
    }

    pub(super) fn synthetic_file(path: &str, size: usize) -> Self {
        Self {
            path: path.into(),
            mode: ST_MODE_FILE | 0o444,
            size: size as u64,
            blocks: (size as u64).div_ceil(512),
        }
    }

    pub(super) fn synthetic_char(path: &str) -> Self {
        Self {
            path: path.into(),
            mode: ST_MODE_CHR | 0o440,
            size: 0,
            blocks: 0,
        }
    }

    pub(super) fn synthetic_block(path: &str) -> Self {
        Self {
            path: path.into(),
            mode: ST_MODE_BLK | 0o660,
            size: 0,
            blocks: 0,
        }
    }

    pub(super) fn stat(&self) -> general::stat {
        if self.mode & ST_MODE_TYPE_MASK == ST_MODE_CHR {
            return synthetic_char_stat_for_path(self.path.as_str(), self.mode);
        }
        let mut st: general::stat = unsafe { core::mem::zeroed() };
        st.st_dev = 1;
        st.st_ino = path_inode(Some(self.path.as_str()));
        st.st_mode = self.mode;
        st.st_nlink = 1;
        st.st_size = self.size as _;
        st.st_blksize = 512;
        st.st_blocks = self.blocks as _;
        st
    }
}

impl MemoryFileEntry {
    pub(super) fn read(&mut self, dst: &mut [u8]) -> usize {
        let start = self.offset.min(self.data.len());
        let end = cmp::min(start + dst.len(), self.data.len());
        let len = end.saturating_sub(start);
        dst[..len].copy_from_slice(&self.data[start..end]);
        self.offset = end;
        len
    }

    pub(super) fn stat(&self) -> general::stat {
        PathEntry::synthetic_file(self.path.as_str(), self.data.len()).stat()
    }

    pub(super) fn seek(&mut self, pos: SeekFrom) -> Result<u64, LinuxError> {
        let next = match pos {
            SeekFrom::Start(offset) => offset as i64,
            SeekFrom::Current(offset) => self.offset as i64 + offset,
            SeekFrom::End(offset) => self.data.len() as i64 + offset,
        };
        if next < 0 {
            return Err(LinuxError::EINVAL);
        }
        self.offset = next as usize;
        Ok(self.offset as u64)
    }
}

impl ProcPagemapEntry {
    const PRESENT: u64 = 1u64 << 63;
    const ENTRY_SIZE: u64 = size_of::<u64>() as u64;

    pub(super) fn read(&mut self, dst: &mut [u8]) -> usize {
        if self.offset >= self.size {
            return 0;
        }
        let available = (self.size - self.offset).min(dst.len() as u64) as usize;
        let mut written = 0usize;
        while written < available {
            let page_index = self.offset / Self::ENTRY_SIZE;
            let entry_offset = (self.offset % Self::ENTRY_SIZE) as usize;
            let entry = self.page_entry(page_index).to_ne_bytes();
            let copy_len = cmp::min(entry.len() - entry_offset, available - written);
            dst[written..written + copy_len]
                .copy_from_slice(&entry[entry_offset..entry_offset + copy_len]);
            self.offset += copy_len as u64;
            written += copy_len;
        }
        written
    }

    pub(super) fn stat(&self) -> general::stat {
        PathEntry::synthetic_file(self.path.as_str(), self.size as usize).stat()
    }

    pub(super) fn seek(&mut self, pos: SeekFrom) -> Result<u64, LinuxError> {
        let next = match pos {
            SeekFrom::Start(offset) => offset as i128,
            SeekFrom::Current(offset) => self.offset as i128 + offset as i128,
            SeekFrom::End(offset) => self.size as i128 + offset as i128,
        };
        if !(0..=u64::MAX as i128).contains(&next) {
            return Err(LinuxError::EINVAL);
        }
        self.offset = next as u64;
        Ok(self.offset)
    }

    fn page_entry(&self, page_index: u64) -> u64 {
        if self
            .present_ranges
            .iter()
            .any(|(start, end)| *start <= page_index && page_index < *end)
        {
            Self::PRESENT
        } else {
            0
        }
    }
}

fn validate_flock(lock: &general::flock) -> Result<(), LinuxError> {
    match lock.l_type as u32 {
        general::F_RDLCK | general::F_WRLCK | general::F_UNLCK => {}
        _ => return Err(LinuxError::EINVAL),
    }
    match lock.l_whence as u32 {
        general::SEEK_SET | general::SEEK_CUR | general::SEEK_END => Ok(()),
        _ => Err(LinuxError::EINVAL),
    }
}

#[derive(Clone)]
struct PosixRecordLock {
    owner_pid: i32,
    typ: i16,
    start: i64,
    len: i64,
}

impl PosixRecordLock {
    fn end(&self) -> i64 {
        if self.len == 0 {
            i64::MAX
        } else {
            self.start.saturating_add(self.len)
        }
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start < other.end() && other.start < self.end()
    }

    fn conflicts_with(&self, request: &Self) -> bool {
        if self.owner_pid == request.owner_pid || !self.overlaps(request) {
            return false;
        }
        match request.typ as u32 {
            general::F_RDLCK => self.typ as u32 == general::F_WRLCK,
            general::F_WRLCK => {
                matches!(self.typ as u32, general::F_RDLCK | general::F_WRLCK)
            }
            _ => false,
        }
    }
}

fn posix_record_lock_table() -> &'static Mutex<BTreeMap<u64, Vec<PosixRecordLock>>> {
    static POSIX_RECORD_LOCKS: LazyInit<Mutex<BTreeMap<u64, Vec<PosixRecordLock>>>> =
        LazyInit::new();
    if !POSIX_RECORD_LOCKS.is_inited() {
        POSIX_RECORD_LOCKS.init_once(Mutex::new(BTreeMap::new()));
    }
    &POSIX_RECORD_LOCKS
}

fn record_lock_key(file: &FileEntry) -> u64 {
    path_inode(Some(file.path.as_str()))
}

fn record_lock_access_allowed(file: &FileEntry, typ: u32) -> bool {
    match typ {
        general::F_RDLCK => file_is_readable(file.status_flags),
        general::F_WRLCK => file_is_writable(file.status_flags),
        general::F_UNLCK => true,
        _ => false,
    }
}

fn normalize_record_lock(
    file: &FileEntry,
    process: &UserProcess,
    lock: &general::flock,
) -> Result<PosixRecordLock, LinuxError> {
    let base = match lock.l_whence as u32 {
        general::SEEK_SET => 0,
        general::SEEK_CUR => {
            i64::try_from(*file.offset.lock()).map_err(|_| LinuxError::EOVERFLOW)?
        }
        general::SEEK_END => i64::try_from(file.file.get_attr().map_err(LinuxError::from)?.size())
            .map_err(|_| LinuxError::EOVERFLOW)?,
        _ => return Err(LinuxError::EINVAL),
    };
    let mut start = base
        .checked_add(lock.l_start as i64)
        .ok_or(LinuxError::EINVAL)?;
    let mut len = lock.l_len as i64;
    if len < 0 {
        start = start.checked_add(len).ok_or(LinuxError::EINVAL)?;
        len = len.checked_neg().ok_or(LinuxError::EINVAL)?;
    }
    if start < 0 {
        return Err(LinuxError::EINVAL);
    }
    if len != 0 {
        start.checked_add(len).ok_or(LinuxError::EINVAL)?;
    }
    Ok(PosixRecordLock {
        owner_pid: process.pid(),
        typ: lock.l_type,
        start,
        len,
    })
}

fn lock_len_from_range(start: i64, end: i64) -> i64 {
    if end == i64::MAX {
        0
    } else {
        end.saturating_sub(start)
    }
}

fn cleanup_dead_record_locks(locks: &mut Vec<PosixRecordLock>, current_pid: i32) {
    locks.retain(|lock| {
        lock.owner_pid == current_pid || user_thread_entry_by_process_pid(lock.owner_pid).is_some()
    });
}

fn merge_record_locks(locks: &mut Vec<PosixRecordLock>) {
    locks.sort_by_key(|lock| (lock.owner_pid, lock.typ, lock.start, lock.end()));
    let mut merged: Vec<PosixRecordLock> = Vec::new();
    for lock in locks.drain(..) {
        if let Some(last) = merged.last_mut() {
            if last.owner_pid == lock.owner_pid && last.typ == lock.typ && lock.start <= last.end()
            {
                let end = last.end().max(lock.end());
                last.len = lock_len_from_range(last.start, end);
                continue;
            }
        }
        merged.push(lock);
    }
    *locks = merged;
}

fn remove_record_lock_range(locks: &mut Vec<PosixRecordLock>, request: &PosixRecordLock) {
    let request_end = request.end();
    let mut next = Vec::new();
    for lock in locks.drain(..) {
        if lock.owner_pid != request.owner_pid || !lock.overlaps(request) {
            next.push(lock);
            continue;
        }
        let lock_end = lock.end();
        if lock.start < request.start {
            next.push(PosixRecordLock {
                len: lock_len_from_range(lock.start, request.start),
                ..lock.clone()
            });
        }
        if request_end < lock_end {
            next.push(PosixRecordLock {
                start: request_end,
                len: lock_len_from_range(request_end, lock_end),
                ..lock
            });
        }
    }
    *locks = next;
}

fn first_record_lock_conflict(key: u64, request: &PosixRecordLock) -> Option<PosixRecordLock> {
    let mut table = posix_record_lock_table().lock();
    let locks = table.get_mut(&key)?;
    cleanup_dead_record_locks(locks, request.owner_pid);
    locks.sort_by_key(|lock| (lock.start, lock.end(), lock.owner_pid));
    let conflict = locks
        .iter()
        .find(|lock| lock.conflicts_with(request))
        .cloned();
    if locks.is_empty() {
        table.remove(&key);
    }
    conflict
}

fn apply_record_lock(key: u64, request: PosixRecordLock, wait: bool) -> Result<(), LinuxError> {
    loop {
        let mut table = posix_record_lock_table().lock();
        let locks = table.entry(key).or_insert_with(Vec::new);
        cleanup_dead_record_locks(locks, request.owner_pid);
        if request.typ as u32 != general::F_UNLCK
            && locks.iter().any(|lock| lock.conflicts_with(&request))
        {
            if !wait {
                if locks.is_empty() {
                    table.remove(&key);
                }
                return Err(LinuxError::EAGAIN);
            }
            drop(table);
            axtask::yield_now();
            continue;
        }
        remove_record_lock_range(locks, &request);
        if request.typ as u32 != general::F_UNLCK {
            locks.push(request);
            merge_record_locks(locks);
        }
        if locks.is_empty() {
            table.remove(&key);
        }
        return Ok(());
    }
}

fn release_posix_record_locks_for_file_owner(key: u64, owner_pid: i32) {
    let mut table = posix_record_lock_table().lock();
    let should_remove = if let Some(locks) = table.get_mut(&key) {
        locks.retain(|lock| lock.owner_pid != owner_pid);
        locks.is_empty()
    } else {
        false
    };
    if should_remove {
        table.remove(&key);
    }
}

pub(super) fn release_posix_record_locks_for_process(owner_pid: i32) {
    let mut table = posix_record_lock_table().lock();
    let empty_keys: Vec<u64> = table
        .iter_mut()
        .filter_map(|(key, locks)| {
            locks.retain(|lock| lock.owner_pid != owner_pid);
            locks.is_empty().then_some(*key)
        })
        .collect();
    for key in empty_keys {
        table.remove(&key);
    }
}

struct FlockState {
    exclusive_owner: Option<usize>,
    shared_owners: Vec<usize>,
}

impl FlockState {
    fn new() -> Self {
        Self {
            exclusive_owner: None,
            shared_owners: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.exclusive_owner.is_none() && self.shared_owners.is_empty()
    }

    fn unlock(&mut self, owner: usize) {
        if self.exclusive_owner == Some(owner) {
            self.exclusive_owner = None;
        }
        self.shared_owners.retain(|held_owner| *held_owner != owner);
    }

    fn lock_shared(&mut self, owner: usize) -> Result<(), LinuxError> {
        if matches!(self.exclusive_owner, Some(held_owner) if held_owner != owner) {
            return Err(LinuxError::EAGAIN);
        }
        self.exclusive_owner = None;
        if !self.shared_owners.contains(&owner) {
            self.shared_owners.push(owner);
        }
        Ok(())
    }

    fn lock_exclusive(&mut self, owner: usize) -> Result<(), LinuxError> {
        if matches!(self.exclusive_owner, Some(held_owner) if held_owner != owner) {
            return Err(LinuxError::EAGAIN);
        }
        if self
            .shared_owners
            .iter()
            .any(|held_owner| *held_owner != owner)
        {
            return Err(LinuxError::EAGAIN);
        }
        self.shared_owners.retain(|held_owner| *held_owner != owner);
        self.exclusive_owner = Some(owner);
        Ok(())
    }
}

fn flock_table() -> &'static Mutex<BTreeMap<u64, FlockState>> {
    static FLOCKS: LazyInit<Mutex<BTreeMap<u64, FlockState>>> = LazyInit::new();
    if !FLOCKS.is_inited() {
        FLOCKS.init_once(Mutex::new(BTreeMap::new()));
    }
    &FLOCKS
}

fn flock_key(file: &FileEntry) -> u64 {
    path_inode(Some(file.path.as_str()))
}

fn flock_owner(file: &FileEntry) -> usize {
    Arc::as_ptr(&file.offset) as usize
}

fn release_flock_on_last_close(file: &FileEntry) {
    if Arc::strong_count(&file.offset) == 1 {
        release_flock_owner(flock_key(file), flock_owner(file));
    }
}

fn release_flock_owner(key: u64, owner: usize) {
    let mut table = flock_table().lock();
    let should_remove = if let Some(state) = table.get_mut(&key) {
        state.unlock(owner);
        state.is_empty()
    } else {
        false
    };
    if should_remove {
        table.remove(&key);
    }
}

fn apply_flock_operation(key: u64, owner: usize, operation: u32) -> Result<(), LinuxError> {
    if operation & !(general::LOCK_SH | general::LOCK_EX | general::LOCK_NB | general::LOCK_UN) != 0
    {
        return Err(LinuxError::EINVAL);
    }
    let mode = operation & !general::LOCK_NB;
    match mode {
        general::LOCK_UN => {
            release_flock_owner(key, owner);
            Ok(())
        }
        general::LOCK_SH | general::LOCK_EX => {
            let mut table = flock_table().lock();
            let state = table.entry(key).or_insert_with(FlockState::new);
            let ret = if mode == general::LOCK_SH {
                state.lock_shared(owner)
            } else {
                state.lock_exclusive(owner)
            };
            if state.is_empty() {
                table.remove(&key);
            }
            ret
        }
        _ => Err(LinuxError::EINVAL),
    }
}

impl FdEntry {
    pub(super) fn duplicate_for_fork(&self) -> Result<Self, LinuxError> {
        match self {
            Self::Stdin => Ok(Self::Stdin),
            Self::Stdout => Ok(Self::Stdout),
            Self::Stderr => Ok(Self::Stderr),
            Self::DevNull => Ok(Self::DevNull),
            Self::BlockDevice(dev) => Ok(Self::BlockDevice(dev.clone())),
            Self::Rtc => Ok(Self::Rtc),
            Self::File(file) => Ok(Self::File(file.clone())),
            Self::Directory(dir) => Ok(Self::Directory(dir.clone())),
            Self::Path(path) => Ok(Self::Path(path.clone())),
            Self::MemoryFile(file) => Ok(Self::MemoryFile(file.clone())),
            Self::ProcPagemap(file) => Ok(Self::ProcPagemap(file.clone())),
            Self::Pipe(pipe) => Ok(Self::Pipe(pipe.clone())),
            Self::Socket(socket) => socket.duplicate().map(Self::Socket),
            Self::LocalSocket(socket) => Ok(Self::LocalSocket(socket.duplicate())),
        }
    }
}

fn open_fd_entry(
    process: &UserProcess,
    table: &FdTable,
    dirfd: i32,
    path: &str,
    flags: u32,
    mode: u32,
) -> Result<FdEntry, LinuxError> {
    if path_exceeds_linux_limits(path) {
        return Err(LinuxError::ENAMETOOLONG);
    }

    let mut opts = OpenOptions::new();
    let access = flags & general::O_ACCMODE;
    if access == general::O_WRONLY {
        opts.write(true);
    } else if access == general::O_RDWR {
        opts.read(true);
        opts.write(true);
    } else {
        opts.read(true);
    }
    if flags & general::O_APPEND != 0 {
        opts.append(true);
    }
    if flags & general::O_TRUNC != 0 {
        opts.truncate(true);
    }
    if flags & general::O_CREAT != 0 {
        opts.create(true);
        if access == general::O_RDONLY {
            opts.write(true);
        }
    }
    if flags & general::O_EXCL != 0 {
        opts.create_new(true);
    }

    let absolute = path.starts_with('/');
    let exec_root = process.exec_root();
    let add_busybox_aliases = busybox_applet_alias_allowed(flags, access);

    if absolute || dirfd == general::AT_FDCWD {
        let mut candidates = if absolute {
            if let Some(path) = dev_shm_host_path(path) {
                ensure_dev_shm_dir()?;
                return open_candidates(process, &[path], &opts, flags, mode);
            }
            runtime_absolute_path_candidates(exec_root.as_str(), path)
        } else {
            let cwd = process.cwd();
            let primary = normalize_path(cwd.as_str(), path).ok_or(LinuxError::EINVAL)?;
            let mut candidates = vec![primary];
            for extra in runtime_library_name_candidates(exec_root.as_str(), path) {
                push_runtime_candidate(&mut candidates, Some(extra));
            }
            candidates
        };
        if add_busybox_aliases {
            append_busybox_applet_alias_candidates(&mut candidates);
        }
        translate_mount_candidates(process, &mut candidates);
        if candidates.is_empty() {
            return Err(LinuxError::EINVAL);
        }
        open_candidates(process, &candidates, &opts, flags, mode)
    } else {
        let FdEntry::Directory(dir) = table.entry(dirfd)? else {
            return Err(LinuxError::ENOTDIR);
        };
        let primary = normalize_path(dir.path.as_str(), path).ok_or(LinuxError::EINVAL)?;
        let mut candidates = vec![primary];
        for extra in runtime_library_name_candidates(exec_root.as_str(), path) {
            push_runtime_candidate(&mut candidates, Some(extra));
        }
        if add_busybox_aliases {
            append_busybox_applet_alias_candidates(&mut candidates);
        }
        translate_mount_candidates(process, &mut candidates);
        open_candidates(process, &candidates, &opts, flags, mode)
    }
}

fn translate_mount_candidates(process: &UserProcess, candidates: &mut Vec<String>) {
    for candidate in candidates.iter_mut() {
        *candidate = process.translate_mount_path(candidate.as_str());
    }
    let mut deduped = Vec::new();
    for candidate in candidates.drain(..) {
        push_runtime_candidate(&mut deduped, Some(candidate));
    }
    *candidates = deduped;
}

fn busybox_applet_alias_allowed(flags: u32, access: u32) -> bool {
    access != general::O_WRONLY
        && access != general::O_RDWR
        && flags & (general::O_CREAT | general::O_TRUNC | general::O_APPEND) == 0
}

fn file_is_readable(status_flags: u32) -> bool {
    (status_flags & general::O_ACCMODE) != general::O_WRONLY
}

fn file_is_writable(status_flags: u32) -> bool {
    matches!(
        status_flags & general::O_ACCMODE,
        general::O_WRONLY | general::O_RDWR
    )
}

fn file_entry_read(file: &mut FileEntry, dst: &mut [u8]) -> Result<usize, LinuxError> {
    let mut offset = file.offset.lock();
    let read = file.file.read_at(*offset, dst).map_err(LinuxError::from)?;
    *offset = (*offset).saturating_add(read as u64);
    Ok(read)
}

fn file_entry_write(file: &mut FileEntry, src: &[u8]) -> Result<usize, LinuxError> {
    let mut offset = file.offset.lock();
    let write_offset = if file.status_flags & general::O_APPEND != 0 {
        file.file.get_attr().map_err(LinuxError::from)?.size()
    } else {
        *offset
    };
    let written = file
        .file
        .write_at(write_offset, src)
        .map_err(LinuxError::from)?;
    *offset = write_offset.saturating_add(written as u64);
    Ok(written)
}

fn file_entry_seek(file: &mut FileEntry, pos: SeekFrom) -> Result<u64, LinuxError> {
    let size = file.file.get_attr().map_err(LinuxError::from)?.size();
    let mut offset = file.offset.lock();
    let next = match pos {
        SeekFrom::Start(pos) => Some(pos),
        SeekFrom::Current(off) => (*offset).checked_add_signed(off),
        SeekFrom::End(off) => size.checked_add_signed(off),
    }
    .ok_or(LinuxError::EINVAL)?;
    *offset = next;
    Ok(next)
}

fn path_exceeds_linux_limits(path: &str) -> bool {
    path.len() >= LINUX_PATH_MAX
        || path
            .split('/')
            .any(|component| component.len() > LINUX_NAME_MAX)
}

fn parent_path(path: &str) -> &str {
    if path == "/" {
        return "/";
    }
    match path.rsplit_once('/') {
        Some(("", _)) => "/",
        Some((parent, _)) if !parent.is_empty() => parent,
        _ => "/",
    }
}

fn stat_absolute_path(process: &UserProcess, path: &str) -> Result<general::stat, LinuxError> {
    let attr = axfs::api::metadata(path).map_err(LinuxError::from)?;
    let mut st: general::stat = unsafe { core::mem::zeroed() };
    st.st_dev = 1;
    st.st_ino = path_inode(Some(path));
    st.st_mode = file_type_mode(attr.file_type()) | attr.permissions().bits() as u32;
    st.st_nlink = 1;
    st.st_size = attr.size() as _;
    st.st_blksize = 512;
    st.st_blocks = attr.blocks() as _;
    Ok(apply_recorded_path_metadata(process, path, st))
}

fn parent_dirs_searchable_absolute(
    process: &UserProcess,
    path: &str,
    uid: u32,
    gid: u32,
) -> Result<bool, LinuxError> {
    if uid == 0 {
        return Ok(true);
    }
    let components: Vec<&str> = path.split('/').filter(|part| !part.is_empty()).collect();
    if components.len() <= 1 {
        return Ok(true);
    }
    let mut parent = String::new();
    for component in &components[..components.len() - 1] {
        parent.push('/');
        parent.push_str(component);
        let st = stat_absolute_path(process, parent.as_str())?;
        if !access_allowed(&st, ACCESS_X_OK, uid, gid) {
            return Ok(false);
        }
    }
    Ok(true)
}

fn check_parent_write_search_permission(
    process: &UserProcess,
    path: &str,
) -> Result<general::stat, LinuxError> {
    let uid = process.fs_uid();
    let gid = process.fs_gid();
    let parent = parent_path(path);
    let parent_st = stat_absolute_path(process, parent)?;
    if parent_st.st_mode & ST_MODE_TYPE_MASK != ST_MODE_DIR {
        return Err(LinuxError::ENOTDIR);
    }
    if uid == 0 {
        return Ok(parent_st);
    }
    if !parent_dirs_searchable_absolute(process, parent, uid, gid)?
        || !access_allowed(&parent_st, ACCESS_W_OK | ACCESS_X_OK, uid, gid)
    {
        return Err(LinuxError::EACCES);
    }
    Ok(parent_st)
}

fn check_sticky_parent_permission(
    process: &UserProcess,
    parent_st: &general::stat,
    target_st: &general::stat,
) -> Result<(), LinuxError> {
    let uid = process.fs_uid();
    if uid == 0 || parent_st.st_mode & FILE_MODE_STICKY == 0 {
        return Ok(());
    }
    if uid == parent_st.st_uid as u32 || uid == target_st.st_uid as u32 {
        Ok(())
    } else {
        Err(LinuxError::EPERM)
    }
}

fn append_busybox_applet_alias_candidates(candidates: &mut Vec<String>) {
    for candidate in candidates.clone() {
        push_runtime_candidate(candidates, busybox_applet_target_path(candidate.as_str()));
    }
}

fn open_permission_mode(flags: u32) -> usize {
    match flags & general::O_ACCMODE {
        general::O_WRONLY => ACCESS_W_OK,
        general::O_RDWR => ACCESS_R_OK | ACCESS_W_OK,
        _ => ACCESS_R_OK,
    }
}

fn check_open_permission(process: &UserProcess, path: &str, flags: u32) -> Result<(), LinuxError> {
    if flags & O_PATH_FLAG != 0 {
        return Ok(());
    }
    let attr = match axfs::api::metadata(path) {
        Ok(attr) => attr,
        Err(_) => return Ok(()),
    };
    let mut st: general::stat = unsafe { core::mem::zeroed() };
    st.st_dev = 1;
    st.st_ino = path_inode(Some(path));
    st.st_mode = file_type_mode(attr.file_type()) | attr.permissions().bits() as u32;
    st.st_nlink = 1;
    st.st_size = attr.size() as _;
    st.st_blksize = 512;
    st.st_blocks = attr.blocks() as _;
    let st = apply_recorded_path_metadata(process, path, st);
    let uid = process.fs_uid();
    let gid = process.fs_gid();
    if !parent_dirs_searchable_absolute(process, path, uid, gid)? {
        return Err(LinuxError::EACCES);
    }
    if access_allowed(&st, open_permission_mode(flags), uid, gid) {
        Ok(())
    } else {
        Err(LinuxError::EACCES)
    }
}

fn fcntl_status_flags(open_flags: u32) -> u32 {
    open_flags
        & (general::O_ACCMODE
            | general::O_APPEND
            | general::O_NONBLOCK
            | general::O_DSYNC
            | general::O_SYNC
            | general::O_DIRECT
            | general::O_NOATIME)
}

fn fcntl_setfl_flags(flags: u32) -> u32 {
    flags & (general::O_APPEND | general::O_NONBLOCK | general::O_DIRECT | general::O_NOATIME)
}

fn open_candidates(
    process: &UserProcess,
    candidates: &[String],
    opts: &OpenOptions,
    flags: u32,
    mode: u32,
) -> Result<FdEntry, LinuxError> {
    let prefer_dir = flags & general::O_DIRECTORY != 0;
    let path_only = flags & O_PATH_FLAG != 0;
    let mut path_opts = OpenOptions::new();
    if path_only {
        path_opts.read(true);
    }
    let file_opts = if path_only { &path_opts } else { opts };
    let mut last_err = LinuxError::ENOENT;
    for path in candidates {
        if flags & O_NOFOLLOW_FLAG != 0 {
            if process.path_symlink(path.as_str()).is_some() {
                if prefer_dir {
                    return Err(LinuxError::ENOTDIR);
                }
                if path_only {
                    return Ok(FdEntry::Path(PathEntry::symlink(path.as_str())));
                }
                return Err(LinuxError::ELOOP);
            }
        } else if let Some(resolved_path) = process.resolve_path_symlink(path.as_str())? {
            return open_candidates(process, &[resolved_path], opts, flags, mode);
        }
        if is_proc_self_maps_path(path.as_str()) {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if !path_only && proc_self_maps_is_writable_open(flags) {
                return Err(LinuxError::EPERM);
            }
            return Ok(if path_only {
                proc_self_maps_path_entry(process)
            } else {
                proc_self_maps_fd_entry(process)
            });
        }
        if let Some(entry) = if path_only {
            proc_pagemap_path_entry(process, path.as_str())
        } else {
            proc_pagemap_fd_entry(process, path.as_str())
        } {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if !path_only && synthetic_file_is_writable_open(flags) {
                return Err(LinuxError::EPERM);
            }
            return Ok(entry);
        }
        if let Some(entry) = if path_only {
            proc_pid_stat_path_entry(process, path.as_str())
        } else {
            proc_pid_stat_fd_entry(process, path.as_str())
        } {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if !path_only && synthetic_file_is_writable_open(flags) {
                return Err(LinuxError::EPERM);
            }
            return Ok(entry);
        }
        if let Some(entry) = if path_only {
            proc_pid_status_path_entry(process, path.as_str())
        } else {
            proc_pid_status_fd_entry(process, path.as_str())
        } {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if !path_only && synthetic_file_is_writable_open(flags) {
                return Err(LinuxError::EPERM);
            }
            return Ok(entry);
        }
        if let Some(entry) = if path_only {
            proc_comm_path_entry(process, path.as_str())
        } else {
            proc_comm_fd_entry(process, path.as_str())
        } {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if !path_only && synthetic_file_is_writable_open(flags) {
                return Err(LinuxError::EPERM);
            }
            return Ok(entry);
        }
        if let Some((synthetic_path, data)) = synthetic_userdb_content(path.as_str()) {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if !path_only && synthetic_file_is_writable_open(flags) {
                return Err(LinuxError::EPERM);
            }
            return Ok(if path_only {
                synthetic_userdb_path_entry(synthetic_path, data)
            } else {
                synthetic_userdb_fd_entry(synthetic_path, data)
            });
        }
        if path == "/dev/null" {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            return Ok(if path_only {
                FdEntry::Path(PathEntry::synthetic_char("/dev/null"))
            } else {
                FdEntry::DevNull
            });
        }
        if is_synthetic_block_device_path(path.as_str()) {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            return Ok(if path_only {
                FdEntry::Path(PathEntry::synthetic_block(path.as_str()))
            } else {
                FdEntry::BlockDevice(BlockDeviceEntry { path: path.clone() })
            });
        }
        if path == "/dev/misc/rtc" || path == "/dev/rtc" {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            return Ok(if path_only {
                FdEntry::Path(PathEntry::synthetic_char(path.as_str()))
            } else {
                FdEntry::Rtc
            });
        }
        if process.path_special_mode(path.as_str()) == Some(ST_MODE_FIFO) {
            if prefer_dir {
                return Err(LinuxError::ENOTDIR);
            }
            if flags & general::O_CREAT != 0 && flags & general::O_EXCL != 0 {
                return Err(LinuxError::EEXIST);
            }
            check_open_permission(process, path.as_str(), flags)?;
            if flags & general::O_ACCMODE == general::O_WRONLY && flags & general::O_NONBLOCK != 0 {
                // This compatibility layer does not keep a rendezvous table
                // for named FIFO opens.  A nonblocking writer therefore has
                // no observable reader and must fail like Linux open(2).
                return Err(LinuxError::ENXIO);
            }
            if path_only {
                let mode = process.path_mode(path.as_str()).unwrap_or(0o666);
                return Ok(FdEntry::Path(PathEntry::fifo(path.as_str(), mode)));
            }
            let status_flags = flags & (general::O_NONBLOCK | general::O_DIRECT);
            let (read_end, write_end) = PipeEndpoint::new_pair(status_flags);
            return Ok(match flags & general::O_ACCMODE {
                general::O_WRONLY => FdEntry::Pipe(write_end),
                // Opening a FIFO with O_RDWR is Linux-specific but common in
                // tests to avoid blocking; any pipe endpoint is non-seekable,
                // preserving the required ESPIPE semantics.
                _ => FdEntry::Pipe(read_end),
            });
        }
        if prefer_dir {
            match open_dir_entry(path.as_str()) {
                Ok(FdEntry::Directory(dir)) if path_only => {
                    return Ok(path_entry_from_directory(dir));
                }
                Ok(entry) if !path_only => return Ok(entry),
                Ok(_) => return Err(LinuxError::EINVAL),
                Err(err) => record_missing_candidate(&mut last_err, err)?,
            }
            continue;
        }
        if flags & O_PATH_FLAG == 0
            && matches!(
                flags & general::O_ACCMODE,
                general::O_WRONLY | general::O_RDWR
            )
            && matches!(open_dir_entry(path.as_str()), Ok(FdEntry::Directory(_)))
        {
            return Err(LinuxError::EISDIR);
        }
        if !path_only && !prefer_dir && flags & general::O_ACCMODE == general::O_RDONLY {
            if let Ok(FdEntry::Directory(dir)) = open_dir_entry(path.as_str()) {
                check_open_permission(process, path.as_str(), flags)?;
                return Ok(FdEntry::Directory(dir));
            }
        }
        let created_by_this_open = !path_only
            && flags & general::O_CREAT != 0
            && axfs::api::metadata(path.as_str()).is_err();
        if flags & general::O_NOATIME != 0 && !created_by_this_open && process.uid() != 0 {
            let owner = process
                .path_owner(path.as_str())
                .map(|(uid, _)| uid)
                .unwrap_or(0);
            if owner != process.uid() {
                return Err(LinuxError::EPERM);
            }
        }
        if created_by_this_open {
            check_parent_write_search_permission(process, path.as_str())?;
        } else {
            check_open_permission(process, path.as_str(), flags)?;
        }
        match File::open(path.as_str(), file_opts) {
            Ok(file) if path_only => {
                let attr = file.get_attr().map_err(LinuxError::from)?;
                return Ok(FdEntry::Path(PathEntry::from_attr(path.as_str(), &attr)));
            }
            Ok(file) => {
                if created_by_this_open {
                    process.set_path_mode(path.clone(), process.apply_umask(mode));
                    process.set_path_owner(
                        path.clone(),
                        Some(process.fs_uid()),
                        Some(process.fs_gid()),
                    );
                }
                return Ok(FdEntry::File(FileEntry {
                    file,
                    path: path.clone(),
                    status_flags: fcntl_status_flags(flags),
                    offset: Arc::new(Mutex::new(0)),
                    lease_type: Arc::new(Mutex::new(general::F_UNLCK)),
                }));
            }
            Err(err) => {
                let err = LinuxError::from(err);
                if err == LinuxError::EISDIR {
                    return match open_dir_entry(path.as_str())? {
                        FdEntry::Directory(dir) if path_only => Ok(path_entry_from_directory(dir)),
                        entry if !path_only => Ok(entry),
                        _ => Err(LinuxError::EINVAL),
                    };
                }
                record_missing_candidate(&mut last_err, err)?;
            }
        }
    }
    Err(last_err)
}

fn path_entry_from_directory(dir: DirectoryEntry) -> FdEntry {
    FdEntry::Path(PathEntry::from_attr(dir.path.as_str(), &dir.attr))
}

fn is_synthetic_block_device_path(path: &str) -> bool {
    matches!(
        normalize_path("/", path).as_deref(),
        Some("/dev/vda" | "/dev/sda" | "/dev/xvda")
    )
}

fn record_missing_candidate(last_err: &mut LinuxError, err: LinuxError) -> Result<(), LinuxError> {
    match err {
        LinuxError::ENOENT => Ok(()),
        LinuxError::ENOTDIR => {
            // Runtime loader paths often probe absolute locations such as
            // `/lib/libc.so.6` before this compatibility layer redirects them
            // to the suite-local runtime root (`/glibc/lib/libc.so.6`,
            // `/musl/lib/libc.so`, etc.).  A missing leading directory is a
            // failed candidate, not proof that later runtime candidates are
            // invalid.  Preserve ENOTDIR as the final error if every candidate
            // misses, but keep searching the candidate list.
            if *last_err == LinuxError::ENOENT {
                *last_err = err;
            }
            Ok(())
        }
        _ => {
            *last_err = err;
            Err(err)
        }
    }
}

pub(super) fn open_dir_entry(path: &str) -> Result<FdEntry, LinuxError> {
    let mut opts = OpenOptions::new();
    opts.read(true);
    let dir = Directory::open_dir(path, &opts).map_err(LinuxError::from)?;
    let attr = dir.get_attr().map_err(LinuxError::from)?;
    Ok(FdEntry::Directory(DirectoryEntry {
        dir,
        attr,
        path: path.into(),
        next_dirent_cookie: 0,
    }))
}

fn directory_create_dir(path: &str) -> Result<(), LinuxError> {
    axfs::api::create_dir(path).map_err(LinuxError::from)
}

fn directory_remove_file(path: &str) -> Result<(), LinuxError> {
    axfs::api::remove_file(path).map_err(LinuxError::from)
}

fn directory_remove_dir(path: &str) -> Result<(), LinuxError> {
    axfs::api::remove_dir(path).map_err(LinuxError::from)
}

pub(super) fn resolve_dirfd_path(
    process: &UserProcess,
    table: &FdTable,
    dirfd: i32,
    path: &str,
) -> Result<String, LinuxError> {
    if path.starts_with('/') {
        return normalize_path("/", path)
            .map(|path| process.translate_mount_path(path.as_str()))
            .ok_or(LinuxError::EINVAL);
    }
    if dirfd == general::AT_FDCWD {
        let cwd = process.cwd();
        return normalize_path(cwd.as_str(), path)
            .map(|path| process.translate_mount_path(path.as_str()))
            .ok_or(LinuxError::EINVAL);
    }
    let FdEntry::Directory(dir) = table.entry(dirfd)? else {
        return Err(LinuxError::ENOTDIR);
    };
    if axfs::api::metadata(dir.path.as_str()).is_err() {
        return Err(LinuxError::ENOENT);
    }
    normalize_path(dir.path.as_str(), path)
        .map(|path| process.translate_mount_path(path.as_str()))
        .ok_or(LinuxError::EINVAL)
}
