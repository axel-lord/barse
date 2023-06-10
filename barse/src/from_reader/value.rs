use crate::{wrap, ByteRead, Error, FromByteReaderWith};

impl<'input, T> FromByteReaderWith<'input, wrap::Value<T>> for T {
    type Err = Error;
    fn from_byte_reader_with<R>(_reader: R, with: wrap::Value<T>) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        Ok(with.0)
    }
}
