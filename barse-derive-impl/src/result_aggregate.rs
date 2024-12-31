//! Syn errors may be combined, this module provides a type for easily collecting syn
//! results/errors without short-circuiting.

use crate::opt::Opt;

/// Result aggregate either a vec of valid values or a combined error value.
/// Once a single error has been added, additions of valid values does nothing.
#[derive(Clone, Debug)]
pub struct ResAggr<T = ()> {
    /// Wrapped result representing aggregation state.
    inner: Result<Vec<T>, ::syn::Error>,
}

impl<T> ResAggr<T> {
    /// Construct a new empty instance.
    pub const fn new() -> Self {
        Self {
            inner: Ok(Vec::new()),
        }
    }

    /// Check for option conflict.
    pub fn conflict<A: Opt, B: Opt>(&mut self, a: &Option<A>, b: &Option<B>) -> &mut Self {
        if let (Some(a), Some(_)) = (a, b) {
            self.push_err(::syn::Error::new(
                a.kw_span(),
                format!(
                    "attributes '{}' and '{}' should not be used together",
                    A::name(),
                    B::name()
                ),
            ));
        };

        self
    }

    /// Add an error message indicating a required feature is missing if the option is set.
    pub fn requires_feature<O: Opt>(&mut self, feature: &str, opt: &Option<O>) -> &mut Self {
        if let Some(opt) = opt {
            self.push_err(::syn::Error::new(
                opt.kw_span(),
                format!("attribute '{}' requires the '{feature}' feature", O::name()),
            ));
        }
        self
    }

    /// Convert into inner result.
    ///
    /// # Errors
    /// If any error was pushed to aggregate.
    pub fn into_inner(self) -> Result<Vec<T>, ::syn::Error> {
        self.inner
    }

    /// Ad a result.
    pub fn push(&mut self, value: Result<T, ::syn::Error>) {
        match value {
            Ok(value) => self.push_value(value),
            Err(err) => self.push_err(err),
        }
    }

    /// Add an error.
    pub fn push_err(&mut self, err: ::syn::Error) {
        if let Err(inner) = &mut self.inner {
            inner.combine(err);
        } else {
            self.inner = Err(err);
        }
    }

    /// Add a value.
    pub fn push_value(&mut self, value: T) {
        if let Ok(values) = &mut self.inner {
            values.push(value);
        }
    }

    /// Merge two aggregators.
    pub fn merge(self, other: Self) -> Self {
        match (self.inner, other.inner) {
            // Only one is an error.
            (Ok(_), Err(err)) | (Err(err), Ok(_)) => Self { inner: Err(err) },

            // None are errors.
            (Ok(mut a), Ok(b)) => {
                a.extend(b);
                Self { inner: Ok(a) }
            }

            // Both are errors.
            (Err(mut a), Err(b)) => {
                a.combine(b);
                Self { inner: Err(a) }
            }
        }
    }
}

impl<T> Default for ResAggr<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<::syn::Error> for ResAggr<T> {
    fn from(value: ::syn::Error) -> Self {
        Self { inner: Err(value) }
    }
}

impl<T> From<Vec<T>> for ResAggr<T> {
    fn from(value: Vec<T>) -> Self {
        Self { inner: Ok(value) }
    }
}

impl<T> TryFrom<ResAggr<T>> for Vec<T> {
    type Error = ::syn::Error;

    fn try_from(value: ResAggr<T>) -> Result<Self, Self::Error> {
        value.into_inner()
    }
}

impl<T> FromIterator<Result<T, ::syn::Error>> for ResAggr<T> {
    fn from_iter<I: IntoIterator<Item = Result<T, ::syn::Error>>>(iter: I) -> Self {
        let mut agg = ResAggr::new();
        for item in iter {
            match item {
                Ok(value) => agg.push_value(value),
                Err(err) => agg.push_err(err),
            }
        }
        agg
    }
}

impl<T> FromIterator<::syn::Error> for ResAggr<T> {
    fn from_iter<I: IntoIterator<Item = ::syn::Error>>(iter: I) -> Self {
        let mut agg = ResAggr::new();
        for item in iter {
            agg.push_err(item);
        }
        agg
    }
}

impl<T> FromIterator<ResAggr<T>> for ResAggr<T> {
    fn from_iter<I: IntoIterator<Item = ResAggr<T>>>(iter: I) -> Self {
        iter.into_iter()
            .reduce(ResAggr::merge)
            .unwrap_or(ResAggr::new())
    }
}
