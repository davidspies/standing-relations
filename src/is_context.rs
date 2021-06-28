use std::iter;

use crate::{core::ExecutionContext, Input};

pub trait IsContext<'a> {
    fn commit(&mut self);

    fn core_context(&mut self) -> &mut ExecutionContext<'a>;

    fn update_to<D: Clone + 'a>(&self, input: &Input<'a, D>, x: D, count: isize) {
        self.send_all_to(input, iter::once((x, count)))
    }

    fn send_all_to<D: Clone + 'a>(
        &self,
        input: &Input<'a, D>,
        iter: impl IntoIterator<Item = (D, isize)>,
    );
}

impl<'a> IsContext<'a> for ExecutionContext<'a> {
    fn commit(&mut self) {
        ExecutionContext::commit(self)
    }

    fn core_context(&mut self) -> &mut ExecutionContext<'a> {
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
}
