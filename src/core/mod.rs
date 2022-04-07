pub use self::{
    context::{
        input::{InputOp, Input_},
        ContextTracker, CreationContext, ExecutionContext, TrackingIndex,
    },
    count_map::{CountMap, Observable},
    op::{
        concat::Concat,
        consolidate::Consolidate,
        dynamic::Dynamic,
        join::{AntiJoin, Join},
        map::FlatMap,
        reduce::{IsReduce, Reduce, ReduceProbe},
        save::{Save, Saved},
        split::Split,
        Op, Op_,
    },
    output::Output,
    relation::Relation,
};

mod context;
mod count_map;
mod dirty;
mod mborrowed;
mod op;
mod output;
mod relation;

pub mod pipes;
