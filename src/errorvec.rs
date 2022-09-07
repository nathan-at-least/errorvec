use std::fmt;

#[derive(Debug, derive_more::From, derive_more::Into)]
pub struct ErrorVec<E>(Vec<E>);

impl<E> ErrorVec<E> {
    pub fn iter(&self) -> impl Iterator<Item = &E> {
        self.0.iter()
    }
}

impl<E> std::error::Error for ErrorVec<E> where E: fmt::Display + fmt::Debug {}

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
        for (i, e) in self.0.iter().enumerate() {
            writeln!(f, "Error {}:\n{}\n", i, e)?;
        }
        Ok(())
    }
}

