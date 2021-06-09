#![feature(map_first_last, type_alias_impl_trait)]

pub mod context_sends;
mod convenience;
pub mod core;
mod feedback;

pub use self::convenience::Collection;
pub use self::core::{CountMap, Dynamic, Either, Input, Op, Output, Relation, Save};
pub use self::feedback::{CreationContext, ExecutionContext, Feedback, Interrupter};

#[cfg(test)]
mod tests;
