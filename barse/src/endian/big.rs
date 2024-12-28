//! [Big] impl.

use crate::Endian;

/// Big [Endian] implementor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Big;

impl crate::sealed::Sealed for Big {}
impl Endian for Big {
    fn write<T: crate::sealed::ToFromEndian>(t: T) -> T::Bytes {
        t.to_big()
    }

    fn read<T: crate::sealed::ToFromEndian>(b: T::Bytes) -> T {
        T::from_big(b)
    }
}
