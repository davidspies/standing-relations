use crate::{Op, Relation};
use std::hash::Hash;

impl<K: Clone + Eq + Hash, V: Clone + Eq + Hash, C: Op<T = ((K, V), isize)>> Relation<C> {
    pub fn semijoin<C2: Op<T = (K, isize)>>(
        self,
        other: Relation<C2>,
    ) -> Relation<impl Op<T = ((K, V), isize)>> {
        self.join(other.map(|x| (x, ()))).map(|(k, v, ())| (k, v))
    }
}

impl<D: Clone + Eq + Hash, C: Op<T = (D, isize)>> Relation<C> {
    pub fn intersection<C2: Op<T = (D, isize)>>(
        self,
        other: Relation<C2>,
    ) -> Relation<impl Op<T = (D, isize)>> {
        self.map(|x| (x, ())).semijoin(other).map(|(x, ())| x)
    }
    pub fn set_minus<C2: Op<T = (D, isize)>>(
        self,
        other: Relation<C2>,
    ) -> Relation<impl Op<T = (D, isize)>> {
        self.map(|x| (x, ())).antijoin(other).map(|(x, ())| x)
    }
}
