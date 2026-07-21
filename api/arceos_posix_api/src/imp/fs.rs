use alloc::{string::String, sync::Arc};
use core::{
    ffi::{c_char, c_int, c_void},
    sync::atomic::{AtomicI32, AtomicU32, Ordering},
};

use axerrno::{LinuxError, LinuxResult};
use axfs::fops::OpenOptions;
use axio::{PollState, SeekFrom};
use axsync::Mutex;

use super::fd_ops::{FileLike, get_file_like};
use crate::{
    ctypes,
    utils::{char_ptr_to_str, writable_user_buffer, write_user_value},
};

pub struct File {
    inner: Mutex<axfs::fops::File>,
    path: String,
    status_flags: AtomicI32,
}

static FILE_MODE_UMASK: AtomicU32 = AtomicU32::new(0o022);

impl File {
    fn new(inner: axfs::fops::File, path: &str, flags: c_int) -> Self {
        Self {
            inner: Mutex::new(inner),
            path: path.into(),
            status_flags: AtomicI32::new(open_status_flags(flags)),
        }
    }

    fn add_to_fd_table(self, fd_flags: c_int) -> LinuxResult<c_int> {
        super::fd_ops::add_file_like_with_flags(Arc::new(self), fd_flags)
    }

    fn from_fd(fd: c_int) -> LinuxResult<Arc<Self>> {
        let f = super::fd_ops::get_file_like(fd)?;
        f.into_any()
            .downcast::<Self>()
            .map_err(|_| LinuxError::EINVAL)
    }
}

const DEFAULT_STAT_DEV: ctypes::dev_t = 1;
const DEFAULT_STAT_UID: ctypes::uid_t = 0;
const DEFAULT_STAT_GID: ctypes::gid_t = 0;
const DEFAULT_STAT_BLKSIZE: ctypes::blksize_t = 512;

fn path_inode(path: Option<&str>) -> ctypes::ino_t {
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
    hash.max(1) as _
}

fn stat_from_parts(
    file_type: u8,
    perm: u32,
    size: u64,
    blocks: u64,
    path: Option<&str>,
) -> ctypes::stat {
    let st_mode = ((file_type as u32) << 12) | perm;
    ctypes::stat {
        st_dev: DEFAULT_STAT_DEV,
        st_ino: path_inode(path),
        st_nlink: if file_type == axfs::fops::FileType::Dir as u8 {
            2
        } else {
            1
        },
        st_mode,
        // axfs currently does not expose uid/gid/timestamps through Metadata.
        // Keep the default centralized and visible rather than scattering
        // ad-hoc owners across stat implementations.
        st_uid: DEFAULT_STAT_UID,
        st_gid: DEFAULT_STAT_GID,
        st_size: size as _,
        st_blocks: blocks as _,
        st_blksize: DEFAULT_STAT_BLKSIZE,
        ..Default::default()
    }
}

fn file_attr_to_stat(metadata: axfs::fops::FileAttr, path: Option<&str>) -> ctypes::stat {
    stat_from_parts(
        metadata.file_type() as u8,
        metadata.perm().bits() as u32,
        metadata.size(),
        metadata.blocks(),
        path,
    )
}

fn api_metadata_to_stat(metadata: axfs::api::Metadata, path: Option<&str>) -> ctypes::stat {
    stat_from_parts(
        metadata.file_type() as u8,
        metadata.permissions().bits() as u32,
        metadata.size(),
        metadata.blocks(),
        path,
    )
}

fn stat_path(path: &str) -> LinuxResult<ctypes::stat> {
    Ok(api_metadata_to_stat(axfs::api::metadata(path)?, Some(path)))
}

fn lstat_path(path: &str) -> LinuxResult<ctypes::stat> {
    Ok(api_metadata_to_stat(
        axfs::api::symlink_metadata(path)?,
        Some(path),
    ))
}

impl FileLike for File {
    fn read(&self, buf: &mut [u8]) -> LinuxResult<usize> {
        Ok(self.inner.lock().read(buf)?)
    }

    fn write(&self, buf: &[u8]) -> LinuxResult<usize> {
        let mut inner = self.inner.lock();
        if self.status_flags.load(Ordering::Acquire) & ctypes::O_APPEND as c_int != 0 {
            inner.seek(SeekFrom::End(0))?;
        }
        Ok(inner.write(buf)?)
    }

