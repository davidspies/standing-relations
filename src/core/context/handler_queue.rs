use std::{
    cell::{RefCell, RefMut},
    mem,
};

pub type HandlerPosition = usize;

pub trait IsInputHandler {
    fn dump(&mut self);
}

struct Handler<'a> {
    dumper: RefCell<Box<dyn 'a + IsInputHandler>>,
    is_queued: bool,
}

#[derive(Default)]
pub struct HandlerQueue<'a> {
    handlers: Vec<Handler<'a>>,
    queued_handler_inds: Vec<usize>,
}

impl<'a> HandlerQueue<'a> {
    pub fn add_handler(&mut self, handler: impl IsInputHandler + 'a) -> HandlerPosition {
        let pos = self.handlers.len();
        self.handlers.push(Handler {
            dumper: RefCell::new(Box::new(handler)),
            is_queued: false,
        });
        pos
    }
    pub fn enqueue(&mut self, i: HandlerPosition) {
        let handler = &mut self.handlers[i];
        if !handler.is_queued {
            handler.is_queued = true;
            self.queued_handler_inds.push(i);
        }
    }
    pub fn take_queued(
        &mut self,
    ) -> impl Iterator<Item = RefMut<'_, Box<dyn 'a + IsInputHandler>>> {
        for &i in self.queued_handler_inds.iter() {
            self.handlers[i].is_queued = false;
        }
        let handlers = &self.handlers;
        mem::take(&mut self.queued_handler_inds)
            .into_iter()
            .map(move |i| handlers[i].dumper.borrow_mut())
    }
}
