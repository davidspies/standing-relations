mod with_clones;

use std::{cell::RefCell, rc::Rc, vec};

use crate::{
    dirty::DirtyReceive,
    op::Op,
    pipes::{self, Receiver, Sender},
    relation::Relation,
};

use self::with_clones::WithClones;

pub struct Split<C: Op> {
    inner: Rc<RefCell<SplitInner<C>>>,
    receiver: Receiver<C::T>,
}

struct SplitInner<C: Op> {
    inner: C,
    senders: Vec<Sender<C::T>>,
    dirty: DirtyReceive,
}

impl<C: Op> Op for Split<C>
where
    C::T: Clone,
{
    type T = C::T;

    fn foreach<'a, F: FnMut(Self::T) + 'a>(&'a mut self, mut continuation: F) {
        if self.inner.borrow().dirty.take_status() {
            let data = self.inner.borrow_mut().inner.get_vec();
            for (sender, data) in self.inner.borrow().senders.iter().with_clones(data) {
                sender.send_all(data)
            }
        }
        for x in self.receiver.receive() {
            continuation(x)
        }
    }
}

impl<C: Op> Relation<C> {
    pub fn split(self) -> Relation<Split<C>>
    where
        C::T: Clone,
    {
        let mut this_dirty = self.dirty.to_receive();
        let dirty = this_dirty.add_target();
        let (sender, receiver) = pipes::new();
        Relation {
            context_id: self.context_id,
            dirty,
            inner: Split {
                inner: Rc::new(RefCell::new(SplitInner {
                    inner: self.inner,
                    senders: vec![sender],
                    dirty: this_dirty,
                })),
                receiver,
            },
        }
    }
}

impl<C: Op> Clone for Relation<Split<C>>
where
    C::T: Clone,
{
    fn clone(&self) -> Self {
        let (sender, receiver) = pipes::new();
        let mut inner = self.inner.inner.borrow_mut();
        let dirty = inner.dirty.add_target();
        inner.senders.push(sender);
        Relation {
            context_id: self.context_id,
            dirty,
            inner: Split {
                inner: Rc::clone(&self.inner.inner),
                receiver,
            },
        }
    }
}
