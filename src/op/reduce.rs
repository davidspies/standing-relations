use std::{
    collections::{hash_map, HashMap, HashSet},
    hash::Hash,
};

use crate::{CountMap, Observable, Op, Relation};

pub struct Reduce<
    K,
    X,
    C: Op<T = ((K, X), isize)>,
    M: CountMap<X> + Observable,
    Y,
    F: Fn(&K, &M) -> Y,
> {
    inner: C,
    in_map: HashMap<K, M>,
    out_map: HashMap<K, Y>,
    f: F,
}

impl<
        K: Clone + Eq + Hash,
        X,
        C: Op<T = ((K, X), isize)>,
        M: CountMap<X> + Observable,
        Y: Clone + Eq,
        F: Fn(&K, &M) -> Y,
    > Op for Reduce<K, X, C, M, Y, F>
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
                    match out_map.entry(k.clone()) {
                        hash_map::Entry::Occupied(mut occ) => {
                            if occ.get() == &new_val {
                                continue 'keys;
                            }
                            let old_val = occ.insert(new_val.clone());
                            continuation(((k.clone(), old_val), -1));
                        }
                        hash_map::Entry::Vacant(vac) => {
                            vac.insert(new_val.clone());
                        }
                    };
                    continuation(((k, new_val), 1));
                }
            }
        }
    }
}

impl<C: Op<T = ((K, X), isize)>, K: Eq + Hash + Clone, X> Relation<C> {
    pub fn reduce<M: CountMap<X> + Observable, Y: Clone + Eq, F: Fn(&K, &M) -> Y>(
        self,
        f: F,
    ) -> Relation<Reduce<K, X, C, M, Y, F>> {
        Relation {
            context_id: self.context_id,
            dirty: self.dirty,
            inner: Reduce {
                inner: self.inner,
                in_map: HashMap::new(),
                out_map: HashMap::new(),
                f,
            },
        }
    }
}
