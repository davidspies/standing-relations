mod global_id;

use crate::op::input::IsInputHandler;

pub type ContextId = usize;

pub struct Context<'a> {
    id: usize,
    handlers: Vec<Box<dyn 'a + IsInputHandler>>,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Context {
            id: global_id::next_id(),
            handlers: Vec::new(),
        }
    }
    pub fn commit(&mut self) {
        for x in &self.handlers {
            x.dump()
        }
    }
    pub(crate) fn get_id(&self) -> ContextId {
        self.id
    }
    pub(crate) fn add_handler(&mut self, handler: Box<dyn 'a + IsInputHandler>) {
        self.handlers.push(handler);
    }
}
