use std::fmt;

/// A newtype wrapper around `Vec<E>` aimed at supporting multi-error scenarios.
#[derive(Debug, derive_more::From, derive_more::Into)]
pub struct ErrorVec<E>(Vec<E>);

impl<E> ErrorVec<E> {
    /// Iterate over the contained errors.
    pub fn iter(&self) -> impl Iterator<Item = &E> {
        self.0.iter()
    }

    /// Push an error onto the end.
    pub fn push(&mut self, error: E) {
        self.0.push(error);
    }

    /// If `self` is empty, `Ok(())`, else, `Err(self)`.
    pub fn into_result(self) -> Result<(), Self> {
        if self.0.is_empty() {
            Ok(())
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
