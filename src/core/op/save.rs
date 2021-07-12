use crate::core::{
    context::{ContextTracker, TrackIndex},
    dirty::DirtyReceive,
    pipes::{self, Receiver, Sender},
    relation::RelationInner,
    Op_, Relation,
};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

pub struct Save<C: Op_> {
    inner: Saved<C>,
    receiver: Receiver<Rc<Vec<C::T>>>,
}

struct SaveInner<C: Op_> {
    inner: RelationInner<C>,
    senders: Vec<Sender<Rc<Vec<C::T>>>>,
    dirty: DirtyReceive,
}

pub struct Saved<C: Op_> {
    context_tracker: ContextTracker,
    inner: Rc<RefCell<SaveInner<C>>>,
    track_index: TrackIndex,
}

impl<C: Op_> Clone for Saved<C> {
    fn clone(&self) -> Self {
        Saved {
            context_tracker: self.context_tracker.clone(),
            inner: Rc::clone(&self.inner),
            track_index: self.track_index.clone(),
        }
    }
}

impl<C: Op_> Saved<C> {
    pub fn new(rel: Relation<C>) -> Self {
        Saved {
            context_tracker: rel.context_tracker,
            inner: Rc::new(RefCell::new(SaveInner {
                inner: rel.inner,
                senders: Vec::new(),
                dirty: rel.dirty.to_receive(),
            })),
            track_index: rel.track_index,
        }
    }
    pub fn get_shown(&self) -> Relation<Save<C>>
    where
        C::T: Clone,
    {
        let (sender, receiver) = pipes::new();
        let dirty = {
            let mut borrowed = self.inner.borrow_mut();
            borrowed.senders.push(sender);
            borrowed.dirty.add_target()
        };
        self.context_tracker.clone().add_relation(
            dirty,
            Save {
                inner: self.clone(),
                receiver,
            },
            vec![self.track_index.clone()],
        )
    }
    pub(super) fn borrow(&self) -> Ref<RelationInner<C>> {
        Ref::map(self.inner.borrow(), |x| &x.inner)
    }
    pub(super) fn propagate(&self) {
        if self.inner.borrow().dirty.take_status() {
            let data = Rc::new(self.inner.borrow_mut().inner.get_vec());
            for sender in &self.inner.borrow().senders {
                sender.send(Rc::clone(&data))
            }
        }
    }
}

impl<C: Op_> Op_ for Save<C>
where
    C::T: Clone,
{
    type T = C::T;

    fn foreach<'a>(&'a mut self, mut continuation: impl FnMut(Self::T) + 'a) {
        self.inner.propagate();
        for data in self.receiver.receive() {
            for x in &*data {
                continuation(x.clone())
            }
        }
    }

    fn get_type_name() -> &'static str {
        "save"
    }
}

impl<C: Op_> Relation<C> {
    pub fn save(self) -> Saved<C>
    where
        C::T: Clone,
    {
        Saved::new(self)
    }
}

impl<C: Op_> Clone for Relation<Save<C>>
where
    C::T: Clone,
{
    fn clone(&self) -> Self {
        self.inner.inner.inner.get()
    }
}
