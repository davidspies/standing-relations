use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{self, Debug, Formatter},
    iter::FromIterator,
};

use crate::{CreationContext, Output};

#[test]
fn it_works() {
    let context = CreationContext::new();
    let (inp, rel) = context.new_input();
    let splitted = rel.split();
    let concatted = splitted.clone().concat(splitted);
    let outp: Output<_, _> = concatted.get_output();

    let mut context = context.begin();
    inp.add(&context, 'a');
    inp.add(&context, 'b');
    inp.add(&context, 'a');
    inp.add(&context, 'b');
    inp.remove(&context, 'b');
    assert_eq!(&*outp.get(&context), &HashMap::new());
    context.commit();
    assert_eq!(
        &*outp.get(&context),
        &HashMap::from_iter(vec![('a', 4), ('b', 2)])
    );
}

#[test]
fn ttt_outcomes_simple() {
    let m = solve();
    assert_eq!(m[&TTTPosition::from_string("---------")], Outcome::Draw);
    assert_eq!(m[&TTTPosition::from_string("----X----")], Outcome::Draw);
    assert_eq!(m[&TTTPosition::from_string("-O--X----")], Outcome::XWin);
}

#[test]
fn ttt_outcomes() {
    let context = CreationContext::new();
    let (position_inp, positions) = context.new_input();
    let positions = positions.split();
    let pos_children = positions
        .clone()
        .flat_map(|p: TTTPosition| p.checked_children().into_iter().map(move |c| (p, c)))
        .split();
    let next_positions: Output<TTTPosition, _> = pos_children
        .clone()
        .map(|(_, c)| c)
        .set_minus(positions.clone())
        .get_output();

    let (outcome_inp, non_draw_outcomes) = context.new_input();
    let non_draw_outcomes = non_draw_outcomes.split();
    let outcomes = non_draw_outcomes
        .clone()
        .concat(
            positions
                .clone()
                .minus(non_draw_outcomes.map(|(p, _)| p))
                .map(|p| (p, Outcome::Draw)),
        )
        .split();

    let immediate = positions.flat_map(|p: TTTPosition| {
        let outcome = p.analyze()?;
        Some((p, outcome))
    });

    let child_outcomes = pos_children
        .map(|(p, c)| (c, p))
        .join(outcomes.clone())
        .map(|(_, p, o)| (p, o));

    let next_outcomes = immediate.concat(child_outcomes.reduce(
        |p: &TTTPosition, outs: &HashMap<_, _>| p.turn.best_outcome(outs.keys().map(Clone::clone)),
    ));

    let new_outcomes: Output<(TTTPosition, Outcome), _> =
        next_outcomes.set_minus(outcomes.clone()).get_output();

    let output: Output<_, _> = outcomes.get_output();

    let mut context = context.begin();
    position_inp.add(&context, TTTPosition::start());
    context.commit();
    loop {
        {
            let known_positions = next_positions.get(&context);
            if known_positions.is_empty() {
                break;
            }
            for (&p, _) in known_positions.iter() {
                position_inp.add(&context, p);
            }
        }
        context.commit();
    }

    loop {
        {
            let known_outcomes = new_outcomes.get(&context);
            if known_outcomes.is_empty() {
                break;
            }
            for (&po, _) in known_outcomes.iter() {
                outcome_inp.add(&context, po);
            }
        }
        context.commit();
    }

    let result: HashMap<TTTPosition, Outcome> =
        output.get(&context).keys().map(Clone::clone).collect();
    assert_eq!(result, solve());
}

