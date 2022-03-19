mod pq_receiver;

use self::pq_receiver::PQReceiver;
use super::op::{Instruct, IsFeedback, IsFeeder};
use crate::{
    core::{self, TrackIndex},
    pipes::{self, Receiver, Sender},
};
use std::{
    io::{self, Write},
    ops::Deref,
    sync::{Arc, RwLock},
};

pub struct CreationContext<'a, I = ()> {
    inner: core::CreationContext<'a>,
    feeders: Vec<Box<dyn IsFeeder<'a, I> + 'a>>,
    extra_edges: Arc<RwLock<Vec<(TrackIndex, TrackIndex)>>>,
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
    extra_edges: Arc<RwLock<Vec<(TrackIndex, TrackIndex)>>>,
    dirty: PQReceiver,
}

pub struct ContextTracker {
    inner: core::ContextTracker,
    extra_edges: Arc<RwLock<Vec<(TrackIndex, TrackIndex)>>>,
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
    pub fn get_tracker(&self) -> ContextTracker {
        ContextTracker {
            inner: self.inner.get_tracker().clone(),
            extra_edges: self.extra_edges.clone(),
        }
    }
}

impl<'a, I> CreationContext<'a, I> {
    pub fn new_() -> Self {
        Default::default()
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
        extra_edge: Option<(TrackIndex, TrackIndex)>,
    ) {
        let cloned = self.dirty_send.clone();
        let i = self.feeders.len();
        feeder.add_listener(self, move || cloned.send(i));
        self.feeders.push(Box::new(feeder));
        if let Some(edge) = extra_edge {
            self.extra_edges.write().unwrap().push(edge);
        }
    }
    pub fn get_tracker(&self) -> ContextTracker {
        ContextTracker {
            inner: self.inner.get_tracker().clone(),
            extra_edges: self.extra_edges.clone(),
        }
    }
}

impl<'a, I> Deref for CreationContext<'a, I> {
    type Target = core::CreationContext<'a>;

    fn deref(&self) -> &core::CreationContext<'a> {
        &self.inner
    }
}

impl<'a, I> Deref for ExecutionContext<'a, I> {
    type Target = core::ExecutionContext<'a>;

    fn deref(&self) -> &core::ExecutionContext<'a> {
        &self.inner
    }
}
