#![feature(map_first_last, min_type_alias_impl_trait, box_into_inner, cell_update)]

mod convenience;
pub mod core;
mod feedback;

pub use self::{
    convenience::{
        input::{Input, InputRelation},
        pair, Collection, Is,
    },
    core::{
        pipes, CountMap, Dynamic, Input_, IsReduce, Observable, Op, Op_, Output, ReduceProbe,
        Relation, Save, Saved,
    },
    feedback::{ContextTracker, CreationContext, ExecutionContext},
};

#[cfg(test)]
mod tests;
