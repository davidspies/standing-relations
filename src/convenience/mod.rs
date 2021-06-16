pub mod concat;
pub mod input;
pub mod join;
pub mod map;
pub mod output;
pub mod reduce;
pub mod split;

use crate::{Dynamic, Op, Relation, Save};

pub type Collection<'a, D> = Relation<Save<Dynamic<'a, (D, isize)>>>;

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
}
