//! Error utilities for crate.

use std::{any, array::TryFromSliceError, borrow::Cow, fmt::Debug, num::TryFromIntError};

use thiserror::Error;

/// Error type in use by crate.
#[derive(Debug, Error)]
pub enum Error {
    /// A flag of the specified type could not be found while parsing.
    #[error("could not find flag of type {0}")]
    FlagNotFound(&'static str),
    /// Slicing of input bytes failed, possibly due to invalid indices.
    #[error("a slice was not valid")]
    SliceFailure,
    /// A checked operation failed.
    #[error("a checked operation failed")]
    CheckedOperation,
    /// A flag had the right type but contents that do not meet the needs of parse returning error.
    #[error("an unsupported value in a flag vas given, hint \"{0}\", value {1:?}")]
    UnsupportedFlagValue(Cow<'static, str>, Box<dyn Debug + Send + Sync>),
    /// [crate::ByteRead::at] was called for a reader not supporting it.
    #[error("at is not supported for reader, {0}")]
    AtNotSupported(Cow<'static, str>),

    /// Forward of standard library error.
    #[error(transparent)]
    TryFromIntError(#[from] TryFromIntError),
    /// Forward of standard library error.
    #[error(transparent)]
    TryFromSliceError(#[from] TryFromSliceError),
}

impl Error {
    /// Create a `flag_not_found` instance with the content set by give type param.
    #[must_use]
    pub fn flag_not_found<T>() -> Self {
        Self::FlagNotFound(any::type_name::<T>())
    }
}
