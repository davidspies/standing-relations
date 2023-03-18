use std::{
    io::{self, Write},
    sync::{Arc, RwLock},
};

use crate::core::{self, TrackingIndex};

#[derive(Clone)]
pub struct ContextTracker {
    inner: core::ContextTracker,
    extra_edges: Arc<RwLock<Vec<(TrackingIndex, TrackingIndex)>>>,
}

impl ContextTracker {
    pub fn dump_dot(&self, file: impl Write) -> Result<(), io::Error> {
        self.inner
            .dump_dot(file, &*self.extra_edges.read().unwrap())
    }

    pub(super) fn new(
        inner: core::ContextTracker,
        extra_edges: Arc<RwLock<Vec<(TrackingIndex, TrackingIndex)>>>,
    ) -> ContextTracker {
        ContextTracker { inner, extra_edges }
    }
}
