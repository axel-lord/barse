//! [WriteSink] impl.

use ::core::ops::{Deref, DerefMut};
use ::std::io::Write;

use crate::ByteSink;

/// [ByteSink] implementor wrapping [Write] implementations.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WriteSink<W>(W);

impl<W> WriteSink<W> {
    /// Construct a new instance from a value implementing [Write].
    #[inline]
    pub const fn new(value: W) -> Self
    where
        W: Write,
    {
        Self(value)
    }

    /// Get wrapped value.
    #[inline]
    pub fn into_inner(self) -> W {
        self.0
    }
}

impl<W> ByteSink for WriteSink<W>
where
    W: Write,
{
    type Err = ::std::io::Error;

    #[inline]
    fn write_slice(&mut self, buf: &[u8]) -> Result<(), Self::Err> {
        W::write_all(self, buf)
    }
}

impl<W> Deref for WriteSink<W> {
    type Target = W;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<W> DerefMut for WriteSink<W> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
