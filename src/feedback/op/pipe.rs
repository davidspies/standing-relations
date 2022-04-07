use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    hash::Hash,
    mem,
};

use crate::core::CountMap;

pub struct Pipe<T>(RefCell<Vec<T>>);

impl<T> Default for Pipe<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Pipe<T> {
    pub fn receive(&self) -> Vec<T> {
        mem::take(&mut self.0.borrow_mut())
    }
}

impl<D> CountMap<D> for Pipe<(D, isize)> {
    fn add(&mut self, k: D, count: isize) {
        self.0.borrow_mut().push((k, count));
    }
}

pub struct OrderedPipe<K, V>(RefCell<BTreeMap<K, HashMap<V, isize>>>);

impl<K, V> Default for OrderedPipe<K, V> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<K: Ord, V> OrderedPipe<K, V> {
    pub fn receive(&self) -> Option<(K, HashMap<V, isize>)> {
        self.0.borrow_mut().pop_first()
    }
}

impl<K: Ord, V: Eq + Hash> CountMap<(K, V)> for OrderedPipe<K, V> {
    fn add(&mut self, x: (K, V), count: isize) {
        self.0.borrow_mut().add(x, count)
    }
}
