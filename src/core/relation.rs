use crate::{
    core::{
        context::{ContextTracker, TrackIndex},
        dirty::ReceiveBuilder,
        Op_,
    },
    pipes::CountSender,
};

pub struct Relation<C: Op_> {
    pub(super) context_tracker: ContextTracker,
    pub(super) track_index: TrackIndex,
    pub(super) shown_index: TrackIndex,
    pub(super) dirty: ReceiveBuilder,
    pub(super) inner: RelationInner<C>,
}

pub(super) struct RelationInner<C: ?Sized> {
    pub counter: CountSender,
    pub inner: C,
}

impl<C> RelationInner<C> {
    pub fn new(inner: C, counter: CountSender) -> Self {
        RelationInner { counter, inner }
    }
}

impl<C: Op_> RelationInner<C> {
    pub fn foreach(&mut self, f: impl FnMut(C::T)) {
        self.inner.foreach(with_counter(&self.counter, f))
    }
    pub fn get_vec(&mut self) -> Vec<C::T> {
        let mut result = Vec::new();
        self.foreach(|x| result.push(x));
        result
    }
}

pub(super) fn with_counter<'a, T>(
    counter: &'a CountSender,
    mut f: impl FnMut(T) + 'a,
) -> impl FnMut(T) + 'a {
    move |x| {
        counter.increment();
        f(x)
    }
}
