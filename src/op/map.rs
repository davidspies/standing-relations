use std::iter;

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
    pub fn flat_map<I: IntoIterator, F: Fn(C::T) -> I>(self, f: F) -> Relation<FlatMap<C, I, F>> {
        Relation {
            context_id: self.context_id,
            dirty: self.dirty,
            inner: FlatMap {
                inner: self.inner,
                f,
            },
        }
    }
    pub fn map<'a, Y: 'a, F: Fn(C::T) -> Y + 'a>(self, f: F) -> Relation<impl Op<T = Y> + 'a>
    where
        C: 'a,
    {
        self.flat_map(move |x| iter::once(f(x)))
    }
    pub fn filter<'a, F: Fn(&C::T) -> bool + 'a>(self, f: F) -> Relation<impl Op<T = C::T> + 'a>
    where
        C: 'a,
    {
        self.flat_map(move |x| if f(&x) { Some(x) } else { None })
    }
}
