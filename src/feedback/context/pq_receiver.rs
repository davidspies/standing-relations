use std::{cmp::Reverse, collections::BinaryHeap};

use crate::pipes::Receiver;

pub struct PQReceiver {
    receiver: Receiver<usize>,
    pending: BinaryHeap<Reverse<usize>>,
}

impl PQReceiver {
    pub fn new(receiver: Receiver<usize>) -> Self {
        PQReceiver {
            receiver,
            pending: BinaryHeap::new(),
        }
    }
    pub fn insert(&mut self, x: usize) {
        self.pending.push(Reverse(x));
    }
    pub fn pop_min(&mut self) -> Option<usize> {
        self.pending
            .extend(self.receiver.receive().into_iter().map(Reverse));
        self.pending.pop().map(|Reverse(x)| x)
    }
}
