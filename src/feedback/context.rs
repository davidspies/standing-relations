use super::op::{Instruct, IsFeedback};
use crate::core;
use std::ops::Deref;

pub struct CreationContext<'a, I> {
    inner: core::CreationContext<'a>,
    feeders: Vec<Box<dyn IsFeedback<'a, I> + 'a>>,
}

pub struct ExecutionContext<'a, I> {
    inner: core::ExecutionContext<'a>,
    feeders: Vec<Box<dyn IsFeedback<'a, I> + 'a>>,
}

impl<'a, I> ExecutionContext<'a, I> {
    pub fn commit(&mut self) -> Option<I> {
        'outer: loop {
            self.inner.commit();
            let inner_context = &self.inner;
            for feeder in &mut self.feeders {
                match feeder.feed(inner_context) {
                    Instruct::Unchanged => (),
                    Instruct::Changed => continue 'outer,
                    Instruct::Interrupt(interrupted) => return Some(interrupted),
                }
            }
            return None;
        }
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
    pub(super) fn add_feeder(&mut self, feeder: impl IsFeedback<'a, I> + 'a) {
        self.feeders.push(Box::new(feeder));
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
