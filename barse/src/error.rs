//! Error utilities for crate.

use std::{any, array::TryFromSliceError, borrow::Cow, fmt::Debug, num::TryFromIntError};

use thiserror::Error;

/// Error type in use by crate.
#[derive(Debug, Error)]
pub enum Error {
    /// A flag of the specified type could not be found while parsing.
    #[error("could not find flag of type {0}")]
    FlagNotFound(&'static str),
    /// Reader cannot read flags.
    #[error("flags cannot be read using reader of this type {0}")]
    FlagReadUnsupported(&'static str),
    /// Slicing of input bytes failed, possibly due to invalid indices.
    #[error("a slice was not valid")]
    SliceFailure,
    /// A checked operation failed.
    #[error("a checked operation failed")]
    CheckedOperation,
    /// A flag had the right type but contents that do not meet the needs of parse returning error.
    #[error("an unsupported value in a flag vas given, hint \"{0}\", value {1:?}")]
    UnsupportedFlagValue(Cow<'static, str>, Box<dyn Debug + Send + Sync>),
    /// [crate::ByteRead::at][ByteRead::at] was called for a reader not supporting it.
    #[error("at is not supported for reader, {0}")]
    AtNotSupported(Cow<'static, str>),

    /// Forward of standard library error.
    #[error(transparent)]
    TryFromIntError(#[from] TryFromIntError),
    /// Forward of standard library error.
    #[error(transparent)]
    TryFromSliceError(#[from] TryFromSliceError),

    /// Anyhow catch-all
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl Error {
    /// Create a `FlagNotFound` instance with the content set by give type param.
    #[must_use]
    pub fn flag_not_found<T>() -> Self {
        Self::FlagNotFound(any::type_name::<T>())
    }

    /// If the error is anyhow it might still be of this type but obscured, this method downcasts
    /// the anyhow error as many times as it needs to to either confirm the anyhow error is not of
    /// [`enum@Error`] or that it is a not [`Error::Anyhow`].
    ///
    /// ```
    /// use barse::Error;
    /// use anyhow::anyhow;
    ///
    /// let err = Error::Anyhow(anyhow![Error::Anyhow(anyhow![Error::SliceFailure])]);
    ///
    /// assert!(matches!(err.anyhow_flatten(), Error::SliceFailure));
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
        let err = Error::Anyhow(anyhow![Error::Anyhow(anyhow![Error::SliceFailure])]);

        assert!(matches!(err.anyhow_flatten(), Error::SliceFailure));
    }
}
