use std::cell::RefCell;

use crate::{
    core::ExecutionContext,
    is_context::{ContextSends, IsContext},
    Input,
};

use super::tracker::ChangeTracker;

pub struct TrackedContext<'a> {
    inner: ExecutionContext<'a>,
    tracker: RefCell<ChangeTracker<'a>>,
}

impl<'a> IsContext<'a> for TrackedContext<'a> {
    fn commit(&mut self) {
        self.inner.commit()
    }

    fn core_context(&self) -> &ExecutionContext<'a> {
        &self.inner
    }
}

impl<'a, D: Clone + 'a> ContextSends<'a, D> for TrackedContext<'a> {
    fn send_all_to<I: IntoIterator<Item = (D, isize)>>(
        &self,
        input: &Input<'a, (D, isize)>,
        data: I,
    ) {
        self.tracker
            .borrow_mut()
            .update_all(&self.inner, input, data.into_iter().collect())
    }
}

impl<'a> TrackedContext<'a> {
    pub fn new(inner: ExecutionContext<'a>) -> Self {
        TrackedContext {
            inner,
            tracker: RefCell::new(ChangeTracker::new()),
        }
    }
    pub fn pieces(self) -> (ExecutionContext<'a>, ChangeTracker<'a>) {
        (self.inner, self.tracker.into_inner())
    }
}
