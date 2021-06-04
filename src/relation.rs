use crate::{context::ContextId, dirty::DirtyReceive, op::Op};

pub struct Relation<C: Op + ?Sized> {
    pub(crate) context_id: ContextId,
    pub(crate) dirty: DirtyReceive,
    pub(crate) inner: C,
}
