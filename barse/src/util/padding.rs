//! [Padding] implementation.

use crate::{Barse, Endian};

/// Padding, read bytes are discarded and N bytes of BYTE are written according to given size.
///
/// If padding contents should be preserved use [ByteArray][crate::util::ByteArray].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Padding<const N: usize, const BYTE: u8 = 0u8>;

impl<const N: usize, const BYTE: u8> Barse for Padding<N, BYTE> {
    type ReadWith = ();
    type WriteWith = ();

    #[inline]
    fn read_with<E, B>(from: &mut B, _with: ()) -> Result<Self, crate::WrappedErr<B::Err>>
    where
        E: Endian,
        B: crate::ByteSource,
    {
        from.read_array::<N>()?;
        Ok(Self)
    }

    #[inline]
    fn write_with<E, B>(&self, to: &mut B, _with: ()) -> Result<(), crate::WrappedErr<B::Err>>
    where
        E: Endian,
        B: crate::ByteSink,
    {
        to.write_array([BYTE; N])?;
        Ok(())
    }
}
