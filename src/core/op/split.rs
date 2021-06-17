use crate::core::{
    dirty::DirtyReceive,
    pipes::{self, Receiver, Sender},
    Op_, Relation,
};
use std::{cell::RefCell, rc::Rc};

pub struct Split<T, C: Op_<T = (LI, RI)>, LI: IntoIterator, RI: IntoIterator> {
    inner: Rc<RefCell<SplitInner<C, LI, RI>>>,
    receiver: Receiver<T>,
}

struct SplitInner<C: Op_<T = (LI, RI)>, LI: IntoIterator, RI: IntoIterator> {
    inner: C,
    left_sender: Sender<LI::Item>,
    right_sender: Sender<RI::Item>,
    dirty: DirtyReceive,
}

impl<T, C: Op_<T = (LI, RI)>, LI: IntoIterator, RI: IntoIterator> Op_ for Split<T, C, LI, RI> {
    type T = T;

    fn foreach<'a, F: FnMut(Self::T) + 'a>(&'a mut self, mut continuation: F) {
        if self.inner.borrow().dirty.take_status() {
            let mut inner = self.inner.borrow_mut();
            let data = inner.inner.get_vec();
            for (xl, xr) in data {
                inner.left_sender.send_all(xl);
                inner.right_sender.send_all(xr)
            }
        }
        for x in self.receiver.receive() {
            continuation(x)
        }
    }
}

impl<C: Op_<T = (LI, RI)>, LI: IntoIterator, RI: IntoIterator> Relation<C> {
    pub fn split_(
        self,
    ) -> (
        Relation<Split<LI::Item, C, LI, RI>>,
        Relation<Split<RI::Item, C, LI, RI>>,
    ) {
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
