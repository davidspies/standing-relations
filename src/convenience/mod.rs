use crate::{Dynamic, Op, Relation, Split};

mod concat;
mod join;
mod map;
mod reduce;

pub type Collection<'a, D> = Relation<Split<Dynamic<'a, (D, isize)>>>;

impl<C: Op<T = (D, isize)>, D: Clone> Relation<C> {
    pub fn collect<'a>(self) -> Collection<'a, D>
    where
        C: 'a,
    {
        self.dynamic().split()
    }
}
