use std::{cell::RefCell, rc::Rc};

pub struct DirtySend(Rc<RefCell<DirtyInner>>);
pub struct DirtyReceive(Rc<RefCell<DirtyInner>>);

struct DirtyInner {
    dirty: bool,
    targets: Vec<Rc<RefCell<DirtyInner>>>,
}

impl DirtyInner {
    fn new() -> Self {
        DirtyInner {
            dirty: false,
            targets: Vec::new(),
        }
    }
}

impl DirtySend {
    pub fn set_dirty(&self) {
        set_dirty_inner(&self.0)
    }
}

fn set_dirty_inner(this: &Rc<RefCell<DirtyInner>>) {
    if this.borrow().dirty {
        return;
    }
    this.borrow_mut().dirty = true;
    for t in &this.borrow().targets {
        set_dirty_inner(t);
    }
}

impl DirtyReceive {
    pub fn add_target(&mut self) -> DirtyReceive {
        let d = Rc::new(RefCell::new(DirtyInner::new()));
        self.0.borrow_mut().targets.push(Rc::clone(&self.0));
        DirtyReceive(d)
    }
    pub fn or(self, other: Self) -> DirtyReceive {
        let d = Rc::new(RefCell::new(DirtyInner::new()));
        self.0.borrow_mut().targets.push(Rc::clone(&self.0));
        other.0.borrow_mut().targets.push(Rc::clone(&other.0));
        DirtyReceive(d)
    }
    pub fn take_status(&self) -> bool {
        if self.0.borrow().dirty {
            self.0.borrow_mut().dirty = false;
            true
        } else {
            false
        }
    }
}

pub fn new() -> (DirtySend, DirtyReceive) {
    let d = Rc::new(RefCell::new(DirtyInner::new()));
    (DirtySend(Rc::clone(&d)), DirtyReceive(d))
}
