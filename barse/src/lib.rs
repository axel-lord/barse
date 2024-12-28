#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod endian;

mod error;

mod barse;

mod byte_source;

mod byte_sink;

#[cfg_attr(docsrs, doc(cfg(feature = "ext")))]
#[cfg(feature = "ext")]
pub mod ext;

#[cfg_attr(docsrs, doc(cfg(feature = "util")))]
#[cfg(feature = "util")]
pub mod util;

#[cfg_attr(docsrs, doc(cfg(feature = "barse_as")))]
#[cfg(feature = "barse_as")]
mod barse_as;

#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[cfg(feature = "std")]
mod if_std;

#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[cfg(feature = "alloc")]
mod if_alloc;

#[cfg_attr(docsrs, doc(cfg(feature = "zerocopy")))]
#[cfg(feature = "zerocopy")]
mod zerocopy;

#[cfg_attr(docsrs, doc(cfg(feature = "bytemuck")))]
#[cfg(feature = "bytemuck")]
mod bytemuck;

mod sealed {
    //! [Sealed] trait.

    /// Trait used to prevent implementations.
    pub trait Sealed {}

    /// Trait used to convert values to/from bytes.
    pub trait ToFromEndian: Sized {
        /// Bytes used by trait.
        type Bytes;

        /// Convert to native.
        fn to_native(self) -> Self::Bytes;

        /// Convert to big.
        fn to_big(self) -> Self::Bytes;

        /// Convert to little.
        fn to_little(self) -> Self::Bytes;

        /// Convert from native.
        fn from_native(bytes: Self::Bytes) -> Self;

        /// Convert from big.
        fn from_big(bytes: Self::Bytes) -> Self;

        /// Convert from little.
        fn from_little(bytes: Self::Bytes) -> Self;
    }
}

pub use self::{
    barse::Barse,
    byte_sink::ByteSink,
    byte_source::ByteSource,
    error::{Error, WrappedErr},
};

#[cfg(feature = "barse_as")]
pub use self::barse_as::{Default, ReadAs, WriteAs};

#[doc(inline)]
pub use self::endian::Endian;

#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
#[cfg(feature = "derive")]
pub use barse_derive::Barse;

#[cfg(feature = "zerocopy")]
pub use zerocopy::Zerocopy;

#[cfg(feature = "bytemuck")]
pub use bytemuck::Bytemuck;
