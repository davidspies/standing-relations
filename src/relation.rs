use crate::{context::ContextId, dirty::ReceiveBuilder, Op};

pub struct Relation<C: Op + ?Sized> {
    pub(crate) context_id: ContextId,
    pub(crate) dirty: ReceiveBuilder,
    pub(crate) inner: C,
}
