use std::collections::HashMap;

use crate::{
    tests::{
        game::{IsGame, IsPlayer, IsPosition},
        ttt::{self, TTT},
    },
    Either,
};

pub fn solve<Game: IsGame>(g: &Game) -> HashMap<Game::Position, Game::Outcome> {
    let mut result = HashMap::new();
    solve_to(g.start(), &mut result);
    result
}

fn solve_to<P: IsPosition>(this: P, known: &mut HashMap<P, P::Outcome>) -> P::Outcome {
    if let Some(outcome) = known.get(&this) {
        return outcome.clone();
    }
    let outcome = match this.status() {
        Either::Left(moves) => this
            .get_turn()
            .best_outcome(moves.into_iter().map(|child| solve_to(child, known))),
        Either::Right(outcome) => outcome,
    };
    known.insert(this, outcome.clone());
    outcome
}

#[test]
fn ttt_outcomes() {
    let m = solve(&TTT);
    assert_eq!(
        m[&ttt::Position::from_string("---------")],
        ttt::Outcome::Draw
    );
    assert_eq!(
        m[&ttt::Position::from_string("----X----")],
        ttt::Outcome::Draw
    );
    assert_eq!(
        m[&ttt::Position::from_string("-O--X----")],
        ttt::Outcome::XWin
    );
}