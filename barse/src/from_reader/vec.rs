use std::convert::identity;

use crate::{wrap, ByteRead, FromByteReader, FromByteReaderWith};

impl<'input, T> FromByteReaderWith<'input, wrap::Length> for Vec<T>
where
    T: FromByteReader<'input>,
{
    type Err = T::Err;
    fn from_byte_reader_with<R>(
        mut reader: R,
        wrap::Length(with): wrap::Length,
    ) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        let mut vec = Vec::with_capacity(with);

        for _ in 0..with {
            vec.push(T::from_byte_reader(&mut reader)?);
        }

        Ok(vec)
    }
}

impl<'input, T, W> FromByteReaderWith<'input, (wrap::Length, W)> for Vec<T>
where
    T: FromByteReaderWith<'input, W>,
    W: Clone,
{
    type Err = T::Err;
    fn from_byte_reader_with<R>(
        mut reader: R,
        (wrap::Length(size), with): (wrap::Length, W),
    ) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        let mut vec = Vec::with_capacity(size);

        for _ in 0..size {
            vec.push(T::from_byte_reader_with(&mut reader, with.clone())?);
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
    fn from_byte_reader_with<R>(
        mut reader: R,
        (with, mut map): (&'slice [S], M),
    ) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        let mut vec = Vec::with_capacity(with.len());

        for with in with {
            vec.push(T::from_byte_reader_with(&mut reader, map(with))?);
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

    fn from_byte_reader_with<R>(
        mut reader: R,
        (wrap::Iter(with), mut map): (wrap::Iter<I>, M),
    ) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        let mut vec = Vec::new();

        for with in with {
            vec.push(T::from_byte_reader_with(&mut reader, map(with))?);
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
    fn from_byte_reader_with<R>(reader: R, with: wrap::Iter<I>) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        Vec::from_byte_reader_with(reader, (with, identity))
    }
}

impl<'input, 'w, T, W> FromByteReaderWith<'input, &'w [W]> for Vec<T>
where
    T: FromByteReaderWith<'input, &'w W>,
{
    type Err = T::Err;

    fn from_byte_reader_with<R>(reader: R, with: &'w [W]) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        Vec::from_byte_reader_with(reader, (with, identity))
    }
}
