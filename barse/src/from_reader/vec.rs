use crate::{wrap, ByteRead, FromByteReader, FromByteReaderWith};

impl<'input, T> FromByteReaderWith<'input, usize> for Vec<T>
where
    T: FromByteReader<'input>,
{
    type Err = T::Err;
    fn from_byte_reader_with<R>(mut reader: R, with: usize) -> Result<Self, Self::Err>
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

impl<'input, T, W> FromByteReaderWith<'input, (usize, W)> for Vec<T>
where
    T: FromByteReaderWith<'input, W>,
    W: Clone,
{
    type Err = T::Err;
    fn from_byte_reader_with<R>(mut reader: R, (size, with): (usize, W)) -> Result<Self, Self::Err>
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

impl<'input, 'w, T, W> FromByteReaderWith<'input, &'w [W]> for Vec<T>
where
    T: FromByteReaderWith<'input, &'w W>,
{
    type Err = T::Err;
    fn from_byte_reader_with<R>(mut reader: R, with: &'w [W]) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        let mut vec = Vec::with_capacity(with.len());

        for with in with {
            vec.push(T::from_byte_reader_with(&mut reader, with)?);
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

    fn from_byte_reader_with<R>(mut reader: R, with: wrap::Iter<I>) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        let mut vec = Vec::new();

        for with in with.into_inner() {
            vec.push(T::from_byte_reader_with(&mut reader, with)?);
        }

        Ok(vec)
    }
}

impl<'input, T, F> FromByteReaderWith<'input, F> for Vec<T>
where
    T: FromByteReader<'input>,
    F: FnOnce() -> usize,
{
    type Err = T::Err;
    fn from_byte_reader_with<R>(reader: R, with: F) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        Self::from_byte_reader_with(reader, with())
    }
}

impl<'input, T, F, W> FromByteReaderWith<'input, (F, W)> for Vec<T>
where
    T: FromByteReaderWith<'input, W>,
    F: FnOnce() -> usize,
    W: Clone,
{
    type Err = T::Err;
    fn from_byte_reader_with<R>(reader: R, (size, with): (F, W)) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        Self::from_byte_reader_with(reader, (size(), with))
    }
}
