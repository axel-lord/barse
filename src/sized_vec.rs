use std::marker::PhantomData;

use crate::{ByteRead, FromByteReader, Result};

pub trait VecLenQuery {
    type Flag;
    fn len(flag: &Self::Flag) -> usize;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FromReaderVec<T, Q>(Vec<T>, PhantomData<Q>);

impl<'input, T, Q> FromByteReader<'input> for FromReaderVec<T, Q>
where
    T: FromByteReader<'input>,
    Q: VecLenQuery + 'static,
{
    fn from_byte_reader<R>(mut reader: R) -> Result<Self>
    where
        R: ByteRead<'input>,
    {
        let flag = reader.flags::<Q::Flag>()?;
        let len = Q::len(flag);
        let items = (0..len)
            .map(|_| T::from_byte_reader(&mut reader))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self(items, PhantomData::default()))
    }
}
