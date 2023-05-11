use std::marker::PhantomData;

use crate::{error::Error, ByteRead, FromByteReader, Result};

/// Condition for [`FlagConditional`].
pub trait Condition {
    /// What type of flag to parse to use this condition.
    type Flag;

    /// Verify the condition using given flag.
    fn verify(flag: &Self::Flag) -> bool;
}

/// Option wrapper that implements [`FromByteReader`] depending on a given condition.
#[derive(Clone, Copy, Debug)]
pub struct FlagConditional<T, C>(Option<T>, PhantomData<C>);

impl<'input, T, C, E> FromByteReader<'input> for FlagConditional<T, C>
where
    T: FromByteReader<'input, Err = E>,
    C: Condition + 'static,
    E: From<Error>,
{
    type Err = E;

    fn from_byte_reader<R>(reader: R) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        let flag = reader.flags::<C::Flag>()?;

        Ok(Self(
            C::verify(flag)
                .then(move || T::from_byte_reader(reader))
                .transpose()?,
            PhantomData::default(),
        ))
    }
}

impl<T, C> From<FlagConditional<T, C>> for Option<T> {
    fn from(val: FlagConditional<T, C>) -> Self {
        val.0
    }
}
