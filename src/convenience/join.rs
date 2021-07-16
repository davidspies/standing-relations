use crate::{Op, Relation};
use std::hash::Hash;

impl<K: Clone + Eq + Hash, V: Clone + Eq + Hash, C: Op<D = (K, V)>> Relation<C> {
    pub fn semijoin(self, other: Relation<impl Op<D = K>>) -> Relation<impl Op<D = (K, V)>> {
        self.join(other.map_h(|x| (x, ())))
            .map_h(|(k, v, ())| (k, v))
            .type_named("semijoin")
    }
}

impl<C: Op> Relation<C>
where
    C::D: Clone + Eq + Hash,
{
    pub fn intersection(self, other: Relation<impl Op<D = C::D>>) -> Relation<impl Op<D = C::D>> {
        self.map_h(|x| (x, ()))
            .semijoin(other)
            .map_h(|(x, ())| x)
            .type_named("intersection")
    }
    pub fn set_minus(self, other: Relation<impl Op<D = C::D>>) -> Relation<impl Op<D = C::D>> {
        self.map_h(|x| (x, ()))
            .antijoin(other)
            .map_h(|(x, ())| x)
            .type_named("set_minus")
    }
}

impl<K: Clone + Eq + Hash, V1: Clone + Eq + Hash, C: Op<D = (K, V1)>> Relation<C> {
    pub fn join_values<V2: Clone + Eq + Hash>(
        self,
        other: Relation<impl Op<D = (K, V2)>>,
    ) -> Relation<impl Op<D = (V1, V2)>> {
        self.join(other).map_h(|(_, v1, v2)| (v1, v2))
    }
}
