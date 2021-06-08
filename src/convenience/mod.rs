use crate::{Dynamic, Op, Relation, Save};

pub mod concat;
pub mod input;
pub mod join;
pub mod map;
pub mod output;
pub mod reduce;
pub mod split;

pub type Collection<'a, D> = Relation<Save<Dynamic<'a, (D, isize)>>>;

impl<C: Op<T = (D, isize)>, D: Clone> Relation<C> {
    pub fn collect<'a>(self) -> Collection<'a, D>
    where
        C: 'a,
    {
        self.dynamic().save()
    }
}
