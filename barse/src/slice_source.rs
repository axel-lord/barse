//! [SliceSrc] implementation.

use ::core::{marker::PhantomData, ops::Range, ptr::NonNull};

use crate::{error::SliceSrcEmpty, ByteSource};

/// [ByteSource] implementor wrapping a slice.
///
/// The head may be higher than the slice length but not higher than isize::MAX (functions using
/// the head may panic).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SliceSrc<'src> {
    /// Current head of slice.
    head: NonNull<u8>,

    /// Current tail of slice.
    tail: NonNull<u8>,

    /// Indicate lifetime.
    _p: PhantomData<&'src [u8]>,
}

impl Default for SliceSrc<'_> {
    fn default() -> Self {
        Self::new(&[])
    }
}

impl<'src> SliceSrc<'src> {
    /// Create a new [SliceSrc] backed by given slice.
    pub const fn new(slice: &'src [u8]) -> Self {
        let Range { start, end } = slice.as_ptr_range();
        Self {
            // Safe since slice pointers may not be null.
            head: unsafe { NonNull::new_unchecked(start as *mut u8) },
            tail: unsafe { NonNull::new_unchecked(end as *mut u8) },
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
    pub const fn next_slice(&mut self, size: usize) -> Option<&'src [u8]> {
        if size > self.remaining_cap() {
            return None;
        }

        // Since size will not push head past tail this is safe.
        let slice = unsafe { ::core::slice::from_raw_parts(self.head.as_ptr(), size) };

        // Move head past slice.
        self.head = unsafe { self.head.add(size) };

        Some(slice)
    }

    /// Get next array ref of specified size.
    /// Head will be moved past it.
    pub const fn next_array<const SIZE: usize>(&mut self) -> Option<&'src [u8; SIZE]> {
        if SIZE > self.remaining_cap() {
            return None;
        }

        // Both of these are safe since we checked against remaining capacity.
        let slice = unsafe { self.head.cast().as_ref() };
        self.head = unsafe { self.head.add(SIZE) };

        Some(slice)
    }
}

impl ByteSource for SliceSrc<'_> {
    type Err = SliceSrcEmpty;

    fn read_slice(&mut self, buf: &mut [u8]) -> Result<(), Self::Err> {
        buf.copy_from_slice(self.next_slice(buf.len()).ok_or(SliceSrcEmpty)?);
        Ok(())
    }

    fn read_array<const N: usize>(&mut self) -> Result<[u8; N], Self::Err> {
        let arr = self.next_array().ok_or(SliceSrcEmpty)?;
        Ok(*arr)
    }

    fn read_byte(&mut self) -> Result<u8, Self::Err> {
        let [byte] = self.next_array().ok_or(SliceSrcEmpty)?;
        Ok(*byte)
    }

    fn skip(&mut self, count: usize) -> Result<(), Self::Err> {
        if self.remaining_cap() < count {
            return Err(SliceSrcEmpty);
        }

        self.head = unsafe { self.head.add(count) };

        Ok(())
    }

    fn skip_n<const N: usize>(&mut self) -> Result<(), Self::Err> {
        if self.remaining_cap() < N {
            return Err(SliceSrcEmpty);
        }

        self.head = unsafe { self.head.add(N) };

        Ok(())
    }

    fn remaining(&self) -> Option<usize> {
        Some(self.remaining_cap())
    }
}
