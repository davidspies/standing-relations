mod rc_collection;

use self::rc_collection::RcCollection;
use std::{cell::RefCell, rc::Rc};

pub struct ReceiveBuilder(RcCollection<RefCell<dyn IsNode>>);
pub struct DirtySend(Rc<RefCell<SendNode>>);
pub struct DirtyReceive(Rc<RefCell<Node>>);

#[derive(Default)]
struct SendNode(Option<Rc<RefCell<Node>>>);
#[derive(Default)]
struct Node {
    dirty: bool,
    targets: RcCollection<RefCell<Node>>,
    on_dirty: Vec<Box<dyn FnMut()>>,
}

trait IsNode {
    fn add_target(&mut self, target: Rc<RefCell<Node>>);
}

impl IsNode for SendNode {
    fn add_target(&mut self, target: Rc<RefCell<Node>>) {
        assert!(self.0.is_none());
        self.0 = Some(target);
    }
}

impl IsNode for Node {
    fn add_target(&mut self, target: Rc<RefCell<Node>>) {
        self.targets.insert(target);
    }
}

pub fn new() -> (DirtySend, ReceiveBuilder) {
    let send_node = Default::default();
    (
        DirtySend(Rc::clone(&send_node)),
        ReceiveBuilder(RcCollection::singleton(send_node)),
    )
}

impl ReceiveBuilder {
    pub fn into_receive(self) -> DirtyReceive {
        let result = Default::default();
        for t in self.0 {
            t.borrow_mut().add_target(Rc::clone(&result));
        }
        DirtyReceive(result)
    }
    pub fn or(self, other: Self) -> Self {
        let mut result = self.0;
        result.extend(other.0);
        ReceiveBuilder(result)
    }
}

impl DirtyReceive {
    pub fn add_target(&mut self) -> ReceiveBuilder {
        ReceiveBuilder(RcCollection::singleton(
            Rc::clone(&self.0) as Rc<RefCell<dyn IsNode>>
        ))
    }
    pub fn take_status(&mut self) -> bool {
        let dirty = self.0.borrow().dirty;
        if dirty {
            self.0.borrow_mut().dirty = false;
        }
        dirty
    }
    pub fn add_listener(&mut self, f: impl FnMut() + 'static) {
        self.0.borrow_mut().on_dirty.push(Box::new(f));
    }
}

fn set_dirty_inner(this: &Rc<RefCell<Node>>) {
    if !this.borrow().dirty {
        {
            let mut this = this.borrow_mut();
            this.dirty = true;
            for f in &mut this.on_dirty {
                f()
            }
            for target in &this.targets {
                set_dirty_inner(target);
            }
        }
    }
}

impl DirtySend {
    pub fn set_dirty(&self) {
        if let Some(inner) = self.0.borrow().0.as_ref() {
            set_dirty_inner(inner);
        }
    }
}
