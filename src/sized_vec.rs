use std::marker::PhantomData;

use crate::{ByteRead, FromByteReader, Result};

/// Trait used to query the length of a vector.
pub trait VecLenQuery {
    /// Type to query on.
    type Flag;

    /// Length of items that should be in vector.
    fn len(flag: &Self::Flag) -> usize;
}

/// A vec with it's length queried from a reader.
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

impl<T, Q> From<FromReaderVec<T, Q>> for Vec<T> {
    fn from(value: FromReaderVec<T, Q>) -> Self {
        value.0
    }
}
