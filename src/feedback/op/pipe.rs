use crate::CountMap;
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    hash::Hash,
    mem,
};

pub struct Pipe<T>(RefCell<Vec<T>>);

impl<T> Pipe<T> {
    pub fn receive(&self) -> Vec<T> {
        mem::take(&mut self.0.borrow_mut())
    }
}

impl<D> CountMap<D> for Pipe<(D, isize)> {
    fn add(&mut self, k: D, count: isize) {
        self.0.borrow_mut().push((k, count));
    }

    fn empty() -> Self {
        Pipe(RefCell::new(Vec::new()))
    }
}

pub struct OrderedPipe<K, V>(RefCell<BTreeMap<K, HashMap<V, isize>>>);

impl<K: Ord, V> OrderedPipe<K, V> {
    pub fn receive(&self) -> Option<(K, HashMap<V, isize>)> {
        self.0.borrow_mut().pop_first()
    }
}

impl<K: Ord, V: Eq + Hash> CountMap<(K, V)> for OrderedPipe<K, V> {
    fn add(&mut self, x: (K, V), count: isize) {
        self.0.borrow_mut().add(x, count)
    }

    fn empty() -> Self {
        OrderedPipe(RefCell::new(BTreeMap::new()))
    }
}
