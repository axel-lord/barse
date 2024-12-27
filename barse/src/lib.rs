#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

mod error;

mod barse;

mod byte_source;

mod byte_sink;

#[cfg(feature = "barse_as")]
mod barse_as;

#[cfg(feature = "ext")]
mod ext;

#[cfg(feature = "std")]
mod if_std;

#[cfg(feature = "alloc")]
mod if_alloc;

mod sealed {
    //! [Sealed] trait.

    /// Trait used to prevent implementations.
    pub trait Sealed {}
}

pub use self::{barse::Barse, byte_sink::ByteSink, byte_source::ByteSource, error::Error};

#[cfg(feature = "barse_as")]
pub use self::barse_as::{ReadAs, WriteAs};

#[cfg(feature = "ext")]
pub use self::ext::{ByteSinkExt, ByteSourceExt};

#[doc(inline)]
pub use self::endian::Endian;

#[cfg(all(feature = "std", feature = "ext"))]
pub use if_std::{AsByteSink, AsByteSource};

#[cfg(feature = "derive")]
pub use barse_derive::Barse;

pub mod endian;

#[cfg(feature = "util")]
pub mod util;

#[cfg(feature = "zerocopy")]
pub mod zerocopy;

#[cfg(feature = "bytemuck")]
pub mod bytemuck;

pub mod prelude {
    //! Crate prelude, gives access to needed traits.

    #[cfg(all(feature = "std", feature = "ext"))]
    pub use crate::{AsByteSink, AsByteSource};

    pub use crate::{ByteSink, ByteSource};

    #[cfg(feature = "barse_as")]
    pub use crate::{ReadAs, WriteAs};

    #[cfg(feature = "ext")]
    pub use crate::{ByteSinkExt, ByteSourceExt};
}
