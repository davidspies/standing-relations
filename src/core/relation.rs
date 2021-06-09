use crate::core::{context::ContextId, dirty::ReceiveBuilder, Op};

pub struct Relation<C: Op> {
    pub(in crate::core) context_id: ContextId,
    pub(in crate::core) dirty: ReceiveBuilder,
    pub(in crate::core) inner: C,
}
