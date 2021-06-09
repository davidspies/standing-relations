use crate::{CountMap, Op, Relation};
use std::{collections::HashMap, hash::Hash};

pub struct Consolidate<C: Op>(C);

impl<D: Eq + Hash, C: Op<T = (D, isize)>> Op for Consolidate<C> {
    type T = (D, isize);

    fn foreach<'a, F: FnMut(Self::T) + 'a>(&'a mut self, mut continuation: F) {
        let mut m = HashMap::new();
        self.0.foreach(|(x, count)| m.add(x, count));
        for x in m {
            continuation(x)
        }
    }
}

impl<C: Op<T = (D, isize)>, D: Eq + Hash> Relation<C> {
    pub fn consolidate(self) -> Relation<Consolidate<C>> {
        Relation {
            context_id: self.context_id,
            dirty: self.dirty,
            inner: Consolidate(self.inner),
        }
    }
}
