use crate::{ByteRead, Error, FromByteReader};

/// Padding in binary data.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Padding<const SIZE: usize>;

impl<const SIZE: usize> std::fmt::Debug for Padding<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Padding({})", SIZE.saturating_mul(8))
    }
}

impl<'input, const SIZE: usize> FromByteReader<'input> for Padding<SIZE> {
    type Err = Error;
    fn from_byte_reader<R>(mut reader: R) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        reader.read::<SIZE>()?;
        Ok(Self)
    }
}
