use crate::{
    context::Context,
    dirty::{self, DirtySend},
    op::Op,
    pipes::{self, Receiver},
    relation::Relation,
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

pub struct InputSender<T>(pipes::Sender<T>);

impl<T> InputSender<T> {
    pub fn send(&self, x: T) {
        self.0.send(x)
    }
}

impl<D> InputSender<(D, isize)> {
    pub fn update(&self, x: D, r: isize) {
        self.send((x, r))
    }
    pub fn add(&self, x: D) {
        self.update(x, 1)
    }
    pub fn remove(&self, x: D) {
        self.update(x, -1)
    }
}

pub struct Input<T>(Receiver<T>);

impl<T> Op for Input<T> {
    type T = T;

    fn get(&mut self) -> Vec<T> {
        self.0.receive()
    }
}

impl<'a> Context<'a> {
    pub fn new_input<T: 'a>(&mut self) -> (InputSender<T>, Relation<Input<T>>) {
        let (sender1, receiver1) = pipes::new();
        let (sender2, receiver2) = pipes::new();
        let (dirty_send, dirty_receive) = dirty::new();
        let handler = InputHandler {
            receiver: receiver1,
            sender: sender2,
            dirty_send,
        };
        self.add_handler(Box::new(handler));
        let result = Relation {
            context_id: self.get_id(),
            dirty: dirty_receive,
            inner: Input(receiver2),
        };
        (InputSender(sender1), result)
    }
}
