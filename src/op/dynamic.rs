use std::marker::PhantomData;

use crate::{Op, Relation};

pub struct Dynamic<'a, T>(Box<dyn 'a + Op<T = T, I = Box<dyn 'a + Iterator<Item = T>>>>);

impl<'a, T> Op for Dynamic<'a, T> {
    type T = T;
    type I = Box<dyn 'a + Iterator<Item = T>>;

    fn get(&mut self) -> Self::I {
        self.0.get()
    }
}

struct BoxedIter<'a, C>(C, PhantomData<&'a ()>);

impl<'a, C: Op> Op for BoxedIter<'a, C>
where
    C::I: 'a,
{
    type T = C::T;
    type I = Box<dyn 'a + Iterator<Item = C::T>>;

    fn get(&mut self) -> Self::I {
        Box::new(self.0.get())
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
            inner: Dynamic(Box::new(BoxedIter(self.inner, PhantomData))),
        }
    }
}
