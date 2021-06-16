#![feature(map_first_last, type_alias_impl_trait)]

mod convenience;
pub mod core;
mod feedback;
pub mod is_context;
mod tracked;

pub use self::convenience::{
    input::{Input, InputRelation},
    Collection,
};
pub use self::core::{
    CountMap, Dynamic, Either, Input_, IsReduce, Op, Op_, Output, ReduceProbe, Relation, Save,
};
pub use self::feedback::{CreationContext, ExecutionContext, Feedback, Interrupter};

#[cfg(test)]
mod tests;
