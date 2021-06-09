use crate::core::{Op, Relation};

pub struct Concat<C1: Op, C2: Op<T = C1::T>>(C1, C2);

impl<C1: Op, C2: Op<T = C1::T>> Op for Concat<C1, C2> {
    type T = C1::T;

    fn foreach<'a, F: FnMut(Self::T) + 'a>(&'a mut self, mut continuation: F) {
        self.0.foreach(&mut continuation);
        self.1.foreach(continuation);
    }
}

impl<C1: Op> Relation<C1> {
    pub fn concat<C2: Op<T = C1::T>>(self, other: Relation<C2>) -> Relation<Concat<C1, C2>> {
        assert_eq!(self.context_id, other.context_id, "Context mismatch");
        Relation {
            context_id: self.context_id,
            dirty: self.dirty.or(other.dirty),
            inner: Concat(self.inner, other.inner),
        }
    }
}
