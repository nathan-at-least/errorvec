use std::fmt;

#[derive(Debug, derive_more::From)]
pub struct ErrorVec<T>(Vec<T>);

impl<A> FromIterator<A> for ErrorVec<A> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A>,
    {
        ErrorVec(iter.into_iter().collect())
    }
}

impl<T> fmt::Display for ErrorVec<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, e) in self.0.iter().enumerate() {
            writeln!(f, "Error {}:\n{}\n", i, e)?;
        }
        Ok(())
    }
}
