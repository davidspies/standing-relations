use std::hash::Hash;

use crate::{Op, Relation};

impl<K: Clone + Eq + Hash, V: Clone + Eq + Hash, C: Op<T = ((K, V), isize)>> Relation<C> {
    pub fn semijoin<C2: Op<T = (K, isize)>>(
        self,
        other: Relation<C2>,
    ) -> Relation<impl Op<T = ((K, V), isize)>> {
        self.join(other.map(|x| (x, ()))).map(|(k, v, ())| (k, v))
    }
}
