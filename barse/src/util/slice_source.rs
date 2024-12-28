//! [SliceSrc] implementation.

use ::core::fmt::Display;

use crate::ByteSource;

/// [ByteSource] implementor wrapping a slice.
///
/// The head may be higher than the slice length but not higher than isize::MAX (functions using
/// the head may panic).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SliceSrc<'src> {
    /// Slice that is read from.
    pub slice: &'src [u8],

    /// Where to read from slice.
    pub head: usize,
}

impl<'src> SliceSrc<'src> {
    /// Create a new [SliceSrc] backed by given slice.
    pub const fn new(slice: &'src [u8]) -> Self {
        Self { slice, head: 0 }
    }
}

/// Error returned by [ByteSource] implementation for [SliceSrc] when bytes cannot be read.
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

impl ByteSource for SliceSrc<'_> {
    type Err = SliceSrcEmpty;

    fn read_slice(&mut self, buf: &mut [u8]) -> Result<(), Self::Err> {
        debug_assert!(self.head <= isize::MAX as usize);

        let start = self.head;
        let end = self.head + buf.len();

        if end > self.slice.len() {
            return Err(SliceSrcEmpty);
        }

        buf.copy_from_slice(&self.slice[start..end]);
        self.head = end;

        Ok(())
    }
}
