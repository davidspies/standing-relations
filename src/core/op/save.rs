use crate::core::{
    context::ContextId,
    dirty::DirtyReceive,
    pipes::{self, Receiver, Sender},
    Op, Relation,
};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

pub struct Save<C: Op> {
    inner: SavedRef<C>,
    receiver: Receiver<Rc<Vec<C::T>>>,
}

struct SaveInner<C: Op> {
    inner: C,
    senders: Vec<Sender<Rc<Vec<C::T>>>>,
    dirty: DirtyReceive,
}

pub(super) struct SavedRef<C: Op>(Rc<RefCell<SaveInner<C>>>);

impl<C: Op> Clone for SavedRef<C> {
    fn clone(&self) -> Self {
        SavedRef(Rc::clone(&self.0))
    }
}

impl<C: Op> SavedRef<C> {
    pub fn new(rel: Relation<C>) -> Self {
        SavedRef(Rc::new(RefCell::new(SaveInner {
            inner: rel.inner,
            senders: Vec::new(),
            dirty: rel.dirty.to_receive(),
        })))
    }
    pub fn to_relation(self, context_id: ContextId) -> Relation<Save<C>>
    where
        C::T: Clone,
    {
        let (sender, receiver) = pipes::new();
        let dirty = {
            let mut borrowed = self.0.borrow_mut();
            borrowed.senders.push(sender);
            borrowed.dirty.add_target()
        };
        Relation {
            context_id,
            dirty,
            inner: Save {
                inner: self,
                receiver,
            },
        }
    }
    pub fn borrow(&self) -> Ref<C> {
        Ref::map(self.0.borrow(), |x| &x.inner)
    }
    pub fn propagate(&self) {
        if self.0.borrow().dirty.take_status() {
            let data = Rc::new(self.0.borrow_mut().inner.get_vec());
            for sender in &self.0.borrow().senders {
                sender.send(Rc::clone(&data))
            }
        }
    }
}

impl<C: Op> Op for Save<C>
where
    C::T: Clone,
{
    type T = C::T;

    fn foreach<'a, F: FnMut(Self::T) + 'a>(&'a mut self, mut continuation: F) {
        self.inner.propagate();
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
        let context_id = self.context_id;
        SavedRef::new(self).to_relation(context_id)
    }
}

impl<C: Op> Clone for Relation<Save<C>>
where
    C::T: Clone,
{
    fn clone(&self) -> Self {
        self.inner.inner.clone().to_relation(self.context_id)
    }
}
