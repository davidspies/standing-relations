pub trait CheckedForeach: Iterator {
    fn checked_foreach<F: FnMut(Self::Item)>(self, f: F) -> bool;
}

impl<I: Iterator> CheckedForeach for I {
    fn checked_foreach<F: FnMut(Self::Item)>(mut self, mut f: F) -> bool {
        match self.next() {
            None => false,
            Some(next) => {
                f(next);
                for x in self {
                    f(x)
                }
                true
            }
        }
    }
}
