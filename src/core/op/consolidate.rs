use crate::core::{relation::RelationInner, CountMap, Op, Op_, Relation};
use std::{collections::HashMap, hash::Hash};

pub struct Consolidate<C: Op_>(RelationInner<C>);

impl<C: Op> Op_ for Consolidate<C>
where
    C::D: Eq + Hash,
{
    type T = (C::D, isize);

    fn foreach<'a>(&'a mut self, mut continuation: impl FnMut(Self::T) + 'a) {
        let mut m = HashMap::new();
        self.0.foreach(|(x, count)| m.add(x, count));
        for x in m {
            continuation(x)
        }
    }

    fn get_type_name() -> &'static str {
        "consolidate"
    }
}

impl<C: Op_<T = (D, isize)>, D: Eq + Hash> Relation<C> {
    pub fn consolidate(self) -> Relation<Consolidate<C>> {
        let inner = self.context_tracker.add_relation(Consolidate(self.inner));
        Relation {
            context_tracker: self.context_tracker,
            dirty: self.dirty,
            inner,
        }
    }
}
