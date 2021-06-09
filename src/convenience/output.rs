use crate::{Op, Output, Relation};
use std::hash::Hash;

impl<C: Op<T = (D, isize)>, D> Relation<C> {
    pub fn get_output(self) -> Output<D, C>
    where
        D: Eq + Hash,
    {
        self.get_output_()
    }
}