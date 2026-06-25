use core::ffi::{c_int, c_ulong};

use axerrno::{LinuxError, LinuxResult};
use axhal::time::wall_time;

use crate::{
    ctypes,
    imp::fd_ops::get_file_like,
    utils::{read_user_value, write_user_value},
};

const FD_SETSIZE: usize = 1024;
const BITS_PER_WORD: usize = c_ulong::BITS as usize;
const FD_SETSIZE_WORDS: usize = FD_SETSIZE.div_ceil(BITS_PER_WORD);
const READ_SET: usize = 0;
const WRITE_SET: usize = 1;
const EXCEPT_SET: usize = 2;
const FD_SET_GROUPS: usize = 3;

struct FdSets {
    nfds: usize,
    bits: [[c_ulong; FD_SETSIZE_WORDS]; FD_SET_GROUPS],
}

fn timeval_to_duration(tv: ctypes::timeval) -> LinuxResult<core::time::Duration> {
    if tv.tv_sec < 0 || tv.tv_usec < 0 || tv.tv_usec > 999_999 {
        return Err(LinuxError::EINVAL);
    }
    Ok(tv.into())
}

impl FdSets {
    fn empty(nfds: usize) -> Self {
        Self {
            nfds,
            bits: [[0; FD_SETSIZE_WORDS]; FD_SET_GROUPS],
        }
    }

    fn from(
        nfds: usize,
        read_fds: *const ctypes::fd_set,
        write_fds: *const ctypes::fd_set,
        except_fds: *const ctypes::fd_set,
    ) -> LinuxResult<Self> {
        let mut sets = Self::empty(nfds);
        sets.copy_from_fd_set(READ_SET, read_fds)?;
        sets.copy_from_fd_set(WRITE_SET, write_fds)?;
        sets.copy_from_fd_set(EXCEPT_SET, except_fds)?;
        Ok(sets)
    }

    fn nfds_words(&self) -> usize {
        self.nfds.div_ceil(BITS_PER_WORD)
    }

    fn clear(&mut self) {
        let words = self.nfds_words();
        for set in &mut self.bits {
            set[..words].fill(0);
        }
    }

    fn copy_from_fd_set(&mut self, set_idx: usize, fds: *const ctypes::fd_set) -> LinuxResult {
        if fds.is_null() {
            return Ok(());
        }
        let words = self.nfds_words();
        let src = unsafe { read_user_value(fds)? };
        self.bits[set_idx][..words].copy_from_slice(&src.fds_bits[..words]);
        Ok(())
    }

    fn set_fd(&mut self, set_idx: usize, fd: usize) {
        self.bits[set_idx][fd / BITS_PER_WORD] |= 1 << (fd % BITS_PER_WORD);
    }

    unsafe fn write_back_to(
        &self,
        read_fds: *mut ctypes::fd_set,
        write_fds: *mut ctypes::fd_set,
        except_fds: *mut ctypes::fd_set,
    ) -> LinuxResult {
        unsafe { self.copy_to_fd_set(READ_SET, read_fds)? };
        unsafe { self.copy_to_fd_set(WRITE_SET, write_fds)? };
        unsafe { self.copy_to_fd_set(EXCEPT_SET, except_fds)? };
        Ok(())
    }

    unsafe fn copy_to_fd_set(&self, set_idx: usize, fds: *mut ctypes::fd_set) -> LinuxResult {
        if fds.is_null() {
            return Ok(());
        }
        let words = self.nfds_words();
        let mut dst = unsafe { read_user_value(fds as *const ctypes::fd_set)? };
        dst.fds_bits[..words].copy_from_slice(&self.bits[set_idx][..words]);
        unsafe { write_user_value(fds, dst)? };
        Ok(())
    }

    fn poll_all(&self, result_sets: &mut FdSets) -> LinuxResult<usize> {
        result_sets.clear();
        let mut res_num = 0;
        for word_idx in 0..self.nfds_words() {
            let read_bits = self.bits[READ_SET][word_idx];
            let write_bits = self.bits[WRITE_SET][word_idx];
            let except_bits = self.bits[EXCEPT_SET][word_idx];

            let all_bits = read_bits | write_bits | except_bits;
            if all_bits == 0 {
                continue;
            }
            let fd_base = word_idx * BITS_PER_WORD;
            let valid_bits = self.nfds.saturating_sub(fd_base).min(BITS_PER_WORD);
            let mut bits = if valid_bits < BITS_PER_WORD {
                all_bits & ((1 << valid_bits) - 1)
            } else {
                all_bits
            };
            while bits != 0 {
                let bit_idx = bits.trailing_zeros() as usize;
                let bit = 1 << bit_idx;
                let fd = fd_base + bit_idx;
                match get_file_like(fd as _)?.poll() {
                    Ok(state) => {
                        if state.readable && read_bits & bit != 0 {
                            result_sets.set_fd(READ_SET, fd);
                            res_num += 1;
                        }
                        if state.writable && write_bits & bit != 0 {
                            result_sets.set_fd(WRITE_SET, fd);
                            res_num += 1;
                        }
                    }
                    Err(e) => {
                        debug!("    except: {} {:?}", fd, e);
                        if except_bits & bit != 0 {
                            result_sets.set_fd(EXCEPT_SET, fd);
                            res_num += 1;
                        }
                    }
                }
                bits &= bits - 1;
            }
        }
        Ok(res_num)
    }
}

/// Monitor multiple file descriptors, waiting until one or more of the file descriptors become "ready" for some class of I/O operation
///
/// # Safety
///
/// Any non-null fd-set pointer must be valid for both reads and writes of the
/// fd-set words covered by `nfds`; a non-null `timeout` must be valid for reads
/// of one `timeval`.
pub unsafe fn sys_select(
    nfds: c_int,
    readfds: *mut ctypes::fd_set,
    writefds: *mut ctypes::fd_set,
    exceptfds: *mut ctypes::fd_set,
    timeout: *mut ctypes::timeval,
) -> c_int {
    debug!(
        "sys_select <= {} {:#x} {:#x} {:#x}",
        nfds, readfds as usize, writefds as usize, exceptfds as usize
    );
    syscall_body!(sys_select, {
        if nfds < 0 || nfds as usize > FD_SETSIZE {
            return Err(LinuxError::EINVAL);
        }
        let nfds = nfds as usize;
        let deadline = if timeout.is_null() {
            None
        } else {
            let tv = unsafe { read_user_value(timeout as *const ctypes::timeval)? };
            Some(wall_time() + timeval_to_duration(tv)?)
        };
        let fd_sets = FdSets::from(nfds, readfds, writefds, exceptfds)?;
        let mut result_sets = FdSets::empty(nfds);

        loop {
            #[cfg(feature = "net")]
            axnet::poll_interfaces();
            let res = match fd_sets.poll_all(&mut result_sets) {
                Ok(res) => res,
                Err(err) => {
                    unsafe { result_sets.write_back_to(readfds, writefds, exceptfds)? };
                    return Err(err);
                }
            };
            if res > 0 {
                unsafe { result_sets.write_back_to(readfds, writefds, exceptfds)? };
                return Ok(res);
            }

            if super::wait_for_poll_retry(deadline) {
                debug!("    timeout!");
                unsafe { result_sets.write_back_to(readfds, writefds, exceptfds)? };
                return Ok(0);
            }
        }
    })
}
