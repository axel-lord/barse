//! Barse as extensions.

use crate::{ByteSink, ByteSource, Empty, Endian, ReadAs, WriteAs};

/// Extension to [ReadAs] to read values without a with value.
pub trait ReadAsExt<T, S> {
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
pub trait WriteAsExt<T, S> {
    /// Use an instance to write a value of type T from source.
    ///
    /// # Errors
    /// If the implementation needs to.
    fn write_with<E, B>(self, value: &T, to: &mut B) -> Result<(), crate::WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSink;
}

impl<T, R, S> ReadAsExt<T, S> for R
where
    R: ReadAs<T, S>,
    S: Empty,
{
    #[inline]
    fn read<E, B>(self, from: &mut B) -> Result<T, crate::WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSource,
    {
        self.read_with::<E, B>(from, S::instance())
    }
}

impl<T, W, S> WriteAsExt<T, S> for W
where
    W: WriteAs<T, S>,
    S: Empty,
{
    fn write_with<E, B>(self, value: &T, to: &mut B) -> Result<(), crate::WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSink,
    {
        self.write_with::<E, B>(value, to, S::instance())
    }
}
