use crate::core::{context::ContextId, dirty::ReceiveBuilder, Op_};

pub struct Relation<C: Op_> {
    pub(in crate::core) context_id: ContextId,
    pub(in crate::core) dirty: ReceiveBuilder,
    pub(in crate::core) inner: C,
}

impl<C: Op_> Relation<C> {
    pub fn get_context_id(&self) -> ContextId {
        self.context_id
    }
}
