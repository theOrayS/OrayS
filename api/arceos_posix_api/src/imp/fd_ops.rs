use alloc::sync::Arc;
use core::ffi::c_int;

use axerrno::{LinuxError, LinuxResult};
use axio::PollState;
use axns::{ResArc, def_resource};
use flatten_objects::FlattenObjects;
use spin::RwLock;

use crate::ctypes;
use crate::imp::stdio::{stdin, stdout};

pub const AX_FILE_LIMIT: usize = 1024;

#[allow(dead_code)]
pub trait FileLike: Send + Sync {
    fn read(&self, buf: &mut [u8]) -> LinuxResult<usize>;
    fn write(&self, buf: &[u8]) -> LinuxResult<usize>;
    fn stat(&self) -> LinuxResult<ctypes::stat>;
    fn into_any(self: Arc<Self>) -> Arc<dyn core::any::Any + Send + Sync>;
    fn poll(&self) -> LinuxResult<PollState>;
    fn status_flags(&self) -> LinuxResult<c_int>;
    fn set_nonblocking(&self, nonblocking: bool) -> LinuxResult;

    fn set_status_flags(&self, flags: c_int) -> LinuxResult {
        let allowed = (ctypes::O_ACCMODE | ctypes::O_NONBLOCK) as c_int;
        if flags & !allowed != 0 {
            return Err(LinuxError::EOPNOTSUPP);
        }
        self.set_nonblocking(flags & ctypes::O_NONBLOCK as c_int != 0)
    }
}

pub(crate) struct FdEntry {
    file: Arc<dyn FileLike>,
    fd_flags: c_int,
}

impl FdEntry {
    fn new(file: Arc<dyn FileLike>) -> Self {
        Self { file, fd_flags: 0 }
    }

    fn with_flags(file: Arc<dyn FileLike>, fd_flags: c_int) -> Self {
        Self {
            file,
            fd_flags: fd_flags & ctypes::FD_CLOEXEC as c_int,
        }
    }
}

def_resource! {
    pub(crate) static FD_TABLE: ResArc<RwLock<FlattenObjects<FdEntry, AX_FILE_LIMIT>>> = ResArc::new();
}

pub fn get_file_like(fd: c_int) -> LinuxResult<Arc<dyn FileLike>> {
    FD_TABLE
        .read()
        .get(fd as usize)
        .map(|entry| entry.file.clone())
        .ok_or(LinuxError::EBADF)
}

pub fn poll_file_like(fd: c_int) -> LinuxResult<PollState> {
    #[cfg(feature = "net")]
    axnet::poll_interfaces();

    get_file_like(fd)?.poll()
}

pub fn add_file_like(f: Arc<dyn FileLike>) -> LinuxResult<c_int> {
    Ok(FD_TABLE
        .write()
        .add(FdEntry::new(f))
        .map_err(|_| LinuxError::EMFILE)? as c_int)
}

pub(crate) fn add_file_like_with_flags(
    f: Arc<dyn FileLike>,
    fd_flags: c_int,
) -> LinuxResult<c_int> {
    Ok(FD_TABLE
        .write()
        .add(FdEntry::with_flags(f, fd_flags))
        .map_err(|_| LinuxError::EMFILE)? as c_int)
}

pub fn close_file_like(fd: c_int) -> LinuxResult {
    let entry = FD_TABLE
        .write()
        .remove(fd as usize)
        .ok_or(LinuxError::EBADF)?;
    drop(entry);
    Ok(())
}

pub fn fd_table_assigned_count() -> usize {
    FD_TABLE.read().count()
}

/// Close a file by `fd`.
pub fn sys_close(fd: c_int) -> c_int {
    debug!("sys_close <= {}", fd);
    // stdin/stdout/stderr are ordinary descriptors once installed in the
    // process FD table.  Closing them must make later I/O observe EBADF and
    // free the descriptor for reuse; returning success without removing the
    // entry hides real POSIX-visible state changes.
    syscall_body!(sys_close, close_file_like(fd).map(|_| 0))
}

