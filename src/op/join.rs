mod flat_iter;

use std::{collections::HashMap, hash::Hash};

use crate::{CountMap, Op, Relation};

use self::flat_iter::IntoFlatIterator;

pub struct Join<K, V1, V2, C1: Op<T = ((K, V1), isize)>, C2: Op<T = ((K, V2), isize)>> {
    left: C1,
    left_map: HashMap<K, HashMap<V1, isize>>,
    right: C2,
    right_map: HashMap<K, HashMap<V2, isize>>,
}

impl<
        K: Eq + Hash + Clone,
        V1: Eq + Hash + Clone,
        V2: Eq + Hash + Clone,
        C1: Op<T = ((K, V1), isize)>,
        C2: Op<T = ((K, V2), isize)>,
    > Op for Join<K, V1, V2, C1, C2>
{
    type T = ((K, V1, V2), isize);

    fn foreach<'a, F: FnMut(Self::T) + 'a>(&'a mut self, mut continuation: F) {
        let Join {
            left,
            left_map,
            right,
            right_map,
        } = self;
        left.foreach(|((k, x), x_count)| {
            for (y, y_count) in right_map.get(&k).into_flat_iter() {
                continuation(((k.clone(), x.clone(), y.clone()), x_count * y_count));
            }
            left_map.add((k, x), x_count);
        });
        right.foreach(|((k, y), y_count)| {
            for (x, x_count) in left_map.get(&k).into_flat_iter() {
                continuation(((k.clone(), x.clone(), y.clone()), x_count * y_count));
            }
            right_map.add((k, y), y_count);
        });
    }
}

impl<K: Clone + Eq + Hash, V1: Clone + Eq + Hash, C1: Op<T = ((K, V1), isize)>> Relation<C1> {
    pub fn join<V2: Clone + Eq + Hash, C2: Op<T = ((K, V2), isize)>>(
        self,
        other: Relation<C2>,
    ) -> Relation<Join<K, V1, V2, C1, C2>> {
        assert_eq!(self.context_id, other.context_id, "Context mismatch");
        Relation {
            context_id: self.context_id,
            dirty: self.dirty.or(other.dirty),
            inner: Join {
                left: self.inner,
                left_map: HashMap::new(),
                right: other.inner,
                right_map: HashMap::new(),
            },
        }
    }
}