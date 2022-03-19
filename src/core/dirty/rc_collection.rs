use std::{collections::HashSet, rc::Rc, slice, vec};

pub struct RcCollection<T: ?Sized> {
    iterable: Vec<Rc<T>>,
    contained: HashSet<*const T>,
}

impl<T: ?Sized> Default for RcCollection<T> {
    fn default() -> Self {
        Self {
            iterable: Default::default(),
            contained: Default::default(),
        }
    }
}

impl<T: ?Sized> RcCollection<T> {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn insert(&mut self, x: Rc<T>) -> bool {
        let inserted = self.contained.insert(Rc::as_ptr(&x));
        if inserted {
            self.iterable.push(x);
        }
        inserted
    }
    pub fn singleton(x: Rc<T>) -> Self {
        let mut result = Self::new();
        result.insert(x);
        result
    }
    pub fn extend(&mut self, other: RcCollection<T>) {
        for x in other {
            self.insert(x);
        }
    }
}

impl<T: ?Sized> IntoIterator for RcCollection<T> {
    type Item = Rc<T>;
    type IntoIter = vec::IntoIter<Rc<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iterable.into_iter()
    }
}

impl<'a, T: ?Sized> IntoIterator for &'a RcCollection<T> {
    type Item = &'a Rc<T>;
    type IntoIter = slice::Iter<'a, Rc<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iterable.iter()
    }
}
