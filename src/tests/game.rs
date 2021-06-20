use std::{cmp::Ordering, hash::Hash};

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub trait IsGame {
    type Position: IsPosition<Player = Self::Player, Outcome = Self::Outcome>;
    type Player: IsPlayer<Outcome = Self::Outcome>;
    type Outcome: IsOutcome;

    fn start(&self) -> Self::Position;
}

pub trait IsPosition: Clone + Eq + Hash {
    type Player: IsPlayer<Outcome = Self::Outcome>;
    type Outcome: IsOutcome;
    type MoveIter: IntoIterator<Item = Self> + Default;

    fn get_turn(&self) -> Self::Player;

    fn status(&self) -> Either<Self::MoveIter, Self::Outcome>;
}

pub trait IsPlayer {
    type Outcome: IsOutcome;

    fn compare(&self, l: &Self::Outcome, r: &Self::Outcome) -> Ordering;
    fn best_outcome(&self, iter: impl IntoIterator<Item = Self::Outcome>) -> Self::Outcome {
        let mut best = None;
        for outcome in iter {
            match best {
                None => {
                    best = Some(outcome);
                }
                Some(ref cur_best) => {
                    if self.compare(&outcome, cur_best) >= Ordering::Greater {
                        best = Some(outcome);
                    }
                }
            }
        }
        best.unwrap()
    }
}

pub trait IsOutcome: Clone + Eq + Hash {
    fn draw() -> Self;
    fn backup(self) -> Self;
}
