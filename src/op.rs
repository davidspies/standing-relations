pub trait Op {
    type T;

    fn get(&mut self) -> Vec<Self::T>;
}
