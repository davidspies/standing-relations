use std::{
    cell::RefCell,
    collections::HashMap,
    hash::Hash,
    io::{self, Write},
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::{Arc, RwLock},
};

use crate::{
    core::{
        self,
        pipes::{self, Receiver, Sender},
        CountMap, TrackingIndex,
    },
    Input, InputRelation,
};

use self::pq_receiver::PQReceiver;

use super::op::{Instruct, IsFeedback, IsFeeder};

mod pq_receiver;

pub struct CreationContext<'a, I = ()> {
    inner: core::CreationContext<'a>,
    feeders: Vec<Box<dyn IsFeeder<'a, I> + 'a>>,
    input_trackers: Vec<Rc<RefCell<dyn IsInputChangeTracker<I> + 'a>>>,
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
            input_trackers: Default::default(),
            extra_edges: Default::default(),
            dirty_send,
            dirty_receive,
        }
    }
}

pub struct ExecutionContext<'a, I = ()> {
    inner: core::ExecutionContext<'a>,
    feeders: Vec<Box<dyn IsFeeder<'a, I> + 'a>>,
    input_trackers: Vec<Rc<RefCell<dyn IsInputChangeTracker<I> + 'a>>>,
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
    pub fn in_frame<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        for input_tracker in self.input_trackers.iter() {
            input_tracker.borrow_mut().push_frame();
        }
        let result = f(self);
        for input_tracker in self.input_trackers.iter() {
            input_tracker.borrow_mut().pop_frame(self);
        }
        result
    }
}

trait IsInputChangeTracker<I> {
    fn push_frame(&mut self);
    fn pop_frame(&mut self, context: &ExecutionContext<I>);
}

struct InputChangeTracker<'a, D> {
    input: Input<'a, D>,
    reversion_stack: Vec<HashMap<D, isize>>,
}

impl<D, I> IsInputChangeTracker<I> for InputChangeTracker<'_, D> {
    fn push_frame(&mut self) {
        self.reversion_stack.push(HashMap::new())
    }
    fn pop_frame(&mut self, context: &ExecutionContext<I>) {
        if let Some(changes) = self.reversion_stack.pop() {
            self.input
                .silent_send_all(context, changes.into_iter().collect())
        }
    }
}

impl<'a, D> InputChangeTracker<'a, D> {
    fn new(input: Input<'a, D>) -> Self {
        Self {
            input,
            reversion_stack: Vec::new(),
        }
    }
}

impl<'a, I> CreationContext<'a, I> {
    pub fn new_() -> Self {
        Default::default()
    }
    pub fn new_trackable_input<D: Eq + Hash + Clone + 'a>(&mut self) -> (Input<'a, D>, InputRelation<D>) {
        let (mut input, relation) = self.inner.new_input_::<(D, isize)>();
        let tracker: Rc<RefCell<InputChangeTracker<D>>> =
            Rc::new(RefCell::new(InputChangeTracker::new(input.clone())));
        {
            let tracker = Rc::clone(&tracker);
            input.add_listener(&self.inner, move |kvs| {
                if let Some(hm) = tracker.borrow_mut().reversion_stack.last_mut() {
                    for &(ref k, v) in kvs {
                        hm.add(k.clone(), -v)
                    }
                }
            });
        }
        self.input_trackers.push(tracker);
        (input, relation)
    }
    pub fn begin(self) -> ExecutionContext<'a, I> {
        ExecutionContext {
            inner: self.inner.begin(),
            feeders: self.feeders,
            input_trackers: self.input_trackers,
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
