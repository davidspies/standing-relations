pub mod concat;
pub mod dynamic;
pub mod feedback;
pub mod input;
pub mod join;
pub mod map;
pub mod output;
pub mod reduce;
pub mod save;
pub mod split;

use crate::{Dynamic, Op, Relation, Saved};

pub type Collection<'a, D> = Saved<Dynamic<'a, (D, isize)>>;

impl<C: Op> Relation<C>
where
    C::D: Clone,
{
    pub fn collect<'a>(self) -> Collection<'a, C::D>
    where
        C: 'a,
    {
        self.dynamic().save()
    }
    /// No-op which nails down the expected item type.
    ///
    /// The compiler will complain if the provided type is wrong. This helps you spot type errors
    /// where they occur rather than downstream.
    ///
    /// Example:
    ///
    /// ```
    ///    use standing_relations::CreationContext;
    ///    use std::{collections::HashMap, iter::FromIterator};
    ///
    ///    let context = CreationContext::new();
    ///    let (_foo_input, foo) = context.new_input::<(char, isize)>();
    ///    let (_bar_input, bar) = context.new_input::<char>();
    ///    let foobar =
    ///        foo.join(bar.counts()).flat_map(|(k,x,y)| if x < y {Some((k,x))} else {None});
    ///    let foobar = foobar.t::<(char, isize)>();
    /// ```
    ///
    /// Note that if instead of `flat_map`, we had accidentally used `map`, the compiler would
    /// complain because `foobar` would have item type `Option<(char, isize)>` where we expect
    /// `(char, isize)`.
    pub fn t<D: Is<Myself = C::D>>(self) -> Self {
        self
    }
}

pub trait Is {
    type Myself;
}

impl<T> Is for T {
    type Myself = T;
}
