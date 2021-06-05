pub mod concat;
pub mod dynamic;
pub mod input;
pub mod split;

pub trait Op {
    type T;
    type I: Iterator<Item = Self::T>;

    fn get(&mut self) -> Self::I;
}
