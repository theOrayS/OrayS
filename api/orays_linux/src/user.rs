//! Typed user addresses and overflow-checked ranges.

use core::fmt;
use core::marker::PhantomData;
use core::mem::size_of;

mod sealed {
    pub trait Sealed {}
}

/// A sealed marker for the permitted direction of a userspace access.
pub trait Access: sealed::Sealed + Copy + fmt::Debug + Eq + 'static {}

/// Userspace memory read by the kernel.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Read {}

/// Userspace memory written by the kernel.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Write {}

impl sealed::Sealed for Read {}
impl sealed::Sealed for Write {}
impl Access for Read {}
impl Access for Write {}

/// An integer userspace address. Constructing it does not validate a mapping.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct UserAddr(usize);

impl UserAddr {
    pub const fn new(value: usize) -> Self {
        Self(value)
    }

    pub const fn get(self) -> usize {
        self.0
    }

    pub const fn checked_add(self, offset: usize) -> Option<Self> {
        match self.0.checked_add(offset) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

/// A half-open byte range carrying its access direction.
pub struct UserRange<A: Access> {
    start: UserAddr,
    len: usize,
    access: PhantomData<fn() -> A>,
}

impl<A: Access> UserRange<A> {
    /// Constructs a range after checking the exclusive end for overflow.
    ///
    /// A zero-length range is valid at every integer address. Null and mapping
    /// checks intentionally belong to the backend.
    pub const fn new(start: UserAddr, len: usize) -> Option<Self> {
        match start.checked_add(len) {
            Some(_) => Some(Self {
                start,
                len,
                access: PhantomData,
            }),
            None => None,
        }
    }

    pub const fn start(self) -> UserAddr {
        self.start
    }

    pub const fn len(self) -> usize {
        self.len
    }

    pub const fn is_empty(self) -> bool {
        self.len == 0
    }

    pub const fn end(self) -> UserAddr {
        // The constructor proves this addition cannot overflow.
        match self.start.checked_add(self.len) {
            Some(end) => end,
            None => unreachable!(),
        }
    }
}

impl<A: Access> Clone for UserRange<A> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<A: Access> Copy for UserRange<A> {}

impl<A: Access> fmt::Debug for UserRange<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserRange")
            .field("start", &self.start)
            .field("len", &self.len)
            .finish()
    }
}

impl<A: Access> PartialEq for UserRange<A> {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.len == other.len
    }
}

impl<A: Access> Eq for UserRange<A> {}

/// A typed userspace address. It never dereferences the address.
pub struct UserPtr<T, A: Access> {
    addr: UserAddr,
    marker: PhantomData<fn() -> (T, A)>,
}

impl<T, A: Access> UserPtr<T, A> {
    pub const fn new(addr: UserAddr) -> Self {
        Self {
            addr,
            marker: PhantomData,
        }
    }

    pub const fn addr(self) -> UserAddr {
        self.addr
    }

    /// Constructs a typed slice and checks its byte length and exclusive end.
    pub const fn slice(self, len: usize) -> Option<UserSlice<T, A>> {
        let Some(byte_len) = len.checked_mul(size_of::<T>()) else {
            return None;
        };
        let Some(_) = UserRange::<A>::new(self.addr, byte_len) else {
            return None;
        };
        Some(UserSlice {
            ptr: self,
            len,
            byte_len,
        })
    }
}

impl<T, A: Access> Clone for UserPtr<T, A> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, A: Access> Copy for UserPtr<T, A> {}

impl<T, A: Access> fmt::Debug for UserPtr<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("UserPtr").field(&self.addr).finish()
    }
}

impl<T, A: Access> PartialEq for UserPtr<T, A> {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}

impl<T, A: Access> Eq for UserPtr<T, A> {}

/// A typed userspace slice with a checked byte range.
pub struct UserSlice<T, A: Access> {
    ptr: UserPtr<T, A>,
    len: usize,
    byte_len: usize,
}

impl<T, A: Access> UserSlice<T, A> {
    pub const fn ptr(self) -> UserPtr<T, A> {
        self.ptr
    }

    pub const fn len(self) -> usize {
        self.len
    }

    pub const fn is_empty(self) -> bool {
        self.len == 0
    }

    pub const fn byte_len(self) -> usize {
        self.byte_len
    }

    pub const fn byte_range(self) -> UserRange<A> {
        // `UserPtr::slice` validated this exact pair.
        match UserRange::new(self.ptr.addr, self.byte_len) {
            Some(range) => range,
            None => unreachable!(),
        }
    }
}

impl<T, A: Access> Clone for UserSlice<T, A> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, A: Access> Copy for UserSlice<T, A> {}

impl<T, A: Access> fmt::Debug for UserSlice<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserSlice")
            .field("ptr", &self.ptr)
            .field("len", &self.len)
            .field("byte_len", &self.byte_len)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_checks_the_exclusive_end_only() {
        let null = UserAddr::new(0);
        assert_eq!(UserRange::<Read>::new(null, 0).unwrap().end(), null);
        assert!(UserRange::<Read>::new(null, 1).is_some());
        assert!(UserRange::<Write>::new(UserAddr::new(usize::MAX), 0).is_some());
        assert!(UserRange::<Write>::new(UserAddr::new(usize::MAX), 1).is_none());
    }

    #[test]
    fn typed_slice_checks_multiplication_and_address_overflow() {
        let ptr = UserPtr::<u32, Read>::new(UserAddr::new(8));
        let slice = ptr.slice(3).unwrap();
        assert_eq!(slice.len(), 3);
        assert_eq!(slice.byte_len(), 12);
        assert_eq!(slice.byte_range().end(), UserAddr::new(20));

        let overflow = UserPtr::<u32, Read>::new(UserAddr::new(usize::MAX - 3));
        assert!(overflow.slice(2).is_none());
        assert!(ptr.slice(usize::MAX).is_none());
    }

    #[test]
    fn zero_sized_slices_have_an_empty_byte_range() {
        let ptr = UserPtr::<(), Write>::new(UserAddr::new(usize::MAX));
        let slice = ptr.slice(usize::MAX).unwrap();
        assert_eq!(slice.byte_len(), 0);
        assert_eq!(slice.byte_range().start(), UserAddr::new(usize::MAX));
        assert!(slice.byte_range().is_empty());
    }
}
