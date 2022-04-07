use std::io::Write;

use goldenfile::Mint;

use super::{
    game::{Either, IsGame, IsPosition},
    player_outcome::{Outcome, Player},
    solve::relational,
};

pub struct Fingers;

impl IsGame for Fingers {
    type Player = Player;
    type Position = Position;
    type Outcome = Outcome;

    fn start(&self) -> Self::Position {
        Position {
            p1hands: Hands::start(),
            p2hands: Hands::start(),
            turn: Player::Player1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Hands([usize; 2]);

impl Hands {
    fn start() -> Self {
        Hands([1, 1])
    }
    fn is_zero(&self) -> bool {
        self.0[0] == 0 && self.0[1] == 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    p1hands: Hands,
    p2hands: Hands,
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
        match self.turn {
            Player::Player1 => {
                if self.p1hands.is_zero() {
                    Either::Right(Outcome::Player2Win(0))
                } else {
                    let mut result = Vec::new();
                    for &h1 in self.p1hands.0.iter() {
                        if h1 != 0 {
                            for (j, &h2) in self.p2hands.0.iter().enumerate() {
                                if h2 != 0 {
                                    let mut pos = self.clone();
                                    pos.p2hands.0[j] = (h1 + h2) % 5;
                                    pos.turn = Player::Player2;
                                    result.push(pos);
                                }
                            }
                        }
                    }
                    Either::Left(result)
                }
            }
            Player::Player2 => {
                if self.p2hands.is_zero() {
                    Either::Right(Outcome::Player1Win(0))
                } else {
                    let mut result = Vec::new();
                    for &h2 in self.p2hands.0.iter() {
                        if h2 != 0 {
                            for (i, &h1) in self.p1hands.0.iter().enumerate() {
                                if h1 != 0 {
                                    let mut pos = self.clone();
                                    pos.p1hands.0[i] = (h1 + h2) % 5;
                                    pos.turn = Player::Player1;
                                    result.push(pos);
                                }
                            }
                        }
                    }
                    Either::Left(result)
                }
            }
        }
    }
}

#[test]
fn fingers() {
    let mut mint = Mint::new("tests/goldenfiles");
    let mut stats_file = mint.new_goldenfile("fingers.txt").unwrap();
    let stats = mint.new_goldenfile("fingers_stats.dot").unwrap();
    let m = relational::solve(&Fingers, stats);
    let mut v = m.into_iter().collect::<Vec<_>>();
    v.sort();
    for (x, r) in v {
        writeln!(stats_file, "{:?}: {:?}", x, r).unwrap();
    }

    // When the Mint goes out of scope, it will check the new contents of file
    // against its version controlled "golden" contents and fail the
    // test if they differ.
    //
    // To update the goldenfiles themselves, run:
    //
    //     env REGENERATE_GOLDENFILES=1 cargo test
    //
}
