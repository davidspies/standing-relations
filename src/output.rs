use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
};

use crate::{count_map::CountMap, dirty::DirtyReceive, op::Op, relation::Relation};

impl<D, C: Op<T = (D, isize)>> Relation<C> {
    pub fn get_output<M: CountMap<D>>(self) -> Output<D, C, M> {
        Output {
            dirty: self.dirty,
            inner: RefCell::new(self.inner),
            data: RefCell::new(M::empty()),
        }
    }
}

pub struct Output<D, C: Op<T = (D, isize)>, M: CountMap<D> = HashMap<D, isize>> {
    dirty: DirtyReceive,
    inner: RefCell<C>,
    data: RefCell<M>,
}

impl<D, C: Op<T = (D, isize)>, M: CountMap<D>> Output<D, C, M> {
    pub fn get(&self) -> Ref<'_, M> {
        if self.dirty.take_status() {
            for (k, v) in self.inner.borrow_mut().get() {
                self.data.borrow_mut().add(k, v);
            }
        }
        self.data.borrow()
    }
}
