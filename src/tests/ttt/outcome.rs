use crate::tests::game::IsOutcome;
use std::cmp::Reverse;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Outcome {
    OWin(usize),
    Draw,
    XWin(usize),
}

impl Outcome {
    fn to_ordered(self) -> OrderedOutcome {
        match self {
            Self::OWin(n) => OrderedOutcome::OWin(n),
            Self::Draw => OrderedOutcome::Draw,
            Self::XWin(n) => OrderedOutcome::XWin(Reverse(n)),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum OrderedOutcome {
    OWin(usize),
    Draw,
    XWin(Reverse<usize>),
}

impl IsOutcome for Outcome {
    fn draw() -> Self {
        Self::Draw
    }

    fn backup(self) -> Self {
        match self {
            Self::OWin(n) => Self::OWin(n + 1),
            Self::Draw => Self::Draw,
            Self::XWin(n) => Self::XWin(n + 1),
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
