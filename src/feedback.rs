mod checked_foreach;
mod pipe;

use self::{checked_foreach::CheckedForeach, pipe::Pipe};
use crate::{context_sends::ContextSends, core, Input, Op, Output, Relation};
use std::{collections::HashMap, hash::Hash, ops::Deref};

pub struct Feedback<'a, C: Op<T = (D, isize)>, D: Eq + Hash> {
    output: Output<D, C>,
    input: Input<'a, C::T>,
}

pub struct FeedbackOnce<'a, C: Op<T = (D, isize)>, D: Eq + Hash> {
    output: Output<D, C, Pipe<(D, isize)>>,
    input: Input<'a, C::T>,
}

pub struct Interrupter<C: Op<T = (D, isize)>, D: Eq + Hash, F: Fn(&HashMap<D, isize>) -> I, I> {
    output: Output<D, C>,
    f: F,
}

pub struct CreationContext<'a, I> {
    inner: core::CreationContext<'a>,
    feeders: Vec<Box<dyn IsFeedback<'a, I> + 'a>>,
}

pub struct ExecutionContext<'a, I> {
    inner: core::ExecutionContext<'a>,
    feeders: Vec<Box<dyn IsFeedback<'a, I> + 'a>>,
}

impl<'a, I, D> ContextSends<'a, D> for ExecutionContext<'a, I> {
    fn update_to(&self, input: &Input<'a, (D, isize)>, x: D, count: isize) {
        input.send(self, (x, count))
    }
    fn send_all_to<Iter: IntoIterator<Item = (D, isize)>>(
        &self,
        input: &Input<'a, (D, isize)>,
        data: Iter,
    ) {
        input.send_all(self, data)
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

impl<'a, C: Op<T = (D, isize)>, D: Clone + Eq + Hash, I> IsFeedback<'a, I> for Feedback<'a, C, D> {
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

impl<'a, C: Op<T = (D, isize)>, D: Clone + Eq + Hash, I> IsFeedback<'a, I>
    for FeedbackOnce<'a, C, D>
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

impl<'a, C: Op<T = (D, isize)>, D: Eq + Hash, F: Fn(&HashMap<D, isize>) -> I, I> IsFeedback<'a, I>
    for Interrupter<C, D, F, I>
{
    fn feed(&self, context: &core::ExecutionContext<'a>) -> Instruct<I> {
        let m = self.output.get(context);
        if m.is_empty() {
            Instruct::Unchanged
        } else {
            Instruct::Interrupt((self.f)(&m))
        }
    }
}

impl<'a, I> ExecutionContext<'a, I> {
    pub fn commit(&mut self) -> Option<I> {
        'outer: loop {
            self.inner.commit();
            for feeder in &self.feeders {
                match feeder.feed(&self.inner) {
                    Instruct::Unchanged => (),
                    Instruct::Changed => continue 'outer,
                    Instruct::Interrupt(interrupted) => return Some(interrupted),
                }
            }
            return None;
        }
    }
}

impl<'a> CreationContext<'a, ()> {
    pub fn new() -> Self {
        Self::new_()
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
            inner: self.inner.begin(),
            feeders: self.feeders,
        }
    }
    pub fn feed<C: Op<T = (D, isize)> + 'a, D: Clone + Eq + Hash + 'a>(
        &mut self,
        rel: Relation<C>,
        input: Input<'a, (D, isize)>,
    ) {
        self.feeders.push(Box::new(Feedback {
            output: rel.get_output(),
            input,
        }))
    }
    pub fn feed_once<C: Op<T = (D, isize)> + 'a, D: Clone + Eq + Hash + 'a>(
        &mut self,
        rel: Relation<C>,
        input: Input<'a, (D, isize)>,
    ) {
        self.feeders.push(Box::new(FeedbackOnce {
            output: rel.get_output_(),
            input,
        }))
    }
}

impl<'a, I> Deref for CreationContext<'a, I> {
    type Target = core::CreationContext<'a>;

    fn deref(&self) -> &core::CreationContext<'a> {
        &self.inner
    }
}
impl<'a, I> Deref for ExecutionContext<'a, I> {
    type Target = core::ExecutionContext<'a>;

    fn deref(&self) -> &core::ExecutionContext<'a> {
        &self.inner
    }
}
