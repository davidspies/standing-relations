mod pq_receiver;

use self::pq_receiver::PQReceiver;
use super::op::{Instruct, IsFeedback, IsFeeder};
use crate::{
    core,
    pipes::{self, Receiver, Sender},
};
use std::ops::Deref;

pub struct CreationContext<'a, I> {
    inner: core::CreationContext<'a>,
    feeders: Vec<Box<dyn IsFeeder<'a, I> + 'a>>,
    dirty_send: Sender<usize>,
    dirty_receive: Receiver<usize>,
}

pub struct ExecutionContext<'a, I> {
    inner: core::ExecutionContext<'a>,
    feeders: Vec<Box<dyn IsFeeder<'a, I> + 'a>>,
    dirty: PQReceiver<usize>,
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
}

impl<'a, I> CreationContext<'a, I> {
    pub fn new_() -> Self {
        let (dirty_send, dirty_receive) = pipes::new();
        CreationContext {
            inner: core::CreationContext::new(),
            feeders: Vec::new(),
            dirty_send,
            dirty_receive,
        }
    }
    pub fn begin(self) -> ExecutionContext<'a, I> {
        ExecutionContext {
            inner: self.inner.begin(),
            feeders: self.feeders,
            dirty: PQReceiver::new(self.dirty_receive),
        }
    }
    pub(super) fn add_feeder(&mut self, mut feeder: impl IsFeedback<'a, I> + 'a) {
        let cloned = self.dirty_send.clone();
        let i = self.feeders.len();
        feeder.add_listener(self, move || cloned.send(i));
        self.feeders.push(Box::new(feeder));
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
