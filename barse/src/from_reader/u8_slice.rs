use crate::{endian::Endian, wrap, ByteRead, Error, FromByteReaderWith};

impl<'input> FromByteReaderWith<'input, wrap::Size> for &'input [u8] {
    type Err = Error;
    fn from_byte_reader_with<R, E>(mut reader: R, with: wrap::Size) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
        E: Endian,
    {
        reader.read_ref(with.0)
    }
}
