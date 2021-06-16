use crate::{Op, Relation};

impl<C: Op> Relation<C> {
    pub fn minus<C2: Op<D = C::D>>(self, other: Relation<C2>) -> Relation<impl Op<D = C::D>> {
        self.concat(other.negate())
    }
}
