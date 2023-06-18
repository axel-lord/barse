use crate::{endian::Endian, wrap, ByteRead, FromByteReaderWith};

impl<'input, T, Err, F> FromByteReaderWith<'input, wrap::Fn<F>> for T
where
    F: FnOnce() -> Result<T, Err>,
{
    type Err = Err;
    fn from_byte_reader_with<R, E>(_reader: R, with: wrap::Fn<F>) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
        E: Endian,
    {
        with.0()
    }
}
