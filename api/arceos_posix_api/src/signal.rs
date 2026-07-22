/// Host-provided POSIX signal hooks for syscall semantics that require a
/// process/signal context outside this generic POSIX API crate.
///
/// The weak default deliberately reports "not delivered": non-userspace
/// consumers still get the honest syscall error (for example `EPIPE`) instead
/// of fake success, while a userspace runtime can override this and deliver the
/// real signal for the current task.
#[crate_interface::def_interface]
pub trait PosixSignalIf {
    /// Delivers `SIGPIPE` to the current task; returns whether it was delivered.
    fn raise_sigpipe() -> bool {
        false
    }

    /// Returns whether a signal is pending that should interrupt blocking calls.
    fn has_interrupting_signal() -> bool {
        false
    }
}

#[allow(dead_code)]
pub(crate) fn raise_sigpipe() -> bool {
    crate_interface::call_interface!(PosixSignalIf::raise_sigpipe)
}

#[allow(dead_code)]
pub(crate) fn has_interrupting_signal() -> bool {
    crate_interface::call_interface!(PosixSignalIf::has_interrupting_signal)
}
