use crate::{Op, Relation};

impl<C: Op> Relation<C> {
    pub fn minus(self, other: Relation<impl Op<D = C::D>>) -> Relation<impl Op<D = C::D>> {
        self.concat(other.negate())
    }
}