    fn stat(&self) -> LinuxResult<ctypes::stat> {
        let metadata = self.inner.lock().get_attr()?;
        Ok(file_attr_to_stat(metadata, Some(self.path.as_str())))
    }

    fn into_any(self: Arc<Self>) -> Arc<dyn core::any::Any + Send + Sync> {
        self
    }

    fn poll(&self) -> LinuxResult<PollState> {
        Ok(PollState {
            readable: true,
            writable: true,
        })
    }

    fn status_flags(&self) -> LinuxResult<c_int> {
        Ok(self.status_flags.load(Ordering::Acquire))
    }

    fn set_nonblocking(&self, nonblocking: bool) -> LinuxResult {
        let mask = ctypes::O_NONBLOCK as c_int;
        if nonblocking {
            self.status_flags.fetch_or(mask, Ordering::AcqRel);
        } else {
            self.status_flags.fetch_and(!mask, Ordering::AcqRel);
        }
        Ok(())
    }

    fn set_status_flags(&self, flags: c_int) -> LinuxResult {
        let allowed = (ctypes::O_ACCMODE | ctypes::O_APPEND | ctypes::O_NONBLOCK) as c_int;
        if flags & !allowed != 0 {
            return Err(LinuxError::EOPNOTSUPP);
        }
        let access = self.status_flags.load(Ordering::Acquire) & ctypes::O_ACCMODE as c_int;
        let mutable = flags & (ctypes::O_APPEND | ctypes::O_NONBLOCK) as c_int;
        self.status_flags.store(access | mutable, Ordering::Release);
        Ok(())
    }
}

unsafe fn write_stat_output(buf: *mut ctypes::stat, value: ctypes::stat) -> LinuxResult {
    unsafe { write_user_value(buf, value) }
}

/// Convert open flags to [`OpenOptions`].
fn flags_to_options(flags: c_int, mode: ctypes::mode_t) -> LinuxResult<OpenOptions> {
    let flags = flags as u32;
    let supported = ctypes::O_RDONLY
        | ctypes::O_WRONLY
        | ctypes::O_RDWR
        | ctypes::O_CREAT
        | ctypes::O_EXCL
        | ctypes::O_TRUNC
        | ctypes::O_APPEND
        | ctypes::O_NONBLOCK
        | ctypes::O_CLOEXEC
        | ctypes::O_LARGEFILE
        | ctypes::O_NOCTTY;
    let unsupported = flags & !supported;
    if unsupported != 0 {
        return Err(LinuxError::EOPNOTSUPP);
    }

    let mut options = OpenOptions::new();
    let create_mode = (mode as u32) & !FILE_MODE_UMASK.load(Ordering::Acquire) & 0o7777;
    options.mode(create_mode);
    match flags & 0b11 {
        ctypes::O_RDONLY => options.read(true),
        ctypes::O_WRONLY => options.write(true),
        ctypes::O_RDWR => {
            options.read(true);
            options.write(true);
        }
        _ => return Err(LinuxError::EINVAL),
    };
    if flags & ctypes::O_APPEND != 0 {
        options.append(true);
    }
    if flags & ctypes::O_TRUNC != 0 {
        options.truncate(true);
    }
    if flags & ctypes::O_CREAT != 0 {
        if flags & ctypes::O_EXCL != 0 {
            options.create_new(true);
        } else {
            options.create(true);
        }
    }
    Ok(options)
}

pub fn sys_umask(mask: ctypes::mode_t) -> ctypes::mode_t {
    FILE_MODE_UMASK.swap((mask as u32) & 0o777, Ordering::AcqRel) as ctypes::mode_t
}

fn open_fd_flags(flags: c_int) -> c_int {
    let flags = flags as u32;
    if flags & ctypes::O_CLOEXEC != 0 {
        ctypes::FD_CLOEXEC as c_int
    } else {
        0
    }
}

fn open_status_flags(flags: c_int) -> c_int {
    let flags = flags as u32;
    (flags & (ctypes::O_ACCMODE | ctypes::O_APPEND | ctypes::O_NONBLOCK)) as c_int
}

/// Open a file by `filename` and insert it into the file descriptor table.
///
/// Return its index in the file table (`fd`). Return `EMFILE` if it already
/// has the maximum number of files open.
///
/// # Safety
///
/// `filename` must be a valid, NUL-terminated C string.
pub unsafe fn sys_open(filename: *const c_char, flags: c_int, mode: ctypes::mode_t) -> c_int {
    let filename = char_ptr_to_str(filename);
    debug!("sys_open <= {:?} {:#o} {:#o}", filename, flags, mode);
    syscall_body!(sys_open, {
        let options = flags_to_options(flags, mode)?;
        let filename = filename?;
        let file = axfs::fops::File::open(filename, &options)?;
        File::new(file, filename, flags).add_to_fd_table(open_fd_flags(flags))
    })
}

