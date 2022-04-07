use crate::{Op, Relation};

impl<C: Op<D = (L, R)>, L, R> Relation<C> {
    /// Splits a collection of 2-tuples into a 2-tuple of collections.
    ///
    /// Example:
    ///
    /// ```
    ///    use standing_relations::CreationContext;
    ///    use std::{collections::HashMap, iter::FromIterator};
    ///
    ///    let mut context = CreationContext::new();
    ///    let (mut foo_input, foo) = context.new_input::<usize>();
    ///    let (evens, odds) = foo.map(|x| (x * 2, x * 2 + 1)).split();
    ///    let evens = evens.into_output(&context);
    ///    let odds = odds.into_output(&context);
    ///    
    ///    let mut context = context.begin();
    ///    foo_input.add_all(&context, 0 .. 2);
    ///    context.commit();
    ///    assert_eq!(&*evens.get(&context), &HashMap::from_iter(vec![(0,1),(2,1)]));
    ///    assert_eq!(&*odds.get(&context), &HashMap::from_iter(vec![(1,1),(3,1)]));
    /// ```
    pub fn split(self) -> (Relation<impl Op<D = L>>, Relation<impl Op<D = R>>) {
        self.map_(|((lx, rx), count)| ((lx, count), (rx, count)))
            .hidden()
            .split_()
    }
}
