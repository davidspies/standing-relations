use super::{
    handler_queue::IsInputHandler, ContextTracker, HandlerPosition, HandlerQueue, TrackIndex,
};
use crate::core::{
    dirty::{self, DirtySend},
    flat_iter::IntoFlatIterator,
    pipes::{self, Receiver},
    CreationContext, ExecutionContext, Op_, Relation,
};
use std::{cell::RefCell, rc::Rc};

struct InputHandler<T> {
    receiver: pipes::Receiver<T>,
    sender: pipes::Sender<Vec<T>>,
    dirty_send: DirtySend,
}

impl<T> IsInputHandler for InputHandler<T> {
    fn dump(&self) {
        self.sender.send(self.receiver.receive());
        self.dirty_send.set_dirty();
    }
}

pub struct Input_<'a, T> {
    context_tracker: ContextTracker,
    track_index: TrackIndex,
    sender: pipes::Sender<T>,
    handler_queue: Rc<RefCell<HandlerQueue<'a>>>,
    self_index: HandlerPosition,
}

impl<T> Clone for Input_<'_, T> {
    fn clone(&self) -> Self {
        Input_ {
            context_tracker: self.context_tracker.clone(),
            track_index: self.track_index.clone(),
            sender: self.sender.clone(),
            handler_queue: Rc::clone(&self.handler_queue),
            self_index: self.self_index,
        }
    }
}

impl<T> Input_<'_, T> {
    pub fn send(&self, context: &ExecutionContext, x: T) {
        assert_eq!(self.context_tracker, context.0.tracker, "Context mismatch");
        self.handler_queue.borrow_mut().enqueue(self.self_index);
        self.sender.send(x)
    }
    pub fn send_all(&self, context: &ExecutionContext, data: impl IntoIterator<Item = T>) {
        assert_eq!(self.context_tracker, context.0.tracker, "Context mismatch");
        self.handler_queue.borrow_mut().enqueue(self.self_index);
        self.sender.send_all(data);
    }
    pub fn get_track_index(&self) -> &TrackIndex {
        &self.track_index
    }
}

pub struct InputOp<T>(Receiver<Vec<T>>);

impl<T> Op_ for InputOp<T> {
    type T = T;

    fn foreach<'a>(&'a mut self, mut continuation: impl FnMut(Self::T) + 'a) {
        for x in self.0.receive().into_flat_iter() {
            continuation(x)
        }
    }
    fn get_type_name() -> &'static str {
        "input"
    }
}

impl<'a> CreationContext<'a> {
    pub fn new_input_<T: 'a>(&self) -> (Input_<'a, T>, Relation<InputOp<T>>) {
        let (sender1, receiver1) = pipes::new();
        let (sender2, receiver2) = pipes::new();
        let (dirty_send, dirty_receive) = dirty::new();
        let handler = InputHandler {
            receiver: receiver1,
            sender: sender2,
            dirty_send,
        };
        let i = self.0.add_handler(handler);
        let relation =
            self.0
                .tracker
                .clone()
                .add_relation(dirty_receive, InputOp(receiver2), vec![]);
        let input_sender = Input_ {
            context_tracker: self.0.tracker.clone(),
            track_index: relation.track_index.clone(),
            sender: sender1,
            handler_queue: Rc::clone(self.0.get_handler_queue()),
            self_index: i,
        };
        (input_sender, relation)
    }
}
