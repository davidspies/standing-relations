use std::{collections::HashMap, hash::Hash};

pub trait Intersectable<Rhs> {
    type Output;

    fn intersection(self, rhs: Rhs) -> Self::Output;
}

impl<'a, K: Eq + Hash + 'a, VL: 'a, VR: 'a> Intersectable<&'a HashMap<K, VR>>
    for &'a HashMap<K, VL>
{
    type Output = impl Iterator<Item = (&'a K, &'a VL, &'a VR)>;

    fn intersection(self, rhs: &'a HashMap<K, VR>) -> Self::Output {
        if self.len() <= rhs.len() {
            EitherIter::Left(
                self.iter()
                    .flat_map(move |(k, vl)| rhs.get(k).map(|vr| (k, vl, vr))),
            )
        } else {
            EitherIter::Right(
                rhs.iter()
                    .flat_map(move |(k, vr)| self.get(k).map(|vl| (k, vl, vr))),
            )
        }
    }
}

enum EitherIter<L, R> {
    Left(L),
    Right(R),
}

impl<T, L: Iterator<Item = T>, R: Iterator<Item = T>> Iterator for EitherIter<L, R> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Left(left) => left.next(),
            Self::Right(right) => right.next(),
        }
    }
}
