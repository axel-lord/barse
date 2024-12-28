//! Barse as extensions.

use crate::{ByteSink, ByteSource, Endian, ReadAs, WriteAs};

/// Extension to [ReadAs] to read values without a with value.
pub trait ReadAsExt<T> {
    /// Use an instance to read a value of type T from source.
    ///
    /// # Errors
    /// If implementation needs to.
    fn read<E, B>(self, from: &mut B) -> Result<T, crate::WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSource;
}

/// Extension to [WriteAs] to write values without a with value.
pub trait WriteAsExt<T> {
    /// Use an instance to write a value of type T from source.
    ///
    /// # Errors
    /// If the implementation needs to.
    fn write_with<E, B>(self, value: &T, to: &mut B) -> Result<(), crate::WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSink;
}

impl<T, R> ReadAsExt<T> for R
where
    R: ReadAs<T, ()>,
{
    #[inline]
    fn read<E, B>(self, from: &mut B) -> Result<T, crate::WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSource,
    {
        self.read_with::<E, B>(from, ())
    }
}

impl<T, W> WriteAsExt<T> for W
where
    W: WriteAs<T, ()>,
{
    fn write_with<E, B>(self, value: &T, to: &mut B) -> Result<(), crate::WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSink,
    {
        self.write_with::<E, B>(value, to, ())
    }
}
