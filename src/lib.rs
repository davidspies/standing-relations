mod count_map;
mod pipes;

use std::collections::HashMap;

use count_map::CountMap;
use pipes::Receiver;

pub trait Op {
    type T;

    fn get(&mut self) -> Vec<Self::T>;
}

pub struct Relation<C: Op + ?Sized> {
    inner: RelationInner<C>,
}

struct RelationInner<C: Op + ?Sized> {
    inner: C,
}

pub struct Input<T>(Receiver<T>);

impl<T> Op for Input<T> {
    type T = T;

    fn get(&mut self) -> Vec<T> {
        self.0.receive()
    }
}

pub struct Output<D, C: Op<T = (D, isize)>, M: CountMap<D> = HashMap<D, isize>> {
    inner: RelationInner<C>,
    data: M,
}

impl<D, C: Op<T = (D, isize)>> Relation<C> {
    pub fn get_output<M: CountMap<D>>(self) -> Output<D, C, M> {
        Output {
            inner: self.inner,
            data: M::empty(),
        }
    }
}

#[cfg(test)]
mod tests;
