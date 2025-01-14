//! Extension traits.

pub use self::{
    barse_read::BarseReadExt, barse_write::BarseWriteExt, byte_sink_ext::ByteSinkExt,
    byte_source_ext::ByteSourceExt,
};

#[cfg(feature = "barse_as")]
pub use self::barse_as::{ReadAsExt, WriteAsExt};

#[cfg(feature = "std")]
pub use if_std::{AsByteSink, AsByteSource};

mod byte_source_ext;

mod byte_sink_ext;

mod barse_read;

mod barse_write;

#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[cfg(feature = "std")]
mod if_std;

#[cfg_attr(docsrs, doc(cfg(feature = "barse_as")))]
#[cfg(feature = "barse_as")]
mod barse_as;
