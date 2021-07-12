use crate::core::{relation::RelationInner, Op_, Relation};

pub struct Concat<C1: Op_, C2: Op_<T = C1::T>>(RelationInner<C1>, RelationInner<C2>);

impl<C1: Op_, C2: Op_<T = C1::T>> Op_ for Concat<C1, C2> {
    type T = C1::T;

    fn foreach<'a>(&'a mut self, mut continuation: impl FnMut(Self::T) + 'a) {
        self.0.foreach(&mut continuation);
        self.1.foreach(continuation);
    }
}

impl<C1: Op_> Relation<C1> {
    pub fn concat<C2: Op_<T = C1::T>>(self, other: Relation<C2>) -> Relation<Concat<C1, C2>> {
        assert_eq!(
            self.context_tracker, other.context_tracker,
            "Context mismatch"
        );
        Relation {
            context_tracker: self.context_tracker,
            dirty: self.dirty.or(other.dirty),
            inner: RelationInner {
                inner: Concat(self.inner, other.inner),
            },
        }
    }
}
