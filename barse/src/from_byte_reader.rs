use crate::{endian::Endian, ByteRead};

/// Trait for types that can be parsed from a [`ByteRead`].
pub trait FromByteReader<'input>: Sized {
    /// Error type returned when parsing bytes fails.
    type Err;
    /// Read Self from a [`ByteRead`].
    ///
    /// # Errors
    /// If the implementor needs to.
    fn from_byte_reader<R, E>(reader: R) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
        E: Endian;
}
