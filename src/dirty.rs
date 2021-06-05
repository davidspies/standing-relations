mod rc_collection;

use std::{cell::RefCell, rc::Rc};

use self::rc_collection::RcCollection;

pub struct ReceiveBuilder(RcCollection<RefCell<dyn IsNode>>);
pub struct DirtySend(Rc<RefCell<SendNode>>);
pub struct DirtyReceive(Rc<RefCell<Node>>);

struct SendNode(Option<Rc<RefCell<Node>>>);
struct Node {
    dirty: bool,
    targets: RcCollection<RefCell<Node>>,
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
    let send_node = Rc::new(RefCell::new(SendNode(None)));
    (
        DirtySend(Rc::clone(&send_node)),
        ReceiveBuilder(RcCollection::singleton(send_node)),
    )
}

impl ReceiveBuilder {
    pub fn to_receive(self) -> DirtyReceive {
        let result = Rc::new(RefCell::new(Node {
            dirty: false,
            targets: RcCollection::new(),
        }));
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
    pub fn take_status(&self) -> bool {
        let dirty = self.0.borrow().dirty;
        if dirty {
            self.0.borrow_mut().dirty = false;
        }
        dirty
    }
}

fn set_dirty_inner(this: &Rc<RefCell<Node>>) {
    if !this.borrow().dirty {
        this.borrow_mut().dirty = true;
        for target in &this.borrow().targets {
            set_dirty_inner(target);
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
