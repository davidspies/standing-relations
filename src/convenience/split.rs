use crate::{Either, Op, Relation};

impl<C: Op<D = Either<L, R>>, L, R> Relation<C> {
    pub fn split(self) -> (Relation<impl Op<D = L>>, Relation<impl Op<D = R>>) {
        self.map_(|(x, count)| match x {
            Either::Left(l) => Either::Left((l, count)),
            Either::Right(r) => Either::Right((r, count)),
        })
        .split_()
    }
}

impl<C: Op<D = (K, Either<L, R>)>, K, L, R> Relation<C> {
    pub fn split_by_value(self) -> (Relation<impl Op<D = (K, L)>>, Relation<impl Op<D = (K, R)>>) {
        self.map(|(k, x)| match x {
            Either::Left(l) => Either::Left((k, l)),
            Either::Right(r) => Either::Right((k, r)),
        })
        .split()
    }
}
