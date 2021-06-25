use super::op::{Instruct, IsFeedback};
use crate::{
    core,
    is_context::{ContextSends, IsContext},
    tracked::{self, ChangeTracker},
    Input,
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
    fn send_all_to(&self, input: &Input<'a, D>, data: impl IntoIterator<Item = (D, isize)>) {
        self.deref().send_all_to(input, data)
    }
}

impl<'a, C: IsContext<'a>, I> ExecutionContext_<'a, C, I> {
    pub fn commit(&mut self) -> Option<I> {
        'outer: loop {
            self.inner.as_mut().unwrap().commit();
            for feeder in &mut self.feeders {
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
    pub fn with<'b>(
        &'b mut self,
        setup: impl FnOnce(&mut TrackedContext<'a, I>),
    ) -> With<'b, 'a, I> {
        let mut setup_context = ExecutionContext_ {
            inner: Some(tracked::TrackedContext::new(
                mem::take(&mut self.inner).unwrap(),
            )),
            feeders: mem::take(&mut self.feeders),
        };
        setup(&mut setup_context);
        let interrupted_in_setup = setup_context.commit();
        let (inner, tracker) = setup_context.inner.unwrap().pieces();
        self.inner = Some(inner);
        self.feeders = setup_context.feeders;
        With(Some(WithInner {
            context: self,
            interrupted_in_setup,
            tracker,
        }))
    }
}

pub struct WithInner<'b, 'a, I> {
    context: &'b mut ExecutionContext<'a, I>,
    interrupted_in_setup: Option<I>,
    tracker: ChangeTracker<'a>,
}

pub struct With<'b, 'a, I>(Option<WithInner<'b, 'a, I>>);

impl<I> Drop for With<'_, '_, I> {
    fn drop(&mut self) {
        self.0.take().map(|inner| inner.go(|_, _| ()));
    }
}

impl<'a, I> With<'_, 'a, I> {
    pub fn go<R>(mut self, body: impl FnOnce(&mut ExecutionContext<'a, I>, Option<I>) -> R) -> R {
        self.0.take().unwrap().go(body)
    }
}

impl<'a, I> WithInner<'_, 'a, I> {
    fn go<R>(self, body: impl FnOnce(&mut ExecutionContext<'a, I>, Option<I>) -> R) -> R {
        let result = body(self.context, self.interrupted_in_setup);
        self.tracker.undo(self.context.inner.as_ref().unwrap());
        result
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

impl<'a, C, I> Deref for ExecutionContext_<'a, C, I> {
    type Target = C;

    fn deref(&self) -> &C {
        self.inner.as_ref().unwrap()
    }
}
