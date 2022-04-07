use std::{
    io::{self, Write},
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock},
};

use crate::{
    core::{
        self,
        pipes::{self, Receiver, Sender},
        InputOp, Input_, Relation, TrackingIndex,
    },
    Input, InputRelation,
};

use self::pq_receiver::PQReceiver;

use super::op::{Instruct, IsFeedback, IsFeeder};

mod pq_receiver;

pub struct CreationContext<'a, I = ()> {
    inner: core::CreationContext<'a>,
    feeders: Vec<Box<dyn IsFeeder<'a, I> + 'a>>,
    extra_edges: Arc<RwLock<Vec<(TrackingIndex, TrackingIndex)>>>,
    dirty_send: Sender<usize>,
    dirty_receive: Receiver<usize>,
}

impl<I> Default for CreationContext<'_, I> {
    fn default() -> Self {
        let (dirty_send, dirty_receive) = pipes::new();
        Self {
            inner: Default::default(),
            feeders: Default::default(),
            extra_edges: Default::default(),
            dirty_send,
            dirty_receive,
        }
    }
}

pub struct ExecutionContext<'a, I = ()> {
    inner: core::ExecutionContext<'a>,
    feeders: Vec<Box<dyn IsFeeder<'a, I> + 'a>>,
    extra_edges: Arc<RwLock<Vec<(TrackingIndex, TrackingIndex)>>>,
    dirty: PQReceiver,
}

pub struct ContextTracker {
    inner: core::ContextTracker,
    extra_edges: Arc<RwLock<Vec<(TrackingIndex, TrackingIndex)>>>,
}

impl ContextTracker {
    pub fn dump_dot(&self, file: impl Write) -> Result<(), io::Error> {
        self.inner
            .dump_dot(file, &*self.extra_edges.read().unwrap())
    }
}

impl<'a, I> ExecutionContext<'a, I> {
    pub fn commit(&mut self) -> Option<I> {
        loop {
            self.inner.commit();
            match self.dirty.pop_min() {
                Some(feeder_index) => match self.feeders[feeder_index].feed(&self.inner) {
                    Instruct::Unchanged => (),
                    Instruct::Changed => self.dirty.insert(feeder_index),
                    Instruct::Interrupt(interrupted) => return Some(interrupted),
                },
                None => return None,
            }
        }
    }
    pub fn tracker(&self) -> ContextTracker {
        ContextTracker {
            inner: self.inner.tracker().clone(),
            extra_edges: self.extra_edges.clone(),
        }
    }
}

impl<'a, I> CreationContext<'a, I> {
    pub fn new_() -> Self {
        Default::default()
    }
    pub fn new_trackable_input<D: 'a>(&mut self) -> (Input<'a, D>, InputRelation<D>) {
        self.inner.new_input()
    }
    pub fn begin(self) -> ExecutionContext<'a, I> {
        ExecutionContext {
            inner: self.inner.begin(),
            feeders: self.feeders,
            extra_edges: self.extra_edges,
            dirty: PQReceiver::new(self.dirty_receive),
        }
    }
    pub(super) fn add_feeder(
        &mut self,
        mut feeder: impl IsFeedback<'a, I> + 'a,
        extra_edge: Option<(TrackingIndex, TrackingIndex)>,
    ) {
        let mut dirty_send = self.dirty_send.clone();
        let i = self.feeders.len();
        feeder.add_listener(self, move || dirty_send.send(i));
        self.feeders.push(Box::new(feeder));
        if let Some(edge) = extra_edge {
            self.extra_edges.write().unwrap().push(edge);
        }
    }
    pub fn tracker(&self) -> ContextTracker {
        ContextTracker {
            inner: self.inner.tracker().clone(),
            extra_edges: self.extra_edges.clone(),
        }
    }
}

impl<'a, I> Deref for CreationContext<'a, I> {
    type Target = core::CreationContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, I> DerefMut for CreationContext<'a, I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a, I> Deref for ExecutionContext<'a, I> {
    type Target = core::ExecutionContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, I> DerefMut for ExecutionContext<'a, I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
