use crate::{
    core,
    pipes::{self, Receiver},
    ContextId, CountMap, CreationContext, Input, Op, Output, Relation,
};
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
    pub fn feed<C: Op + 'a>(&mut self, rel: Relation<C>, input: Input<'a, C::D>)
    where
        C::D: Clone + Eq + Hash + 'a,
    {
        self.feed_and(rel, input, |_, _| ())
    }
    pub fn feed_and_track<M: CountMap<C2::D>, C1: Op + 'a, C2: Op + 'a>(
        &mut self,
        rel: Relation<C1>,
        input: Input<'a, C1::D>,
        tracked: Output<C2::D, C2>,
    ) -> TrackedOutput<C2::D, M>
    where
        C1::D: Clone + Eq + Hash + 'a,
        C2::D: Clone + Eq + Hash + 'a,
    {
        let context_id = rel.get_context_id();
        let (sender, receiver) = pipes::new();
        self.feed_and(rel, input, move |context, _| {
            sender.send(tracked.get(context).clone())
        });
        TrackedOutput {
            context_id,
            receiver,
            total: RefCell::new(M::empty()),
        }
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
