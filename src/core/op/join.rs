use std::{collections::HashMap, hash::Hash};

use crate::core::{
    mborrowed::OrOwnedDefault, relation::RelationInner, CountMap, Op, Op_, Relation,
};

pub struct Join<K, V1, V2, C1: Op<D = (K, V1)>, C2: Op<D = (K, V2)>> {
    left: RelationInner<C1>,
    left_map: HashMap<K, HashMap<V1, isize>>,
    right: RelationInner<C2>,
    right_map: HashMap<K, HashMap<V2, isize>>,
}

impl<
        K: Eq + Hash + Clone,
        V1: Eq + Hash + Clone,
        V2: Eq + Hash + Clone,
        C1: Op<D = (K, V1)>,
        C2: Op<D = (K, V2)>,
    > Op_ for Join<K, V1, V2, C1, C2>
{
    type T = ((K, V1, V2), isize);

    fn foreach<'a>(&'a mut self, mut continuation: impl FnMut(Self::T) + 'a) {
        let Join {
            left,
            left_map,
            right,
            right_map,
        } = self;
        left.foreach(|((k, x), x_count)| {
            for (y, y_count) in &*right_map.get(&k).or_owned_default() {
                continuation(((k.clone(), x.clone(), y.clone()), x_count * y_count));
            }
            left_map.add((k, x), x_count);
        });
        right.foreach(|((k, y), y_count)| {
            for (x, x_count) in &*left_map.get(&k).or_owned_default() {
                continuation(((k.clone(), x.clone(), y.clone()), x_count * y_count));
            }
            right_map.add((k, y), y_count);
        });
    }

    fn get_type_name() -> &'static str {
        "join"
    }
}

pub struct AntiJoin<K, V, C1: Op<D = (K, V)>, C2: Op<D = K>> {
    left: RelationInner<C1>,
    left_map: HashMap<K, HashMap<V, isize>>,
    right: RelationInner<C2>,
    right_map: HashMap<K, isize>,
}

impl<K: Eq + Hash + Clone, V: Eq + Hash + Clone, C1: Op<D = (K, V)>, C2: Op<D = K>> Op_
    for AntiJoin<K, V, C1, C2>
{
    type T = ((K, V), isize);

    fn foreach<'a>(&'a mut self, mut continuation: impl FnMut(Self::T) + 'a) {
        let AntiJoin {
            left,
            left_map,
            right,
            right_map,
        } = self;
        left.foreach(|((k, x), x_count)| {
            if !right_map.contains_key(&k) {
                continuation(((k.clone(), x.clone()), x_count));
            }
            left_map.add((k, x), x_count);
        });
        right.foreach(|(k, y_count)| {
            if y_count != 0 {
                let old_count = right_map.get(&k).map(Clone::clone).unwrap_or(0);
                if old_count == -y_count {
                    for (x, &x_count) in &*left_map.get(&k).or_owned_default() {
                        continuation(((k.clone(), x.clone()), x_count));
                    }
                } else if old_count == 0 {
                    for (x, &x_count) in &*left_map.get(&k).or_owned_default() {
                        continuation(((k.clone(), x.clone()), -x_count));
                    }
                }
                right_map.add(k, y_count);
            }
        });
    }

    fn get_type_name() -> &'static str {
        "antijoin"
    }
}

impl<K: Clone + Eq + Hash, V1: Clone + Eq + Hash, C1: Op<D = (K, V1)>> Relation<C1> {
    pub fn join<V2: Clone + Eq + Hash, C2: Op<D = (K, V2)>>(
        self,
        other: Relation<C2>,
    ) -> Relation<Join<K, V1, V2, C1, C2>> {
        assert_eq!(
            self.context_tracker, other.context_tracker,
            "Context mismatch"
        );
        self.context_tracker.add_relation(
            self.dirty.or(other.dirty),
            Join {
                left: self.inner,
                left_map: HashMap::new(),
                right: other.inner,
                right_map: HashMap::new(),
            },
            vec![self.tracking_index, other.tracking_index],
        )
    }

    /// Retains only those keys which have count 0 in the argument relation.
    pub fn antijoin<C2: Op<D = K>>(self, other: Relation<C2>) -> Relation<AntiJoin<K, V1, C1, C2>> {
        assert_eq!(
            self.context_tracker, other.context_tracker,
            "Context mismatch"
        );
        self.context_tracker.add_relation(
            self.dirty.or(other.dirty),
            AntiJoin {
                left: self.inner,
                left_map: HashMap::new(),
                right: other.inner,
                right_map: HashMap::new(),
            },
            vec![self.tracking_index, other.tracking_index],
        )
    }
}
