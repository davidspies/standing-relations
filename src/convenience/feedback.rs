use crate::{CreationContext, Op, Output};
use std::{collections::HashMap, hash::Hash};

impl<'a, I: 'a> CreationContext<'a, I> {
    pub fn interrupt<D: Eq + Hash + 'a>(
        &mut self,
        output: Output<D, impl Op<D = D> + 'a>,
        f: impl Fn(&HashMap<D, isize>) -> I + 'a,
    ) {
        self.interrupt_(output, move |m| Some(f(m)))
    }
}

impl<'a> CreationContext<'a, ()> {
    pub fn new() -> Self {
        Self::new_()
    }
}
