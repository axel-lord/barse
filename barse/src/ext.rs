//! Extension traits.

pub use self::{byte_sink_ext::ByteSinkExt, byte_source_ext::ByteSourceExt};

#[cfg(feature = "barse_as")]
pub use self::barse_as::{ReadAsExt, WriteAsExt};

mod byte_source_ext;

mod byte_sink_ext;

#[cfg_attr(docsrs, doc(cfg(feature = "barse_as")))]
#[cfg(feature = "barse_as")]
mod barse_as;