fn dup_fd_from(old_fd: c_int, min_fd: c_int, fd_flags: c_int) -> LinuxResult<c_int> {
    if min_fd < 0 || min_fd as usize >= AX_FILE_LIMIT {
        return Err(LinuxError::EINVAL);
    }

    let f = get_file_like(old_fd)?;
    let mut fd_table = FD_TABLE.write();
    for new_fd in min_fd as usize..AX_FILE_LIMIT {
        if !fd_table.is_assigned(new_fd) {
            fd_table
                .add_at(new_fd, FdEntry::with_flags(f, fd_flags))
                .map_err(|_| LinuxError::EMFILE)?;
            return Ok(new_fd as c_int);
        }
    }
    Err(LinuxError::EMFILE)
}

fn dup_fd(old_fd: c_int) -> LinuxResult<c_int> {
    dup_fd_from(old_fd, 0, 0)
}

fn fd_flags(fd: c_int) -> LinuxResult<c_int> {
    FD_TABLE
        .read()
        .get(fd as usize)
        .map(|entry| entry.fd_flags)
        .ok_or(LinuxError::EBADF)
}

fn set_fd_flags(fd: c_int, flags: c_int) -> LinuxResult<c_int> {
    let mut fd_table = FD_TABLE.write();
    let entry = fd_table.get_mut(fd as usize).ok_or(LinuxError::EBADF)?;
    entry.fd_flags = flags & ctypes::FD_CLOEXEC as c_int;
    Ok(0)
}

/// Duplicate a file descriptor.
pub fn sys_dup(old_fd: c_int) -> c_int {
    debug!("sys_dup <= {}", old_fd);
    syscall_body!(sys_dup, dup_fd(old_fd))
}

/// Duplicate a file descriptor, but it uses the file descriptor number specified in `new_fd`.
///
/// TODO: `dup2` should forcibly close new_fd if it is already opened.
pub fn sys_dup2(old_fd: c_int, new_fd: c_int) -> c_int {
    debug!("sys_dup2 <= old_fd: {}, new_fd: {}", old_fd, new_fd);
    syscall_body!(sys_dup2, {
        if old_fd == new_fd {
            let r = sys_fcntl(old_fd, ctypes::F_GETFD as _, 0);
            if r >= 0 {
                return Ok(old_fd);
            } else {
                return Ok(r);
            }
        }
        if new_fd as usize >= AX_FILE_LIMIT {
            return Err(LinuxError::EBADF);
        }

        let f = get_file_like(old_fd)?;
        match FD_TABLE
            .write()
            .add_or_replace_at(new_fd as usize, FdEntry::new(f))
        {
            Ok(_) | Err(Some(_)) => Ok(new_fd),
            Err(None) => Err(LinuxError::EBADF),
        }
    })
}

/// Manipulate file descriptor.
pub fn sys_fcntl(fd: c_int, cmd: c_int, arg: usize) -> c_int {
    debug!("sys_fcntl <= fd: {} cmd: {} arg: {}", fd, cmd, arg);
    syscall_body!(sys_fcntl, {
        match cmd as u32 {
            ctypes::F_DUPFD => dup_fd_from(fd, arg as c_int, 0),
            ctypes::F_DUPFD_CLOEXEC => dup_fd_from(fd, arg as c_int, ctypes::FD_CLOEXEC as c_int),
            ctypes::F_GETFD => fd_flags(fd),
            ctypes::F_SETFD => set_fd_flags(fd, arg as c_int),
            ctypes::F_GETFL => get_file_like(fd)?.status_flags(),
            ctypes::F_SETFL => get_file_like(fd)?.set_status_flags(arg as c_int).map(|_| 0),
            _ => {
                warn!("unsupported fcntl parameters: cmd {}", cmd);
                Err(LinuxError::EINVAL)
            }
        }
    })
}

#[ctor_bare::register_ctor]
fn init_stdio() {
    let mut fd_table = flatten_objects::FlattenObjects::new();
    fd_table
        .add_at(0, FdEntry::new(Arc::new(stdin()) as _))
        .unwrap_or_else(|_| panic!()); // stdin
    fd_table
        .add_at(1, FdEntry::new(Arc::new(stdout()) as _))
        .unwrap_or_else(|_| panic!()); // stdout
    fd_table
        .add_at(2, FdEntry::new(Arc::new(stdout()) as _))
        .unwrap_or_else(|_| panic!()); // stderr
    FD_TABLE.init_new(spin::RwLock::new(fd_table));
}
