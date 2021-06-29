use std::{
    mem,
    ops::{Deref, DerefMut},
};

use crate::{core::ExecutionContext, is_context::IsContext, Input};

pub struct ChangeTracker<'a>(Vec<Box<dyn IsTrackedInput<'a> + 'a>>);

pub trait IsTrackedInput<'a> {
    fn perform(self, context: &ExecutionContext<'a>);
    fn perform_steal(&mut self, context: &ExecutionContext<'a>);
    fn perform_unchanged(&self, context: &ExecutionContext<'a>);
    fn undo(self, context: &ExecutionContext<'a>);
    fn undo_steal(&mut self, context: &ExecutionContext<'a>);
}

pub(crate) struct TrackedChange<'a, D> {
    pub(crate) input: Input<'a, D>,
    pub(crate) data: Vec<(D, isize)>,
}

impl<'a, D: Clone + 'a> IsTrackedInput<'a> for TrackedChange<'a, D> {
    fn perform(mut self, context: &ExecutionContext<'a>) {
        self.perform_steal(context)
    }
    fn perform_steal(&mut self, context: &ExecutionContext<'a>) {
        context.send_all_to(&self.input, mem::take(&mut self.data))
    }
    fn perform_unchanged(&self, context: &ExecutionContext<'a>) {
        context.send_all_to(&self.input, self.data.clone())
    }
    fn undo(mut self, context: &ExecutionContext<'a>) {
        self.undo_steal(context)
    }
    fn undo_steal(&mut self, context: &ExecutionContext<'a>) {
        context.send_all_to(
            &self.input,
            mem::take(&mut self.data)
                .into_iter()
                .map(|(x, count)| (x, -count)),
        )
    }
}

impl<'a, T: IsTrackedInput<'a> + ?Sized> IsTrackedInput<'a> for Box<T> {
    fn perform(mut self, context: &ExecutionContext<'a>) {
        self.perform_steal(context)
    }
    fn perform_steal(&mut self, context: &ExecutionContext<'a>) {
        self.deref_mut().perform_steal(context)
    }
    fn perform_unchanged(&self, context: &ExecutionContext<'a>) {
        self.deref().perform_unchanged(context)
    }
    fn undo(mut self, context: &ExecutionContext<'a>) {
        self.undo_steal(context)
    }
    fn undo_steal(&mut self, context: &ExecutionContext<'a>) {
        self.deref_mut().undo_steal(context)
    }
}

impl<'a> ChangeTracker<'a> {
    pub fn new() -> Self {
        ChangeTracker(Vec::new())
    }
    pub fn update_all<D: Clone + 'a>(
        &mut self,
        context: &ExecutionContext<'a>,
        input: &Input<'a, D>,
        data: Vec<(D, isize)>,
    ) {
        self.update_tracked(
            context,
            TrackedChange {
                input: input.clone(),
                data,
            },
        )
    }
    pub fn update_tracked(
        &mut self,
        context: &ExecutionContext<'a>,
        tracked: impl IsTrackedInput<'a> + 'a,
    ) {
        tracked.perform_unchanged(context);
        self.0.push(Box::new(tracked))
    }
}

impl<'a> ChangeTracker<'a> {
    pub fn undo(self, context: &ExecutionContext<'a>) {
        for change in self.0 {
            change.undo(context);
        }
    }
}
