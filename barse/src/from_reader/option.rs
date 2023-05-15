use crate::{ByteRead, FromByteReader, FromByteReaderWith};

impl<'input, T> FromByteReaderWith<'input, bool> for Option<T>
where
    T: FromByteReader<'input>,
{
    type Err = T::Err;
    fn from_byte_reader_with<R>(reader: R, with: bool) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        with.then(|| T::from_byte_reader(reader)).transpose()
    }
}

impl<'input, T, W> FromByteReaderWith<'input, (bool, W)> for Option<T>
where
    T: FromByteReaderWith<'input, W>,
{
    type Err = T::Err;
    fn from_byte_reader_with<R>(reader: R, (with, inner_with): (bool, W)) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        with.then(|| T::from_byte_reader_with(reader, inner_with))
            .transpose()
    }
}

impl<'input, F, T> FromByteReaderWith<'input, F> for Option<T>
where
    T: FromByteReader<'input>,
    F: FnOnce() -> bool,
{
    type Err = T::Err;

    fn from_byte_reader_with<R>(reader: R, with: F) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        Option::from_byte_reader_with(reader, with())
    }
}

impl<'input, F, T, W> FromByteReaderWith<'input, (F, W)> for Option<T>
where
    T: FromByteReaderWith<'input, W>,
    F: FnOnce() -> bool,
{
    type Err = T::Err;
    fn from_byte_reader_with<R>(reader: R, (with, inner_with): (F, W)) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        Option::from_byte_reader_with(reader, (with(), inner_with))
    }
}
