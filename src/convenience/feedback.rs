use crate::{core, pipes::Receiver, ContextId, CountMap, CreationContext, Op, Relation};
use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    hash::Hash,
};

impl<'a, I> CreationContext<'a, I> {
    pub fn interrupt<C: Op + 'a, F: Fn(&HashMap<C::D, isize>) -> Option<I> + 'a>(
        &mut self,
        rel: Relation<C>,
        f: F,
    ) where
        I: 'a,
        C::D: Eq + Hash,
    {
        self.interrupt_(rel, f)
    }
    pub fn interrupt_nonempty<C: Op + 'a>(&mut self, rel: Relation<C>, i: I)
    where
        I: Clone + 'a,
        C::D: Eq + Hash,
    {
        self.interrupt(
            rel,
            move |m| if m.is_empty() { None } else { Some(i.clone()) },
        )
    }
}

pub struct TrackedOutput<D, M = HashMap<D, isize>> {
    context_id: ContextId,
    receiver: Receiver<HashMap<D, isize>>,
    total: RefCell<M>,
}

impl<D: Eq + Hash> TrackedOutput<D> {
    pub fn get<'a>(
        &'a self,
        context: &'a core::ExecutionContext<'a>,
    ) -> Ref<'a, HashMap<D, isize>> {
        assert_eq!(self.context_id, context.get_id(), "Context mismatch");
        let received = self.receiver.receive();
        if !received.is_empty() {
            let mut borrowed = self.total.borrow_mut();
            for m in received {
                for (x, count) in m {
                    borrowed.add(x, count);
                }
            }
        }
        self.total.borrow()
    }
}

impl<'a> CreationContext<'a, ()> {
    pub fn new() -> Self {
        Self::new_()
    }
}
