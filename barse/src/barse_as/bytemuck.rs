//! [Barse] implementations using bytemuck.

use ::bytemuck::{AnyBitPattern, NoUninit};

use crate::{ReadAs, WriteAs};

/// Type to read/write bytemuck types using [ReadAs] and [WriteAs].
///
/// Endianess will always be native (same as using [endian::Native][crate::endian::Native]).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bytemuck;

impl<T> ReadAs<T> for Bytemuck
where
    T: AnyBitPattern + NoUninit,
{
    #[inline]
    fn read_with<E, B>(self, from: &mut B, _with: ()) -> Result<T, crate::WrappedErr<B::Err>>
    where
        E: crate::Endian,
        B: crate::ByteSource,
    {
        let mut value = T::zeroed();
        from.read_slice(::bytemuck::bytes_of_mut(&mut value))?;
        Ok(value)
    }
}

impl<T> WriteAs<T> for Bytemuck
where
    T: NoUninit,
{
    #[inline]
    fn write_with<E, B>(
        self,
        value: &T,
        to: &mut B,
        _with: (),
    ) -> Result<(), crate::WrappedErr<B::Err>>
    where
        E: crate::Endian,
        B: crate::ByteSink,
    {
        to.write_slice(::bytemuck::bytes_of(value))?;
        Ok(())
    }
}
