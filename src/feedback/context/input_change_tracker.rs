use std::{collections::HashMap, hash::Hash};

use crate::{CountMap, Input};

use super::ExecutionContext;

pub(super) trait IsInputChangeTracker<I> {
    fn push_frame(&mut self);
    fn pop_frame(&mut self, context: &ExecutionContext<I>);
}

pub(super) struct InputChangeTracker<'a, D> {
    input: Input<'a, D>,
    reversion_stack: Vec<HashMap<D, isize>>,
}

impl<D, I> IsInputChangeTracker<I> for InputChangeTracker<'_, D> {
    fn push_frame(&mut self) {
        self.reversion_stack.push(HashMap::new())
    }
    fn pop_frame(&mut self, context: &ExecutionContext<I>) {
        if let Some(changes) = self.reversion_stack.pop() {
            self.input
                .silent_send_all(context, changes.into_iter().collect())
        }
    }
}

impl<'a, D> InputChangeTracker<'a, D> {
    pub(super) fn new(input: Input<'a, D>) -> Self {
        Self {
            input,
            reversion_stack: Vec::new(),
        }
    }
    pub(super) fn add_changes(&mut self, kvs: &[(D, isize)])
    where
        D: Eq + Hash + Clone,
    {
        if let Some(change) = self.reversion_stack.last_mut() {
            for &(ref k, v) in kvs {
                change.add(k.clone(), -v);
            }
        }
    }
}
