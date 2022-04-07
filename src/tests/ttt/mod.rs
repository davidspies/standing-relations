use std::fmt::{self, Debug, Formatter};

use super::game::{Either, IsGame, IsPosition};

pub use super::player_outcome::{Outcome, Player};

mod parse;
mod win;

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
            Self::X => Some(Player::Player1),
            Self::O => Some(Player::Player2),
        }
    }
    fn to_chr(&self) -> char {
        match self {
            Self::E => '-',
            Self::X => 'X',
            Self::O => 'O',
        }
    }
    fn from_player(player: &Player) -> Self {
        match player {
            Player::Player1 => Piece::X,
            Player::Player2 => Piece::O,
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
                    new_board[row][col] = Piece::from_player(&self.turn);
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
