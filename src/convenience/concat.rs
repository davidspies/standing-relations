use crate::{Op, Relation};

impl<D, C: Op<T = (D, isize)>> Relation<C> {
    pub fn minus<C2: Op<T = (D, isize)>>(
        self,
        other: Relation<C2>,
    ) -> Relation<impl Op<T = (D, isize)>> {
        self.concat(other.negate())
    }
}
