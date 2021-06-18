use crate::{CreationContext, Input, Op, Relation};
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
    pub fn interrupt_nonempty<C: Op + 'a>(&mut self, rel: Relation<C>, i: I)
    where
        I: Clone + 'a,
        C::D: Eq + Hash,
    {
        self.interrupt(
            rel,
            move |m| if m.is_empty() { None } else { Some(i.clone()) },
        )
    }
    pub fn feed<C: Op + 'a>(&mut self, rel: Relation<C>, input: Input<'a, C::D>)
    where
        C::D: Clone + Eq + Hash + 'a,
    {
        self.feed_and(rel, input, |_| ())
    }
}

impl<'a> CreationContext<'a, ()> {
    pub fn new() -> Self {
        Self::new_()
    }
}
