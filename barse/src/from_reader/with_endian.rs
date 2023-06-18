use crate::{endian, ByteRead, FromByteReader, FromByteReaderWith};

impl<'input, T> FromByteReaderWith<'input, endian::Big> for T
where
    T: FromByteReader<'input>,
{
    type Err = T::Err;

    fn from_byte_reader_with<R, E>(reader: R, _: endian::Big) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
        E: endian::Endian,
    {
        T::from_byte_reader::<_, endian::Big>(reader)
    }
}

impl<'input, T> FromByteReaderWith<'input, endian::Little> for T
where
    T: FromByteReader<'input>,
{
    type Err = T::Err;

    fn from_byte_reader_with<R, E>(reader: R, _: endian::Little) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
        E: endian::Endian,
    {
        T::from_byte_reader::<_, endian::Little>(reader)
    }
}

impl<'input, T> FromByteReaderWith<'input, endian::Either> for T
where
    T: FromByteReader<'input>,
{
    type Err = T::Err;

    fn from_byte_reader_with<R, E>(reader: R, with: endian::Either) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
        E: endian::Endian,
    {
        match with {
            endian::Either::Big => T::from_byte_reader::<_, endian::Big>(reader),
            endian::Either::Little => T::from_byte_reader::<_, endian::Little>(reader),
        }
    }
}

impl<'input, T, W> FromByteReaderWith<'input, (endian::Big, W)> for T
where
    T: FromByteReaderWith<'input, W>,
{
    type Err = T::Err;

    fn from_byte_reader_with<R, E>(
        reader: R,
        (_, with): (endian::Big, W),
    ) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
        E: endian::Endian,
    {
        T::from_byte_reader_with::<_, endian::Big>(reader, with)
    }
}

impl<'input, T, W> FromByteReaderWith<'input, (endian::Little, W)> for T
where
    T: FromByteReaderWith<'input, W>,
{
    type Err = T::Err;

    fn from_byte_reader_with<R, E>(
        reader: R,
        (_, with): (endian::Little, W),
    ) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
        E: endian::Endian,
    {
        T::from_byte_reader_with::<_, endian::Little>(reader, with)
    }
}

impl<'input, T, W> FromByteReaderWith<'input, (endian::Either, W)> for T
where
    T: FromByteReaderWith<'input, W>,
{
    type Err = T::Err;

    fn from_byte_reader_with<R, E>(
        reader: R,
        (end, with): (endian::Either, W),
    ) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
        E: endian::Endian,
    {
        match end {
            endian::Either::Big => T::from_byte_reader_with::<_, endian::Big>(reader, with),
            endian::Either::Little => T::from_byte_reader_with::<_, endian::Little>(reader, with),
        }
    }
}
