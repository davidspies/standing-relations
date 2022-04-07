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
        self.inner.foreach(with_counter(&mut self.counter, f))
    }
    pub fn get_vec(&mut self) -> Vec<C::T> {
        let mut result = Vec::new();
        self.foreach(|x| result.push(x));
        result
    }
}

pub(super) fn with_counter<'a, T>(
    counter: &'a mut CountSender,
    mut f: impl FnMut(T) + 'a,
) -> impl FnMut(T) + 'a {
    move |x| {
        counter.increment();
        f(x)
    }
}

impl<C: Op_> Relation<C> {
    pub fn get_track_index(&self) -> TrackIndex {
        self.track_index
    }
    pub fn named(mut self, name: &str) -> Self {
        self.context_tracker
            .set_name(self.shown_index, name.to_string());
        self
    }
    pub fn type_named(mut self, type_name: &str) -> Self {
        self.context_tracker
            .set_type_name(self.shown_index, type_name.to_string());
        self
    }
    pub fn hidden(mut self) -> Self {
        self.context_tracker.set_hidden(self.shown_index);
        self.shown_index = self.context_tracker.find_shown_index(self.shown_index);
        self
    }
}
