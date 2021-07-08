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
    /// Feed the output of one relation into the input of another.
    ///
    /// When calling `context.commit`, anything in the output will be dumped into the input
    /// and this operation will repeat
    /// until the output is empty. Therefore, in order to ensure that the `commit` operation halts,
    /// it is the caller's responsibility to structure the relation in such a way that there is
    /// a negative feedback loop between the arguments.
    pub fn feed_while<D: Clone + Eq + Hash + 'a>(
        &mut self,
        output: Output<D, impl Op<D = D> + 'a>,
        input: Input<'a, D>,
    ) {
        self.add_feeder(Feedback { output, input })
    }

    /// Connect a relation directly into an input without consolidating.
    ///
    /// Whereas `feed_while` feeds the collection _output_ into the input every time `commit` is
    /// called, `feed` feeds the _changes_ to the collection into the input every time `commit` is
    /// called. This operation will repeat until the collection stops changing.
    /// Like with `feed_while`, it is the caller's responsibility to structure things in such a way
    /// that `commit` calls will halt.
    pub fn feed<D>(&mut self, rel: Relation<impl Op<D = D> + 'a>, input: Input<'a, D>) {
        self.add_feeder(FeedbackOnce {
            output: rel.get_output_(&self),
            input,
        })
    }

    /// `feed_ordered` is like `feed`, but has an ordering key to help ensure `commit` halts.
    ///
    /// Upon calling `commit`, only the changes with the lowest key will be fed back into the input.
    /// These are allowed to propagate through the system before feeding any more changes in.
    /// If any remaining pending changes are cancelled out (in a delta=0 sense) by this, then
    /// they will never be fed in.
    /// Like with `feed`, this repeats until the collection stops changing.
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
}
