use std::convert::identity;

use crate::{endian::Endian, wrap, ByteRead, FromByteReader, FromByteReaderWith};

impl<'input, T> FromByteReaderWith<'input, wrap::Len> for Vec<T>
where
    T: FromByteReader<'input>,
{
    type Err = T::Err;
    fn from_byte_reader_with<R, E>(
        mut reader: R,
        wrap::Len(with): wrap::Len,
    ) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
        E: Endian,
    {
        let mut vec = Vec::with_capacity(with);

        for _ in 0..with {
            vec.push(T::from_byte_reader::<_, E>(reader.by_ref())?);
        }

        Ok(vec)
    }
}

impl<'input, T, W> FromByteReaderWith<'input, (wrap::Len, W)> for Vec<T>
where
    T: FromByteReaderWith<'input, W>,
    W: Clone,
{
    type Err = T::Err;
    fn from_byte_reader_with<R, E>(
        mut reader: R,
        (wrap::Len(size), with): (wrap::Len, W),
    ) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
        E: Endian,
    {
        let mut vec = Vec::with_capacity(size);

        for _ in 0..size {
            vec.push(T::from_byte_reader_with::<_, E>(
                reader.by_ref(),
                with.clone(),
            )?);
        }

        Ok(vec)
    }
}

impl<'input, 'slice, T, S, W, M> FromByteReaderWith<'input, (&'slice [S], M)> for Vec<T>
where
    T: FromByteReaderWith<'input, W>,
    M: FnMut(&'slice S) -> W,
{
    type Err = T::Err;
    fn from_byte_reader_with<R, E>(
        mut reader: R,
        (with, mut map): (&'slice [S], M),
    ) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
        E: Endian,
    {
        let mut vec = Vec::with_capacity(with.len());

        for with in with {
            vec.push(T::from_byte_reader_with::<_, E>(
                reader.by_ref(),
                map(with),
            )?);
        }

        Ok(vec)
    }
}

impl<'input, T, I, W, M> FromByteReaderWith<'input, (wrap::Iter<I>, M)> for Vec<T>
where
    T: FromByteReaderWith<'input, W>,
    I: IntoIterator,
    M: FnMut(I::Item) -> W,
{
    type Err = T::Err;

    fn from_byte_reader_with<R, E>(
        mut reader: R,
        (wrap::Iter(with), mut map): (wrap::Iter<I>, M),
    ) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
        E: Endian,
    {
        let mut vec = Vec::new();

        for with in with {
            vec.push(T::from_byte_reader_with::<_, E>(
                reader.by_ref(),
                map(with),
            )?);
        }

        Ok(vec)
    }
}

impl<'input, T, I> FromByteReaderWith<'input, wrap::Iter<I>> for Vec<T>
where
    T: FromByteReaderWith<'input, I::Item>,
    I: IntoIterator,
{
    type Err = T::Err;
    fn from_byte_reader_with<R, E>(reader: R, with: wrap::Iter<I>) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
        E: Endian,
    {
        Vec::from_byte_reader_with::<_, E>(reader, (with, identity))
    }
}

impl<'input, 'w, T, W> FromByteReaderWith<'input, &'w [W]> for Vec<T>
where
    T: FromByteReaderWith<'input, &'w W>,
{
    type Err = T::Err;

    fn from_byte_reader_with<R, E>(reader: R, with: &'w [W]) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
        E: Endian,
    {
        Vec::from_byte_reader_with::<_, E>(reader, (with, identity))
    }
}

impl<'input> FromByteReaderWith<'input, wrap::Size> for Vec<u8> {
    type Err = crate::Error;

    fn from_byte_reader_with<R, E>(reader: R, with: wrap::Size) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
        E: Endian,
    {
        <&[u8]>::from_byte_reader_with::<_, E>(reader, with).map(Vec::from)
    }
}
