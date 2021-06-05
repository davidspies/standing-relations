use crate::{Op, Relation};

pub struct Dynamic<'a, T>(Box<dyn 'a + Op<T = T>>);

impl<T> Op for Dynamic<'_, T> {
    type T = T;

    fn get(&mut self) -> Vec<T> {
        self.0.get()
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
