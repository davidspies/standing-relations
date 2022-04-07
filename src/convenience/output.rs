use std::hash::Hash;

use crate::core::{CreationContext, Op, Output, Relation};

use super::map::KVMap;

impl<C: Op> Relation<C> {
    pub fn get_output(self, context: &CreationContext) -> Output<C::D, C>
    where
        C::D: Eq + Hash,
    {
        self.get_output_(context)
    }
}

impl<K, V, C: Op<D = (K, V)>> Relation<C> {
    pub fn get_kv_output(self, context: &CreationContext) -> Output<(K, V), C, KVMap<K, V>>
    where
        K: Eq + Hash,
        V: Eq + Hash,
    {
        self.get_output_(context)
    }
}
