pub use self::{
    convenience::{
        input::{Input, InputRelation},
        map::ExtremaMap,
        output::{CollectionOutput, DynamicOutput},
        pair, Collection, Is,
    },
    core::{
        pipes, CountMap, Dynamic, Input_, IsReduce, Observable, Op, Op_, Output, ReduceProbe,
        Relation, Save, Saved,
    },
    feedback::{CommitId, ContextTracker, CreationContext, ExecutionContext},
};

mod convenience;
mod feedback;

pub mod core;

#[cfg(test)]
mod tests;
