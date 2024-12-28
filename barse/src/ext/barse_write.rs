//! Barse write extension.

use crate::{Barse, ByteSink, Endian, WrappedErr};

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
    T: Barse<WriteWith = ()>,
{
    #[inline]
    fn write<E, B>(&self, to: &mut B) -> Result<(), WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSink,
    {
        T::write_with::<E, B>(self, to, ())
    }
}
