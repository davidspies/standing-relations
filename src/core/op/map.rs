use crate::core::{Op_, Relation};

pub struct FlatMap<C: Op_, I: IntoIterator, F: Fn(C::T) -> I> {
    inner: C,
    f: F,
}

impl<C: Op_, I: IntoIterator, F: Fn(C::T) -> I> Op_ for FlatMap<C, I, F> {
    type T = I::Item;

    fn foreach<'a>(&'a mut self, mut continuation: impl FnMut(Self::T) + 'a) {
        let FlatMap { inner, f } = self;
        inner.foreach(|x| {
            for y in f(x) {
                continuation(y)
            }
        })
    }
}

impl<C: Op_> Relation<C> {
    pub fn flat_map_<I: IntoIterator, F: Fn(C::T) -> I>(self, f: F) -> Relation<FlatMap<C, I, F>> {
        Relation {
            context_tracker: self.context_tracker,
            dirty: self.dirty,
            inner: FlatMap {
                inner: self.inner,
                f,
            },
        }
    }
}
