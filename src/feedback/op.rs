mod checked_foreach;
mod pipe;

use self::pipe::{OrderedPipe, Pipe};
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

pub struct FeedbackOnce<'a, C: Op> {
    output: Output<C::D, C, Pipe<(C::D, isize)>>,
    input: Input<'a, C::D>,
}

pub struct FeedbackOrdered<'a, K: Ord, V: Eq + Hash, C: Op<D = (K, V)>> {
    output: Output<(K, V), C, OrderedPipe<K, V>>,
    input: Input<'a, V>,
}

pub struct Interrupter<C: Op, M: CountMap<C::D>, F: Fn(&M) -> Option<I>, I> {
    output: Output<C::D, C, M>,
    f: F,
}

pub(crate) trait IsFeeder<'a, I> {
    fn feed(&mut self, context: &core::ExecutionContext<'a>) -> Instruct<I>;
}

pub(crate) trait IsFeedback<'a, I>: IsFeeder<'a, I> {
    fn add_listener(&mut self, context: &core::CreationContext, f: impl FnMut() + 'static);
}

pub enum Instruct<I> {
    Unchanged,
    Changed,
    Interrupt(I),
}

impl<'a, C: Op, I> IsFeeder<'a, I> for Feedback<'a, C>
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

impl<'a, C: Op, I> IsFeedback<'a, I> for Feedback<'a, C>
where
    C::D: Clone + Eq + Hash + 'a,
{
    fn add_listener(&mut self, context: &core::CreationContext, f: impl FnMut() + 'static) {
        self.output.add_listener(context, f);
    }
}

impl<'a, C: Op, I> IsFeeder<'a, I> for FeedbackOnce<'a, C> {
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

impl<'a, C: Op, I> IsFeedback<'a, I> for FeedbackOnce<'a, C> {
    fn add_listener(&mut self, context: &core::CreationContext, f: impl FnMut() + 'static) {
        self.output.add_listener(context, f);
    }
}

impl<'a, K: Ord, V: Eq + Hash, C: Op<D = (K, V)>, I> IsFeeder<'a, I>
    for FeedbackOrdered<'a, K, V, C>
{
    fn feed(&mut self, context: &core::ExecutionContext<'a>) -> Instruct<I> {
        let m = self.output.get(context);
        match m.receive() {
            None => Instruct::Unchanged,
            Some((_, changes)) => {
                self.input.send_all(context, changes);
                Instruct::Changed
            }
        }
    }
}

impl<'a, K: Ord, V: Eq + Hash, C: Op<D = (K, V)>, I> IsFeedback<'a, I>
    for FeedbackOrdered<'a, K, V, C>
{
    fn add_listener(&mut self, context: &core::CreationContext, f: impl FnMut() + 'static) {
        self.output.add_listener(context, f);
    }
}

impl<'a, C: Op, M: CountMap<C::D>, F: Fn(&M) -> Option<I>, I> IsFeeder<'a, I>
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
impl<'a, C: Op, M: CountMap<C::D>, F: Fn(&M) -> Option<I>, I> IsFeedback<'a, I>
    for Interrupter<C, M, F, I>
{
    fn add_listener(&mut self, context: &core::CreationContext, f: impl FnMut() + 'static) {
        self.output.add_listener(context, f);
    }
}

impl<'a, I> CreationContext<'a, I> {
    /// Connects a `Relation` to an `Input` such that whenever `ExecutionContext::commit` is called,
    /// any changes to the collection represented by the `Relation` argument are fed back into the
    /// `Input` argument. This repeats until the collection stops changing.
    pub fn feed<D>(&mut self, rel: Relation<impl Op<D = D> + 'a>, input: Input<'a, D>) {
        self.add_feeder(FeedbackOnce {
            output: rel.get_output_(&self),
            input,
        })
    }

    /// Similar to `feed` except that the `Relation` argument
    /// additionally has an ordering key. Rather than feeding _all_ changes back into the `Input`, only
    /// those with the minimum present ordering key are fed back in. If any later changes are cancelled
    /// out as a result of this (if their count goes to zero), then they will not be fed in at all.
    /// This can be handy in situations where using `feed` can cause an infinite loop.
    pub fn feed_ordered<K: Ord + 'a, V: Eq + Hash + 'a>(
        &mut self,
        rel: Relation<impl Op<D = (K, V)> + 'a>,
        input: Input<'a, V>,
    ) {
        self.add_feeder(FeedbackOrdered {
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

    /// Takes an `Output` as an argument rather than a `Relation` and rather
    /// than propagating _changes_ to it's argument through will instead send the entire contents of that
    /// `Output` on every visit. `feed_while` is intended to be used in circumstances where there exists
    /// a negative feedback loop between the arguments and the caller wants to retain any visited values
    /// rather than have them be immediately deleted.
    pub fn feed_while<D: Clone + Eq + Hash + 'a>(
        &mut self,
        output: Output<D, impl Op<D = D> + 'a>,
        input: Input<'a, D>,
    ) {
        self.add_feeder(Feedback { output, input })
    }
}
