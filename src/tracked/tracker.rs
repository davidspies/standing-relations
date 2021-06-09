use crate::{core::ExecutionContext, Input};

pub struct ChangeTracker<'a>(Vec<Box<dyn IsTrackedInput<'a> + 'a>>);

trait IsTrackedInput<'a> {
    fn undo(&mut self, context: &ExecutionContext<'a>);
}

struct TrackedChange<'a, D> {
    input: Input<'a, (D, isize)>,
    x: Option<D>,
    count: isize,
}

impl<'a, D: Clone + 'a> IsTrackedInput<'a> for TrackedChange<'a, D> {
    fn undo(&mut self, context: &ExecutionContext<'a>) {
        self.input
            .update(context, self.x.take().unwrap(), -self.count)
    }
}

impl<'a> ChangeTracker<'a> {
    pub fn new() -> Self {
        ChangeTracker(Vec::new())
    }
    pub fn update<D: Clone + 'a>(
        &mut self,
        context: &ExecutionContext<'a>,
        input: &Input<'a, (D, isize)>,
        x: D,
        count: isize,
    ) {
        input.update(context, x.clone(), count);
        self.0.push(Box::new(TrackedChange {
            input: input.clone(),
            x: Some(x),
            count,
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
