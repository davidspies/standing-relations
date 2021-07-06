mod checked_foreach;
mod pipe;

use self::pipe::Pipe;
use super::context::CreationContext;
use crate::{core, CountMap, Input, Op, Output, Relation};
use std::hash::Hash;

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

pub(crate) trait IsFeedback<'a, I> {
    fn feed(&mut self, context: &core::ExecutionContext<'a>) -> Instruct<I>;
}

pub enum Instruct<I> {
    Unchanged,
    Changed,
    Interrupt(I),
}

impl<'a, C: Op, I> IsFeedback<'a, I> for Feedback<'a, C>
where
    C::D: Clone + Eq + Hash + 'a,
{
    fn feed(&mut self, context: &core::ExecutionContext) -> Instruct<I> {
        let m = self.output.get(context);
        if m.is_empty() {
            Instruct::Unchanged
        } else {
            self.input
                .send_all(context, m.iter().map(|(x, &count)| (x.clone(), count)));
            Instruct::Changed
        }
    }
}

impl<'a, C: Op, I> IsFeedback<'a, I> for FeedbackOnce<'a, C>
where
    C::D: Eq + Hash,
{
    fn feed(&mut self, context: &core::ExecutionContext<'a>) -> Instruct<I> {
        let m = self.output.get(context);
        let changes = m.receive();
        if changes.is_empty() {
            Instruct::Unchanged
        } else {
            self.input.send_all(context, changes);
            Instruct::Changed
        }
    }
}

impl<'a, C: Op, M: CountMap<C::D>, F: Fn(&M) -> Option<I>, I> IsFeedback<'a, I>
    for Interrupter<C, M, F, I>
{
    fn feed(&mut self, context: &core::ExecutionContext<'a>) -> Instruct<I> {
        let m = self.output.get(context);
        match (self.f)(&m) {
            None => Instruct::Unchanged,
            Some(i) => Instruct::Interrupt(i),
        }
    }
}

impl<'a, I> CreationContext<'a, I> {
    pub fn feed<D: Clone + Eq + Hash + 'a>(
        &mut self,
        output: Output<D, impl Op<D = D> + 'a>,
        input: Input<'a, D>,
    ) {
        self.add_feeder(Feedback { output, input })
    }
    pub fn feed_once<C: Op + 'a>(&mut self, rel: Relation<C>, input: Input<'a, C::D>)
    where
        C::D: Eq + Hash,
    {
        self.add_feeder(FeedbackOnce {
            output: rel.get_output_(&self),
            input,
        })
    }
    pub fn interrupt_<D, M: CountMap<D> + 'a>(
        &mut self,
        output: Output<D, impl Op<D = D> + 'a, M>,
        f: impl Fn(&M) -> Option<I> + 'a,
    ) where
        I: 'a,
    {
        self.add_feeder(Interrupter { output, f })
    }
}
