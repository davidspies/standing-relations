mod global_id;

use std::{cell::RefCell, mem, rc::Rc};

use crate::op::input::IsInputHandler;

pub(crate) type ContextId = usize;
pub(crate) type HandlerPosition = usize;

struct Handler<'a> {
    dumper: Box<dyn 'a + IsInputHandler>,
    is_queued: bool,
}

pub(crate) struct HandlerQueue<'a> {
    handlers: Vec<Handler<'a>>,
    queued_handler_inds: Vec<usize>,
}

impl<'a> HandlerQueue<'a> {
    fn new() -> Self {
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
    pub(crate) fn enqueue(&mut self, i: HandlerPosition) {
        let is_queued = &mut self.handlers[i].is_queued;
        if !*is_queued {
            *is_queued = true;
            self.queued_handler_inds.push(i);
        }
    }
    fn take_queued<'b>(&'b mut self) -> impl Iterator<Item = &'b Box<dyn 'a + IsInputHandler>> {
        for &i in &self.queued_handler_inds {
            self.handlers[i].is_queued = false;
        }
        let handlers = &self.handlers;
        mem::take(&mut self.queued_handler_inds)
            .into_iter()
            .map(move |i| &handlers[i].dumper)
    }
}

pub struct Context<'a> {
    id: usize,
    handler_queue: Rc<RefCell<HandlerQueue<'a>>>,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Context {
            id: global_id::next_id(),
            handler_queue: Rc::new(RefCell::new(HandlerQueue::new())),
        }
    }
    pub fn commit(&mut self) {
        for x in self.handler_queue.borrow_mut().take_queued() {
            x.dump()
        }
    }
    pub(crate) fn add_handler<H: IsInputHandler + 'a>(&mut self, handler: H) -> HandlerPosition {
        self.handler_queue.borrow_mut().add_handler(handler)
    }
    pub(crate) fn get_id(&self) -> ContextId {
        self.id
    }
    pub(crate) fn get_handler_queue(&self) -> &Rc<RefCell<HandlerQueue<'a>>> {
        &self.handler_queue
    }
}
