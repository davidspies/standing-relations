mod global_id;
mod handler_queue;
pub mod input;

use std::{cell::RefCell, rc::Rc};

use self::handler_queue::{HandlerPosition, HandlerQueue};

pub(crate) type ContextId = usize;

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
    pub(crate) fn get_id(&self) -> ContextId {
        self.id
    }
    fn get_handler_queue(&self) -> &Rc<RefCell<HandlerQueue<'a>>> {
        &self.handler_queue
    }
}
