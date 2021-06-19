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
    pub fn interrupt_nonempty<C: Op + 'a, F: Fn(&HashMap<C::D, isize>) -> I + 'a>(
        &mut self,
        rel: Relation<C>,
        f: F,
    ) where
        I: Clone + 'a,
        C::D: Eq + Hash,
    {
        self.interrupt(rel, move |m| if m.is_empty() { None } else { Some(f(m)) })
    }
}

impl<'a> CreationContext<'a, ()> {
    pub fn new() -> Self {
        Self::new_()
    }
}
