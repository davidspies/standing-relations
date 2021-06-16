use super::{handler_queue::IsInputHandler, ContextId, HandlerPosition, HandlerQueue};
use crate::core::{
    dirty::{self, DirtySend},
    flat_iter::IntoFlatIterator,
    pipes::{self, Receiver},
    CreationContext, ExecutionContext, Op, Relation,
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
    context_id: ContextId,
    sender: pipes::Sender<T>,
    handler_queue: Rc<RefCell<HandlerQueue<'a>>>,
    self_index: HandlerPosition,
}

impl<T> Clone for Input_<'_, T> {
    fn clone(&self) -> Self {
        Input_ {
            context_id: self.context_id,
            sender: self.sender.clone(),
            handler_queue: Rc::clone(&self.handler_queue),
            self_index: self.self_index,
        }
    }
}

impl<T> Input_<'_, T> {
    pub fn send(&self, context: &ExecutionContext, x: T) {
        assert_eq!(self.context_id, context.0.id, "Context mismatch");
        self.handler_queue.borrow_mut().enqueue(self.self_index);
        self.sender.send(x)
    }
    pub fn send_all<I: IntoIterator<Item = T>>(&self, context: &ExecutionContext, data: I) {
        assert_eq!(self.context_id, context.0.id, "Context mismatch");
        self.handler_queue.borrow_mut().enqueue(self.self_index);
        self.sender.send_all(data);
    }
}

pub struct InputOp<T>(Receiver<Vec<T>>);

impl<T> Op for InputOp<T> {
    type T = T;

    fn foreach<'a, F: FnMut(Self::T) + 'a>(&'a mut self, mut continuation: F) {
        for x in self.0.receive().into_flat_iter() {
            continuation(x)
        }
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
        let input_sender = Input_ {
            context_id: self.0.id,
            sender: sender1,
            handler_queue: Rc::clone(self.0.get_handler_queue()),
            self_index: i,
        };
        let relation = Relation {
            context_id: self.0.id,
            dirty: dirty_receive,
            inner: InputOp(receiver2),
        };
        (input_sender, relation)
    }
}
