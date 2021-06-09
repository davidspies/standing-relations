use std::{cell::RefCell, mem, rc::Rc};

pub struct Sender<T>(Rc<RefCell<Vec<T>>>);

pub struct Receiver<T>(Rc<RefCell<Vec<T>>>);

impl<T> Sender<T> {
    pub fn send(&self, data: T) {
        self.0.borrow_mut().push(data)
    }
    pub fn send_all<I: IntoIterator<Item = T>>(&self, data: I) {
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