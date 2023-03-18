use crate::{Dynamic, Op_, Relation};

pub type RelD<'a, D> = Relation<Dynamic<'a, (D, isize)>>;

impl<C: Op_> Relation<C> {
    /// Simplifies the inferred type-signature of a relation at the cost of requiring dynamic
    /// dispatch at runtime.
    ///
    /// Try inserting this in the middle of a big relation if the compiler is running slowly or
    /// using up too much memory.
    pub fn dynamic<'a>(self) -> Relation<Dynamic<'a, C::T>>
    where
        C: 'a,
    {
        self.dynamic_shown().hidden()
    }
}
