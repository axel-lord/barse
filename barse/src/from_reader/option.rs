use crate::{endian::Endian, ByteRead, FromByteReader, FromByteReaderWith};

impl<'input, T> FromByteReaderWith<'input, bool> for Option<T>
where
    T: FromByteReader<'input>,
{
    type Err = T::Err;
    fn from_byte_reader_with<R, E>(reader: R, with: bool) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
        E: Endian,
    {
        with.then(|| T::from_byte_reader::<_, E>(reader))
            .transpose()
    }
}

impl<'input, T, W> FromByteReaderWith<'input, (bool, W)> for Option<T>
where
    T: FromByteReaderWith<'input, W>,
{
    type Err = T::Err;
    fn from_byte_reader_with<R, E>(
        reader: R,
        (with, inner_with): (bool, W),
    ) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
        E: Endian,
    {
        with.then(|| T::from_byte_reader_with::<_, E>(reader, inner_with))
            .transpose()
    }
}
