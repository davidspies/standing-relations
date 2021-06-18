use super::op::{Instruct, IsFeedback};
use crate::{
    core,
    is_context::{ContextSends, IsContext},
    tracked, Input,
};
use std::{mem, ops::Deref};

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

impl<'a, C: IsContext<'a>, I> ExecutionContext_<'a, C, I> {
    pub fn commit(&mut self) -> Option<I> {
        'outer: loop {
            self.inner.as_mut().unwrap().commit();
            for feeder in &self.feeders {
                match feeder.feed(self.inner.as_mut().unwrap().core_context()) {
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
    pub(super) fn add_feeder<F: IsFeedback<'a, I> + 'a>(&mut self, feeder: F) {
        self.feeders.push(Box::new(feeder));
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
