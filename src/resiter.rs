use crate::ErrorVec;

pub trait ResultIterator<O, E>: Sized + Iterator<Item = Result<O, E>> {
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
