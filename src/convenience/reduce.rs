use crate::{IsReduce, Op, Relation};
use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

impl<C: Op> Relation<C>
where
    C::D: Clone + Eq + Hash,
{
    pub fn counts(
        self,
    ) -> Relation<impl IsReduce<T = ((C::D, isize), isize), OM = HashMap<C::D, isize>>> {
        self.map(|x| (x, ())).reduce_(|_, &n| n)
    }
    pub fn distinct(self) -> Relation<impl Op<D = C::D>> {
        self.counts().map(|(x, _)| x)
    }
}

impl<K: Clone + Eq + Hash, X: Eq + Hash, C: Op<D = (K, X)>> Relation<C> {
    pub fn reduce<Y: Clone + Eq>(
        self,
        f: impl Fn(&K, &HashMap<X, isize>) -> Y,
    ) -> Relation<impl IsReduce<T = ((K, Y), isize), OM = HashMap<K, Y>>> {
        self.reduce_::<HashMap<X, isize>, _, _>(f)
    }
}

impl<K: Clone + Eq + Hash, V: Clone + Ord, C: Op<D = (K, V)>> Relation<C> {
    pub fn group_min(self) -> Relation<impl IsReduce<T = ((K, V), isize), OM = HashMap<K, V>>> {
        self.reduce_(|_, m: &BTreeMap<V, isize>| m.first_key_value().unwrap().0.clone())
    }
    pub fn group_max(self) -> Relation<impl IsReduce<T = ((K, V), isize), OM = HashMap<K, V>>> {
        self.reduce_(|_, m: &BTreeMap<V, isize>| m.last_key_value().unwrap().0.clone())
    }
}
