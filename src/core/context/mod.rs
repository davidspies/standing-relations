mod global_id;
mod handler_queue;
pub mod input;

use self::handler_queue::{HandlerPosition, HandlerQueue};
use std::{cell::RefCell, rc::Rc};

pub(crate) type ContextId = usize;

struct Context<'a> {
    id: usize,
    handler_queue: Rc<RefCell<HandlerQueue<'a>>>,
}

impl<'a> Context<'a> {
    fn new() -> Self {
        Context {
            id: global_id::next_id(),
            handler_queue: Rc::new(RefCell::new(HandlerQueue::new())),
        }
    }
    fn commit(&mut self) {
        for x in self.handler_queue.borrow_mut().take_queued() {
            x.dump()
        }
    }
    fn get_handler_queue(&self) -> &Rc<RefCell<HandlerQueue<'a>>> {
        &self.handler_queue
    }
}

pub struct CreationContext<'a>(Context<'a>);

pub struct ExecutionContext<'a>(Context<'a>);

impl<'a> CreationContext<'a> {
    pub fn new() -> Self {
        CreationContext(Context::new())
    }
    pub fn begin(self) -> ExecutionContext<'a> {
        ExecutionContext(self.0)
    }
}

impl<'a> ExecutionContext<'a> {
    pub fn commit(&mut self) {
        self.0.commit()
    }
    pub(crate) fn get_id(&self) -> ContextId {
        self.0.id
    }
}
