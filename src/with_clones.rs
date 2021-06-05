use std::iter::Peekable;

pub struct WithClonesImpl<I: Iterator, T: Clone> {
    iter: Peekable<I>,
    snd: Option<T>,
}

impl<I: Iterator, T: Clone> Iterator for WithClonesImpl<I, T> {
    type Item = (I::Item, T);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.iter.next()?;
        let snd = match self.iter.peek() {
            Some(_) => self.snd.clone().unwrap(),
            None => self.snd.take().unwrap(),
        };
        Some((x, snd))
    }
}

pub trait WithClones: Iterator + Sized {
    /// Equivalent to .map(|x| (x, snd.clone())) but moves the last element rather than cloning it.
    fn with_clones<T: Clone>(self, snd: T) -> WithClonesImpl<Self, T>;
}

impl<I: Iterator> WithClones for I {
    fn with_clones<T: Clone>(self, snd: T) -> WithClonesImpl<Self, T> {
        WithClonesImpl {
            iter: self.peekable(),
            snd: Some(snd),
        }
    }
}
