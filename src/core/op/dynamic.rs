use crate::core::{
    relation::{self, RelationInner},
    Op_, Relation,
};

pub struct Dynamic<'a, T>(Box<RelationInner<dyn DynOp<T = T> + 'a>>);

impl<'b, T> Op_ for Dynamic<'b, T> {
    type T = T;

    fn foreach<'a>(&'a mut self, continuation: impl FnMut(Self::T) + 'a) {
        self.0.inner.foreach(Box::new(relation::with_counter(
            &mut self.0.counter,
            continuation,
        )))
    }

    fn get_type_name() -> &'static str {
        "dynamic"
    }
}

trait DynOp {
    type T;

    fn foreach<'a>(&'a mut self, continuation: Box<dyn FnMut(Self::T) + 'a>);
}

impl<C: Op_> DynOp for C {
    type T = C::T;

    fn foreach<'a>(&'a mut self, continuation: Box<dyn FnMut(Self::T) + 'a>) {
        Op_::foreach(self, continuation)
    }
}

impl<C: Op_> Relation<C> {
    pub fn dynamic_shown<'a>(self) -> Relation<Dynamic<'a, C::T>>
    where
        C: 'a,
    {
        self.context_tracker.add_relation(
            self.dirty,
            Dynamic(Box::new(self.inner)),
            vec![self.track_index],
        )
    }
}
