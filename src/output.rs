use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
};

use crate::{context::ContextId, dirty::DirtyReceive, Context, CountMap, Op, Relation};

impl<D, C: Op<T = (D, isize)>> Relation<C> {
    pub fn get_output<M: CountMap<D>>(self) -> Output<D, C, M> {
        Output {
            context_id: self.context_id,
            dirty: self.dirty.to_receive(),
            inner: RefCell::new(self.inner),
            data: RefCell::new(M::empty()),
        }
    }
}

pub struct Output<D, C: Op<T = (D, isize)>, M: CountMap<D> = HashMap<D, isize>> {
    context_id: ContextId,
    dirty: DirtyReceive,
    inner: RefCell<C>,
    data: RefCell<M>,
}

impl<D, C: Op<T = (D, isize)>, M: CountMap<D>> Output<D, C, M> {
    pub fn get<'a>(&'a self, context: &'a Context<'_>) -> Ref<'a, M> {
        assert_eq!(self.context_id, context.get_id(), "Context mismatch");
        if self.dirty.take_status() {
            let mut m = self.data.borrow_mut();
            for (k, v) in self.inner.borrow_mut().get() {
                m.add(k, v);
            }
        }
        self.data.borrow()
    }
}
