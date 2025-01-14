//! Barse read extension.

use crate::{Barse, ByteSource, Empty, Endian, WrappedErr};

/// Extension to [Barse] for reading where no with value is needed.
pub trait BarseReadExt: Barse {
    /// Read an instnce from source with given endianess.
    ///
    /// # Errors
    /// If Soure or implementation errors.
    fn read<E, B>(from: &mut B) -> Result<Self, WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSource;
}

impl<T> BarseReadExt for T
where
    T: Barse,
    T::ReadWith: Empty,
{
    #[inline]
    fn read<E, B>(from: &mut B) -> Result<Self, WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSource,
    {
        T::read_with::<E, B>(from, T::ReadWith::instance())
    }
}
