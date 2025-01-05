//! Utilities for reading/writing external types.
//!
//! Some types are reexported from other modules as to collect all implementors here as well.

use crate::{Barse, ByteSink, ByteSource, Endian};

#[cfg_attr(docsrs, doc(cfg(feature = "util")))]
#[cfg(feature = "util")]
mod bytes;

#[cfg_attr(docsrs, doc(cfg(feature = "zerocopy")))]
#[cfg(feature = "zerocopy")]
mod zerocopy;

#[cfg_attr(docsrs, doc(cfg(feature = "bytemuck")))]
#[cfg(feature = "bytemuck")]
mod bytemuck;

#[cfg(feature = "util")]
pub use bytes::Bytes;

#[doc(inline)]
pub use crate::endian::{Big as BigEndian, Little as LittleEndian, Native as NativeEndian};

#[cfg(feature = "util")]
#[doc(inline)]
pub use crate::endian::Runtime as RuntimeEndian;

#[cfg(feature = "zerocopy")]
pub use zerocopy::Zerocopy;

#[cfg(feature = "bytemuck")]
pub use bytemuck::Bytemuck;

/// Read another type.
pub trait ReadAs<T, W = ()> {
    /// Use an instance to read a value of type T from source.
    ///
    /// # Errors
    /// If implementation needs to.
    fn read_with<E, B>(self, from: &mut B, with: W) -> Result<T, crate::WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSource;
}

/// Write another type.
pub trait WriteAs<T, W = ()> {
    /// Use an instance to write a value of type T from source.
    ///
    /// # Errors
    /// If the implementation needs to.
    fn write_with<E, B>(
        self,
        value: &T,
        to: &mut B,
        with: W,
    ) -> Result<(), crate::WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSink;
}

/// [ReadAs]/[WriteAs] using [Barse] implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Default;

impl<T, W> ReadAs<T, W> for Default
where
    T: Barse<ReadWith = W>,
{
    #[inline]
    fn read_with<E, B>(self, from: &mut B, with: W) -> Result<T, crate::WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSource,
    {
        T::read_with::<E, B>(from, with)
    }
}

impl<T, W> WriteAs<T, W> for Default
where
    T: Barse<WriteWith = W>,
{
    fn write_with<E, B>(
        self,
        value: &T,
        to: &mut B,
        with: W,
    ) -> Result<(), crate::WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSink,
    {
        T::write_with::<E, B>(value, to, with)
    }
}
