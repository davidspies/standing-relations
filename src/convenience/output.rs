use crate::{core::CreationContext, Op, Output, Relation};
use std::hash::Hash;

impl<C: Op> Relation<C> {
    pub fn get_output(self, context: &CreationContext) -> Output<C::D, C>
    where
        C::D: Eq + Hash,
    {
        self.get_output_(context)
    }
}
