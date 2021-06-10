use std::mem;

use crate::{core::ExecutionContext, Input};

pub struct ChangeTracker<'a>(Vec<Box<dyn IsTrackedInput<'a> + 'a>>);

trait IsTrackedInput<'a> {
    fn undo(&mut self, context: &ExecutionContext<'a>);
}

struct TrackedChange<'a, D> {
    input: Input<'a, (D, isize)>,
    data: Vec<(D, isize)>,
}

impl<'a, D: Clone + 'a> IsTrackedInput<'a> for TrackedChange<'a, D> {
    fn undo(&mut self, context: &ExecutionContext<'a>) {
        for (x, count) in mem::take(&mut self.data) {
            self.input.update(context, x, -count)
        }
    }
}

impl<'a> ChangeTracker<'a> {
    pub fn new() -> Self {
        ChangeTracker(Vec::new())
    }
    pub fn update_all<D: Clone + 'a>(
        &mut self,
        context: &ExecutionContext<'a>,
        input: &Input<'a, (D, isize)>,
        data: Vec<(D, isize)>,
    ) {
        input.send_all(context, data.clone());
        self.0.push(Box::new(TrackedChange {
            input: input.clone(),
            data,
        }))
    }
}

impl<'a> ChangeTracker<'a> {
    pub fn undo(self, context: &ExecutionContext<'a>) {
        for mut change in self.0 {
            change.undo(context);
        }
    }
}
