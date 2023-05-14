use std::marker::PhantomData;

use crate::{error::Error, ByteRead, FromByteReader, Result};

impl<'input, T> FromByteReader<'input> for PhantomData<T> {
    type Err = Error;
    fn from_byte_reader<R>(_reader: R) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        Ok(PhantomData::default())
    }
}
