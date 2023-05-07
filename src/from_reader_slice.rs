use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use bytesize::ByteSize;

use crate::{error::Error, ByteRead, FromByteReader};

pub trait SizeQuery {
    type Flag;
    fn len(flag: &Self::Flag) -> usize;
}

#[derive(Clone, PartialEq, Eq)]
pub struct FromReaderSlice<'input, T, Q>(Cow<'input, [u8]>, PhantomData<(T, Q)>);

impl<'input, T, Q> FromByteReader<'input> for FromReaderSlice<'input, T, Q>
where
    T: FromByteReader<'input>,
    Q: SizeQuery + 'static,
{
    fn from_byte_reader<R>(mut reader: R) -> Result<Self, Error>
    where
        R: ByteRead<'input>,
    {
        let flag = reader.flags::<Q::Flag>()?;

        Ok(Self(
            Cow::Borrowed(reader.read_ref(Q::len(flag))?),
            PhantomData::default(),
        ))
    }
}

impl<'input, T, Q> FromReaderSlice<'input, T, Q> {
    pub fn bytes(&'input self) -> &'input [u8] {
        self.0.as_ref()
    }

    pub fn to_owned(self) -> FromReaderSlice<'static, T, Q> {
        FromReaderSlice::<'static, T, Q>(Cow::Owned(self.0.into()), PhantomData::default())
    }
}

impl<T, Q> Debug for FromReaderSlice<'_, T, Q> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FromReaderSlice({})", ByteSize::b(self.0.len() as u64))
    }
}
