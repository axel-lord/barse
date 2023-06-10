use crate::{wrap, ByteRead, FromByteReaderWith};

impl<'input, T, E, F> FromByteReaderWith<'input, wrap::Fn<F>> for T
where
    F: FnOnce() -> Result<T, E>,
{
    type Err = E;
    fn from_byte_reader_with<R>(_reader: R, with: wrap::Fn<F>) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        with.0()
    }
}
