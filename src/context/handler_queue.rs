use std::mem;

use super::Context;

pub type HandlerPosition = usize;

pub trait IsInputHandler {
    fn dump(&self);
}

struct Handler<'a> {
    dumper: Box<dyn 'a + IsInputHandler>,
    is_queued: bool,
}

pub struct HandlerQueue<'a> {
    handlers: Vec<Handler<'a>>,
    queued_handler_inds: Vec<usize>,
}

impl<'a> HandlerQueue<'a> {
    pub fn new() -> Self {
        HandlerQueue {
            handlers: Vec::new(),
            queued_handler_inds: Vec::new(),
        }
    }
    fn add_handler<H: IsInputHandler + 'a>(&mut self, handler: H) -> HandlerPosition {
        let pos = self.handlers.len();
        self.handlers.push(Handler {
            dumper: Box::new(handler),
            is_queued: false,
        });
        pos
    }
    pub fn enqueue(&mut self, i: HandlerPosition) {
        let is_queued = &mut self.handlers[i].is_queued;
        if !*is_queued {
            *is_queued = true;
            self.queued_handler_inds.push(i);
        }
    }
    pub fn take_queued<'b>(&'b mut self) -> impl Iterator<Item = &'b Box<dyn 'a + IsInputHandler>> {
        for &i in &self.queued_handler_inds {
            self.handlers[i].is_queued = false;
        }
        let handlers = &self.handlers;
        mem::take(&mut self.queued_handler_inds)
            .into_iter()
            .map(move |i| &handlers[i].dumper)
    }
}

impl<'a> Context<'a> {
    pub(super) fn add_handler<H: IsInputHandler + 'a>(&self, handler: H) -> HandlerPosition {
        self.handler_queue.borrow_mut().add_handler(handler)
    }
}
