use std::{collections::BTreeMap, hash::Hash};

use crate::{Op, Relation};

impl<D: Clone + Eq + Hash, C: Op<T = (D, isize)>> Relation<C> {
    pub fn distinct(self) -> Relation<impl Op<T = (D, isize)>> {
        self.map(|x| (x, ()))
            .reduce(|_, _: &isize| ())
            .map(|(x, ())| x)
    }
}

impl<K: Clone + Eq + Hash, V: Clone + Ord, C: Op<T = ((K, V), isize)>> Relation<C> {
    pub fn group_min(self) -> Relation<impl Op<T = ((K, V), isize)>> {
        self.reduce(|_, m: &BTreeMap<V, isize>| m.first_key_value().unwrap().0.clone())
    }
    pub fn group_max(self) -> Relation<impl Op<T = ((K, V), isize)>> {
        self.reduce(|_, m: &BTreeMap<V, isize>| m.last_key_value().unwrap().0.clone())
    }
}
