use std::{collections::HashMap, hash::Hash};

use crate::{
    core::{CreationContext, Op, Output, Relation},
    Dynamic, Save,
};

use super::map::{ExtremaMap, KVMap};

pub type DynamicOutput<'a, D, M = HashMap<D, isize>> = Output<D, Dynamic<'a, (D, isize)>, M>;
pub type CollectionOutput<'a, D, M = HashMap<D, isize>> =
    Output<D, Save<Dynamic<'a, (D, isize)>>, M>;

impl<C: Op> Relation<C> {
    pub fn into_output(self, context: &CreationContext) -> Output<C::D, C>
    where
        C::D: Eq + Hash,
    {
        self.into_output_(context)
    }
}

impl<K, V, C: Op<D = (K, V)>> Relation<C> {
    pub fn into_kv_output(self, context: &CreationContext) -> Output<(K, V), C, KVMap<K, V>>
    where
        K: Eq + Hash,
        V: Eq + Hash,
    {
        self.into_output_(context)
    }
    pub fn into_extrema_output(
        self,
        context: &CreationContext,
    ) -> Output<(K, V), C, ExtremaMap<K, V>>
    where
        K: Ord,
        V: Eq + Hash,
    {
        self.into_output_(context)
    }
}
