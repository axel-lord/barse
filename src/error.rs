use std::{any, array::TryFromSliceError, borrow::Cow, fmt::Debug, num::TryFromIntError};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("could not find flag of type {0}")]
    FlagNotFound(&'static str),
    #[error("a slice was not valid")]
    SliceFailure,
    #[error("a checked operation failed")]
    CheckedOperation,
    #[error("an unsupported value in a flag vas given, hint \"{0}\", value {1:?}")]
    UnsupportedFlagValue(Cow<'static, str>, Box<dyn Debug + Send + Sync>),

    #[error(transparent)]
    TryFromIntError(#[from] TryFromIntError),
    #[error(transparent)]
    TryFromSliceError(#[from] TryFromSliceError),
}

impl Error {
    pub fn flag_not_found<T>() -> Self {
        Self::FlagNotFound(any::type_name::<T>())
    }
}
