use std::{cmp::Ordering, hash::Hash};

pub trait IsGame {
    type Position: IsPosition<Player = Self::Player, Outcome = Self::Outcome>;
    type Player: IsPlayer<Outcome = Self::Outcome>;
    type Outcome: IsOutcome;

    fn start(&self) -> Self::Position;
}

pub trait IsPosition: Clone + Eq + Hash {
    type Player: IsPlayer<Outcome = Self::Outcome>;
    type Outcome: IsOutcome;
    type MoveIter: IntoIterator<Item = Self>;

    fn get_turn(&self) -> Self::Player;

    fn is_ended(&self) -> Option<Self::Outcome>;

    fn moves(&self) -> Self::MoveIter;
}

pub trait IsPlayer {
    type Outcome: IsOutcome;

    fn compare(&self, l: &Self::Outcome, r: &Self::Outcome) -> Ordering;
    fn best_outcome<I: IntoIterator<Item = Self::Outcome>>(&self, iter: I) -> Self::Outcome {
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
}
