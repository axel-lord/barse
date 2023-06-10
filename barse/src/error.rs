//! Error utilities for crate.

use std::{fmt::Debug, ops::Range};

use thiserror::Error;

/// Error type in use by crate.
#[derive(Debug, Error)]
pub enum Error {
    /// Slicing of input bytes failed, possibly due to invalid indices.
    #[error("a slice ({0:?}) was not valid")]
    SliceFailure(Range<usize>),
    /// An overflow occured on a read.
    #[error("an overflow happened while reading {count} bytes starting at index {start}")]
    ReadOverflow {
        /// Index of first byte in failed read.
        start: usize,
        /// Amount of bytes that should have been read.
        count: usize,
    },
    /// The [ByteRead::at][crate::ByteRead::at] function was called with an out of bounds location.
    #[error("at was called for a byte reader with an out of bounds location, requested location: {requested}, highest possible location: {max}")]
    OutOfBoundsAt {
        /// Requested locations
        requested: usize,
        /// Highest possible location.
        max: usize,
    },

    /// Anyhow catch-all
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl Error {
    /// If the error is anyhow it might still be of this type but obscured, this method downcasts
    /// the anyhow error as many times as it needs to to either confirm the anyhow error is not of
    /// [`enum@Error`] or that it is a not [`Error::Anyhow`].
    ///
    /// ```
    /// use barse::Error;
    /// use anyhow::anyhow;
    ///
    /// let err = Error::Anyhow(anyhow![Error::Anyhow(anyhow![Error::SliceFailure(0..0)])]);
    ///
    /// assert!(matches!(err.anyhow_flatten(), Error::SliceFailure(..)));
    /// ```
    #[must_use]
    pub fn anyhow_flatten(self) -> Self {
        let Self::Anyhow(mut err) = self else {
            return self;
        };

        loop {
            match err.downcast::<Self>() {
                Err(err) => break Self::Anyhow(err),
                Ok(Self::Anyhow(any_err)) => err = any_err,
                Ok(err) => break err,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;

    use super::*;

    #[test]
    pub fn anyhow_flatten() {
        let err = Error::Anyhow(anyhow![Error::Anyhow(anyhow![Error::SliceFailure(0..0)])]);

        assert!(matches!(err.anyhow_flatten(), Error::SliceFailure(..)));
    }
}
