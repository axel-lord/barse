use crate::{ByteRead, Endian, Result};

#[deny(clippy::missing_trait_methods)]
impl<'input, B> ByteRead<'input> for &mut B
where
    B: ByteRead<'input>,
{
    type AtByteRead = B::AtByteRead;

    fn read<const COUNT: usize>(&mut self) -> Result<[u8; COUNT]> {
        (*self).read()
    }

    fn endian(&self) -> Endian {
        (**self).endian()
    }

    fn read_ref(&mut self, count: usize) -> Result<&'input [u8]> {
        (*self).read_ref(count)
    }

    fn remaining(&mut self) -> Result<&'input [u8]> {
        (*self).remaining()
    }

    fn all(&self) -> Result<&'input [u8]> {
        (**self).all()
    }

    fn at(&self, location: usize) -> Result<Self::AtByteRead> {
        (**self).at(location)
    }
}
