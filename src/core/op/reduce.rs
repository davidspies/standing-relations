mod map;

use self::map::{InsertResult, OutputMap};
use crate::core::{
    context::ContextTracker, relation::RelationInner, CountMap, CreationContext, ExecutionContext,
    Observable, Op, Op_, Relation, Save,
};
use std::{
    cell::Ref,
    collections::{HashMap, HashSet},
    hash::Hash,
};

use super::save::Saved;

pub struct Reduce<
    K,
    X,
    C: Op<D = (K, X)>,
    M: CountMap<X> + Observable,
    Y,
    OM: OutputMap<K, Y>,
    F: Fn(&K, &M) -> Y,
> {
    inner: RelationInner<C>,
    in_map: HashMap<K, M>,
    out_map: OM,
    f: F,
}

impl<
        K: Clone + Eq + Hash,
        X,
        C: Op<D = (K, X)>,
        M: CountMap<X> + Observable,
        Y: Clone + Eq,
        OM: OutputMap<K, Y>,
        F: Fn(&K, &M) -> Y,
    > Op_ for Reduce<K, X, C, M, Y, OM, F>
{
    type T = ((K, Y), isize);

    fn foreach<'a>(&'a mut self, mut continuation: impl FnMut(Self::T) + 'a) {
        let Reduce {
            inner,
            in_map,
            out_map,
            f,
        } = self;
        let mut changed_keys = HashSet::new();
        inner.foreach(|((k, v), count)| {
            in_map.add((k.clone(), v), count);
            changed_keys.insert(k);
        });
        'keys: for k in changed_keys {
            match in_map.get(&k) {
                None => {
                    if let Some(old_val) = out_map.remove(&k) {
                        continuation(((k, old_val), -1))
                    }
                }
                Some(m) => {
                    let new_val = f(&k, m);
                    match out_map.insert_if_different(k.clone(), new_val.clone()) {
                        InsertResult::NoOldValue => (),
                        InsertResult::OldValue(old_val) => continuation(((k.clone(), old_val), -1)),
                        InsertResult::Unchanged => continue 'keys,
                    };
                    continuation(((k, new_val), 1));
                }
            }
        }
    }

    fn get_type_name() -> &'static str {
        "reduce"
    }
}

impl<C: Op<D = (K, X)>, K: Clone + Eq + Hash, X> Relation<C> {
    pub fn reduce_with_output_<
        M: CountMap<X> + Observable,
        OM: OutputMap<K, Y> + Default,
        Y: Clone + Eq,
        F: Fn(&K, &M) -> Y,
    >(
        self,
        f: F,
    ) -> Relation<Reduce<K, X, C, M, Y, OM, F>> {
        let inner = self.context_tracker.add_relation(Reduce {
            inner: self.inner,
            in_map: HashMap::new(),
            out_map: Default::default(),
            f,
        });
        Relation {
            context_tracker: self.context_tracker,
            dirty: self.dirty,
            inner,
        }
    }
    pub fn reduce_<M: CountMap<X> + Observable, Y: Clone + Eq, F: Fn(&K, &M) -> Y>(
        self,
        f: F,
    ) -> Relation<Reduce<K, X, C, M, Y, HashMap<K, Y>, F>> {
        self.reduce_with_output_(f)
    }
}

pub trait IsReduce: Op_ {
    type OM;

    fn get_map(&self) -> &Self::OM;
}

impl<
        K: Clone + Eq + Hash,
        X,
        C: Op<D = (K, X)>,
        M: CountMap<X> + Observable,
        Y: Clone + Eq,
        OM: OutputMap<K, Y>,
        F: Fn(&K, &M) -> Y,
    > IsReduce for Reduce<K, X, C, M, Y, OM, F>
{
    type OM = OM;

    fn get_map(&self) -> &OM {
        &self.out_map
    }
}

impl<C: IsReduce> Relation<C> {
    pub fn probe(self, context: &CreationContext) -> ReduceProbe<C> {
        assert_eq!(
            &self.context_tracker,
            context.get_tracker(),
            "Context mismatch"
        );
        ReduceProbe {
            context_tracker: self.context_tracker.clone(),
            inner: Saved::new(self),
        }
    }
}

pub struct ReduceProbe<C: IsReduce> {
    context_tracker: ContextTracker,
    inner: Saved<C>,
}

impl<C: IsReduce> ReduceProbe<C> {
    pub fn get_relation(&self) -> Relation<Save<C>>
    where
        C::T: Clone,
    {
        self.inner.clone().get()
    }
    pub fn get<'a>(&'a self, context: &'a ExecutionContext<'_>) -> Ref<'a, C::OM> {
        assert_eq!(
            &self.context_tracker,
            context.get_tracker(),
            "Context mismatch"
        );
        self.inner.propagate();
        Ref::map(self.inner.borrow(), |x| x.inner.get_map())
    }
}

impl<C: IsReduce> Clone for ReduceProbe<C> {
    fn clone(&self) -> Self {
        ReduceProbe {
            context_tracker: self.context_tracker.clone(),
            inner: self.inner.clone(),
        }
    }
}
