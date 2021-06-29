#![feature(map_first_last, min_type_alias_impl_trait, box_into_inner)]

mod convenience;
pub mod core;
mod feedback;
pub mod is_context;
mod tracked;

pub use self::convenience::{
    input::{Input, InputRelation},
    Collection, Is,
};
pub use self::core::{
    pipes, ContextId, CountMap, Dynamic, Input_, IsReduce, Op, Op_, Output, ReduceProbe, Relation,
    Save, Saved,
};
pub use self::feedback::{CreationContext, ExecutionContext, With};

#[cfg(test)]
mod tests;
