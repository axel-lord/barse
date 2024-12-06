//! Helper types implementing [Barse] for common usages.

use crate::Barse;

/// Byte array wrapper with specialized barse read/write impl.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteArray<const N: usize>(pub [u8; N]);

impl<const N: usize> From<ByteArray<N>> for [u8; N] {
    fn from(value: ByteArray<N>) -> Self {
        value.0
    }
}

impl<const N: usize> From<[u8; N]> for ByteArray<N> {
    fn from(value: [u8; N]) -> Self {
        Self(value)
    }
}

impl<const N: usize> Barse for ByteArray<N> {
    fn read<E, B>(from: &mut B) -> Result<Self, crate::Error<B::Err>>
    where
        E: crate::Endian,
        B: crate::ByteSource,
    {
        Ok(ByteArray(from.read_array()?))
    }

    fn write<E, B>(&self, to: &mut B) -> Result<(), crate::Error<B::Err>>
    where
        E: crate::Endian,
        B: crate::ByteSink,
    {
        Ok(to.write_array(self.0)?)
    }
}


