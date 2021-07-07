use crate::{Op, Relation};

impl<C: Op<D = (LI, RI)>, LI: IntoIterator, RI: IntoIterator> Relation<C> {
    /// Splits a collection of 2-tuples into a 2-tuple of collections.
    ///
    /// Note that the argument's elements are iterators which get flat-mapped into the outputs.
    ///
    /// Example:
    ///
    /// ```
    ///    use standing_relations::CreationContext;
    ///    use std::{collections::HashMap, iter::FromIterator};
    ///
    ///    let context = CreationContext::new();
    ///    let (foo_input, foo) = context.new_input::<usize>();
    ///    let (evens, odds) = foo.map(|x| if x % 2 == 0 {(Some(x), None)} else {(None, Some(x))}).split();
    ///    let evens = evens.get_output(&context);
    ///    let odds = odds.get_output(&context);
    ///    
    ///    let mut context = context.begin();
    ///    foo_input.add_all(&context, 1 ..= 4);
    ///    context.commit();
    ///    assert_eq!(&*evens.get(&context), &HashMap::from_iter(vec![(2,1),(4,1)]));
    ///    assert_eq!(&*odds.get(&context), &HashMap::from_iter(vec![(1,1),(3,1)]));
    /// ```
    pub fn split(
        self,
    ) -> (
        Relation<impl Op<D = LI::Item>>,
        Relation<impl Op<D = RI::Item>>,
    ) {
        self.map_(|((lx, rx), count)| {
            (
                lx.into_iter().map(move |l| (l, count)),
                rx.into_iter().map(move |r| (r, count)),
            )
        })
        .split_()
    }
}
