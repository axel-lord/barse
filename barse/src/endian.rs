//! Trait implementations of [Endian].

use crate::sealed::Sealed;

pub use self::{big::Big, little::Little, native::Native};

#[cfg(feature = "util")]
pub use runtime::Runtime;

#[doc = "Trait defining endianess, [Big], [Little] and [Native] is available."]
pub trait Endian: Sealed {
    #[doc(hidden)]
    fn write<T: crate::sealed::ToFromEndian>(t: T) -> T::Bytes;

    #[doc(hidden)]
    fn read<T: crate::sealed::ToFromEndian>(b: T::Bytes) -> T;
}

mod big;

mod little;

mod native;

#[cfg(feature = "util")]
#[cfg_attr(docsrs, doc(cfg(feature = "util")))]
mod runtime;
