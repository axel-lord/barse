use std::borrow::Cow;

use crate::{ByteRead, Error, FromByteReader, Result};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Remaining<'data>(&'data [u8]);

impl<'input: 'data, 'data> FromByteReader<'input> for Remaining<'data> {
    type Err = Error;
    fn from_byte_reader<R>(mut reader: R) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        Ok(Self(reader.remaining()?))
    }
}

impl<'data> From<Remaining<'data>> for Vec<u8> {
    fn from(value: Remaining<'data>) -> Self {
        Vec::from(value.0)
    }
}

impl<'data> From<Remaining<'data>> for Cow<'data, [u8]> {
    fn from(value: Remaining<'data>) -> Self {
        Cow::Borrowed(value.0)
    }
}

impl<'data> From<Remaining<'data>> for &'data [u8] {
    fn from(value: Remaining<'data>) -> Self {
        value.0
    }
}

impl<'data> From<Remaining<'data>> for Box<[u8]> {
    fn from(value: Remaining<'data>) -> Self {
        Box::from(value.0)
    }
}
