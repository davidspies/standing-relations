use crate::{Op, Op_, Relation};
use std::{iter, ops::Neg};

impl<C: Op_> Relation<C> {
    pub fn map_<Y, F: Fn(C::T) -> Y>(self, f: F) -> Relation<impl Op_<T = Y>> {
        self.flat_map_(move |x| iter::once(f(x)))
    }
}

impl<C: Op> Relation<C> {
    pub fn flat_map<I: IntoIterator, F: Fn(C::D) -> I>(
        self,
        f: F,
    ) -> Relation<impl Op<D = I::Item>> {
        self.flat_map_(move |(x, count)| f(x).into_iter().map(move |y| (y, count)))
    }

    pub fn map<Y, F: Fn(C::D) -> Y>(self, f: F) -> Relation<impl Op<D = Y>> {
        self.flat_map(move |x| iter::once(f(x)))
    }

    pub fn filter<F: Fn(&C::D) -> bool>(self, f: F) -> Relation<impl Op<D = C::D>> {
        self.flat_map(move |x| if f(&x) { Some(x) } else { None })
    }

    pub fn map_counts<F: Fn(isize) -> isize>(self, f: F) -> Relation<impl Op<D = C::D>> {
        self.flat_map_(move |(x, count)| iter::once((x, f(count))))
    }

    pub fn negate(self) -> Relation<impl Op<D = C::D>> {
        self.map_counts(Neg::neg)
    }
}
