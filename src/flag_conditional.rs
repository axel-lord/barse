use std::marker::PhantomData;

use crate::{ByteRead, FromByteReader, Result};

// Condition for [FlagConditional].
pub trait Condition {
    type Flag;

    fn verify(flag: &Self::Flag) -> bool;
}

// Option wrapper that implements FromByteReader dependinf on a given condition.
#[derive(Clone, Copy, Debug)]
pub struct FlagConditional<T, C>(Option<T>, PhantomData<C>);

impl<'input, T, C> FromByteReader<'input> for FlagConditional<T, C>
where
    T: FromByteReader<'input>,
    C: Condition + 'static,
{
    fn from_byte_reader<R>(reader: R) -> Result<Self>
    where
        R: ByteRead<'input>,
    {
        let flag = reader.flags::<C::Flag>()?;

        Ok(Self(
            if C::verify(flag) {
                Some(T::from_byte_reader(reader)?)
            } else {
                None
            },
            PhantomData::default(),
        ))
    }
}
