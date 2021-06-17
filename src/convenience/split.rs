use crate::{Op, Relation};

impl<C: Op<D = (LI, RI)>, LI: IntoIterator, RI: IntoIterator> Relation<C> {
    pub fn split(
        self,
    ) -> (
        Relation<impl Op<D = LI::Item>>,
        Relation<impl Op<D = RI::Item>>,
    ) {
        self.map_(|((lx, rx), count)| {
            (
                lx.into_iter().map(move |l| (l, count)),
                rx.into_iter().map(move |r| (r, count)),
            )
        })
        .split_()
    }
}
