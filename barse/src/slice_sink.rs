//! [SliceSink] implementation.

use ::core::{marker::PhantomData, ops::Range, ptr::NonNull};

use crate::{error::SliceSinkFull, ByteSink};

/// [ByteSink] implementor wrapping a slice.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SliceSink<'src> {
    /// Current head of slice.
    head: NonNull<u8>,

    /// Current tail of slice.
    tail: NonNull<u8>,

    /// Indicate ownership of slice.
    _p: PhantomData<&'src mut [u8]>,
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
    pub const fn new(slice: &'src mut [u8]) -> Self {
        let Range { start, end } = slice.as_mut_ptr_range();
        Self {
            // Safe since slice pointers may not be null.
            head: unsafe { NonNull::new_unchecked(start) },
            tail: unsafe { NonNull::new_unchecked(end) },
            _p: PhantomData,
        }
    }

    /// Get current length, same as remaining write capacity.
    const fn remaining_cap(&self) -> usize {
        (unsafe { self.tail.offset_from(self.head) } as usize)
    }

    /// Get next slice of specified size if possible.
    /// Head will be moved past it.
    ///
    /// # Safety
    /// If a slice is returned it is guaranteed to have a length of size.
    pub const fn next_slice(&mut self, size: usize) -> Option<&'src mut [u8]> {
        if size > self.remaining_cap() {
            return None;
        }

        // Since size will not push head past tail this is safe.
        let slice = unsafe { ::core::slice::from_raw_parts_mut(self.head.as_ptr(), size) };

        // Move head past slice.
        self.head = unsafe { self.head.add(size) };

        Some(slice)
    }

    /// Get next array ref of specified size.
    /// Head will be moved past it.
    pub const fn next_array_mut<const SIZE: usize>(&mut self) -> Option<&'src mut [u8; SIZE]> {
        if SIZE > self.remaining_cap() {
            return None;
        }

        // Both of these are safe since we checked against remaining capacity.
        let slice = unsafe { self.head.cast().as_mut() };
        self.head = unsafe { self.head.add(SIZE) };

        Some(slice)
    }
}

impl ByteSink for SliceSink<'_> {
    type Err = SliceSinkFull;

    fn write_slice(&mut self, buf: &[u8]) -> Result<(), Self::Err> {
        self.next_slice(buf.len())
            .ok_or(SliceSinkFull)?
            .copy_from_slice(buf);

        Ok(())
    }

    fn write_byte(&mut self, byte: u8) -> Result<(), Self::Err> {
        let [to] = self.next_array_mut().ok_or(SliceSinkFull)?;
        *to = byte;
        Ok(())
    }

    fn write_array<const N: usize>(&mut self, bytes: [u8; N]) -> Result<(), Self::Err> {
        *self.next_array_mut().ok_or(SliceSinkFull)? = bytes;
        Ok(())
    }

    fn remaining(&self) -> Option<usize> {
        Some(self.remaining_cap())
    }
}
