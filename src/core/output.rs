use crate::core::{
    context::ContextId, dirty::DirtyReceive, CountMap, CreationContext, ExecutionContext, Op,
    Relation,
};
use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
};

impl<C: Op> Relation<C> {
    pub fn get_output_<M: CountMap<C::D>>(self, context: &CreationContext) -> Output<C::D, C, M> {
        assert_eq!(self.context_id, context.get_id(), "Context mismatch");
        Output {
            context_id: self.context_id,
            dirty: self.dirty.to_receive(),
            inner: RefCell::new(self.inner),
            data: RefCell::new(M::empty()),
        }
    }
}

pub struct Output<D, C: Op<D = D>, M: CountMap<D> = HashMap<D, isize>> {
    context_id: ContextId,
    dirty: DirtyReceive,
    inner: RefCell<C>,
    data: RefCell<M>,
}

impl<C: Op, M: CountMap<C::D>> Output<C::D, C, M> {
    pub fn get<'a>(&'a self, context: &'a ExecutionContext<'_>) -> Ref<'a, M> {
        assert_eq!(self.context_id, context.get_id(), "Context mismatch");
        if self.dirty.take_status() {
            let mut m = self.data.borrow_mut();
            self.inner.borrow_mut().foreach(|(k, v)| {
                m.add(k, v);
            })
        }
        self.data.borrow()
    }
}
