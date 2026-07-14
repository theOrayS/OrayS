//! Implementation-independent syscall identity and audit metadata.

/// Linux exposes at most six register arguments at the syscall boundary.
pub const MAX_SYSCALL_ARGUMENTS: usize = 6;

/// A target-selected Linux syscall number.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SyscallNumber(u32);

impl SyscallNumber {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

/// The six raw register arguments supplied by the architecture entry path.
///
/// PR1 keeps this as an implementation-independent audit value only. The
/// existing shell dispatcher remains the sole runtime consumer of trap-frame
/// arguments and is deliberately not wired through this type yet.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct SyscallArgs([usize; MAX_SYSCALL_ARGUMENTS]);

impl SyscallArgs {
    pub const fn new(values: [usize; MAX_SYSCALL_ARGUMENTS]) -> Self {
        Self(values)
    }

    pub const fn get(&self, index: usize) -> Option<usize> {
        if index < MAX_SYSCALL_ARGUMENTS {
            Some(self.0[index])
        } else {
            None
        }
    }

    pub const fn as_array(&self) -> &[usize; MAX_SYSCALL_ARGUMENTS] {
        &self.0
    }
}

/// Architecture labels used by metadata; they do not select a backend.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SyscallArchitecture {
    Riscv64,
    Aarch64,
    LoongArch64,
    Other,
}

/// The target set on which a dispatcher registration exists.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SyscallAvailability {
    All,
    Only(SyscallArchitecture),
    Except(&'static [SyscallArchitecture]),
}

impl SyscallAvailability {
    pub fn supports(self, architecture: SyscallArchitecture) -> bool {
        match self {
            Self::All => true,
            Self::Only(required) => required == architecture,
            Self::Except(excluded) => !excluded.contains(&architecture),
        }
    }
}

/// Descriptive metadata for an existing dispatcher registration.
///
/// This type does not invoke a handler and is not a dispatcher-generation API.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SyscallMeta {
    number: SyscallNumber,
    name: &'static str,
    argument_count: u8,
    availability: SyscallAvailability,
    handler: &'static str,
    alias_of: Option<&'static str>,
    audit_id: &'static str,
}

impl SyscallMeta {
    pub const fn new(
        number: SyscallNumber,
        name: &'static str,
        argument_count: u8,
        availability: SyscallAvailability,
        handler: &'static str,
        alias_of: Option<&'static str>,
        audit_id: &'static str,
    ) -> Self {
        assert!(
            argument_count as usize <= MAX_SYSCALL_ARGUMENTS,
            "Linux syscall metadata cannot declare more than six arguments"
        );
        Self {
            number,
            name,
            argument_count,
            availability,
            handler,
            alias_of,
            audit_id,
        }
    }

    pub const fn number(&self) -> SyscallNumber {
        self.number
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn argument_count(&self) -> u8 {
        self.argument_count
    }

    pub const fn availability(&self) -> SyscallAvailability {
        self.availability
    }

    pub const fn handler(&self) -> &'static str {
        self.handler
    }

    pub const fn alias_of(&self) -> Option<&'static str> {
        self.alias_of
    }

    pub const fn audit_id(&self) -> &'static str {
        self.audit_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_and_arguments_are_lossless_values() {
        let number = SyscallNumber::new(220);
        let args = SyscallArgs::new([1, 2, 3, 4, 5, 6]);

        assert_eq!(number.get(), 220);
        assert_eq!(args.get(0), Some(1));
        assert_eq!(args.get(5), Some(6));
        assert_eq!(args.get(6), None);
        assert_eq!(args.as_array(), &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn availability_describes_only_target_selection() {
        const PRIMARY: &[SyscallArchitecture] = &[
            SyscallArchitecture::Riscv64,
            SyscallArchitecture::Aarch64,
            SyscallArchitecture::LoongArch64,
        ];

        assert!(SyscallAvailability::All.supports(SyscallArchitecture::Riscv64));
        assert!(
            SyscallAvailability::Only(SyscallArchitecture::LoongArch64)
                .supports(SyscallArchitecture::LoongArch64)
        );
        assert!(
            !SyscallAvailability::Only(SyscallArchitecture::LoongArch64)
                .supports(SyscallArchitecture::Riscv64)
        );
        assert!(SyscallAvailability::Except(PRIMARY).supports(SyscallArchitecture::Other));
        assert!(!SyscallAvailability::Except(PRIMARY).supports(SyscallArchitecture::Aarch64));
    }

    #[test]
    fn metadata_preserves_handler_alias_and_audit_identity() {
        let metadata = SyscallMeta::new(
            SyscallNumber::new(83),
            "fdatasync",
            1,
            SyscallAvailability::All,
            "sys_fsync",
            Some("fsync"),
            "shared-fsync-handler",
        );

        assert_eq!(metadata.number().get(), 83);
        assert_eq!(metadata.name(), "fdatasync");
        assert_eq!(metadata.argument_count(), 1);
        assert_eq!(metadata.availability(), SyscallAvailability::All);
        assert_eq!(metadata.handler(), "sys_fsync");
        assert_eq!(metadata.alias_of(), Some("fsync"));
        assert_eq!(metadata.audit_id(), "shared-fsync-handler");
    }

    #[test]
    #[should_panic(expected = "cannot declare more than six arguments")]
    fn metadata_rejects_more_than_six_arguments() {
        let _ = SyscallMeta::new(
            SyscallNumber::new(0),
            "invalid",
            7,
            SyscallAvailability::All,
            "none",
            None,
            "invalid-test",
        );
    }
}
