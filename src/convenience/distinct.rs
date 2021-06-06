use std::hash::Hash;

use crate::{Op, Relation};

impl<D: Clone + Eq + Hash, C: Op<T = (D, isize)>> Relation<C> {
    pub fn distinct(self) -> Relation<impl Op<T = (D, isize)>> {
        self.map(|x| (x, ()))
            .reduce(|_, _: &isize| ())
            .map(|(x, ())| x)
    }
}
