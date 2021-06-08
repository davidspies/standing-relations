use super::{Player, Position};

const WINS: &[&[(usize, usize)]] = &[
    &[(0, 0), (0, 1), (0, 2)],
    &[(1, 0), (1, 1), (1, 2)],
    &[(2, 0), (2, 1), (2, 2)],
    &[(0, 0), (1, 0), (2, 0)],
    &[(0, 1), (1, 1), (2, 1)],
    &[(0, 2), (1, 2), (2, 2)],
    &[(0, 0), (1, 1), (2, 2)],
    &[(0, 2), (1, 1), (2, 0)],
];

impl Position {
    pub(super) fn has_line(&self) -> Option<Player> {
        'outer: for win in WINS {
            let (r0, c0) = win[0];
            let piece = &self.board[r0][c0];
            if let Some(player) = piece.player() {
                for &(r, c) in &win[1..] {
                    if &self.board[r][c] != piece {
                        continue 'outer;
                    }
                }
                return Some(player);
            }
        }
        None
    }
}
