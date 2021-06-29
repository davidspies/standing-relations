use std::cell::RefCell;

use crate::{core::ExecutionContext, is_context::IsContext, Input};

use super::{tracker::ChangeTracker, IsTrackedInput};

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

    fn send_all_to<D: Clone + 'a>(
        &self,
        input: &Input<'a, D>,
        data: impl IntoIterator<Item = (D, isize)>,
    ) {
        self.tracker
            .borrow_mut()
            .update_all(&self.inner, input, data.into_iter().collect())
    }

    fn update_tracked(&self, tracked: impl IsTrackedInput<'a> + 'a) {
        self.tracker
            .borrow_mut()
            .update_tracked(&self.inner, tracked)
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
