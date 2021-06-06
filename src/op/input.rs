use std::{cell::RefCell, rc::Rc};

use crate::{
    context::{HandlerPosition, HandlerQueue},
    dirty::{self, DirtySend},
    pipes::{self, Receiver},
    Context, Op, Relation,
};

pub trait IsInputHandler {
    fn dump(&self);
}

struct InputHandler<T> {
    receiver: pipes::Receiver<T>,
    sender: pipes::Sender<T>,
    dirty_send: DirtySend,
}

impl<T> IsInputHandler for InputHandler<T> {
    fn dump(&self) {
        self.sender.send_all(self.receiver.receive());
        self.dirty_send.set_dirty();
    }
}

pub struct InputSender<'a, T> {
    sender: pipes::Sender<T>,
    handler_queue: Rc<RefCell<HandlerQueue<'a>>>,
    self_index: HandlerPosition,
}

impl<T> InputSender<'_, T> {
    pub fn send(&self, x: T) {
        self.handler_queue.borrow_mut().enqueue(self.self_index);
        self.sender.send(x)
    }
}

pub struct Input<T>(Receiver<T>);

impl<T> Op for Input<T> {
    type T = T;

    fn foreach<'a, F: FnMut(Self::T) + 'a>(&'a mut self, mut continuation: F) {
        for x in self.0.receive() {
            continuation(x)
        }
    }
}

impl<'a> Context<'a> {
    pub fn new_input<T: 'a>(&mut self) -> (InputSender<'a, T>, Relation<Input<T>>) {
        let (sender1, receiver1) = pipes::new();
        let (sender2, receiver2) = pipes::new();
        let (dirty_send, dirty_receive) = dirty::new();
        let handler = InputHandler {
            receiver: receiver1,
            sender: sender2,
            dirty_send,
        };
        let i = self.add_handler(handler);
        let input_sender = InputSender {
            sender: sender1,
            handler_queue: Rc::clone(self.get_handler_queue()),
            self_index: i,
        };
        let relation = Relation {
            context_id: self.get_id(),
            dirty: dirty_receive,
            inner: Input(receiver2),
        };
        (input_sender, relation)
    }
}
