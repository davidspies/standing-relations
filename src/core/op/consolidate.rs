use crate::core::{CountMap, Op, Op_, Relation};
use std::{collections::HashMap, hash::Hash};

pub struct Consolidate<C: Op_>(C);

impl<C: Op> Op_ for Consolidate<C>
where
    C::D: Eq + Hash,
{
    type T = (C::D, isize);

    fn foreach<'a>(&'a mut self, mut continuation: impl FnMut(Self::T) + 'a) {
        let mut m = HashMap::new();
        self.0.foreach(|(x, count)| m.add(x, count));
        for x in m {
            continuation(x)
        }
    }
}

impl<C: Op_<T = (D, isize)>, D: Eq + Hash> Relation<C> {
    pub fn consolidate(self) -> Relation<Consolidate<C>> {
        Relation {
            context_id: self.context_id,
            dirty: self.dirty,
            inner: Consolidate(self.inner),
        }
    }
}
