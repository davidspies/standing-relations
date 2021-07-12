use crate::core::{context::ContextTracker, dirty::ReceiveBuilder, Op_};

pub struct Relation<C: Op_> {
    pub(super) context_tracker: ContextTracker,
    pub(super) dirty: ReceiveBuilder,
    pub(super) inner: RelationInner<C>,
}

pub(super) struct RelationInner<C: ?Sized> {
    pub inner: C,
}

impl<C> RelationInner<C> {
    pub fn new(inner: C) -> Self {
        RelationInner { inner }
    }
}

impl<C: Op_> RelationInner<C> {
    pub fn foreach(&mut self, f: impl FnMut(C::T)) {
        self.inner.foreach(f)
    }
    pub fn get_vec(&mut self) -> Vec<C::T> {
        self.inner.get_vec()
    }
}
