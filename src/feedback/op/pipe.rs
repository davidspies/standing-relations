use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    mem,
};

use crate::core::CountMap;

use self::unordered::Unordered;

mod unordered;

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

pub struct OrderedPipe<K, V>(RefCell<BinaryHeap<(Reverse<K>, Unordered<(V, isize)>)>>);

impl<K: Ord, V> Default for OrderedPipe<K, V> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<K: Ord, V: Eq + Hash> OrderedPipe<K, V> {
    pub fn receive(&self) -> Option<(K, HashMap<V, isize>)> {
        let mut this = self.0.borrow_mut();
        let mut result = HashMap::new();
        let (Reverse(mut k), Unordered((v, count))) = this.pop()?;
        result.add(v, count);
        while let Some((Reverse(k2), _)) = this.peek() {
            if k2 > &k && !result.is_empty() {
                break;
            }
            let (Reverse(k2), Unordered((v, count))) = this.pop().unwrap();
            k = k2;
            result.add(v, count)
        }
        (!result.is_empty()).then_some((k, result))
    }
}

impl<K: Ord, V: Eq + Hash> CountMap<(K, V)> for OrderedPipe<K, V> {
    fn add(&mut self, (k, v): (K, V), count: isize) {
        self.0
            .borrow_mut()
            .push((Reverse(k), Unordered((v, count))));
    }
}
