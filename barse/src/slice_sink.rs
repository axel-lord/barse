//! [SliceSink] implementation.

use crate::{error::SliceSinkFull, ByteSink};

/// [ByteSink] implementor wrapping a slice.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SliceSink<'src> {
    /// Wrapped slice.
    slice: &'src mut [u8],
}

impl Default for SliceSink<'_> {
    fn default() -> Self {
        Self::new(&mut [])
    }
}

unsafe impl Send for SliceSink<'_> {}
unsafe impl Sync for SliceSink<'_> {}

impl<'src> SliceSink<'src> {
    /// Create a new [SliceSink] backed by given slice.
    #[inline]
    pub const fn new(slice: &'src mut [u8]) -> Self {
        Self { slice }
    }

    /// Get next slice of specified size if possible.
    /// Head will be moved past it.
    ///
    /// # Safety
    /// If a slice is returned it is guaranteed to have a length of size.
    #[inline]
    pub const fn next_slice(&mut self, size: usize) -> Option<&'src mut [u8]> {
        // Get length of replacement slice. If size is larger than current length fail.
        let Some(new_len) = self.slice.len().checked_sub(size) else {
            return None;
        };

        // Save start position.
        let start = self.slice.as_mut_ptr();

        // New start position.
        // Safe as new_len exists.
        let new_start = unsafe { start.add(size) };

        // Replace contained slice.
        self.slice = unsafe { ::core::slice::from_raw_parts_mut(new_start, new_len) };

        // Return slice we have moved past.
        Some(unsafe { ::core::slice::from_raw_parts_mut(start, size) })
    }

    /// Get next array ref of specified size.
    /// Head will be moved past it.
    #[inline]
    pub const fn next_array_mut<const SIZE: usize>(&mut self) -> Option<&'src mut [u8; SIZE]> {
        // Get length of replacement slice. If size is larger than current length fail.
        let Some(new_len) = self.slice.len().checked_sub(SIZE) else {
            return None;
        };

        // Save start position.
        let start = self.slice.as_mut_ptr();

        // New start position.
        // Safe as new_len exists.
        let new_start = unsafe { start.add(SIZE) };

        // Replace contained slice.
        self.slice = unsafe { ::core::slice::from_raw_parts_mut(new_start, new_len) };

        // Return slice we have moved past.
        Some(unsafe { &mut *start.cast() })
    }
}

impl ByteSink for SliceSink<'_> {
    type Err = SliceSinkFull;

    #[inline]
    fn write_slice(&mut self, buf: &[u8]) -> Result<(), Self::Err> {
        self.next_slice(buf.len())
            .ok_or(SliceSinkFull)?
            .copy_from_slice(buf);

        Ok(())
    }

    #[inline]
    fn write_byte(&mut self, byte: u8) -> Result<(), Self::Err> {
        let [to] = self.next_array_mut().ok_or(SliceSinkFull)?;
        *to = byte;
        Ok(())
    }

    #[inline]
    fn write_array<const N: usize>(&mut self, bytes: [u8; N]) -> Result<(), Self::Err> {
        *self.next_array_mut().ok_or(SliceSinkFull)? = bytes;
        Ok(())
    }

    #[inline]
    fn remaining(&self) -> Option<usize> {
        Some(self.slice.len())
    }
}
