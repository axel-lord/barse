#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

mod error;

mod barse;

mod byte_source;

mod byte_sink;

#[cfg(feature = "std")]
mod if_std;

mod sealed {
    //! [Sealed] trait.

    /// Trait used to prevent implementations.
    pub trait Sealed {}
}

pub use self::{barse::Barse, byte_sink::ByteSink, byte_source::ByteSource, error::Error};

#[doc(inline)]
pub use self::endian::Endian;

#[cfg(feature = "std")]
pub use if_std::{AsByteSink, AsByteSource};

pub mod endian;

pub mod util;

pub mod prelude {
    //! Crate prelude, gives access to needed traits.

    pub use crate::{AsByteSink, AsByteSource, ByteSink, ByteSource};
}
