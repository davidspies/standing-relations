use crate::{ExecutionContext, InputSender};

impl<D> InputSender<'_, (D, isize)> {
    pub fn update(&self, context: &ExecutionContext, x: D, r: isize) {
        self.send(context, (x, r))
    }
    pub fn add(&self, context: &ExecutionContext, x: D) {
        self.update(context, x, 1)
    }
    pub fn remove(&self, context: &ExecutionContext, x: D) {
        self.update(context, x, -1)
    }
}
