use crate::core::{Op, Relation};

pub struct Dynamic<'a, T>(Box<dyn DynOp<T = T> + 'a>);

impl<'b, T> Op for Dynamic<'b, T> {
    type T = T;

    fn foreach<'a, F: FnMut(Self::T) + 'a>(&'a mut self, continuation: F) {
        self.0.foreach(Box::new(continuation))
    }
}

trait DynOp {
    type T;

    fn foreach<'a>(&'a mut self, continuation: Box<dyn FnMut(Self::T) + 'a>);
}

impl<C: Op> DynOp for C {
    type T = C::T;

    fn foreach<'a>(&'a mut self, continuation: Box<dyn FnMut(Self::T) + 'a>) {
        Op::foreach(self, continuation)
    }
}

impl<C: Op> Relation<C> {
    pub fn dynamic<'a>(self) -> Relation<Dynamic<'a, C::T>>
    where
        C: 'a,
    {
        Relation {
            context_id: self.context_id,
            dirty: self.dirty,
            inner: Dynamic(Box::new(self.inner)),
        }
    }
}
