pub trait Pair {
    type Fst;
    type Snd;

    fn swap(self) -> (Self::Snd, Self::Fst);
    fn fst(self) -> Self::Fst;
    fn snd(self) -> Self::Snd;
}

impl<A, B> Pair for (A, B) {
    type Fst = A;
    type Snd = B;

    fn swap(self) -> (B, A) {
        (self.1, self.0)
    }

    fn fst(self) -> Self::Fst {
        self.0
    }

    fn snd(self) -> Self::Snd {
        self.1
    }
}
