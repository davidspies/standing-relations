use crate::core::{relation::RelationInner, Op_, Relation};

pub struct FlatMap<C: Op_, I: IntoIterator, F: Fn(C::T) -> I> {
    inner: RelationInner<C>,
    f: F,
}

impl<C: Op_, I: IntoIterator, F: Fn(C::T) -> I> Op_ for FlatMap<C, I, F> {
    type T = I::Item;

    fn foreach<'a>(&'a mut self, mut continuation: impl FnMut(Self::T) + 'a) {
        let FlatMap { inner, f } = self;
        inner.foreach(|x| {
            for y in f(x) {
                continuation(y)
            }
        })
    }

    fn get_type_name() -> &'static str {
        "flat_map"
    }
}

impl<C: Op_> Relation<C> {
    pub fn flat_map_<I: IntoIterator, F: Fn(C::T) -> I>(self, f: F) -> Relation<FlatMap<C, I, F>> {
        let inner = self.context_tracker.add_relation(FlatMap {
            inner: self.inner,
            f,
        });
        Relation {
            context_tracker: self.context_tracker,
            dirty: self.dirty,
            inner,
        }
    }
}
