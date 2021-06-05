use crate::{Op, Relation};

pub struct FlatMap<C: Op, I: IntoIterator, F: Fn(C::T) -> I> {
    inner: C,
    f: F,
}

impl<C: Op, I: IntoIterator, F: Fn(C::T) -> I> Op for FlatMap<C, I, F> {
    type T = I::Item;

    fn foreach<'a, G: FnMut(Self::T) + 'a>(&'a mut self, mut continuation: G) {
        let FlatMap { inner, f } = self;
        inner.foreach(|x| {
            for y in f(x) {
                continuation(y)
            }
        })
    }
}

impl<C: Op> Relation<C> {
    pub fn flat_map_<I: IntoIterator, F: Fn(C::T) -> I>(self, f: F) -> Relation<FlatMap<C, I, F>> {
        Relation {
            context_id: self.context_id,
            dirty: self.dirty,
            inner: FlatMap {
                inner: self.inner,
                f,
            },
        }
    }
}
