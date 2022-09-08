use crate::ErrorVec;

/// Extend [Iterator] with `Item = Result<T, E>` to support gathering multiple errors.
///
/// # Example - Fail on First Error
///
/// One common pattern is processing an iterator over `Result` and propagating the first error
/// encountered which is facilitated in [std] with the [FromIterator] impl on [Result]:
///
/// ```
/// use std::path::Path;
///
/// fn read_paths_fail_fast<'a, I>(paths: I) -> std::io::Result<Vec<String>>
///     where I: Iterator<Item = &'a Path>,
/// {
///     paths.map(std::fs::read_to_string).collect()
/// }
/// ```
///
/// # Example - Gather all Errors
///
/// However, another common pattern is to gather all possible errors. This pattern is often useful
/// in user-facing error reporting, such as a compiler reporting all detected errors when building
/// a source project. [ResultIterator] along with [ErrorVec] streamline this pattern:
///
/// ```
/// use std::path::Path;
/// use errorvec::{ErrorVec, ResultIterator};
///
/// fn read_paths_gathering_all_errors<'a, I>(paths: I) -> Result<Vec<String>, ErrorVec<std::io::Error>>
///     where I: Iterator<Item = &'a Path>,
/// {
///     paths.map(std::fs::read_to_string).into_errorvec_result()
/// }
/// ```
pub trait ResultIterator<O, E>: Sized + Iterator<Item = Result<O, E>> {
    /// Gather all `Ok` and `Err` values, returning `Err` if there are 1 or more errors.
    fn into_errorvec_result(self) -> Result<Vec<O>, ErrorVec<E>> {
        let mut oks = vec![];
        let mut ev = ErrorVec::default();

        for result in self {
            if let Some(v) = ev.take_error(result) {
                oks.push(v);
            }
        }

        ev.into_result().map(|()| oks)
    }
}

impl<T, O, E> ResultIterator<O, E> for T where T: Sized + Iterator<Item = Result<O, E>> {}
