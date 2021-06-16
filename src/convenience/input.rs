use crate::{
    core::{CreationContext, InputOp},
    is_context::ContextSends,
    Input_, Relation,
};

pub type Input<'a, D> = Input_<'a, (D, isize)>;

impl<'a, D> Input<'a, D> {
    pub fn update<C: ContextSends<'a, D>>(&self, context: &C, x: D, r: isize) {
        context.update_to(self, x, r)
    }
    pub fn add<C: ContextSends<'a, D>>(&self, context: &C, x: D) {
        self.update(context, x, 1)
    }
    pub fn add_all<I: IntoIterator<Item = D>, C: ContextSends<'a, D>>(&self, context: &C, data: I) {
        context.send_all_to(self, data.into_iter().map(|x| (x, 1)));
    }
    pub fn remove<C: ContextSends<'a, D>>(&self, context: &C, x: D) {
        self.update(context, x, -1)
    }
    pub fn remove_all<I: IntoIterator<Item = D>, C: ContextSends<'a, D>>(
        &self,
        context: &C,
        data: I,
    ) {
        context.send_all_to(self, data.into_iter().map(|x| (x, -1)));
    }
}

impl<'a> CreationContext<'a> {
    pub fn new_input<D: 'a>(&self) -> (Input<'a, D>, Relation<InputOp<(D, isize)>>) {
        self.new_input_()
    }
}
