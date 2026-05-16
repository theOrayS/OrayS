use core::cmp;
use core::mem::offset_of;
use core::ptr;

use axerrno::LinuxError;
use axfs::fops::{self, Directory, File, FileAttr, OpenOptions};
use axio::SeekFrom;
use linux_raw_sys::general;
use std::string::{String, ToString};
use std::sync::Arc;
use std::vec::Vec;

use super::UserProcess;
use super::credentials::access_allowed;
use super::fd_pipe::PipeEndpoint;
use super::fd_socket::{LocalSocketEntry, SocketEntry};
use super::linux_abi::{
    ACCESS_X_OK, MAX_IN_MEMORY_FILE_SIZE, O_PATH_FLAG, ST_MODE_CHR, ST_MODE_DIR, ST_MODE_FILE,
    ST_MODE_TYPE_MASK, fd_cloexec_flag, neg_errno, posix_ret_i32,
};
use super::memory_map::align_up;
use super::metadata::{
    apply_recorded_path_metadata, canonical_permission_path, dirent_type, fd_entry_path,
    fd_entry_statfs_path, file_attr_to_stat, file_type_mode, generic_statfs, path_inode,
    stdio_stat,
};
use super::runtime_paths::{
    busybox_applet_target_path, normalize_path, push_runtime_candidate, resolve_host_path,
    runtime_absolute_path_candidates, runtime_library_name_candidates,
};
use super::select_fdset::SelectMode;
use super::synthetic_fs::{
    dev_shm_host_path, ensure_dev_shm_dir, is_proc_self_maps_path, proc_self_maps_fd_entry,
    proc_self_maps_is_writable_open, proc_self_maps_path_entry, synthetic_file_is_writable_open,
    synthetic_userdb_content, synthetic_userdb_fd_entry, synthetic_userdb_path_entry,
};
use super::user_memory::{validate_user_write, write_user_bytes};

pub(super) struct FdTable {
    pub(super) entries: Vec<Option<FdEntry>>,
    pub(super) fd_flags: Vec<u32>,
}

pub(super) enum FdEntry {
    Stdin,
    Stdout,
    Stderr,
    DevNull,
    Rtc,
    File(FileEntry),
    Directory(DirectoryEntry),
    Path(PathEntry),
    MemoryFile(MemoryFileEntry),
    Pipe(PipeEndpoint),
    Socket(SocketEntry),
    LocalSocket(LocalSocketEntry),
}

#[derive(Clone)]
pub(super) struct FileEntry {
    pub(super) file: File,
    pub(super) path: String,
}

