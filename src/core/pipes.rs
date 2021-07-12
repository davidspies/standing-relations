use std::{
    cell::RefCell,
    mem,
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

pub struct Sender<T>(Rc<RefCell<Vec<T>>>);

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender(Rc::clone(&self.0))
    }
}

pub struct Receiver<T>(Rc<RefCell<Vec<T>>>);

impl<T> Sender<T> {
    pub fn send(&self, data: T) {
        self.0.borrow_mut().push(data)
    }
    pub fn send_all(&self, data: impl IntoIterator<Item = T>) {
        self.0.borrow_mut().extend(data)
    }
}

impl<T> Receiver<T> {
    pub fn receive(&self) -> Vec<T> {
        mem::take(&mut self.0.borrow_mut())
    }
}

pub fn new<T>() -> (Sender<T>, Receiver<T>) {
    let rc = Rc::new(RefCell::new(Vec::new()));
    (Sender(Rc::clone(&rc)), Receiver(rc))
}

#[derive(Clone)]
pub struct CountSender(Arc<AtomicUsize>);
impl CountSender {
    pub fn increment(&self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

#[derive(Clone)]
pub struct CountReceiver(Arc<AtomicUsize>);
impl CountReceiver {
    pub fn get(&self) -> usize {
        self.0.load(Ordering::Relaxed)
    }
}

pub fn new_count() -> (CountSender, CountReceiver) {
    let arc = Arc::new(AtomicUsize::new(0));
    (CountSender(Arc::clone(&arc)), CountReceiver(arc))
}
