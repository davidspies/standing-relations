use std::{
    collections::{btree_map, hash_map, BTreeMap, HashMap},
    hash::Hash,
};

pub trait CountMap<K>: Sized {
    fn add(&mut self, k: K, count: isize);
    fn is_empty(&self) -> bool;
    fn empty() -> Self;
    fn singleton(k: K, count: isize) -> Self {
        let mut result: Self = Self::empty();
        result.add(k, count);
        result
    }
}

impl<K1: Eq + Hash, K2, M: CountMap<K2>> CountMap<(K1, K2)> for HashMap<K1, M> {
    fn add(&mut self, k: (K1, K2), count: isize) {
        if count == 0 {
            return;
        }
        let e = self.entry(k.0);
        match e {
            hash_map::Entry::Vacant(v) => {
                v.insert(CountMap::singleton(k.1, count));
            }
            hash_map::Entry::Occupied(mut o) => {
                let m = o.get_mut();
                m.add(k.1, count);
                if m.is_empty() {
                    o.remove();
                }
            }
        }
    }

    fn is_empty(&self) -> bool {
        HashMap::is_empty(self)
    }

    fn empty() -> Self {
        HashMap::new()
    }
}

impl<K1: Eq + Ord, K2, M: CountMap<K2>> CountMap<(K1, K2)> for BTreeMap<K1, M> {
    fn add(&mut self, k: (K1, K2), count: isize) {
        if count == 0 {
            return;
        }
        let e = self.entry(k.0);
        match e {
            btree_map::Entry::Vacant(v) => {
                v.insert(CountMap::singleton(k.1, count));
            }
            btree_map::Entry::Occupied(mut o) => {
                let m = o.get_mut();
                m.add(k.1, count);
                if m.is_empty() {
                    o.remove();
                }
            }
        }
    }

    fn is_empty(&self) -> bool {
        BTreeMap::is_empty(self)
    }

    fn empty() -> Self {
        BTreeMap::new()
    }
}

impl CountMap<()> for isize {
    fn add(&mut self, (): (), count: isize) {
        *self += count
    }

    fn is_empty(&self) -> bool {
        *self == 0
    }

    fn empty() -> Self {
        0
    }
}

impl<K: Eq + Hash> CountMap<K> for HashMap<K, isize> {
    fn add(&mut self, k: K, count: isize) {
        CountMap::<(K, ())>::add(self, (k, ()), count)
    }

    fn is_empty(&self) -> bool {
        CountMap::<(K, ())>::is_empty(self)
    }

    fn empty() -> Self {
        CountMap::<(K, ())>::empty()
    }
}

impl<K: Eq + Ord> CountMap<K> for BTreeMap<K, isize> {
    fn add(&mut self, k: K, count: isize) {
        CountMap::<(K, ())>::add(self, (k, ()), count)
    }

    fn is_empty(&self) -> bool {
        CountMap::<(K, ())>::is_empty(self)
    }

    fn empty() -> Self {
        CountMap::<(K, ())>::empty()
    }
}
