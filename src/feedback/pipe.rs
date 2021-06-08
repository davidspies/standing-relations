use std::{cell::RefCell, mem};

use crate::CountMap;

pub struct Pipe<T>(RefCell<Vec<T>>);

impl<T> Pipe<T> {
    pub fn take(&self) -> Vec<T> {
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
