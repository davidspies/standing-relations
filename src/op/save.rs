use std::{cell::RefCell, rc::Rc, vec};

use crate::{
    dirty::DirtyReceive,
    op::Op,
    pipes::{self, Receiver, Sender},
    relation::Relation,
};

pub struct Save<C: Op> {
    inner: Rc<RefCell<SaveInner<C>>>,
    receiver: Receiver<Rc<Vec<C::T>>>,
}

struct SaveInner<C: Op> {
    inner: C,
    senders: Vec<Sender<Rc<Vec<C::T>>>>,
    dirty: DirtyReceive,
}

impl<C: Op> Op for Save<C>
where
    C::T: Clone,
{
    type T = C::T;

    fn foreach<'a, F: FnMut(Self::T) + 'a>(&'a mut self, mut continuation: F) {
        if self.inner.borrow().dirty.take_status() {
            let data = Rc::new(self.inner.borrow_mut().inner.get_vec());
            for sender in &self.inner.borrow().senders {
                sender.send(Rc::clone(&data))
            }
        }
        for data in self.receiver.receive() {
            for x in &*data {
                continuation(x.clone())
            }
        }
    }
}

impl<C: Op> Relation<C> {
    pub fn save(self) -> Relation<Save<C>>
    where
        C::T: Clone,
    {
        let mut this_dirty = self.dirty.to_receive();
        let dirty = this_dirty.add_target();
        let (sender, receiver) = pipes::new();
        Relation {
            context_id: self.context_id,
            dirty,
            inner: Save {
                inner: Rc::new(RefCell::new(SaveInner {
                    inner: self.inner,
                    senders: vec![sender],
                    dirty: this_dirty,
                })),
                receiver,
            },
        }
    }
}

impl<C: Op> Clone for Relation<Save<C>>
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
            inner: Save {
                inner: Rc::clone(&self.inner.inner),
                receiver,
            },
        }
    }
}
