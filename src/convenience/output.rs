use std::hash::Hash;

use crate::{Op, Output, Relation};

impl<C: Op<T = (D, isize)>, D: Eq + Hash> Relation<C> {
    pub fn get_output(self) -> Output<D, C> {
        self.get_output_()
    }
}
