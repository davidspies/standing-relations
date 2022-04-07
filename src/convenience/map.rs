use std::{collections::HashMap, convert::identity, fmt::Debug, iter, ops::Neg};

use crate::{pair::Pair, Op, Op_, Relation};

pub type KVMap<K, V> = HashMap<K, HashMap<V, isize>>;

impl<C: Op_> Relation<C> {
    pub fn map_<Y, F: Fn(C::T) -> Y>(self, f: F) -> Relation<impl Op_<T = Y>> {
        self.flat_map_(move |x| iter::once(f(x)))
    }
    pub fn debug(self, name: &'static str) -> Relation<impl Op_<T = C::T>>
    where
        C::T: Debug,
    {
        self.map_(move |x| {
            log::debug!("{}: {:?}", name, x);
            x
        })
        .hidden()
    }
}

impl<C: Op> Relation<C> {
    pub fn flat_map<I: IntoIterator>(
        self,
        f: impl Fn(C::D) -> I,
    ) -> Relation<impl Op<D = I::Item>> {
        self.flat_map_(move |(x, count)| f(x).into_iter().map(move |y| (y, count)))
    }
    pub fn flatten(self) -> Relation<impl Op<D = <C::D as IntoIterator>::Item>>
    where
        C::D: IntoIterator,
    {
        self.flat_map(identity).type_named("flatten")
    }
    pub fn map<Y>(self, f: impl Fn(C::D) -> Y) -> Relation<impl Op<D = Y>> {
        self.flat_map(move |x| iter::once(f(x))).type_named("map")
    }

    pub fn map_h<Y>(self, f: impl Fn(C::D) -> Y) -> Relation<impl Op<D = Y>> {
        self.map(f).hidden()
    }

    pub fn filter(self, f: impl Fn(&C::D) -> bool) -> Relation<impl Op<D = C::D>> {
        self.flat_map(move |x| if f(&x) { Some(x) } else { None })
            .type_named("filter")
    }

    pub fn map_counts(self, f: impl Fn(isize) -> isize) -> Relation<impl Op<D = C::D>> {
        self.flat_map_(move |(x, count)| iter::once((x, f(count))))
            .type_named("map_counts")
    }

    pub fn negate(self) -> Relation<impl Op<D = C::D>> {
        self.map_counts(Neg::neg).type_named("negate")
    }
}

impl<A, B, C: Op<D = (A, B)>> Relation<C> {
    pub fn fsts(self) -> Relation<impl Op<D = A>> {
        self.map_h(Pair::fst)
    }
    pub fn snds(self) -> Relation<impl Op<D = B>> {
        self.map_h(Pair::snd)
    }
    pub fn swaps(self) -> Relation<impl Op<D = (B, A)>> {
        self.map_h(Pair::swap)
    }
}
