pub mod concat;
pub mod consolidate;
pub mod dynamic;
pub mod join;
pub mod map;
pub mod reduce;
pub mod save;
pub mod split;

pub trait Op_ {
    type T;

    fn foreach<'a>(&'a mut self, continuation: impl FnMut(Self::T) + 'a);
    fn get_vec(&mut self) -> Vec<Self::T> {
        let mut result = Vec::new();
        self.foreach(|x| result.push(x));
        result
    }
}

pub trait Op: Op_<T = (Self::D, isize)> {
    type D;
}

impl<C: Op_<T = (D, isize)>, D> Op for C {
    type D = D;
}
