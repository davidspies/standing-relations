use std::hash::Hash;

use crate::{CreationContext, Op, Relation};

impl<'a> CreationContext<'a> {
    pub fn new() -> Self {
        Self::new_()
    }
}

impl<'a, K: Clone + Eq + Hash + 'a, V: Clone + Eq + Hash + 'a, C: Op<D = (K, V)> + 'a> Relation<C> {
    pub fn first_occurences<I>(
        self,
        context: &mut CreationContext<'a, I>,
    ) -> Relation<impl Op<D = (K, V)>> {
        let (input, input_rel) = context.new_trackable_input();
        let input_rel = input_rel.save();
        let new_occurences = self.antijoin(input_rel.get().fsts()).into_output(context);
        context.feed_while(new_occurences, input);
        input_rel.get()
    }
}
