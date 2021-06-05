#![feature(type_alias_impl_trait)]

mod context;
mod count_map;
mod dirty;
mod op;
mod output;
mod pipes;
mod relation;

pub use context::Context;
pub use count_map::CountMap;
pub use op::concat::Concat;
pub use op::dynamic::Dynamic;
pub use op::input::{Input, InputSender};
pub use op::map::FlatMap;
pub use op::split::Split;
pub use op::Op;
pub use output::Output;
pub use relation::Relation;

#[cfg(test)]
mod tests;
