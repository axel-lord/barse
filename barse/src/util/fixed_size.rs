//! [FixedSize] impl.

use crate::{Barse, SliceSink, SliceSrc};

/// Wrap a [Barse] implementor always writing and reading SIZE bytes, when writing PAD will be
/// used to fill empty space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedSize<T, const SIZE: usize, const PAD: u8 = 0u8>(T);

impl<T, const SIZE: usize, const PAD: u8> Barse for FixedSize<T, SIZE, PAD>
where
    T: Barse,
{
    type ReadWith = T::ReadWith;

    type WriteWith = T::WriteWith;

    fn read_with<E, B>(
        from: &mut B,
        with: Self::ReadWith,
    ) -> Result<Self, crate::WrappedErr<B::Err>>
    where
        E: crate::Endian,
        B: crate::ByteSource,
    {
        let bytes = from.read_array::<SIZE>()?;
        let mut from = SliceSrc::new(&bytes);

        T::read_with::<E, _>(&mut from, with)
            .map_err(|err| err.merge_into::<crate::Error>().into_wrapped::<B::Err>())
            .map(Self)
    }

    fn write_with<E, B>(
        &self,
        to: &mut B,
        with: Self::WriteWith,
    ) -> Result<(), crate::WrappedErr<B::Err>>
    where
        E: crate::Endian,
        B: crate::ByteSink,
    {
        let mut bytes = [PAD; SIZE];
        {
            let mut to = SliceSink::new(&mut bytes);
            T::write_with::<E, _>(&self.0, &mut to, with)
                .map_err(|err| err.merge_into::<crate::Error>().into_wrapped::<B::Err>())?;
        }
        to.write_array(bytes).map_err(From::from)
    }
}

impl<T, const SIZE: usize, const PAD: u8> FixedSize<T, SIZE, PAD> {
    /// Construct a new [FixedSize] from value.
    #[inline]
    pub const fn new(value: T) -> Self {
        Self(value)
    }

    /// Unwrap [FixedSize] to wrapped value.
    #[inline]
    pub fn into_inner(self) -> T {
        let Self(value) = self;
        value
    }
}
