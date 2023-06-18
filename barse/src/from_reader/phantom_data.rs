use std::marker::PhantomData;

use crate::{endian::Endian, error::Error, ByteRead, FromByteReader, Result};

impl<'input, T> FromByteReader<'input> for PhantomData<T> {
    type Err = Error;
    fn from_byte_reader<R, E>(_reader: R) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
        E: Endian,
    {
        Ok(PhantomData::default())
    }
}
