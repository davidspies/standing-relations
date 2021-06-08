use std::collections::HashMap;

use crate::{
    tests::{
        game::{IsGame, IsOutcome, IsPlayer, IsPosition},
        solve::non_loopy,
        ttt::TTT,
    },
    CreationContext, Either, Output,
};

fn solve<Game: IsGame>(g: &Game) -> HashMap<Game::Position, Game::Outcome> {
    let context = CreationContext::new();
    let (position_inp, positions) = context.new_input();
    let positions = positions.save();
    let (pos_child_vec, immediate) = positions
        .clone()
        .map(|p: Game::Position| match p.status() {
            Either::Left(moves) => Either::Left((p, moves)),
            Either::Right(outcome) => Either::Right((p, outcome)),
        })
        .split();
    let pos_children = pos_child_vec
        .flat_map(|(p, children)| children.into_iter().map(move |c| (p.clone(), c)))
        .save();
    let next_positions: Output<Game::Position, _> = pos_children
        .clone()
        .map(|(_, c)| c)
        .set_minus(positions.clone())
        .get_output();

    let (outcome_inp, non_draw_outcomes) = context.new_input();
    let non_draw_outcomes = non_draw_outcomes.save();
    let outcomes = non_draw_outcomes
        .clone()
        .concat(
            positions
                .minus(non_draw_outcomes.map(|(p, _)| p))
                .map(|p| (p, IsOutcome::draw())),
        )
        .save();

    let child_outcomes = pos_children
        .map(|(p, c)| (c, p))
        .join(outcomes.clone())
        .map(|(_, p, o)| (p, o));

    let next_outcomes = immediate.concat(child_outcomes.reduce(
        |p: &Game::Position, outs: &HashMap<_, _>| {
            p.get_turn().best_outcome(outs.keys().map(Clone::clone))
        },
    ));

    let new_outcomes: Output<(Game::Position, Game::Outcome), _> =
        next_outcomes.set_minus(outcomes.clone()).get_output();

    let output: Output<_, _> = outcomes.get_output();

    let mut context = context.begin();
    position_inp.add(&context, g.start());
    context.commit();
    loop {
        {
            let known_positions = next_positions.get(&context);
            if known_positions.is_empty() {
                break;
            }
            position_inp.add_all(&context, known_positions.iter().map(|(p, _)| p.clone()));
        }
        context.commit();
    }

    loop {
        {
            let known_outcomes = new_outcomes.get(&context);
            if known_outcomes.is_empty() {
                break;
            }
            outcome_inp.add_all(&context, known_outcomes.iter().map(|(po, _)| po.clone()));
        }
        context.commit();
    }

    let result = output.get(&context).keys().map(Clone::clone).collect();
    result
}

#[test]
fn ttt_outcomes() {
    assert_eq!(solve(&TTT), non_loopy::solve(&TTT))
}
