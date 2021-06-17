use crate::{CreationContext, Op, Relation};
use std::{collections::HashMap, hash::Hash};

impl<'a, I> CreationContext<'a, I> {
    pub fn interrupt<C: Op + 'a, F: Fn(&HashMap<C::D, isize>) -> Option<I> + 'a>(
        &mut self,
        rel: Relation<C>,
        f: F,
    ) where
        I: 'a,
        C::D: Eq + Hash,
    {
        self.interrupt_(rel, f)
    }
}

impl<'a> CreationContext<'a, ()> {
    pub fn new() -> Self {
        Self::new_()
    }
}
