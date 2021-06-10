mod map;

use self::map::{InsertResult, OutputMap};
use crate::core::{CountMap, Observable, Op, Relation};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub struct Reduce<
    K,
    X,
    C: Op<T = ((K, X), isize)>,
    M: CountMap<X> + Observable,
    Y,
    OM: OutputMap<K, Y>,
    F: Fn(&K, &M) -> Y,
> {
    inner: C,
    in_map: HashMap<K, M>,
    out_map: OM,
    f: F,
}

impl<
        K: Clone + Eq + Hash,
        X,
        C: Op<T = ((K, X), isize)>,
        M: CountMap<X> + Observable,
        Y: Clone + Eq,
        OM: OutputMap<K, Y>,
        F: Fn(&K, &M) -> Y,
    > Op for Reduce<K, X, C, M, Y, OM, F>
{
    type T = ((K, Y), isize);

    fn foreach<'a, G: FnMut(Self::T) + 'a>(&'a mut self, mut continuation: G) {
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
}

impl<C: Op<T = ((K, X), isize)>, K: Clone + Eq + Hash, X> Relation<C> {
    pub fn reduce_with_output_<
        M: CountMap<X> + Observable,
        OM: OutputMap<K, Y> + Default,
        Y: Clone + Eq,
        F: Fn(&K, &M) -> Y,
    >(
        self,
        f: F,
    ) -> Relation<Reduce<K, X, C, M, Y, OM, F>> {
        Relation {
            context_id: self.context_id,
            dirty: self.dirty,
            inner: Reduce {
                inner: self.inner,
                in_map: HashMap::new(),
                out_map: Default::default(),
                f,
            },
        }
    }
    pub fn reduce_<M: CountMap<X> + Observable, Y: Clone + Eq, F: Fn(&K, &M) -> Y>(
        self,
        f: F,
    ) -> Relation<Reduce<K, X, C, M, Y, HashMap<K, Y>, F>> {
        self.reduce_with_output_(f)
    }
}
