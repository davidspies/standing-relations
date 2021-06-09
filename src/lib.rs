#![feature(map_first_last, type_alias_impl_trait)]

mod convenience;
pub mod core;
mod feedback;

pub use self::convenience::Collection;
pub use self::core::{
    CountMap, CreationContext, Dynamic, Either, ExecutionContext, Input, Op, Output, Relation, Save,
};
pub use self::feedback::{Feedback, FeedbackContext, Interrupter};

#[cfg(test)]
mod tests;
