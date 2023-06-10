use crate::{ByteRead, Endian, Result};

/// [`ByteRead`] wrapper using little endian.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LittleEndianByteReader<T>(pub T);

#[deny(clippy::missing_trait_methods)]
impl<'input, R> ByteRead<'input> for LittleEndianByteReader<R>
where
    R: ByteRead<'input>,
{
    type AtByteRead = LittleEndianByteReader<R::AtByteRead>;

    fn read_ref(&mut self, count: usize) -> Result<&'input [u8]> {
        self.0.read_ref(count)
    }

    fn remaining(&mut self) -> Result<&'input [u8]> {
        self.0.remaining()
    }

    fn all(&self) -> Result<&'input [u8]> {
        self.0.all()
    }

    fn at(&self, location: usize) -> Result<Self::AtByteRead> {
        Ok(LittleEndianByteReader(self.0.at(location)?))
    }

    fn read<const COUNT: usize>(&mut self) -> Result<[u8; COUNT]> {
        self.0.read()
    }

    fn endian(&self) -> Endian {
        Endian::Little
    }
}
