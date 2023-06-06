use crate::{wrap, ByteRead, Error, FromByteReaderWith};

impl<'input, T> FromByteReaderWith<'input, wrap::Default> for T
where
    T: Default,
{
    type Err = Error;
    fn from_byte_reader_with<R>(_reader: R, _with: wrap::Default) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        Ok(T::default())
    }
}
