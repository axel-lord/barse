#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;

mod barse;

mod byte_source;

mod byte_sink;

#[cfg_attr(docsrs, doc(cfg(feature = "barse_as")))]
#[cfg(feature = "barse_as")]
mod barse_as;

#[cfg_attr(docsrs, doc(cfg(feature = "ext")))]
#[cfg(feature = "ext")]
pub mod ext;

#[cfg_attr(docsrs, doc(cfg(all(feature = "std", feature = "ext"))))]
#[cfg(feature = "std")]
mod if_std_ext;

#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[cfg(feature = "std")]
mod if_std;

#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[cfg(feature = "alloc")]
mod if_alloc;

mod sealed {
    //! [Sealed] trait.

    /// Trait used to prevent implementations.
    pub trait Sealed {}
}

pub use self::{
    barse::Barse,
    byte_sink::ByteSink,
    byte_source::ByteSource,
    error::{Error, WrappedErr},
};

#[cfg(feature = "barse_as")]
pub use self::barse_as::{ReadAs, WriteAs};

#[doc(inline)]
pub use self::endian::Endian;

#[cfg(all(feature = "std", feature = "ext"))]
pub use if_std_ext::{AsByteSink, AsByteSource};

#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
#[cfg(feature = "derive")]
pub use barse_derive::Barse;

#[cfg(feature = "zerocopy")]
pub use zerocopy::Zerocopy;

pub mod endian;

#[cfg_attr(docsrs, doc(cfg(feature = "util")))]
#[cfg(feature = "util")]
pub mod util;

#[cfg_attr(docsrs, doc(cfg(feature = "zerocopy")))]
#[cfg(feature = "zerocopy")]
mod zerocopy;

#[cfg_attr(docsrs, doc(cfg(feature = "bytemuck")))]
#[cfg(feature = "bytemuck")]
pub mod bytemuck;