#[derive(Clone)]
pub(super) struct DirectoryEntry {
    pub(super) dir: Directory,
    pub(super) attr: FileAttr,
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

pub(super) fn sys_ftruncate(process: &UserProcess, fd: usize, length: usize) -> isize {
    let length = length as isize;
    if length < 0 {
        return neg_errno(LinuxError::EINVAL);
    }
    match process.fds.lock().truncate(fd as i32, length as u64) {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_close(process: &UserProcess, fd: usize) -> isize {
    match process.fds.lock().close(fd as i32) {
        Ok(()) => 0,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_getdents64(process: &UserProcess, fd: usize, dirp: usize, count: usize) -> isize {
    if let Err(err) = validate_user_write(process, dirp, count) {
        return neg_errno(err);
    }
    let bytes = match process.fds.lock().getdents64(fd as i32, count) {
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
        .dup3(oldfd as i32, newfd as i32, flags as u32)
    {
        Ok(fd) => fd as isize,
        Err(err) => neg_errno(err),
    }
}

pub(super) fn sys_fcntl(process: &UserProcess, fd: usize, cmd: usize, arg: usize) -> isize {
    match process.fds.lock().fcntl(fd as i32, cmd as u32, arg) {
        Ok(v) => v as isize,
        Err(err) => neg_errno(err),
    }
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

    pub(super) fn poll(&self, fd: i32, mode: SelectMode) -> bool {
        let Ok(entry) = self.entry(fd) else {
            return matches!(mode, SelectMode::Except);
        };
        match mode {
            SelectMode::Read => match entry {
                FdEntry::Stdin => false,
                FdEntry::Stdout | FdEntry::Stderr => false,
                FdEntry::DevNull
                | FdEntry::Rtc
                | FdEntry::File(_)
                | FdEntry::Directory(_)
                | FdEntry::MemoryFile(_) => true,
                FdEntry::Path(_) => false,
                FdEntry::Pipe(pipe) => pipe.poll().readable,
                FdEntry::Socket(socket) => socket.poll(mode),
                FdEntry::LocalSocket(socket) => socket.poll(mode),
            },
            SelectMode::Write => match entry {
                FdEntry::Stdin => false,
                FdEntry::Stdout | FdEntry::Stderr | FdEntry::DevNull | FdEntry::Rtc => true,
                FdEntry::File(_) => true,
                FdEntry::Directory(_) | FdEntry::Path(_) | FdEntry::MemoryFile(_) => false,
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
            FdEntry::Rtc => Ok(0),
            FdEntry::File(file) => file.file.read(dst).map_err(LinuxError::from),
            FdEntry::MemoryFile(file) => Ok(file.read(dst)),
            FdEntry::Directory(_) => Err(LinuxError::EISDIR),
            FdEntry::Pipe(pipe) => pipe.read(dst),
            FdEntry::Socket(socket) => socket.read(dst),
            FdEntry::LocalSocket(socket) => socket.read(dst),
            _ => Err(LinuxError::EBADF),
        }
    }

    pub(super) fn write(&mut self, fd: i32, src: &[u8]) -> Result<usize, LinuxError> {
        match self.entry_mut(fd)? {
            FdEntry::Stdout | FdEntry::Stderr => {
                axhal::console::write_bytes(src);
                Ok(src.len())
            }
            FdEntry::DevNull => Ok(src.len()),
            FdEntry::Rtc => Ok(src.len()),
            FdEntry::File(file) => file.file.write(src).map_err(LinuxError::from),
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
            return Err(LinuxError::EBADF);
        };
        let mut written = 0usize;
        while written < src.len() {
            let count = file
                .file
                .write_at(offset + written as u64, &src[written..])
                .map_err(LinuxError::from)?;
            if count == 0 {
                break;
            }
            written += count;
        }
        Ok(written)
    }

    fn close_slot(&mut self, idx: usize) -> Result<(), LinuxError> {
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
                if size > MAX_IN_MEMORY_FILE_SIZE {
                    return Err(LinuxError::ENOSPC);
                }
                file.file.truncate(size).map_err(LinuxError::from)
            }
            FdEntry::DevNull => Ok(()),
            FdEntry::Rtc => Ok(()),
            FdEntry::Path(_) | FdEntry::MemoryFile(_) => Err(LinuxError::EBADF),
            _ => Err(LinuxError::EINVAL),
        }
    }

    pub(super) fn lseek(&mut self, fd: i32, offset: i64, whence: u32) -> Result<u64, LinuxError> {
        let pos = match whence {
            general::SEEK_SET => SeekFrom::Start(offset.max(0) as u64),
            general::SEEK_CUR => SeekFrom::Current(offset),
            general::SEEK_END => SeekFrom::End(offset),
            _ => return Err(LinuxError::EINVAL),
        };
        match self.entry_mut(fd)? {
            FdEntry::File(file) => file.file.seek(pos).map_err(LinuxError::from),
            FdEntry::DevNull => Ok(0),
            FdEntry::Rtc => Ok(0),
            FdEntry::Directory(_) => Err(LinuxError::EISDIR),
            FdEntry::Path(_) => Err(LinuxError::EBADF),
            FdEntry::MemoryFile(file) => file.seek(pos),
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
        let entry = self.entry(fd)?.duplicate_for_fork()?;
        self.insert_min_with_flags(entry, min_fd as usize, fd_flags & general::FD_CLOEXEC)
    }

    pub(super) fn dup3(&mut self, oldfd: i32, newfd: i32, flags: u32) -> Result<i32, LinuxError> {
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
        if self.entries.len() <= newfd {
            self.entries.resize_with(newfd + 1, || None);
            self.fd_flags.resize(newfd + 1, 0);
        } else if self.entries[newfd].is_some() {
            let _ = self.close(newfd as i32);
        }
        if self.fd_flags.len() <= newfd {
            self.fd_flags.resize(newfd + 1, 0);
        }
        self.entries[newfd] = Some(entry);
        self.fd_flags[newfd] = fd_cloexec_flag(flags & general::O_CLOEXEC != 0);
        Ok(newfd as i32)
    }

    pub(super) fn getdents64(&mut self, fd: i32, max_len: usize) -> Result<Vec<u8>, LinuxError> {
        let entry = self.entry_mut(fd)?;
        let FdEntry::Directory(dir) = entry else {
            return Err(LinuxError::ENOTDIR);
        };
        let mut read_buf: [fops::DirEntry; 16] =
            core::array::from_fn(|_| fops::DirEntry::default());
        let count = dir.dir.read_dir(&mut read_buf).map_err(LinuxError::from)?;
        let mut out = Vec::new();
        for (idx, item) in read_buf[..count].iter().enumerate() {
            let name = item.name_as_bytes();
            let reclen = align_up(
                offset_of!(general::linux_dirent64, d_name) + name.len() + 1,
                8,
            );
            if out.len() + reclen > max_len {
                break;
            }
            let start = out.len();
            out.resize(start + reclen, 0);
            unsafe {
                let dirent = out[start..].as_mut_ptr() as *mut general::linux_dirent64;
                ptr::write_unaligned(
                    dirent,
                    general::linux_dirent64 {
                        d_ino: (idx + 1) as _,
                        d_off: 0,
                        d_reclen: reclen as _,
                        d_type: dirent_type(item.entry_type()) as u8,
                        d_name: Default::default(),
                    },
                );
            }
            let name_start = start + offset_of!(general::linux_dirent64, d_name);
            out[name_start..name_start + name.len()].copy_from_slice(name);
        }
        Ok(out)
    }

    pub(super) fn read_file_at(
        &mut self,
        fd: i32,
        offset: u64,
        len: usize,
    ) -> Result<Vec<u8>, LinuxError> {
        let FdEntry::File(file) = self.entry_mut(fd)? else {
            return Err(LinuxError::EBADF);
        };
        let mut buf = vec![0u8; len];
        let filled = read_file_at_into(&file.file, offset, &mut buf)?;
        buf.truncate(filled);
        Ok(buf)
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
            .skip(min_fd)
            .find(|(_, slot)| slot.is_none())
        {
            *slot = Some(entry);
            self.fd_flags[idx] = fd_flags & general::FD_CLOEXEC;
            return Ok(idx as i32);
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
        if path.starts_with('/') || dirfd == general::AT_FDCWD {
            let cwd = process.cwd();
            let abs_path = resolve_host_path(cwd, path).map_err(|_| LinuxError::EINVAL)?;
            directory_create_dir(abs_path.as_str())?;
            process.set_path_mode(abs_path, mode);
            return Ok(());
        }
        let FdEntry::Directory(dir) = self.entry(dirfd)? else {
            return Err(LinuxError::ENOTDIR);
        };
        let abs_path = normalize_path(dir.path.as_str(), path).ok_or(LinuxError::EINVAL)?;
        dir.dir.create_dir(path).map_err(LinuxError::from)?;
        process.set_path_mode(abs_path, mode);
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
        if path.starts_with('/') || dirfd == general::AT_FDCWD {
            let cwd = process.cwd();
            let abs_path = resolve_host_path(cwd, path).map_err(|_| LinuxError::EINVAL)?;
            return if remove_dir {
                directory_remove_dir(abs_path.as_str())
            } else {
                directory_remove_file(abs_path.as_str())
            };
        }
        let FdEntry::Directory(dir) = self.entry(dirfd)? else {
            return Err(LinuxError::ENOTDIR);
        };
        if remove_dir {
            dir.dir.remove_dir(path).map_err(LinuxError::from)
        } else {
            dir.dir.remove_file(path).map_err(LinuxError::from)
        }
    }

    pub(super) fn stat(&mut self, fd: i32) -> Result<general::stat, LinuxError> {
        match self.entry_mut(fd)? {
            FdEntry::Stdin => Ok(stdio_stat(true)),
            FdEntry::Stdout | FdEntry::Stderr => Ok(stdio_stat(false)),
            FdEntry::DevNull => Ok(stdio_stat(false)),
            FdEntry::Rtc => Ok(stdio_stat(false)),
            FdEntry::File(file) => Ok(file_attr_to_stat(
                &file.file.get_attr().map_err(LinuxError::from)?,
                Some(file.path.as_str()),
            )),
            FdEntry::Directory(dir) => Ok(file_attr_to_stat(&dir.attr, Some(dir.path.as_str()))),
            FdEntry::Path(path) => Ok(path.stat()),
            FdEntry::MemoryFile(file) => Ok(file.stat()),
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
        match open_fd_entry(process, self, dirfd, path, general::O_RDONLY, 0) {
            Ok(FdEntry::DevNull) | Ok(FdEntry::Rtc) => Ok(stdio_stat(false)),
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
        let entry = open_fd_entry(process, self, dirfd, path, general::O_RDONLY, 0)?;
        Ok(generic_statfs(fd_entry_statfs_path(&entry)))
    }

    pub(super) fn fcntl(&mut self, fd: i32, cmd: u32, arg: usize) -> Result<i32, LinuxError> {
        if matches!(self.entry(fd)?, FdEntry::Path(_)) && cmd == general::F_GETFL {
            return Ok(O_PATH_FLAG as i32);
        }
        let local_socket = match self.entry(fd)? {
            FdEntry::LocalSocket(socket) => Some(socket.clone()),
            _ => None,
        };
        if let Some(socket) = local_socket {
            return match cmd {
                general::F_DUPFD => {
                    self.insert_min_with_flags(FdEntry::LocalSocket(socket.duplicate()), arg, 0)
                }
                general::F_DUPFD_CLOEXEC => self.insert_min_with_flags(
                    FdEntry::LocalSocket(socket.duplicate()),
                    arg,
                    general::FD_CLOEXEC,
                ),
                general::F_GETFD => self.get_fd_flags(fd),
                general::F_SETFD => self.set_fd_flags(fd, arg as u32),
                general::F_GETFL => Ok(socket.status_flags()),
                general::F_SETFL => Ok(0),
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
            general::F_GETFL | general::F_SETFL => Ok(0),
            _ => Ok(0),
        }
    }
}

impl PathEntry {
    pub(super) fn from_attr(path: &str, attr: &FileAttr) -> Self {
        Self {
            path: path.into(),
            mode: file_type_mode(attr.file_type()) | attr.perm().bits() as u32,
            size: attr.size(),
            blocks: attr.blocks(),
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

    pub(super) fn stat(&self) -> general::stat {
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

impl FdEntry {
    pub(super) fn duplicate_for_fork(&self) -> Result<Self, LinuxError> {
        match self {
            Self::Stdin => Ok(Self::Stdin),
            Self::Stdout => Ok(Self::Stdout),
            Self::Stderr => Ok(Self::Stderr),
            Self::DevNull => Ok(Self::DevNull),
            Self::Rtc => Ok(Self::Rtc),
            Self::File(file) => Ok(Self::File(file.clone())),
            Self::Directory(dir) => Ok(Self::Directory(dir.clone())),
            Self::Path(path) => Ok(Self::Path(path.clone())),
            Self::MemoryFile(file) => Ok(Self::MemoryFile(file.clone())),
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
        open_candidates(process, &candidates, &opts, flags, mode)
    }
}

fn busybox_applet_alias_allowed(flags: u32, access: u32) -> bool {
    access != general::O_WRONLY
        && access != general::O_RDWR
        && flags & (general::O_CREAT | general::O_TRUNC | general::O_APPEND) == 0
}

fn append_busybox_applet_alias_candidates(candidates: &mut Vec<String>) {
    for candidate in candidates.clone() {
        push_runtime_candidate(candidates, busybox_applet_target_path(candidate.as_str()));
    }
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
        let created_by_this_open = !path_only
            && flags & general::O_CREAT != 0
            && axfs::api::metadata(path.as_str()).is_err();
        match File::open(path.as_str(), file_opts) {
            Ok(file) if path_only => {
                let attr = file.get_attr().map_err(LinuxError::from)?;
                return Ok(FdEntry::Path(PathEntry::from_attr(path.as_str(), &attr)));
            }
            Ok(file) => {
                if created_by_this_open {
                    process.set_path_mode(path.clone(), mode);
                    process.set_path_owner(path.clone(), Some(process.uid()), Some(process.gid()));
                }
                return Ok(FdEntry::File(FileEntry {
                    file,
                    path: path.clone(),
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

fn record_missing_candidate(last_err: &mut LinuxError, err: LinuxError) -> Result<(), LinuxError> {
    *last_err = err;
    if err == LinuxError::ENOENT {
        Ok(())
    } else {
        Err(err)
    }
}

pub(super) fn open_dir_entry(path: &str) -> Result<FdEntry, LinuxError> {
    let mut opts = OpenOptions::new();
    opts.read(true);
    let dir = Directory::open_dir(path, &opts).map_err(LinuxError::from)?;
    let file = File::open(path, &opts).map_err(LinuxError::from)?;
    let attr = file.get_attr().map_err(LinuxError::from)?;
    Ok(FdEntry::Directory(DirectoryEntry {
        dir,
        attr,
        path: path.into(),
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
        return normalize_path("/", path).ok_or(LinuxError::EINVAL);
    }
    if dirfd == general::AT_FDCWD {
        let cwd = process.cwd();
        return normalize_path(cwd.as_str(), path).ok_or(LinuxError::EINVAL);
    }
    let FdEntry::Directory(dir) = table.entry(dirfd)? else {
        return Err(LinuxError::ENOTDIR);
    };
    normalize_path(dir.path.as_str(), path).ok_or(LinuxError::EINVAL)
}
