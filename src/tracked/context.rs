use std::cell::RefCell;

use crate::{core::ExecutionContext, is_context::IsContext, Input};

use super::tracker::ChangeTracker;

pub struct TrackedContext<'a> {
    inner: ExecutionContext<'a>,
    tracker: RefCell<ChangeTracker<'a>>,
}

impl<'a> IsContext<'a> for TrackedContext<'a> {
    fn commit(&mut self) {
        self.inner.commit()
    }

    fn core_context(&mut self) -> &mut ExecutionContext<'a> {
        &mut self.inner
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
