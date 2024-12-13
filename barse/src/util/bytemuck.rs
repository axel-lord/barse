//! [Barse] implementations using bytemuck.

use ::bytemuck::{AnyBitPattern, NoUninit};

use crate::Barse;

/// Value implementing [AnyBitPattern] and [NoUninit].
///
/// Endianess will be platform specific ([Native][crate::endian::Native]) in all cases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct UseAnyBitPattern<T>(T);

impl<T> UseAnyBitPattern<T>
where
    T: AnyBitPattern + NoUninit + Barse,
{
    /// Construct a new [UseAnyBitPattern] from a value.
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self(value)
    }

    /// Unwrap [UseAnyBitPattern] to wrapped value.
    #[inline(always)]
    pub const fn into_inner(self) -> T {
        let Self(value) = self;
        value
    }
}

impl<T> Barse for UseAnyBitPattern<T>
where
    T: AnyBitPattern + NoUninit,
{
    type ReadWith = ();
    type WriteWith = ();

    #[inline(always)]
    fn read<E, B>(from: &mut B, _with: ()) -> Result<Self, crate::Error<B::Err>>
    where
        E: crate::Endian,
        B: crate::ByteSource,
    {
        let mut value = T::zeroed();

        from.read_slice(::bytemuck::bytes_of_mut(&mut value))?;

        Ok(Self(value))
    }

    #[inline(always)]
    fn write<E, B>(&self, to: &mut B, _with: ()) -> Result<(), crate::Error<B::Err>>
    where
        E: crate::Endian,
        B: crate::ByteSink,
    {
        to.write_slice(::bytemuck::bytes_of(&self.0))?;
        Ok(())
    }
}

