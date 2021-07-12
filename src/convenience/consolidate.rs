use crate::{core::Consolidate, Op_, Relation};
use std::hash::Hash;

impl<C: Op_<T = (D, isize)>, D: Eq + Hash> Relation<C> {
    pub fn consolidate(self) -> Relation<Consolidate<C>> {
        self.consolidate_shown().hidden()
    }
}
