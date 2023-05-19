use crate::{ByteRead, FromByteReader, Result};

from_byte_reader_tuple_impl_recursive! {
    A, B, C,
    D, E, F,
    G, H, I,
    J, K, L
}

macro_rules! from_byte_reader_tuple_impl {
    ($($templs:ident),+ $(,)?) => {
        impl<'input, SharedErr, $($templs),*> FromByteReader<'input> for ($($templs,)*)
        where
            $(
            $templs: FromByteReader<'input, Err = SharedErr>,
            )*
        {
            type Err = SharedErr;
            fn from_byte_reader<R>(mut reader: R) -> Result<Self, SharedErr>
            where
                R: ByteRead<'input>,
            {
                Ok((
                    $(
                    <$templs as FromByteReader>::from_byte_reader(&mut reader)?,
                    )*
                ))
            }
        }

    };
}
use from_byte_reader_tuple_impl;

macro_rules! from_byte_reader_tuple_impl_recursive {
    ($first: tt) => {
        from_byte_reader_tuple_impl! {$first}
    };
    ($first: tt, $($other: tt),+) => {
        from_byte_reader_tuple_impl! {$first, $($other),*}
        from_byte_reader_tuple_impl_recursive! {$($other),*}
    }
}
use from_byte_reader_tuple_impl_recursive;
