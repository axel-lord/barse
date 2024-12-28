//! read as and write as implementations using zerocopy.

use ::zerocopy::{FromBytes, Immutable, IntoBytes};

use crate::{ReadAs, WriteAs};

/// Type to read/write zerocopy types using [ReadAs] and [WriteAs].
///
/// Endianess will always be native (same as using [endian::Native][crate::endian::Native]).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Zerocopy;

impl<T> ReadAs<T> for Zerocopy
where
    T: IntoBytes + FromBytes,
{
    #[inline]
    fn read_with<E, B>(self, from: &mut B, _with: ()) -> Result<T, crate::WrappedErr<B::Err>>
    where
        E: crate::Endian,
        B: crate::ByteSource,
    {
        let mut value = T::new_zeroed();
        from.read_slice(value.as_mut_bytes())?;
        Ok(value)
    }
}

impl<T> WriteAs<T> for Zerocopy
where
    T: IntoBytes + Immutable,
{
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
        to.write_slice(value.as_bytes())?;
        Ok(())
    }
}
