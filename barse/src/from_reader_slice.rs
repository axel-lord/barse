use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use bytesize::ByteSize;

use crate::{error::Error, ByteRead, FromByteReader};

/// Trait to query the size in bytes of something.
pub trait ByteSizeQuery {
    /// Type of input to use for query.
    type Flag;
    /// Query the input returning a size.
    fn size(flag: &Self::Flag) -> usize;
}

/// An array of bytes with a queried length.
#[derive(PartialEq, Eq)]
pub struct FromReaderSlice<'input, Q>(Cow<'input, [u8]>, PhantomData<Q>);

impl<'input, Q> FromByteReader<'input> for FromReaderSlice<'input, Q>
where
    Q: ByteSizeQuery + 'static,
{
    fn from_byte_reader<R>(mut reader: R) -> Result<Self, Error>
    where
        R: ByteRead<'input>,
    {
        let flag = reader.flags::<Q::Flag>()?;

        Ok(Self(
            Cow::Borrowed(reader.read_ref(Q::size(flag))?),
            PhantomData::default(),
        ))
    }
}

impl<Q> AsRef<[u8]> for FromReaderSlice<'_, Q> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<Q> From<FromReaderSlice<'_, Q>> for Vec<u8> {
    fn from(value: FromReaderSlice<'_, Q>) -> Self {
        value.0.into()
    }
}

impl<Q> Debug for FromReaderSlice<'_, Q> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FromReaderSlice({})", ByteSize::b(self.0.len() as u64))
    }
}
