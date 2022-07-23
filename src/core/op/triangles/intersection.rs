use std::{collections::HashMap, hash::Hash};

pub(super) fn intersection<'a, K: Eq + Hash, VL, VR>(
    lhs: &'a HashMap<K, VL>,
    rhs: &'a HashMap<K, VR>,
) -> impl Iterator<Item = (&'a K, &'a VL, &'a VR)> {
    if lhs.len() <= rhs.len() {
        EitherIter::Left(
            lhs.iter()
                .flat_map(move |(k, vl)| rhs.get(k).map(|vr| (k, vl, vr))),
        )
    } else {
        EitherIter::Right(
            rhs.iter()
                .flat_map(move |(k, vr)| lhs.get(k).map(|vl| (k, vl, vr))),
        )
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
