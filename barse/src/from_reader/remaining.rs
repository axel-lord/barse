use std::{borrow::Cow, ops::Deref};

use crate::{endian::Endian, ByteRead, Error, FromByteReader, Result};

/// A type that reads all remaining bytes in a [`crate::ByteRead`][ByteRead].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Remaining<'data>(&'data [u8]);

impl<'input: 'data, 'data> FromByteReader<'input> for Remaining<'data> {
    type Err = Error;
    fn from_byte_reader<R, E>(mut reader: R) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
        E: Endian,
    {
        Ok(Self(reader.remaining()?))
    }
}

impl<'data> Deref for Remaining<'data> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'data> AsRef<[u8]> for Remaining<'data> {
    fn as_ref(&self) -> &[u8] {
        self.0
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
