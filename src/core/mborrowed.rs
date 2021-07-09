use std::ops::Deref;

pub enum MBorrowed<'a, T> {
    Owned(T),
    Borrowed(&'a T),
}

impl<T> Deref for MBorrowed<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Owned(x) => x,
            Self::Borrowed(x) => x,
        }
    }
}

pub trait OrOwnedDefault {
    type Target;

    fn or_owned_default(self) -> Self::Target;
}

impl<'a, T: Default> OrOwnedDefault for Option<&'a T> {
    type Target = MBorrowed<'a, T>;

    fn or_owned_default(self) -> Self::Target {
        match self {
            None => MBorrowed::Owned(Default::default()),
            Some(x) => MBorrowed::Borrowed(x),
        }
    }
}
