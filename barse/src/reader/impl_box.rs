use crate::{ByteRead, Endian, Result};

#[deny(clippy::missing_trait_methods)]
impl<'input, R> ByteRead<'input> for Box<R>
where
    R: ByteRead<'input>,
{
    type AtByteRead = R::AtByteRead;

    type ByRefByteRead<'s> = R::ByRefByteRead<'s> where Self: 's;

    fn read_ref(&mut self, count: usize) -> Result<&'input [u8]> {
        (**self).read_ref(count)
    }

    fn remaining(&mut self) -> Result<&'input [u8]> {
        (**self).remaining()
    }

    fn all(&self) -> Result<&'input [u8]> {
        (**self).all()
    }

    fn read<const COUNT: usize>(&mut self) -> Result<[u8; COUNT]> {
        (**self).read()
    }

    fn endian(&self) -> Endian {
        (**self).endian()
    }

    fn at(&self, location: usize) -> Result<Self::AtByteRead> {
        (**self).at(location)
    }

    fn by_ref(&mut self) -> Self::ByRefByteRead<'_> {
        (**self).by_ref()
    }
}
