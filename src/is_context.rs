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
    fn update_to(&self, input: &Input<'a, (D, isize)>, x: D, count: isize);
    fn send_all_to<I: IntoIterator<Item = (D, isize)>>(
        &self,
        input: &Input<'a, (D, isize)>,
        iter: I,
    ) {
        for (x, count) in iter {
            self.update_to(input, x, count)
        }
    }
}

impl<'a, D> ContextSends<'a, D> for ExecutionContext<'a> {
    fn update_to(&self, input: &Input<'a, (D, isize)>, x: D, count: isize) {
        input.send(self, (x, count))
    }
    fn send_all_to<I: IntoIterator<Item = (D, isize)>>(
        &self,
        input: &Input<'a, (D, isize)>,
        iter: I,
    ) {
        input.send_all(self, iter)
    }
}
