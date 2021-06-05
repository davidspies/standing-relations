use std::vec;

use crate::{Op, Relation};

pub struct Dynamic<'a, T>(Box<dyn 'a + Op<T = T, I = vec::IntoIter<T>>>);

impl<T> Op for Dynamic<'_, T> {
    type T = T;
    type I = vec::IntoIter<T>;

    fn get(&mut self) -> Self::I {
        self.0.get()
    }
}

struct Vected<C>(C);

impl<C: Op> Op for Vected<C> {
    type T = C::T;
    type I = vec::IntoIter<C::T>;

    fn get(&mut self) -> Self::I {
        self.0.get().collect::<Vec<_>>().into_iter()
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
            inner: Dynamic(Box::new(Vected(self.inner))),
        }
    }
}
