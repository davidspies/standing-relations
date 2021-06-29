use std::iter;

use crate::{core::ExecutionContext, tracked::IsTrackedInput, Input};

pub trait IsContext<'a> {
    fn commit(&mut self);

    fn core_context(&self) -> &ExecutionContext<'a>;

    fn update_to<D: Clone + 'a>(&self, input: &Input<'a, D>, x: D, count: isize) {
        self.send_all_to(input, iter::once((x, count)))
    }

    fn send_all_to<D: Clone + 'a>(
        &self,
        input: &Input<'a, D>,
        iter: impl IntoIterator<Item = (D, isize)>,
    );

    fn update_tracked(&self, tracked: impl IsTrackedInput<'a> + 'a);
}

impl<'a> IsContext<'a> for ExecutionContext<'a> {
    fn commit(&mut self) {
        ExecutionContext::commit(self)
    }

    fn core_context(&self) -> &ExecutionContext<'a> {
        self
    }

    fn update_to<D: Clone>(&self, input: &Input<'a, D>, x: D, count: isize) {
        input.send(self, (x, count))
    }

    fn send_all_to<D: Clone>(
        &self,
        input: &Input<'a, D>,
        iter: impl IntoIterator<Item = (D, isize)>,
    ) {
        input.send_all(self, iter)
    }

    fn update_tracked(&self, tracked: impl IsTrackedInput<'a> + 'a) {
        tracked.perform(self)
    }
}

pub(crate) trait IsDynContext<'a> {
    fn core_context(&self) -> &ExecutionContext<'a>;
    fn update_dyn(&self, tracked: Box<dyn IsTrackedInput<'a> + 'a>);
}
impl<'a, T: IsContext<'a>> IsDynContext<'a> for T {
    fn core_context(&self) -> &ExecutionContext<'a> {
        IsContext::core_context(self)
    }
    fn update_dyn(&self, tracked: Box<dyn IsTrackedInput<'a> + 'a>) {
        self.update_tracked(tracked)
    }
}
