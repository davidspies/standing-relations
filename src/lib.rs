#![feature(map_first_last, type_alias_impl_trait)]
#![feature(label_break_value)]

mod context;
mod convenience;
mod count_map;
mod dirty;
mod flat_iter;
mod op;
mod output;
mod pipes;
mod relation;

pub use context::input::{Input, InputSender};
pub use context::{CreationContext, ExecutionContext};
pub use convenience::Collection;
pub use count_map::{CountMap, Observable};
pub use op::concat::Concat;
pub use op::consolidate::Consolidate;
pub use op::dynamic::Dynamic;
pub use op::join::{AntiJoin, Join};
pub use op::map::FlatMap;
pub use op::reduce::Reduce;
pub use op::split::Split;
pub use op::Op;
pub use output::Output;
pub use relation::Relation;

#[cfg(test)]
mod tests;
