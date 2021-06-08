use std::{collections::HashMap, hash::Hash, ops::Deref};

use crate::{CreationContext, ExecutionContext, InputSender, Op, Output};

pub struct Feedback<'a, C: Op<T = (D, isize)>, D: Eq + Hash> {
    output: Output<D, C>,
    input: InputSender<'a, C::T>,
}

pub struct Interrupter<C: Op<T = (D, isize)>, D: Eq + Hash, F: Fn(&HashMap<D, isize>) -> I, I> {
    output: Output<D, C>,
    f: F,
}

pub struct FeedbackContext<'a, I, C> {
    inner: C,
    feeders: Vec<Box<dyn IsFeedback<I> + 'a>>,
}

trait IsFeedback<I> {
    fn feed(&self, context: &ExecutionContext) -> Instruct<I>;
}

enum Instruct<I> {
    Unchanged,
    Changed,
    Interrupt(I),
}

impl<C: Op<T = (D, isize)>, D: Clone + Eq + Hash, I> IsFeedback<I> for Feedback<'_, C, D> {
    fn feed(&self, context: &ExecutionContext) -> Instruct<I> {
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

impl<C: Op<T = (D, isize)>, D: Eq + Hash, F: Fn(&HashMap<D, isize>) -> I, I> IsFeedback<I>
    for Interrupter<C, D, F, I>
{
    fn feed(&self, context: &ExecutionContext) -> Instruct<I> {
        let m = self.output.get(context);
        if m.is_empty() {
            Instruct::Unchanged
        } else {
            Instruct::Interrupt((self.f)(&m))
        }
    }
}

impl<'a, I> FeedbackContext<'a, I, ExecutionContext<'a>> {
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

impl<'a> FeedbackContext<'a, (), CreationContext<'a>> {
    pub fn new() -> Self {
        Self::new_()
    }
}

impl<'a, I> FeedbackContext<'a, I, CreationContext<'a>> {
    pub fn new_() -> Self {
        FeedbackContext {
            inner: CreationContext::new(),
            feeders: Vec::new(),
        }
    }
    pub fn begin(self) -> FeedbackContext<'a, I, ExecutionContext<'a>> {
        FeedbackContext {
            inner: self.inner.begin(),
            feeders: self.feeders,
        }
    }
    pub fn feed<C: Op<T = (D, isize)> + 'a, D: Clone + Eq + Hash + 'a>(
        &mut self,
        output: Output<D, C>,
        input: InputSender<'a, (D, isize)>,
    ) {
        self.feeders.push(Box::new(Feedback { output, input }))
    }
}

impl<I, C> Deref for FeedbackContext<'_, I, C> {
    type Target = C;

    fn deref(&self) -> &C {
        &self.inner
    }
}
