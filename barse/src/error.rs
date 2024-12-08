//! [Error] type.

/// Crate error type used to report sink/source specific errors and general read/write errors.
#[derive(Debug, thiserror::Error)]
pub enum Error<E> {
    /// Wrapped source/sink error.
    #[error(transparent)]
    Wrapped(#[from] E),

    /// Some means of identifying a custom error by a code.
    #[error("Custom Error: {0}")]
    Custom(u64),

    /// Some means of identifying a custom error by reference.
    #[error("Custom Error: {:?}", .0)]
    Any(&'static (dyn ::core::any::Any + Send + Sync)),

    /// Some means of identifying a custom error as a message.
    #[error("{0}")]
    Message(&'static str),
}
