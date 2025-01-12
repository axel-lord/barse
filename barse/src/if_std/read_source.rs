//! [ReadSource] impl.

use ::core::ops::{Deref, DerefMut};
use ::std::io::Read;

use crate::ByteSource;

/// [ByteSource] implementor wrapping [Read] implementations.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReadSource<R>(R);

impl<R> ReadSource<R> {
    /// Construct a new instance from a value implementing [Read].
    #[inline]
    pub const fn new(value: R) -> Self
    where
        R: Read,
    {
        Self(value)
    }

    /// Get wrapped value.
    #[inline]
    pub fn into_inner(self) -> R {
        self.0
    }
}

impl<R> ByteSource for ReadSource<R>
where
    R: Read,
{
    type Err = ::std::io::Error;

    #[inline]
    fn read_slice(&mut self, buf: &mut [u8]) -> Result<(), Self::Err> {
        R::read_exact(self, buf)
    }
}

impl<R> Deref for ReadSource<R> {
    type Target = R;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R> DerefMut for ReadSource<R> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
