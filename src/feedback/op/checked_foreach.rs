pub trait CheckedForeach: Iterator {
    fn checked_foreach(self, f: impl FnMut(Self::Item)) -> bool;
}

impl<I: Iterator> CheckedForeach for I {
    fn checked_foreach(mut self, mut f: impl FnMut(Self::Item)) -> bool {
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
