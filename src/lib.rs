#![feature(map_first_last, type_alias_impl_trait)]

mod context;
mod convenience;
mod count_map;
mod dirty;
mod feedback;
mod flat_iter;
mod op;
mod output;
mod pipes;
mod relation;

pub use context::{
    input::{Input, InputOp},
    CreationContext, ExecutionContext,
};
pub use convenience::Collection;
pub use count_map::{CountMap, Observable};
pub use feedback::{Feedback, FeedbackContext, Interrupter};
pub use op::{
    concat::Concat,
    consolidate::Consolidate,
    dynamic::Dynamic,
    join::{AntiJoin, Join},
    map::FlatMap,
    reduce::Reduce,
    save::Save,
    split::{Either, Split},
    Op,
};
pub use output::Output;
pub use relation::Relation;

#[cfg(test)]
mod tests;
