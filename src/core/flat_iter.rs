pub trait IntoFlatIterator {
    type IntoFlatIter;

    fn into_flat_iter(self) -> Self::IntoFlatIter;
}

impl<I: IntoIterator<Item = J>, J: IntoIterator> IntoFlatIterator for I {
    type IntoFlatIter = impl Iterator<Item = J::Item>;

    fn into_flat_iter(self) -> Self::IntoFlatIter {
        self.into_iter().flatten()
    }
}
