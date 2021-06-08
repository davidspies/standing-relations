use crate::{Dynamic, Op, Relation, Save};

mod concat;
mod input;
mod join;
mod map;
mod output;
mod reduce;
mod split;

pub type Collection<'a, D> = Relation<Save<Dynamic<'a, (D, isize)>>>;

impl<C: Op<T = (D, isize)>, D: Clone> Relation<C> {
    pub fn collect<'a>(self) -> Collection<'a, D>
    where
        C: 'a,
    {
        self.dynamic().save()
    }
}
