//! [ByteArray] implementation.

use crate::Barse;

/// Byte array wrapper with specialized barse read/write impl.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ByteArray<const N: usize>([u8; N]);

impl<const N: usize> ByteArray<N> {
    /// Construct a new [ByteArray] from passed bytes.
    #[inline(always)]
    pub const fn new(bytes: [u8; N]) -> Self {
        Self(bytes)
    }

    /// Unwrap [ByteArray] to wrapped bytes.
    #[inline(always)]
    pub const fn into_inner(self) -> [u8; N] {
        self.0
    }
}

impl<const N: usize> Barse for ByteArray<N> {
    type ReadWith = ();
    type WriteWith = ();

    #[inline(always)]
    fn read<E, B>(from: &mut B, _with: ()) -> Result<Self, crate::WrappedErr<B::Err>>
    where
        E: crate::Endian,
        B: crate::ByteSource,
    {
        Ok(ByteArray(from.read_array()?))
    }

    #[inline(always)]
    fn write<E, B>(&self, to: &mut B, _with: ()) -> Result<(), crate::WrappedErr<B::Err>>
    where
        E: crate::Endian,
        B: crate::ByteSink,
    {
        Ok(to.write_array(self.0)?)
    }
}
