//! Utilities for reading/writing external types.

use crate::{ByteSink, ByteSource, Endian};

/// Read another type.
pub trait ReadAs<T, W = ()> {
    /// Use an instance to read a value of type T from source.
    ///
    /// # Errors
    /// If implementation needs to.
    fn read<E: Endian, B: ByteSource>(
        self,
        from: &mut B,
        with: W,
    ) -> Result<T, crate::Error<B::Err>>;
}

/// Write another type.
pub trait WriteAs<T, W = ()> {
    /// Use an instance to write a value of type T from source.
    ///
    /// # Errors
    /// If the implementation needs to.
    fn write<E: Endian, B: ByteSink>(self, to: &mut B, with: W)
        -> Result<(), crate::Error<B::Err>>;
}
