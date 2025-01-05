//! [Native] impl.

use crate::Endian;

/// Native [Endian] implementor.
#[cfg_attr(
    feature = "barse_as",
    doc = "Implements [ReadAs][crate::ReadAs] and [WriteAs][crate::WriteAs]."
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Native;

impl crate::sealed::Sealed for Native {}
impl Endian for Native {
    fn write<T: crate::sealed::ToFromEndian>(t: T) -> T::Bytes {
        t.to_native()
    }

    fn read<T: crate::sealed::ToFromEndian>(b: T::Bytes) -> T {
        T::from_native(b)
    }
}

#[cfg(feature = "barse_as")]
impl<T, W> crate::ReadAs<T, W> for Native
where
    T: crate::Barse<ReadWith = W>,
{
    #[inline]
    fn read_with<E, B>(self, from: &mut B, with: W) -> Result<T, crate::WrappedErr<B::Err>>
    where
        E: Endian,
        B: crate::ByteSource,
    {
        T::read_with::<Self, B>(from, with)
    }
}

#[cfg(feature = "barse_as")]
impl<T, W> crate::WriteAs<T, W> for Native
where
    T: crate::Barse<WriteWith = W>,
{
    #[inline]
    fn write_with<E, B>(
        self,
        value: &T,
        to: &mut B,
        with: W,
    ) -> Result<(), crate::WrappedErr<B::Err>>
    where
        E: Endian,
        B: crate::ByteSink,
    {
        T::write_with::<Self, B>(value, to, with)
    }
}
