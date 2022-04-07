use crate::{
    core::{CreationContext, ExecutionContext, InputOp},
    Input_, Relation,
};

pub type Input<'a, D> = Input_<'a, (D, isize)>;
pub type InputRelation<D> = Relation<InputOp<(D, isize)>>;

impl<'a, D: Clone + 'a> Input<'a, D> {
    pub fn update(&mut self, context: &ExecutionContext<'a>, x: D, r: isize) {
        self.send(context, (x, r))
    }
    pub fn add(&mut self, context: &ExecutionContext<'a>, x: D) {
        self.update(context, x, 1)
    }
    pub fn add_all(&mut self, context: &ExecutionContext<'a>, data: impl IntoIterator<Item = D>) {
        self.send_all(context, data.into_iter().map(|x| (x, 1)));
    }
    pub fn remove(&mut self, context: &ExecutionContext<'a>, x: D) {
        self.update(context, x, -1)
    }
    pub fn remove_all(
        &mut self,
        context: &ExecutionContext<'a>,
        data: impl IntoIterator<Item = D>,
    ) {
        self.send_all(context, data.into_iter().map(|x| (x, -1)));
    }
}

impl<'a> CreationContext<'a> {
    pub fn new_input<D: 'a>(&mut self) -> (Input<'a, D>, InputRelation<D>) {
        self.new_input_()
    }
}
