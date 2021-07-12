use crate::core::{context::ContextTracker, dirty::ReceiveBuilder, Op_};

pub struct Relation<C: Op_> {
    pub(in crate::core) context_tracker: ContextTracker,
    pub(in crate::core) dirty: ReceiveBuilder,
    pub(in crate::core) inner: C,
}
