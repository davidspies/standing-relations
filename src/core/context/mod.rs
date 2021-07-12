mod handler_queue;
pub mod input;
mod tracking;

use self::handler_queue::{HandlerPosition, HandlerQueue};
pub use self::tracking::{ContextTracker, TrackIndex};
use std::{cell::RefCell, rc::Rc};

struct Context<'a> {
    tracker: ContextTracker,
    handler_queue: Rc<RefCell<HandlerQueue<'a>>>,
}

impl<'a> Context<'a> {
    fn new() -> Self {
        Context {
            tracker: ContextTracker::new(),
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
    pub(in crate::core) fn get_tracker(&self) -> &ContextTracker {
        &self.0.tracker
    }
}

impl<'a> ExecutionContext<'a> {
    pub fn commit(&mut self) {
        self.0.commit()
    }
    pub fn get_tracker(&self) -> &ContextTracker {
        &self.0.tracker
    }
}
