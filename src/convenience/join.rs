use std::hash::Hash;

use crate::{pair::Pair, Op, Relation};

impl<K: Clone + Eq + Hash, V: Clone + Eq + Hash, C: Op<D = (K, V)>> Relation<C> {
    pub fn semijoin(self, other: Relation<impl Op<D = K>>) -> Relation<impl Op<D = (K, V)>> {
        self.join(other.map_h(|x| (x, ())))
            .map_h(|(k, v, ())| (k, v))
            .type_named("semijoin")
    }
    pub fn join_values<V2: Clone + Eq + Hash>(
        self,
        other: Relation<impl Op<D = (K, V2)>>,
    ) -> Relation<impl Op<D = (V, V2)>> {
        self.join(other).map_h(|(_, v1, v2)| (v1, v2))
    }
    // TODO Make this native rather than a convenience function
    pub fn left_join<V2: Clone + Eq + Hash>(
        self,
        other: Relation<impl Op<D = (K, V2)>>,
    ) -> Relation<impl Op<D = (K, V, Option<V2>)>> {
        let self_saved = self.save();
        let other_saved = other.save();
        self_saved
            .get()
            .join(other_saved.get())
            .map_h(|(k, l, r)| (k, l, Some(r)))
            .concat(
                self_saved
                    .get()
                    .antijoin(other_saved.get().map(Pair::fst))
                    .map(|(k, l)| (k, l, None)),
            )
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