/// Set the position of the file indicated by `fd`.
///
/// Return its position after seek.
pub fn sys_lseek(fd: c_int, offset: ctypes::off_t, whence: c_int) -> ctypes::off_t {
    debug!("sys_lseek <= {} {} {}", fd, offset, whence);
    syscall_body!(sys_lseek, {
        let pos = match whence {
            0 => SeekFrom::Start(offset as _),
            1 => SeekFrom::Current(offset as _),
            2 => SeekFrom::End(offset as _),
            _ => return Err(LinuxError::EINVAL),
        };
        let off = File::from_fd(fd)?.inner.lock().seek(pos)?;
        Ok(off)
    })
}

/// Get the file metadata by `path` and write into `buf`.
///
/// Return 0 if success.
///
/// # Safety
///
/// `path` must be a valid, NUL-terminated C string. `buf` must be writable for
/// one `stat` value when non-null.
pub unsafe fn sys_stat(path: *const c_char, buf: *mut ctypes::stat) -> c_int {
    let path = char_ptr_to_str(path);
    debug!("sys_stat <= {:?} {:#x}", path, buf as usize);
    syscall_body!(sys_stat, {
        let st = stat_path(path?)?;
        unsafe { write_stat_output(buf, st)? };
        Ok(0)
    })
}

/// Get file metadata by `fd` and write into `buf`.
///
/// Return 0 if success.
///
/// # Safety
///
/// `buf` must be writable for one `stat` value when non-null.
pub unsafe fn sys_fstat(fd: c_int, buf: *mut ctypes::stat) -> c_int {
    debug!("sys_fstat <= {} {:#x}", fd, buf as usize);
    syscall_body!(sys_fstat, {
        let st = get_file_like(fd)?.stat()?;
        unsafe { write_stat_output(buf, st)? };
        Ok(0)
    })
}

/// Get the metadata of the symbolic link and write into `buf`.
///
/// Return 0 if success.
///
/// # Safety
///
/// `path` must be a valid, NUL-terminated C string. `buf` must be writable for
/// one `stat` value when non-null.
pub unsafe fn sys_lstat(path: *const c_char, buf: *mut ctypes::stat) -> ctypes::ssize_t {
    let path = char_ptr_to_str(path);
    debug!("sys_lstat <= {:?} {:#x}", path, buf as usize);
    syscall_body!(sys_lstat, {
        let st = lstat_path(path?)?;
        unsafe { write_stat_output(buf, st)? };
        Ok(0)
    })
}

/// Get the path of the current directory.
///
/// # Safety
///
/// `buf` must be writable for `size` bytes when non-null.
#[allow(clippy::unnecessary_cast)] // `c_char` is either `i8` or `u8`
pub unsafe fn sys_getcwd(buf: *mut c_char, size: usize) -> *mut c_char {
    debug!("sys_getcwd <= {:#x} {}", buf as usize, size);
    syscall_body!(sys_getcwd, {
        if buf.is_null() {
            return Err(LinuxError::EFAULT);
        }
        if size == 0 {
            return Err(LinuxError::EINVAL);
        }
        let cwd = axfs::api::current_dir()?;
        let cwd = cwd.as_bytes();
        let dst = unsafe { writable_user_buffer(buf.cast::<c_void>(), size)? };
        if cwd.len() < dst.len() {
            dst[..cwd.len()].copy_from_slice(cwd);
            dst[cwd.len()] = 0;
            Ok(buf)
        } else {
            Err(LinuxError::ERANGE)
        }
    })
}

/// Rename `old` to `new`
/// If new exists, it is first removed.
///
/// Return 0 if the operation succeeds, otherwise return -1.
///
/// # Safety
///
/// `old` and `new` must be valid, NUL-terminated C strings.
pub unsafe fn sys_rename(old: *const c_char, new: *const c_char) -> c_int {
    syscall_body!(sys_rename, {
        let old_path = char_ptr_to_str(old)?;
        let new_path = char_ptr_to_str(new)?;
        debug!("sys_rename <= old: {:?}, new: {:?}", old_path, new_path);
        axfs::api::rename(old_path, new_path)?;
        Ok(0)
    })
}
