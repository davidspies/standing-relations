mod handler_queue;
pub mod input;

use self::handler_queue::{HandlerPosition, HandlerQueue};
use std::{
    cell::RefCell,
    fmt::{self, Debug},
    ptr,
    rc::Rc,
};

struct Context<'a> {
    tracker: ContextTracker,
    handler_queue: Rc<RefCell<HandlerQueue<'a>>>,
}

pub struct ContextTracker(Rc<RefCell<ContextTrackerInner>>);
struct ContextTrackerInner;

impl PartialEq for ContextTracker {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.0.as_ptr(), other.0.as_ptr())
    }
}
impl Eq for ContextTracker {}
impl Clone for ContextTracker {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}
impl ContextTracker {
    fn new() -> Self {
        ContextTracker(Rc::new(RefCell::new(ContextTrackerInner)))
    }
}
impl Debug for ContextTracker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0.as_ptr())
    }
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
    pub fn get_tracker(&self) -> &ContextTracker {
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