fn solve() -> HashMap<TTTPosition, Outcome> {
    let mut result = HashMap::new();
    TTTPosition::start().solve(&mut result);
    result
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
    fn player(self) -> Option<Player> {
        match self {
            Self::E => None,
            Self::X => Some(Player::X),
            Self::O => Some(Player::O),
        }
    }
    fn to_chr(self) -> char {
        match self {
            Self::E => '-',
            Self::X => 'X',
            Self::O => 'O',
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Player {
    X,
    O,
}

impl Default for Player {
    fn default() -> Self {
        Self::X
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Outcome {
    OWin,
    Draw,
    XWin,
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
    fn compare(self, l: Outcome, r: Outcome) -> Ordering {
        match self {
            Self::X => l.cmp(&r),
            Self::O => r.cmp(&l),
        }
    }
    fn win(self) -> Outcome {
        match self {
            Self::X => Outcome::XWin,
            Self::O => Outcome::OWin,
        }
    }
    fn best_outcome<I: IntoIterator<Item = Outcome>>(self, iter: I) -> Outcome {
        let mut best = None;
        for outcome in iter {
            match best {
                None => {
                    best = Some(outcome);
                }
                Some(cur_best) => {
                    if self.compare(outcome, cur_best) >= Ordering::Greater {
                        best = Some(outcome);
                    }
                }
            }
        }
        best.unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
struct TTTPosition {
    board: [[Piece; 3]; 3],
    turn: Player,
}

impl Debug for TTTPosition {
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

impl TTTPosition {
    fn start() -> Self {
        Self::default()
    }
    fn children(self) -> Vec<TTTPosition> {
        let mut result = Vec::new();
        for row in 0..3 {
            for col in 0..3 {
                if self.board[row][col] == Piece::E {
                    let mut new_board = self.board;
                    new_board[row][col] = self.turn.piece();
                    result.push(TTTPosition {
                        board: new_board,
                        turn: self.turn.opponent(),
                    })
                }
            }
        }
        result
    }
    fn checked_children(self) -> Vec<TTTPosition> {
        if self.analyze().is_some() {
            Vec::new()
        } else {
            self.children()
        }
    }
    fn analyze(self) -> Option<Outcome> {
        'outer: for w in wins(3, 3, 3) {
            let piece = self.board_at(w[0]);
            if let Some(player) = piece.player() {
                for &rc in &w[1..] {
                    if self.board_at(rc) != piece {
                        continue 'outer;
                    }
                }
                return Some(player.win());
            }
        }
        for row in 0..3 {
            for col in 0..3 {
                if self.board[row][col] == Piece::E {
                    return None;
                }
            }
        }
        Some(Outcome::Draw)
    }
    fn board_at(self, row_col: (usize, usize)) -> Piece {
        let (row, col) = row_col;
        self.board[row][col]
    }
    fn solve(self, known: &mut HashMap<TTTPosition, Outcome>) -> Outcome {
        if let Some(&outcome) = known.get(&self) {
            return outcome;
        }
        if let Some(outcome) = self.analyze() {
            known.insert(self, outcome);
            return outcome;
        }
        let mut best = None;
        for child in self.children() {
            let outcome = child.solve(known);
            match best {
                None => {
                    best = Some(outcome);
                }
                Some(cur_best) => {
                    if self.turn.compare(outcome, cur_best) >= Ordering::Greater {
                        best = Some(outcome);
                    }
                }
            }
        }
        let outcome = best.unwrap();
        known.insert(self, outcome);
        outcome
    }
    fn from_string(s: &str) -> Self {
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
            Player::X
        } else if x_count == o_count + 1 {
            Player::O
        } else {
            panic!("Bad string")
        };
        TTTPosition { board, turn }
    }
}

fn wins(height: usize, width: usize, inarow: usize) -> Vec<Vec<(usize, usize)>> {
    let mut result = Vec::new();
    for row in 0..height {
        for col in 0..=width - inarow {
            let mut v = Vec::new();
            for d in 0..inarow {
                v.push((row, col + d));
            }
            result.push(v);
        }
    }
    for row in 0..=height - inarow {
        for col in 0..width {
            let mut v = Vec::new();
            for d in 0..inarow {
                v.push((row + d, col));
            }
            result.push(v);
        }
    }
    for row in 0..=height - inarow {
        for col in 0..=width - inarow {
            let mut v = Vec::new();
            for d in 0..inarow {
                v.push((row + d, col + d));
            }
            result.push(v);
        }
    }
    for row in 0..=height - inarow {
        for col in inarow - 1..width {
            let mut v = Vec::new();
            for d in 0..inarow {
                v.push((row + d, col - d));
            }
            result.push(v);
        }
    }
    result
}
