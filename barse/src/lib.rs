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
pub mod barse_as;

#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[cfg(feature = "std")]
mod if_std;

#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[cfg(feature = "alloc")]
mod if_alloc;

mod sealed;

pub use self::{
    barse::Barse,
    byte_sink::ByteSink,
    byte_source::ByteSource,
    error::{Error, WrappedErr},
};

#[cfg(feature = "barse_as")]
#[doc(inline)]
pub use self::barse_as::{ReadAs, WriteAs};

#[doc(inline)]
pub use self::endian::Endian;

#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
#[cfg(feature = "derive")]
pub use barse_derive::Barse;

#[cfg(all(feature = "std", feature = "ext"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "std", feature = "ext"))))]
pub use if_std::{read_value, write_value};
