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
