use crate::{IsReduce, Op, Relation};
use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

impl<D: Clone + Eq + Hash, C: Op<T = (D, isize)>> Relation<C> {
    pub fn distinct(self) -> Relation<impl Op<T = (D, isize)>> {
        self.map(|x| (x, ()))
            .reduce_(|_, _: &isize| ())
            .map(|(x, ())| x)
    }
}

impl<K: Clone + Eq + Hash, X: Eq + Hash, C: Op<T = ((K, X), isize)>> Relation<C> {
    pub fn reduce<Y: Clone + Eq, F: Fn(&K, &HashMap<X, isize>) -> Y>(
        self,
        f: F,
    ) -> Relation<impl IsReduce<T = ((K, Y), isize), OM = HashMap<K, Y>>> {
        self.reduce_::<HashMap<X, isize>, _, _>(f)
    }
}

impl<K: Clone + Eq + Hash, V: Clone + Ord, C: Op<T = ((K, V), isize)>> Relation<C> {
    pub fn group_min(self) -> Relation<impl IsReduce<T = ((K, V), isize), OM = HashMap<K, V>>> {
        self.reduce_(|_, m: &BTreeMap<V, isize>| m.first_key_value().unwrap().0.clone())
    }
    pub fn group_max(self) -> Relation<impl IsReduce<T = ((K, V), isize), OM = HashMap<K, V>>> {
        self.reduce_(|_, m: &BTreeMap<V, isize>| m.last_key_value().unwrap().0.clone())
    }
}
