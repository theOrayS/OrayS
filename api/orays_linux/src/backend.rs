//! Implementation-independent user-memory backend contract.

use crate::user::{Read, UserRange, Write};

/// Byte-oriented operations implemented by the process-owning backend.
///
/// Implementations must reject a copy when the range length differs from the
/// supplied slice length. Mapping, fault-in and errno behavior remain backend
/// responsibilities.
pub trait UserMemoryBackend {
    /// Backend-specific copy/validation error carried back to the caller.
    type Error;

    /// Validates that `range` may be read from by subsequent copy operations.
    fn validate_read(&self, range: UserRange<Read>) -> Result<(), Self::Error>;

    /// Validates that `range` may be written to by subsequent copy operations.
    fn validate_write(&self, range: UserRange<Write>) -> Result<(), Self::Error>;

    /// Copies bytes from a readable user range into `dst`.
    fn read_bytes(&self, src: UserRange<Read>, dst: &mut [u8]) -> Result<(), Self::Error>;

    /// Copies bytes from `src` into a writable user range.
    fn write_bytes(&self, dst: UserRange<Write>, src: &[u8]) -> Result<(), Self::Error>;
}

#[cfg(test)]
mod tests {
    use core::cell::RefCell;

    use super::*;
    use crate::user::UserAddr;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum FakeError {
        OutOfBounds,
        LengthMismatch,
    }

    struct FakeMemory {
        bytes: RefCell<[u8; 8]>,
    }

    impl FakeMemory {
        fn bounds<A: crate::user::Access>(
            &self,
            range: UserRange<A>,
        ) -> Result<core::ops::Range<usize>, FakeError> {
            let start = range.start().get();
            let end = range.end().get();
            if end > self.bytes.borrow().len() {
                return Err(FakeError::OutOfBounds);
            }
            Ok(start..end)
        }
    }

    impl UserMemoryBackend for FakeMemory {
        type Error = FakeError;

        fn validate_read(&self, range: UserRange<Read>) -> Result<(), Self::Error> {
            self.bounds(range).map(drop)
        }

        fn validate_write(&self, range: UserRange<Write>) -> Result<(), Self::Error> {
            self.bounds(range).map(drop)
        }

        fn read_bytes(&self, src: UserRange<Read>, dst: &mut [u8]) -> Result<(), Self::Error> {
            if src.len() != dst.len() {
                return Err(FakeError::LengthMismatch);
            }
            let bounds = self.bounds(src)?;
            dst.copy_from_slice(&self.bytes.borrow()[bounds]);
            Ok(())
        }

        fn write_bytes(&self, dst: UserRange<Write>, src: &[u8]) -> Result<(), Self::Error> {
            if dst.len() != src.len() {
                return Err(FakeError::LengthMismatch);
            }
            let bounds = self.bounds(dst)?;
            self.bytes.borrow_mut()[bounds].copy_from_slice(src);
            Ok(())
        }
    }

    #[test]
    fn backend_contract_keeps_read_and_write_ranges_typed() {
        let memory = FakeMemory {
            bytes: RefCell::new(*b"abcdefgh"),
        };
        let read = UserRange::<Read>::new(UserAddr::new(2), 3).unwrap();
        let write = UserRange::<Write>::new(UserAddr::new(4), 2).unwrap();
        let mut dst = [0; 3];

        memory.validate_read(read).unwrap();
        memory.validate_write(write).unwrap();
        memory.read_bytes(read, &mut dst).unwrap();
        assert_eq!(&dst, b"cde");
        memory.write_bytes(write, b"XY").unwrap();
        assert_eq!(&*memory.bytes.borrow(), b"abcdXYgh");
    }

    #[test]
    fn backend_rejects_bounds_and_length_mismatches() {
        let memory = FakeMemory {
            bytes: RefCell::new([0; 8]),
        };
        let read = UserRange::<Read>::new(UserAddr::new(7), 1).unwrap();
        let outside = UserRange::<Write>::new(UserAddr::new(8), 1).unwrap();

        assert_eq!(
            memory.read_bytes(read, &mut [0; 2]),
            Err(FakeError::LengthMismatch)
        );
        assert_eq!(memory.validate_write(outside), Err(FakeError::OutOfBounds));
    }
}
