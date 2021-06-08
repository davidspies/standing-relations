pub mod concat;
pub mod consolidate;
pub mod dynamic;
pub mod join;
pub mod map;
pub mod reduce;
pub mod save;

pub trait Op {
    type T;

    fn foreach<'a, F: FnMut(Self::T) + 'a>(&'a mut self, continuation: F);
    fn get_vec(&mut self) -> Vec<Self::T> {
        let mut result = Vec::new();
        self.foreach(|x| result.push(x));
        result
    }
}
