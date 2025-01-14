//! Barse write extension.

use crate::{Barse, ByteSink, Empty, Endian, WrappedErr};

/// Extension to [Barse] for wrtiting where no with value is needed.
pub trait BarseWriteExt: Barse {
    /// Write an instance to a sink with given endianess.
    ///
    /// # Errors
    /// If Sink or implementation errors.
    fn write<E, B>(&self, to: &mut B) -> Result<(), WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSink;
}

impl<T> BarseWriteExt for T
where
    T: Barse,
    T::WriteWith: Empty,
{
    #[inline]
    fn write<E, B>(&self, to: &mut B) -> Result<(), WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSink,
    {
        T::write_with::<E, B>(self, to, T::WriteWith::instance())
    }
}
