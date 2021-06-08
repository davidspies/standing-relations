use crate::{Either, Op, Relation};

impl<C: Op<T = (Either<L, R>, isize)>, L, R> Relation<C> {
    pub fn split(
        self,
    ) -> (
        Relation<impl Op<T = (L, isize)>>,
        Relation<impl Op<T = (R, isize)>>,
    ) {
        self.map_(|(x, count)| match x {
            Either::Left(l) => Either::Left((l, count)),
            Either::Right(r) => Either::Right((r, count)),
        })
        .split_()
    }
}

impl<C: Op<T = ((K, Either<L, R>), isize)>, K, L, R> Relation<C> {
    pub fn split_by_value(
        self,
    ) -> (
        Relation<impl Op<T = ((K, L), isize)>>,
        Relation<impl Op<T = ((K, R), isize)>>,
    ) {
        self.map(|(k, x)| match x {
            Either::Left(l) => Either::Left((k, l)),
            Either::Right(r) => Either::Right((k, r)),
        })
        .split()
    }
}
