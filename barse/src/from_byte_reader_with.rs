use crate::{endian::Endian, ByteRead};

/// Trait for types that can be read from a byte reader provided some extra information is passed.
pub trait FromByteReaderWith<'input, W>: Sized {
    /// Error type returned when parsing fails.
    type Err;
    /// Read Self from a [`ByteRead`] and some other value.
    ///
    /// # Errors
    /// If the implementor needs to.
    fn from_byte_reader_with<R, E>(reader: R, with: W) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
        E: Endian;
}
