use std::{collections::HashMap, hash::Hash};

use crate::core::{
    mborrowed::{MBorrowed, OrOwnedDefault},
    CountMap,
};

pub struct BiMap<X, Y> {
    forward: HashMap<X, HashMap<Y, isize>>,
    backward: HashMap<Y, HashMap<X, isize>>,
}

impl<X, Y> Default for BiMap<X, Y> {
    fn default() -> Self {
        Self {
            forward: Default::default(),
            backward: Default::default(),
        }
    }
}

impl<X: Eq + Hash + Clone, Y: Eq + Hash + Clone> CountMap<(X, Y)> for BiMap<X, Y> {
    fn add(&mut self, (x, y): (X, Y), count: isize) {
        self.forward.add((x.clone(), y.clone()), count);
        self.backward.add((y, x), count);
    }
}

impl<X: Eq + Hash, Y: Eq + Hash> BiMap<X, Y> {
    pub fn new() -> Self {
        BiMap {
            forward: HashMap::new(),
            backward: HashMap::new(),
        }
    }
    pub fn get_forward<'a>(&'a self, x: &X) -> MBorrowed<'a, HashMap<Y, isize>> {
        self.forward.get(x).or_owned_default()
    }
    pub fn get_backward<'a>(&'a self, y: &Y) -> MBorrowed<'a, HashMap<X, isize>> {
        self.backward.get(y).or_owned_default()
    }
}
