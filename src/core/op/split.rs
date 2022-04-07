use std::{cell::RefCell, rc::Rc};

use crate::core::{
    dirty::DirtyReceive,
    pipes::{self, Receiver, Sender},
    relation::RelationInner,
    Op_, Relation,
};

pub struct Split<T, C: Op_<T = (L, R)>, L, R> {
    inner: Rc<RefCell<SplitInner<C, L, R>>>,
    receiver: Receiver<T>,
}

struct SplitInner<C: Op_<T = (L, R)>, L, R> {
    inner: RelationInner<C>,
    left_sender: Sender<L>,
    right_sender: Sender<R>,
    dirty: DirtyReceive,
}

impl<T, C: Op_<T = (L, R)>, L, R> Op_ for Split<T, C, L, R> {
    type T = T;

    fn foreach<'a>(&'a mut self, mut continuation: impl FnMut(Self::T) + 'a) {
        if self.inner.borrow_mut().dirty.take_status() {
            let mut inner = self.inner.borrow_mut();
            let data = inner.inner.get_vec();
            for (xl, xr) in data {
                inner.left_sender.send(xl);
                inner.right_sender.send(xr)
            }
        }
        for x in self.receiver.receive() {
            continuation(x)
        }
    }

    fn get_type_name() -> &'static str {
        "split"
    }
}

#[allow(clippy::type_complexity)]
impl<C: Op_<T = (L, R)>, L, R> Relation<C> {
    pub fn split_(self) -> (Relation<Split<L, C, L, R>>, Relation<Split<R, C, L, R>>) {
        let mut this_dirty = self.dirty.into_receive();
        let left_dirty = this_dirty.add_target();
        let right_dirty = this_dirty.add_target();
        let (left_sender, left_receiver) = pipes::new();
        let (right_sender, right_receiver) = pipes::new();
        let inner = Rc::new(RefCell::new(SplitInner {
            inner: self.inner,
            left_sender,
            right_sender,
            dirty: this_dirty,
        }));
        let left_result = self.context_tracker.clone().add_relation(
            left_dirty,
            Split {
                inner: Rc::clone(&inner),
                receiver: left_receiver,
            },
            vec![self.tracking_index],
        );
        let right_result = self.context_tracker.add_relation(
            right_dirty,
            Split {
                inner,
                receiver: right_receiver,
            },
            vec![self.tracking_index],
        );
        (left_result, right_result)
    }
}
