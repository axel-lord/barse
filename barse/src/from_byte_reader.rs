use crate::{ByteRead, Result};

/// Trait for types that can be parsed from a [`ByteRead`].
pub trait FromByteReader<'input>: Sized {
    /// Error type return when parsing bytes fails.
    type Err;
    /// Read the Self from a [`ByteRead`].
    ///
    /// # Errors
    /// If the implementor needs to.
    fn from_byte_reader<R>(reader: R) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>;
}
