mod checked_foreach;
mod pipe;

use self::{checked_foreach::CheckedForeach, pipe::Pipe};
use crate::{
    core,
    is_context::{ContextSends, IsContext},
    tracked, CountMap, Input, Op, Output, Relation,
};
use std::{hash::Hash, mem, ops::Deref};

pub struct Feedback<'a, C: Op>
where
    C::D: Eq + Hash,
{
    output: Output<C::D, C>,
    input: Input<'a, C::D>,
}

pub struct FeedbackOnce<'a, C: Op>
where
    C::D: Eq + Hash,
{
    output: Output<C::D, C, Pipe<(C::D, isize)>>,
    input: Input<'a, C::D>,
}

pub struct Interrupter<C: Op, M: CountMap<C::D>, F: Fn(&M) -> Option<I>, I> {
    output: Output<C::D, C, M>,
    f: F,
}

pub struct CreationContext<'a, I> {
    inner: core::CreationContext<'a>,
    feeders: Vec<Box<dyn IsFeedback<'a, I> + 'a>>,
}

pub struct ExecutionContext_<'a, C, I> {
    inner: Option<C>,
    feeders: Vec<Box<dyn IsFeedback<'a, I> + 'a>>,
}

pub type ExecutionContext<'a, I> = ExecutionContext_<'a, core::ExecutionContext<'a>, I>;
pub type TrackedContext<'a, I> = ExecutionContext_<'a, tracked::TrackedContext<'a>, I>;

impl<'a, C: ContextSends<'a, D>, I, D> ContextSends<'a, D> for ExecutionContext_<'a, C, I> {
    fn update_to(&self, input: &Input<'a, D>, x: D, count: isize) {
        self.deref().update_to(input, x, count)
    }
    fn send_all_to<Iter: IntoIterator<Item = (D, isize)>>(&self, input: &Input<'a, D>, data: Iter) {
        self.deref().send_all_to(input, data)
    }
}

trait IsFeedback<'a, I> {
    fn feed(&self, context: &core::ExecutionContext<'a>) -> Instruct<I>;
}

enum Instruct<I> {
    Unchanged,
    Changed,
    Interrupt(I),
}

impl<'a, C: Op, I> IsFeedback<'a, I> for Feedback<'a, C>
where
    C::D: Clone + Eq + Hash,
{
    fn feed(&self, context: &core::ExecutionContext<'a>) -> Instruct<I> {
        let m = self.output.get(context);
        if m.is_empty() {
            Instruct::Unchanged
        } else {
            for (x, &count) in &*m {
                self.input.update(context, x.clone(), count);
            }
            Instruct::Changed
        }
    }
}

impl<'a, C: Op, I> IsFeedback<'a, I> for FeedbackOnce<'a, C>
where
    C::D: Clone + Eq + Hash,
{
    fn feed(&self, context: &core::ExecutionContext<'a>) -> Instruct<I> {
        let m = self.output.get(context);
        if m.receive()
            .into_iter()
            .checked_foreach(|(x, count)| self.input.update(context, x, count))
        {
            Instruct::Changed
        } else {
            Instruct::Unchanged
        }
    }
}

impl<'a, C: Op, M: CountMap<C::D>, F: Fn(&M) -> Option<I>, I> IsFeedback<'a, I>
    for Interrupter<C, M, F, I>
{
    fn feed(&self, context: &core::ExecutionContext<'a>) -> Instruct<I> {
        let m = self.output.get(context);
        match (self.f)(&m) {
            None => Instruct::Unchanged,
            Some(i) => Instruct::Interrupt(i),
        }
    }
}

impl<'a, C: IsContext<'a>, I> ExecutionContext_<'a, C, I> {
    pub fn commit(&mut self) -> Option<I> {
        'outer: loop {
            self.inner.as_mut().unwrap().commit();
            for feeder in &self.feeders {
                match feeder.feed(self.core_context()) {
                    Instruct::Unchanged => (),
                    Instruct::Changed => continue 'outer,
                    Instruct::Interrupt(interrupted) => return Some(interrupted),
                }
            }
            return None;
        }
    }
}

impl<'a, I> ExecutionContext<'a, I> {
    pub fn with<
        Setup: FnOnce(&mut TrackedContext<'a, I>),
        Body: FnOnce(&mut Self, Option<I>) -> Result,
        Result,
    >(
        &mut self,
        setup: Setup,
        body: Body,
    ) -> (Result, Option<I>) {
        let mut setup_context = ExecutionContext_ {
            inner: Some(tracked::TrackedContext::new(
                mem::take(&mut self.inner).unwrap(),
            )),
            feeders: mem::take(&mut self.feeders),
        };
        setup(&mut setup_context);
        let interrupted = setup_context.commit();
        let (inner, tracker) = setup_context.inner.unwrap().pieces();
        self.inner = Some(inner);
        self.feeders = setup_context.feeders;
        let result = body(self, interrupted);
        tracker.undo(self.inner.as_ref().unwrap());
        (result, self.commit())
    }
}

impl<'a, I> CreationContext<'a, I> {
    pub fn new_() -> Self {
        CreationContext {
            inner: core::CreationContext::new(),
            feeders: Vec::new(),
        }
    }
    pub fn begin(self) -> ExecutionContext<'a, I> {
        ExecutionContext {
            inner: Some(self.inner.begin()),
            feeders: self.feeders,
        }
    }
    pub fn feed<C: Op + 'a>(&mut self, rel: Relation<C>, input: Input<'a, C::D>)
    where
        C::D: Clone + Eq + Hash + 'a,
    {
        self.feeders.push(Box::new(Feedback {
            output: rel.get_output(&self),
            input,
        }))
    }
    pub fn feed_once<C: Op + 'a>(&mut self, rel: Relation<C>, input: Input<'a, C::D>)
    where
        C::D: Clone + Eq + Hash + 'a,
    {
        self.feeders.push(Box::new(FeedbackOnce {
            output: rel.get_output_(&self),
            input,
        }))
    }
    pub fn interrupt_<C: Op + 'a, M: CountMap<C::D> + 'a, F: Fn(&M) -> Option<I> + 'a>(
        &mut self,
        rel: Relation<C>,
        f: F,
    ) where
        I: 'a,
    {
        self.feeders.push(Box::new(Interrupter {
            output: rel.get_output_(&self),
            f,
        }))
    }
}

impl<'a, I> Deref for CreationContext<'a, I> {
    type Target = core::CreationContext<'a>;

    fn deref(&self) -> &core::CreationContext<'a> {
        &self.inner
    }
}

impl<'a, C, I> Deref for ExecutionContext_<'a, C, I> {
    type Target = C;

    fn deref(&self) -> &C {
        self.inner.as_ref().unwrap()
    }
}
