use std::{iter, ops::Neg};

use crate::{Op, Relation};

impl<C: Op> Relation<C> {
    pub fn map_<Y, F: Fn(C::T) -> Y>(self, f: F) -> Relation<impl Op<T = Y>> {
        self.flat_map_(move |x| iter::once(f(x)))
    }
}

impl<D, C: Op<T = (D, isize)>> Relation<C> {
    pub fn flat_map<I: IntoIterator, F: Fn(D) -> I>(
        self,
        f: F,
    ) -> Relation<impl Op<T = (I::Item, isize)>> {
        self.flat_map_(move |(x, count)| f(x).into_iter().map(move |y| (y, count)))
    }

    pub fn map<Y, F: Fn(D) -> Y>(self, f: F) -> Relation<impl Op<T = (Y, isize)>> {
        self.flat_map(move |x| iter::once(f(x)))
    }

    pub fn filter<F: Fn(&D) -> bool>(self, f: F) -> Relation<impl Op<T = (D, isize)>> {
        self.flat_map(move |x| if f(&x) { Some(x) } else { None })
    }

    pub fn map_counts<F: Fn(isize) -> isize>(self, f: F) -> Relation<impl Op<T = (D, isize)>> {
        self.flat_map_(move |(x, count)| iter::once((x, f(count))))
    }

    pub fn negate(self) -> Relation<impl Op<T = (D, isize)>> {
        self.map_counts(Neg::neg)
    }
}
