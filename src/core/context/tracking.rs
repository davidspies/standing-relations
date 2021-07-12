use crate::core::{
    dirty::ReceiveBuilder, pipes, pipes::CountReceiver, relation::RelationInner, Op_, Relation,
};
use std::{
    cell::{RefCell, RefMut},
    fmt::{self, Debug, Display},
    ptr,
    rc::Rc,
};

#[derive(Clone)]
pub struct TrackIndex(usize);

impl TrackIndex {
    fn new(i: usize) -> Self {
        TrackIndex(i)
    }
}

impl Display for TrackIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

pub struct ContextTracker(Rc<RefCell<ContextTrackerInner>>);
struct TrackingInfo {
    name: String,
    type_name: String,
    hidden: bool,
    count: CountReceiver,
    deps: Vec<TrackIndex>,
}
struct ContextTrackerInner(Vec<TrackingInfo>);

impl PartialEq for ContextTracker {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.0.as_ptr(), other.0.as_ptr())
    }
}
impl Eq for ContextTracker {}
impl Clone for ContextTracker {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}
impl ContextTracker {
    pub(super) fn new() -> Self {
        ContextTracker(Rc::new(RefCell::new(ContextTrackerInner(Vec::new()))))
    }
    pub(in crate::core) fn add_relation<C: Op_>(
        self,
        dirty: ReceiveBuilder,
        inner: C,
        deps: Vec<TrackIndex>,
    ) -> Relation<C> {
        let (count_send, count_receive) = pipes::new_count();
        let track_index = TrackIndex::new(self.0.borrow().0.len());
        self.0.borrow_mut().0.push(TrackingInfo {
            name: format!("relation{}", track_index),
            type_name: C::get_type_name().to_string(),
            hidden: false,
            count: count_receive,
            deps,
        });
        Relation {
            context_tracker: self,
            shown_index: track_index.clone(),
            track_index,
            dirty,
            inner: RelationInner::new(inner, count_send),
        }
    }
    fn borrow_mut<'a>(&'a self, track_index: &TrackIndex) -> RefMut<'a, TrackingInfo> {
        RefMut::map(self.0.borrow_mut(), |v| &mut v.0[track_index.0])
    }
}
impl Debug for ContextTracker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0.as_ptr())
    }
}

impl ContextTrackerInner {
    fn find_shown_index<'a>(&'a self, mut ind: &'a TrackIndex) -> &'a TrackIndex {
        loop {
            let t = &self.0[ind.0];
            if t.hidden {
                assert_eq!(
                    t.deps.len(),
                    1,
                    "unreachable; hidden node with non-one deps"
                );
                ind = &t.deps[0];
            } else {
                return ind;
            }
        }
    }
}

impl<C: Op_> Relation<C> {
    pub fn named(self, name: &str) -> Self {
        self.context_tracker.borrow_mut(&self.shown_index).name = name.to_string();
        self
    }
    pub fn type_named(self, type_name: &str) -> Self {
        self.context_tracker.borrow_mut(&self.shown_index).type_name = type_name.to_string();
        self
    }
    pub fn hidden(mut self) -> Self {
        {
            let mut info = self.context_tracker.borrow_mut(&self.shown_index);
            assert_eq!(
                info.deps.len(),
                1,
                "Can only hide nodes with exactly one dependency"
            );
            info.hidden = true;
        }
        self.shown_index = self
            .context_tracker
            .0
            .borrow()
            .find_shown_index(&self.shown_index)
            .clone();
        self
    }
}
