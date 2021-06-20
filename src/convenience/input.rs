use crate::{
    core::{CreationContext, InputOp},
    is_context::ContextSends,
    Input_, Relation,
};

pub type Input<'a, D> = Input_<'a, (D, isize)>;
pub type InputRelation<D> = Relation<InputOp<(D, isize)>>;

impl<'a, D> Input<'a, D> {
    pub fn update(&self, context: &impl ContextSends<'a, D>, x: D, r: isize) {
        context.update_to(self, x, r)
    }
    pub fn add(&self, context: &impl ContextSends<'a, D>, x: D) {
        self.update(context, x, 1)
    }
    pub fn add_all(&self, context: &impl ContextSends<'a, D>, data: impl IntoIterator<Item = D>) {
        context.send_all_to(self, data.into_iter().map(|x| (x, 1)));
    }
    pub fn remove(&self, context: &impl ContextSends<'a, D>, x: D) {
        self.update(context, x, -1)
    }
    pub fn remove_all(
        &self,
        context: &impl ContextSends<'a, D>,
        data: impl IntoIterator<Item = D>,
    ) {
        context.send_all_to(self, data.into_iter().map(|x| (x, -1)));
    }
}

impl<'a> CreationContext<'a> {
    pub fn new_input<D: 'a>(&self) -> (Input<'a, D>, InputRelation<D>) {
        self.new_input_()
    }
}
