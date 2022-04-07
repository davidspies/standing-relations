use std::cmp::{Ordering, Reverse};

use crate::tests::game::{IsOutcome, IsPlayer};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Outcome {
    Player2Win(usize),
    Draw,
    Player1Win(usize),
}

impl Outcome {
    fn to_ordered(self) -> OrderedOutcome {
        match self {
            Self::Player2Win(n) => OrderedOutcome::Player2Win(n),
            Self::Draw => OrderedOutcome::Draw,
            Self::Player1Win(n) => OrderedOutcome::Player1Win(Reverse(n)),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum OrderedOutcome {
    Player2Win(usize),
    Draw,
    Player1Win(Reverse<usize>),
}

impl IsOutcome for Outcome {
    fn draw() -> Self {
        Self::Draw
    }

    fn backup(self) -> Self {
        match self {
            Self::Player2Win(n) => Self::Player2Win(n + 1),
            Self::Draw => Self::Draw,
            Self::Player1Win(n) => Self::Player1Win(n + 1),
        }
    }
}

impl PartialOrd for Outcome {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Outcome {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_ordered().cmp(&other.to_ordered())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Player {
    Player1,
    Player2,
}

impl Default for Player {
    fn default() -> Self {
        Self::Player1
    }
}

impl IsPlayer for Player {
    type Outcome = Outcome;

    fn compare(&self, l: &Outcome, r: &Outcome) -> Ordering {
        match self {
            Self::Player1 => l.cmp(&r),
            Self::Player2 => r.cmp(&l),
        }
    }
}

impl Player {
    pub fn opponent(self) -> Player {
        match self {
            Self::Player1 => Self::Player2,
            Self::Player2 => Self::Player1,
        }
    }
    pub fn win(self) -> Outcome {
        match self {
            Self::Player1 => Outcome::Player1Win(0),
            Self::Player2 => Outcome::Player2Win(0),
        }
    }
}
