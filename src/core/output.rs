use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
};

use super::{
    dirty::DirtyReceive, relation::RelationInner, ContextTracker, CountMap, CreationContext,
    ExecutionContext, Op, Relation, TrackingIndex,
};

impl<C: Op> Relation<C> {
    pub fn into_output_<M: CountMap<C::D>>(self, context: &CreationContext) -> Output<C::D, C, M> {
        assert_eq!(&self.context_tracker, context.tracker(), "Context mismatch");
        Output {
            context_tracker: self.context_tracker,
            tracking_index: self.tracking_index,
            dirty: RefCell::new(self.dirty.into_receive()),
            inner: RefCell::new(self.inner),
            data: RefCell::new(M::empty()),
        }
    }
}

pub struct Output<D, C: Op<D = D>, M: CountMap<D> = HashMap<D, isize>> {
    context_tracker: ContextTracker,
    tracking_index: TrackingIndex,
    dirty: RefCell<DirtyReceive>,
    inner: RefCell<RelationInner<C>>,
    data: RefCell<M>,
}

impl<C: Op, M: CountMap<C::D>> Output<C::D, C, M> {
    pub fn get<'a>(&'a self, context: &'a ExecutionContext<'_>) -> Ref<'a, M> {
        assert_eq!(&self.context_tracker, context.tracker(), "Context mismatch");
        if self.dirty.borrow_mut().take_status() {
            let mut m = self.data.borrow_mut();
            self.inner.borrow_mut().foreach(|(k, v)| {
                m.add(k, v);
            })
        }
        self.data.borrow()
    }
    pub fn add_listener(&mut self, context: &CreationContext<'_>, f: impl FnMut() + 'static) {
        assert_eq!(&self.context_tracker, context.tracker(), "Context mismatch");
        self.dirty.borrow_mut().add_listener(f)
    }
    pub fn tracking_index(&self) -> TrackingIndex {
        self.tracking_index
    }
}
