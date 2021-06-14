use super::{Piece, Player, Position};

impl Position {
    pub fn from_string(s: &str) -> Self {
        let bs: Vec<char> = s.chars().collect();
        assert_eq!(bs.len(), 9);
        let mut i = 0;
        let mut board: [[Piece; 3]; 3] = Default::default();
        let mut x_count = 0;
        let mut o_count = 0;
        for row in 0..3 {
            for col in 0..3 {
                if bs[i] == 'X' {
                    board[row][col] = Piece::X;
                    x_count += 1;
                } else if bs[i] == 'O' {
                    board[row][col] = Piece::O;
                    o_count += 1;
                } else if bs[i] == '-' {
                    board[row][col] = Piece::E;
                } else {
                    panic!("Bad position")
                }
                i += 1;
            }
        }
        let turn = if x_count == o_count {
            Player::Player1
        } else if x_count == o_count + 1 {
            Player::Player2
        } else {
            panic!("Bad string")
        };
        Position { board, turn }
    }
}
