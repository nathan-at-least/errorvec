use crate::ErrorVec;

pub trait ResultIterator<O, E>: Sized + Iterator<Item = Result<O, E>> {
    fn into_errorvec_result(self) -> Result<Vec<O>, ErrorVec<E>> {
        let (oks, errs) = self.aggregate_results();
        if errs.is_empty() {
            Ok(oks)
        } else {
            Err(ErrorVec::from(errs))
        }
    }

    fn aggregate_results(self) -> (Vec<O>, Vec<E>) {
        let mut oks = vec![];
        let mut errs = vec![];

        for result in self {
            match result {
                Ok(o) => oks.push(o),
                Err(e) => errs.push(e),
            }
        }

        (oks, errs)
    }
}

impl<T, O, E> ResultIterator<O, E> for T where T: Sized + Iterator<Item = Result<O, E>> {}
