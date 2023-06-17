use crate::{ByteRead, Endian, Result};

/// [`ByteRead`] wrapper using big endian.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BigEndianByteReader<T>(pub T);

#[deny(clippy::missing_trait_methods)]
impl<'input, R> ByteRead<'input> for BigEndianByteReader<R>
where
    R: ByteRead<'input>,
{
    type AtByteRead = BigEndianByteReader<R::AtByteRead>;

    type ByRefByteRead<'s> = BigEndianByteReader<R::ByRefByteRead<'s>> where Self: 's;

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
        Ok(BigEndianByteReader(self.0.at(location)?))
    }

    fn read<const COUNT: usize>(&mut self) -> Result<[u8; COUNT]> {
        self.0.read()
    }

    fn endian(&self) -> Endian {
        Endian::Big
    }

    fn by_ref(&mut self) -> Self::ByRefByteRead<'_> {
        BigEndianByteReader(self.0.by_ref())
    }
}
