//! [Bytes] impl.

use crate::{ReadAs, WriteAs};

/// [ReadAs]/[WriteAs] implementor reading bytes.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bytes<const SIZE: usize = 0>;

impl<T, const SIZE: usize> ReadAs<T, ()> for Bytes<SIZE>
where
    T: From<[u8; SIZE]>,
{
    #[inline]
    fn read_with<E, B>(self, from: &mut B, _with: ()) -> Result<T, crate::WrappedErr<B::Err>>
    where
        E: crate::Endian,
        B: crate::ByteSource,
    {
        Ok(T::from(from.read_array()?))
    }
}

impl<T, const SIZE: usize> WriteAs<T, ()> for Bytes<SIZE>
where
    T: AsRef<[u8; SIZE]>,
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
        Ok(to.write_slice(value.as_ref())?)
    }
}
