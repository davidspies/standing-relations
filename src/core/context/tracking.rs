use crate::core::{
    dirty::ReceiveBuilder, pipes, pipes::CountReceiver, relation::RelationInner, Op_, Relation,
};
use std::{
    fmt::{self, Debug, Display},
    io::{self, Write},
    sync::{Arc, RwLock},
};

#[derive(Clone, Copy)]
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

#[derive(Clone, Default)]
pub struct ContextTracker(Arc<RwLock<ContextTrackerInner>>);
struct TrackingInfo {
    name: String,
    type_name: String,
    hidden: bool,
    count: CountReceiver,
    deps: Vec<TrackIndex>,
}

#[derive(Default)]
struct ContextTrackerInner(Vec<TrackingInfo>);

impl PartialEq for ContextTracker {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for ContextTracker {}
impl ContextTracker {
    pub(in crate::core) fn add_relation<C: Op_>(
        self,
        dirty: ReceiveBuilder,
        inner: C,
        deps: Vec<TrackIndex>,
    ) -> Relation<C> {
        let (count_send, count_receive) = pipes::new_count();
        let track_index = TrackIndex::new(self.0.read().unwrap().0.len());
        self.0.write().unwrap().0.push(TrackingInfo {
            name: format!("relation{}", track_index),
            type_name: C::get_type_name().to_string(),
            hidden: false,
            count: count_receive,
            deps,
        });
        Relation {
            context_tracker: self,
            shown_index: track_index,
            track_index,
            dirty,
            inner: RelationInner::new(inner, count_send),
        }
    }
    pub fn dump_dot(
        &self,
        file: impl Write,
        extra_edges: &[(TrackIndex, TrackIndex)],
    ) -> Result<(), io::Error> {
        self.0.read().unwrap().dump_dot(file, extra_edges)
    }

    pub(crate) fn set_name(&mut self, index: TrackIndex, name: String) {
        self.0.write().unwrap().set_name(index, name)
    }

    pub(crate) fn set_type_name(&mut self, index: TrackIndex, type_name: String) {
        self.0.write().unwrap().set_type_name(index, type_name)
    }

    pub(crate) fn set_hidden(&mut self, index: TrackIndex) {
        self.0.write().unwrap().set_hidden(index)
    }

    pub(crate) fn find_shown_index(&self, index: TrackIndex) -> TrackIndex {
        self.0.read().unwrap().find_shown_index(index)
    }
}
impl Debug for ContextTracker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", Arc::as_ptr(&self.0))
    }
}

impl ContextTrackerInner {
    fn find_shown_index(&self, mut ind: TrackIndex) -> TrackIndex {
        loop {
            let t = &self.0[ind.0];
            if t.hidden {
                assert_eq!(
                    t.deps.len(),
                    1,
                    "unreachable; hidden node with non-one deps"
                );
                ind = t.deps[0];
            } else {
                return ind;
            }
        }
    }
    fn dump_dot(
        &self,
        mut file: impl Write,
        extra_edges: &[(TrackIndex, TrackIndex)],
    ) -> Result<(), io::Error> {
        writeln!(file, "digraph flow {{")?;
        for (i, info) in self.0.iter().enumerate() {
            if info.hidden {
                continue;
            }
            let name = format!("{} <br/>", info.name);
            writeln!(
                file,
                "  node{} [label=< {} {} <br/> {} >];",
                i,
                name,
                info.type_name,
                info.count.get()
            )?;
            for &dep in info.deps.iter() {
                writeln!(file, "  node{} -> node{};", self.find_shown_index(dep), i)?;
            }
        }
        for &(i, j) in extra_edges {
            writeln!(
                file,
                "  node{} -> node{} [style=dotted];",
                self.find_shown_index(i),
                self.find_shown_index(j)
            )?;
        }
        writeln!(file, "}}")
    }
    pub fn set_name(&mut self, index: TrackIndex, name: String) {
        self.0[index.0].name = name
    }
    pub fn set_type_name(&mut self, index: TrackIndex, type_name: String) {
        self.0[index.0].type_name = type_name
    }
    pub fn set_hidden(&mut self, index: TrackIndex) {
        let mut info = &mut self.0[index.0];
        assert_eq!(
            info.deps.len(),
            1,
            "Can only hide nodes with exactly one dependency"
        );
        info.hidden = true;
    }
}
