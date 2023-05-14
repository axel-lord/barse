use crate::{error::Error, ByteRead, FromByteReader, Result};

impl<'input, const COUNT: usize> FromByteReader<'input> for [u8; COUNT] {
    type Err = Error;
    fn from_byte_reader<R>(mut reader: R) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        reader.read::<COUNT>()
    }
}
