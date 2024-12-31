//! Errors used by utilities.

use ::core::fmt::Display;

/// Error returned by [ByteSink][crate::ByteSink] implementation for
/// [SliceSink][crate::util::SliceSink] when no more bytes can
/// be written to sink.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SliceSinkFull;

impl Display for SliceSinkFull {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("a value was too large to be written to remaining length at head of SliceSink")
    }
}

impl ::core::error::Error for SliceSinkFull {}

impl From<SliceSinkFull> for crate::Error {
    fn from(_value: SliceSinkFull) -> Self {
        static ERR: SliceSinkFull = SliceSinkFull;
        crate::Error::Dyn(&ERR)
    }
}

/// Error returned by [ByteSource][crate::ByteSource] implementation for
/// [SliceSrc][crate::util::SliceSrc] when bytes cannot be read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SliceSrcEmpty;

impl Display for SliceSrcEmpty {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("a value was too large to be read from remaining length at head of SliceSrc")
    }
}

impl ::core::error::Error for SliceSrcEmpty {}

impl From<SliceSrcEmpty> for crate::Error {
    fn from(_value: SliceSrcEmpty) -> Self {
        static ERR: SliceSrcEmpty = SliceSrcEmpty;
        crate::Error::Dyn(&ERR)
    }
}
