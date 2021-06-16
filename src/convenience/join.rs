use crate::{Op, Relation};
use std::hash::Hash;

impl<K: Clone + Eq + Hash, V: Clone + Eq + Hash, C: Op<D = (K, V)>> Relation<C> {
    pub fn semijoin<C2: Op<D = K>>(self, other: Relation<C2>) -> Relation<impl Op<D = (K, V)>> {
        self.join(other.map(|x| (x, ()))).map(|(k, v, ())| (k, v))
    }
}

impl<C: Op> Relation<C>
where
    C::D: Clone + Eq + Hash,
{
    pub fn intersection<C2: Op<D = C::D>>(
        self,
        other: Relation<C2>,
    ) -> Relation<impl Op<D = C::D>> {
        self.map(|x| (x, ())).semijoin(other).map(|(x, ())| x)
    }
    pub fn set_minus<C2: Op<D = C::D>>(self, other: Relation<C2>) -> Relation<impl Op<D = C::D>> {
        self.map(|x| (x, ())).antijoin(other).map(|(x, ())| x)
    }
}
