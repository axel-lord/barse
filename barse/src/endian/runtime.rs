//! [Runtime] impl.

use crate::{
    endian::{Big, Little, Native},
    Barse, ReadAs, WriteAs,
};

/// Endian selected at runtime, does not implement [Endian][crate::Endian] trait.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Runtime {
    /// Use big endian.
    Big,

    /// Use little endian.
    Little,

    /// Use platform native endian.
    #[default]
    Native,
}

impl From<Big> for Runtime {
    fn from(_value: Big) -> Self {
        Self::Big
    }
}

impl From<Little> for Runtime {
    fn from(_value: Little) -> Self {
        Self::Little
    }
}

impl From<Native> for Runtime {
    fn from(_value: Native) -> Self {
        Self::Native
    }
}

impl<T, W> ReadAs<T, W> for Runtime
where
    T: Barse<ReadWith = W>,
{
    #[inline]
    fn read_with<E, B>(self, from: &mut B, with: W) -> Result<T, crate::WrappedErr<B::Err>>
    where
        E: super::Endian,
        B: crate::ByteSource,
    {
        match self {
            Runtime::Big => T::read_with::<Big, B>(from, with),
            Runtime::Little => T::read_with::<Little, B>(from, with),
            Runtime::Native => T::read_with::<Native, B>(from, with),
        }
    }
}

impl<T, W> WriteAs<T, W> for Runtime
where
    T: Barse<WriteWith = W>,
{
    #[inline]
    fn write_with<E, B>(
        self,
        value: &T,
        to: &mut B,
        with: W,
    ) -> Result<(), crate::WrappedErr<B::Err>>
    where
        E: super::Endian,
        B: crate::ByteSink,
    {
        match self {
            Runtime::Big => T::write_with::<Big, B>(value, to, with),
            Runtime::Little => T::write_with::<Little, B>(value, to, with),
            Runtime::Native => T::write_with::<Native, B>(value, to, with),
        }
    }
}
