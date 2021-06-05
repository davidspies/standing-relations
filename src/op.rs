pub mod concat;
pub mod dynamic;
pub mod input;
pub mod split;

pub trait Op {
    type T;

    fn get(&mut self) -> Vec<Self::T>;
}
