//! [Barse] implementations using zerocopy.

use ::zerocopy::{FromBytes, Immutable, IntoBytes};

use crate::Barse;

/// Value implementing [FromBytes] and [IntoBytes].
///
/// Endianess will be platform specific ([Native][crate::endian::Native]) in all cases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UseIntoFromBytes<T>(T);

impl<T> UseIntoFromBytes<T>
where
    T: FromBytes + IntoBytes + Immutable + Barse,
{
    /// Construct a new [UseIntoFromBytes] from a value.
    pub const fn new(value: T) -> Self {
        Self(value)
    }

    /// Unwrap [UseIntoFromBytes] to wrapped value.
    pub fn into_inner(self) -> T {
        let Self(value) = self;
        value
    }
}

impl<T> Barse for UseIntoFromBytes<T>
where
    T: FromBytes + IntoBytes + Immutable + Barse,
{
    type ReadWith = ();
    type WriteWith = ();

    fn read<E, B>(from: &mut B, _with: ()) -> Result<Self, crate::Error<B::Err>>
    where
        E: crate::Endian,
        B: crate::ByteSource,
    {
        let mut value = T::new_zeroed();

        from.read_slice(value.as_mut_bytes())?;

        Ok(Self(value))
    }

    fn write<E, B>(&self, to: &mut B, _with: ()) -> Result<(), crate::Error<B::Err>>
    where
        E: crate::Endian,
        B: crate::ByteSink,
    {
        to.write_slice(self.0.as_bytes())?;

        Ok(())
    }
}

