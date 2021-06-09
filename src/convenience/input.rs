use crate::{core::ExecutionContext, Input};

impl<D> Input<'_, (D, isize)> {
    pub fn update(&self, context: &ExecutionContext, x: D, r: isize) {
        self.send(context, (x, r))
    }
    pub fn add(&self, context: &ExecutionContext, x: D) {
        self.update(context, x, 1)
    }
    pub fn add_all<I: IntoIterator<Item = D>>(&self, context: &ExecutionContext, data: I) {
        self.send_all(context, data.into_iter().map(|x| (x, 1)));
    }
    pub fn remove(&self, context: &ExecutionContext, x: D) {
        self.update(context, x, -1)
    }
    pub fn remove_all<I: IntoIterator<Item = D>>(&self, context: &ExecutionContext, data: I) {
        self.send_all(context, data.into_iter().map(|x| (x, -1)));
    }
}
