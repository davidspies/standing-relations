use crate::{Op_, Relation, Save, Saved};

impl<C: Op_> Saved<C> {
    pub fn get(&self) -> Relation<Save<C>>
    where
        C::T: Clone,
    {
        self.get_shown().hidden()
    }
}
