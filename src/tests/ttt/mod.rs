mod outcome;
mod parse;
mod win;

pub use self::outcome::Outcome;
use super::game::{IsGame, IsPlayer, IsPosition};
use crate::Either;
use std::{
    cmp::Ordering,
    fmt::{self, Debug, Formatter},
};

#[derive(Clone, PartialEq, Eq, Hash)]
enum Piece {
    E,
    X,
    O,
}

impl Default for Piece {
    fn default() -> Self {
        Self::E
    }
}

impl Piece {
    fn player(&self) -> Option<Player> {
        match self {
            Self::E => None,
            Self::X => Some(Player::X),
            Self::O => Some(Player::O),
        }
    }
    fn to_chr(&self) -> char {
        match self {
            Self::E => '-',
            Self::X => 'X',
            Self::O => 'O',
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    X,
    O,
}

impl Default for Player {
    fn default() -> Self {
        Self::X
    }
}

impl IsPlayer for Player {
    type Outcome = Outcome;

    fn compare(&self, l: &Outcome, r: &Outcome) -> Ordering {
        match self {
            Self::X => l.cmp(&r),
            Self::O => r.cmp(&l),
        }
    }
}

impl Player {
    fn opponent(self) -> Player {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
    fn piece(self) -> Piece {
        match self {
            Self::X => Piece::X,
            Self::O => Piece::O,
        }
    }
    fn win(self) -> Outcome {
        match self {
            Self::X => Outcome::XWin(0),
            Self::O => Outcome::OWin(0),
        }
    }
}

pub struct TTT;

#[derive(Clone, PartialEq, Eq, Hash, Default)]
pub struct Position {
    board: [[Piece; 3]; 3],
    turn: Player,
}

impl IsPosition for Position {
    type Player = Player;

    type Outcome = Outcome;

    type MoveIter = Vec<Position>;

    fn get_turn(&self) -> Self::Player {
        self.turn
    }

    fn status(&self) -> Either<Self::MoveIter, Self::Outcome> {
        if let Some(p) = self.has_line() {
            return Either::Right(p.win());
        }
        let mut moves = Vec::new();
        for row in 0..3 {
            for col in 0..3 {
                if self.board[row][col] == Piece::E {
                    let mut new_board = self.board.clone();
                    new_board[row][col] = self.turn.piece();
                    moves.push(Position {
                        board: new_board,
                        turn: self.turn.opponent(),
                    })
                }
            }
        }
        if moves.is_empty() {
            Either::Right(Outcome::Draw)
        } else {
            Either::Left(moves)
        }
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut chrs = Vec::new();
        for row in 0..3 {
            for col in 0..3 {
                chrs.push(self.board[row][col].to_chr());
            }
        }
        write!(f, "{}", chrs.into_iter().collect::<String>())
    }
}

impl IsGame for TTT {
    type Position = Position;

    type Player = Player;

    type Outcome = Outcome;

    fn start(&self) -> Self::Position {
        Default::default()
    }
}
