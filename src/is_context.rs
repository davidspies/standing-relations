use std::iter;

use crate::{core::ExecutionContext, Input};

pub trait IsContext<'a> {
    fn commit(&mut self);

    fn core_context(&self) -> &ExecutionContext<'a>;
}

impl<'a> IsContext<'a> for ExecutionContext<'a> {
    fn commit(&mut self) {
        ExecutionContext::commit(self)
    }

    fn core_context(&self) -> &ExecutionContext<'a> {
        self
    }
}

pub trait ContextSends<'a, D> {
    fn update_to(&self, input: &Input<'a, D>, x: D, count: isize) {
        self.send_all_to(input, iter::once((x, count)))
    }
    fn send_all_to<I: IntoIterator<Item = (D, isize)>>(&self, input: &Input<'a, D>, iter: I);
}

impl<'a, D> ContextSends<'a, D> for ExecutionContext<'a> {
    fn update_to(&self, input: &Input<'a, D>, x: D, count: isize) {
        input.send(self, (x, count))
    }
    fn send_all_to<I: IntoIterator<Item = (D, isize)>>(&self, input: &Input<'a, D>, iter: I) {
        input.send_all(self, iter)
    }
}
