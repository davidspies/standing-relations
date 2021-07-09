use crate::core::{
    mborrowed::{MBorrowed, OrOwnedDefault},
    CountMap,
};
use std::{collections::HashMap, hash::Hash};

pub struct BiMap<X, Y> {
    forward: HashMap<X, HashMap<Y, isize>>,
    backward: HashMap<Y, HashMap<X, isize>>,
}

impl<X: Eq + Hash + Clone, Y: Eq + Hash + Clone> CountMap<(X, Y)> for BiMap<X, Y> {
    fn add(&mut self, (x, y): (X, Y), count: isize) {
        self.forward.add((x.clone(), y.clone()), count);
        self.backward.add((y, x), count);
    }

    fn empty() -> Self {
        BiMap {
            forward: HashMap::new(),
            backward: HashMap::new(),
        }
    }
}

impl<X: Eq + Hash, Y: Eq + Hash> BiMap<X, Y> {
    pub fn get_forward<'a>(&'a self, x: &X) -> MBorrowed<'a, HashMap<Y, isize>> {
        self.forward.get(x).or_owned_default()
    }
    pub fn get_backward<'a>(&'a self, y: &Y) -> MBorrowed<'a, HashMap<X, isize>> {
        self.backward.get(y).or_owned_default()
    }
}
