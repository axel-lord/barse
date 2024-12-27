//! Syn errors may be combined, this module provides a type for easily collecting syn
//! results/errors without short-circuiting.

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

    /// Convert into inner result.
    ///
    /// # Errors
    /// If any error was pushed to aggregate.
    pub fn into_inner(self) -> Result<Vec<T>, ::syn::Error> {
        self.inner
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
