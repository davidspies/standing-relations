use crate::{core::Reduce, CountMap, IsReduce, Observable, Op, Relation};
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
        self.map_h(|x| (x, ()))
            .reduce_(|_, &n| n)
            .type_named("counts")
    }
    pub fn distinct(self) -> Relation<impl Op<D = C::D>> {
        self.map_h(|x| (x, ()))
            .reduce_(|_, _: &isize| ())
            .map_h(|(x, ())| x)
            .type_named("distinct")
    }
}

impl<C: Op<D = (K, V)>, K: Clone + Eq + Hash, V> Relation<C> {
    #[allow(clippy::type_complexity)]
    pub fn reduce_<M: CountMap<V> + Observable, Y: Clone + Eq, F: Fn(&K, &M) -> Y>(
        self,
        f: F,
    ) -> Relation<Reduce<K, V, C, M, Y, HashMap<K, Y>, F>> {
        self.reduce_with_output_(f)
    }
    pub fn reduce<Y: Clone + Eq>(
        self,
        f: impl Fn(&K, &HashMap<V, isize>) -> Y,
    ) -> Relation<impl IsReduce<T = ((K, Y), isize), OM = HashMap<K, Y>>>
    where
        V: Eq + Hash,
    {
        self.reduce_::<HashMap<V, isize>, _, _>(f)
    }
    pub fn group_min(self) -> Relation<impl IsReduce<T = ((K, V), isize), OM = HashMap<K, V>>>
    where
        V: Clone + Ord,
    {
        self.reduce_(|_, m: &BTreeMap<V, isize>| m.first_key_value().unwrap().0.clone())
            .type_named("group_min")
    }
    pub fn group_max(self) -> Relation<impl IsReduce<T = ((K, V), isize), OM = HashMap<K, V>>>
    where
        V: Clone + Ord,
    {
        self.reduce_(|_, m: &BTreeMap<V, isize>| m.last_key_value().unwrap().0.clone())
            .type_named("group_max")
    }
}
