use std::{
    cell::{Ref, RefCell},
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::core::{
    relation::RelationInner, ContextTracker, CountMap, CreationContext, ExecutionContext,
    Observable, Op, Op_, Relation, Save, Saved,
};

use self::map::{InsertResult, OutputMap};

mod map;

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
        self.context_tracker.add_relation(
            self.dirty,
            Reduce {
                inner: self.inner,
                in_map: HashMap::new(),
                out_map: Default::default(),
                f,
            },
            vec![self.tracking_index],
        )
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
        assert_eq!(&self.context_tracker, context.tracker(), "Context mismatch");
        ReduceProbe {
            context_tracker: self.context_tracker.clone(),
            inner: RefCell::new(Saved::new(self)),
        }
    }
}

pub struct ReduceProbe<C: IsReduce> {
    context_tracker: ContextTracker,
    inner: RefCell<Saved<C>>,
}

impl<C: IsReduce> ReduceProbe<C> {
    pub fn get_relation(&self) -> Relation<Save<C>>
    where
        C::T: Clone,
    {
        self.inner.borrow().clone().get()
    }
    pub fn inspect<'a>(&'a self, context: &'a ExecutionContext<'_>) -> ProbeRef<'a, C> {
        assert_eq!(&self.context_tracker, context.tracker(), "Context mismatch");
        self.inner.borrow_mut().propagate();
        ProbeRef(self.inner.borrow())
    }
}

pub struct ProbeRef<'a, C: IsReduce>(Ref<'a, Saved<C>>);

impl<'a, C: IsReduce> ProbeRef<'a, C> {
    pub fn get(&self) -> Ref<'_, C::OM> {
        Ref::map(self.0.borrow(), |x| x.inner.get_map())
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
