use std::collections::BTreeSet;

use crate::pipes::Receiver;

pub struct PQReceiver<T: Ord> {
    receiver: Receiver<T>,
    pending: BTreeSet<T>,
}

impl<T: Ord> PQReceiver<T> {
    pub fn new(receiver: Receiver<T>) -> Self {
        PQReceiver {
            receiver,
            pending: BTreeSet::new(),
        }
    }
    pub fn insert(&mut self, x: T) {
        self.pending.insert(x);
    }
    pub fn pop_min(&mut self) -> Option<T> {
        self.pending.extend(self.receiver.receive());
        self.pending.pop_first()
    }
}
