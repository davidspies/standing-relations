mod checked_foreach;
mod pipe;

use self::{checked_foreach::CheckedForeach, pipe::Pipe};
use super::context::CreationContext;
use crate::{core, CountMap, Input, Op, Output, Relation};
use std::{collections::HashMap, hash::Hash};

pub struct Feedback<'a, C: Op, F: FnMut(&HashMap<C::D, isize>)>
where
    C::D: Eq + Hash,
{
    output: Output<C::D, C>,
    input: Input<'a, C::D>,
    f: F,
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

pub trait IsFeedback<'a, I> {
    fn feed(&mut self, context: &core::ExecutionContext<'a>) -> Instruct<I>;
}

pub enum Instruct<I> {
    Unchanged,
    Changed,
    Interrupt(I),
}

impl<'a, C: Op, I, F: FnMut(&HashMap<C::D, isize>)> IsFeedback<'a, I> for Feedback<'a, C, F>
where
    C::D: Clone + Eq + Hash,
{
    fn feed(&mut self, context: &core::ExecutionContext<'a>) -> Instruct<I> {
        let m = self.output.get(context);
        if m.is_empty() {
            Instruct::Unchanged
        } else {
            (self.f)(&*m);
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
    fn feed(&mut self, context: &core::ExecutionContext<'a>) -> Instruct<I> {
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
    fn feed(&mut self, context: &core::ExecutionContext<'a>) -> Instruct<I> {
        let m = self.output.get(context);
        match (self.f)(&m) {
            None => Instruct::Unchanged,
            Some(i) => Instruct::Interrupt(i),
        }
    }
}

impl<'a, I> CreationContext<'a, I> {
    pub fn feed_and<C: Op + 'a, F: FnMut(&HashMap<C::D, isize>) + 'a>(
        &mut self,
        rel: Relation<C>,
        input: Input<'a, C::D>,
        f: F,
    ) where
        C::D: Clone + Eq + Hash + 'a,
    {
        self.add_feeder(Feedback {
            output: rel.get_output(&self),
            input,
            f,
        })
    }
    pub fn feed_once<C: Op + 'a>(&mut self, rel: Relation<C>, input: Input<'a, C::D>)
    where
        C::D: Clone + Eq + Hash + 'a,
    {
        self.add_feeder(FeedbackOnce {
            output: rel.get_output_(&self),
            input,
        })
    }
    pub fn interrupt_<C: Op + 'a, M: CountMap<C::D> + 'a, F: Fn(&M) -> Option<I> + 'a>(
        &mut self,
        rel: Relation<C>,
        f: F,
    ) where
        I: 'a,
    {
        self.add_feeder(Interrupter {
            output: rel.get_output_(&self),
            f,
        })
    }
}
