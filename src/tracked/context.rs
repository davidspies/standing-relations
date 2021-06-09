use std::cell::RefCell;

use crate::{context_sends::ContextSends, core::ExecutionContext, Input};

use super::tracker::ChangeTracker;

pub struct TrackedContext<'a> {
    inner: ExecutionContext<'a>,
    tracker: RefCell<ChangeTracker<'a>>,
}

impl<'a, D: Clone + 'a> ContextSends<'a, D> for TrackedContext<'a> {
    fn update_to(&self, input: &Input<'a, (D, isize)>, x: D, count: isize) {
        self.tracker
            .borrow_mut()
            .update(&self.inner, input, x, count);
    }
}
