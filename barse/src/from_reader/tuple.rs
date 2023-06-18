use crate::{endian::Endian, ByteRead, FromByteReader, Result};

from_byte_reader_tuple_impl_recursive! {
    A_, B_, C_,
    D_, E_, F_,
    G_, H_, I_,
    J_, K_, L_
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
            fn from_byte_reader<R, E>(mut reader: R) -> Result<Self, SharedErr>
            where
                R: ByteRead<'input>,
                E: Endian,
            {
                Ok((
                    $(
                    <$templs as FromByteReader>::from_byte_reader::<_, E>(reader.by_ref())?,
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
