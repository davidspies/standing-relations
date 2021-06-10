use std::{
    collections::{btree_map, hash_map, BTreeMap, HashMap},
    hash::Hash,
};

pub enum InsertResult<T> {
    NoOldValue,
    OldValue(T),
    Unchanged,
}

pub trait OutputMap<K, V> {
    fn remove(&mut self, k: &K) -> Option<V>;
    fn insert_if_different(&mut self, k: K, v: V) -> InsertResult<V>;
}

impl<K: Eq + Hash, V: Eq> OutputMap<K, V> for HashMap<K, V> {
    fn remove(&mut self, k: &K) -> Option<V> {
        HashMap::remove(self, k)
    }
    fn insert_if_different(&mut self, k: K, v: V) -> InsertResult<V> {
        match self.entry(k) {
            hash_map::Entry::Vacant(vac) => {
                vac.insert(v);
                InsertResult::NoOldValue
            }
            hash_map::Entry::Occupied(mut occ) => {
                if occ.get() == &v {
                    InsertResult::Unchanged
                } else {
                    InsertResult::OldValue(occ.insert(v))
                }
            }
        }
    }
}

impl<K: Ord, V: Eq> OutputMap<K, V> for BTreeMap<K, V> {
    fn remove(&mut self, k: &K) -> Option<V> {
        BTreeMap::remove(self, k)
    }
    fn insert_if_different(&mut self, k: K, v: V) -> InsertResult<V> {
        match self.entry(k) {
            btree_map::Entry::Vacant(vac) => {
                vac.insert(v);
                InsertResult::NoOldValue
            }
            btree_map::Entry::Occupied(mut occ) => {
                if occ.get() == &v {
                    InsertResult::Unchanged
                } else {
                    InsertResult::OldValue(occ.insert(v))
                }
            }
        }
    }
}
