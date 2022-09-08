use std::fmt;

/// A newtype wrapper around `Vec<E>` aimed at supporting multi-error scenarios.
///
/// # `Display`
///
/// [ErrorVec] implements [std::fmt::Display] by prepending each error's display with `[error K of N]`.
///
/// # `ResultIterator` usage
///
/// A common usage is via
/// [ResultIterator::into_errorvec_result](crate::ResultIterator::into_errorvec_result) for gathering
/// all errors in an [Iterator] over [Result] values.
///
/// # Empty `ErrorVec`
///
/// An [ErrorVec] containing no values (ie `ErrorVec::is_empty() == true`) typically does not
/// represent an error, and the [ErrorVec::into_result] and [ErrorVec::into_result_with] are often
/// useful in this case:
///
/// ```
/// use errorvec::ErrorVec;
///
/// let ev: ErrorVec<()> = ErrorVec::default();
/// assert!(ev.into_result().is_ok());
/// ```
///
/// # Example - Gathering errors with `take_error` and `into_result_with`
///
/// For scenarios where [ResultIterator](crate::ResultIterator) isn't
/// useful, the [ErrorVec::take_error] and [ErrorVec::into_result_with]
/// methods may be useful.
///
/// ```
/// use std::path::Path;
/// use errorvec::ErrorVec;
///
/// /// Return the string contents of all the paths listed in the `manifest_file`, reporting all
/// /// errors encountered.
/// fn read_manifest_files(manifest_file: &Path) -> Result<Vec<String>, ErrorVec<std::io::Error>> {
///     use std::fs::read_to_string;
///
///     let mut contents = vec![];
///     let mut errs = ErrorVec::default();
///
///     if let Some(manifest) = errs.take_error(read_to_string(manifest_file)) {
///         for line in manifest.lines() {
///             let path = &Path::new(line.trim_end());
///             if let Some(content) = errs.take_error(read_to_string(path)) {
///                 contents.push(content)
///             }
///         }
///     }
///
///     errs.into_result_with(contents)
/// }
/// ```
#[derive(Debug, derive_more::From, derive_more::Into)]
pub struct ErrorVec<E>(Vec<E>);

impl<E> ErrorVec<E> {
    /// An empty `ErrorVec` contains no errors.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Iterate over the contained errors.
    pub fn iter(&self) -> impl Iterator<Item = &E> {
        self.0.iter()
    }

    /// Push an error onto the end.
    pub fn push(&mut self, error: E) {
        self.0.push(error);
    }

    /// If `self.is_empty()`, signifying no errors, `Ok(())`, else, `Err(self)`.
    pub fn into_result(self) -> Result<(), Self> {
        self.into_result_with(())
    }

    /// If `self.is_empty()`, signifying no errors, `Ok(value)`, else, `Err(self)`.
    pub fn into_result_with<T>(self, value: T) -> Result<T, Self> {
        if self.is_empty() {
            Ok(value)
        } else {
            Err(self)
        }
    }

    /// Collect the error from a result, if present, otherwise return the `Ok` value.
    pub fn take_error<T>(&mut self, r: Result<T, E>) -> Option<T> {
        match r {
            Ok(x) => Some(x),
            Err(e) => {
                self.push(e);
                None
            }
        }
    }
}

impl<E> std::error::Error for ErrorVec<E> where E: fmt::Display + fmt::Debug {}

impl<E> Default for ErrorVec<E> {
    fn default() -> Self {
        ErrorVec(vec![])
    }
}

impl<E> FromIterator<E> for ErrorVec<E> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = E>,
    {
        ErrorVec(iter.into_iter().collect())
    }
}

impl<E> IntoIterator for ErrorVec<E> {
    type Item = E;
    type IntoIter = <Vec<E> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<E> fmt::Display for ErrorVec<E>
where
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let total = self.0.len();
        for (i, e) in self.0.iter().enumerate() {
            let edisp = e.to_string();
            writeln!(f, "[error {} of {}] {}", i + 1, total, edisp.trim_end())?;
            if i + 1 < total {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
