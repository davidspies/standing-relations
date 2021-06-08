use std::{cell::RefCell, rc::Rc};

use crate::{
    dirty::DirtyReceive,
    pipes::{self, Receiver, Sender},
    Op, Relation,
};

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub struct Split<T, C: Op<T = Either<L, R>>, L, R> {
    inner: Rc<RefCell<SplitInner<C, L, R>>>,
    receiver: Receiver<T>,
}

struct SplitInner<C: Op<T = Either<L, R>>, L, R> {
    inner: C,
    left_sender: Sender<L>,
    right_sender: Sender<R>,
    dirty: DirtyReceive,
}

impl<T, C: Op<T = Either<L, R>>, L, R> Op for Split<T, C, L, R> {
    type T = T;

    fn foreach<'a, F: FnMut(Self::T) + 'a>(&'a mut self, mut continuation: F) {
        if self.inner.borrow().dirty.take_status() {
            let mut inner = self.inner.borrow_mut();
            let data = inner.inner.get_vec();
            for x in data {
                match x {
                    Either::Left(l) => inner.left_sender.send(l),
                    Either::Right(r) => inner.right_sender.send(r),
                }
            }
        }
        for x in self.receiver.receive() {
            continuation(x)
        }
    }
}

impl<C: Op<T = Either<L, R>>, L, R> Relation<C> {
    pub fn split_(self) -> (Relation<Split<L, C, L, R>>, Relation<Split<R, C, L, R>>) {
        let mut this_dirty = self.dirty.to_receive();
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
        let left_result = Relation {
            context_id: self.context_id,
            dirty: left_dirty,
            inner: Split {
                inner: Rc::clone(&inner),
                receiver: left_receiver,
            },
        };
        let right_result = Relation {
            context_id: self.context_id,
            dirty: right_dirty,
            inner: Split {
                inner,
                receiver: right_receiver,
            },
        };
        (left_result, right_result)
    }
}
