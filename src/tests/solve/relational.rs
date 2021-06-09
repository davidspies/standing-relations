use crate::{
    tests::{
        game::{IsGame, IsOutcome, IsPlayer, IsPosition},
        solve::non_loopy,
        ttt::TTT,
    },
    CreationContext,
};
use std::collections::HashMap;

fn solve<Game: IsGame>(g: &Game) -> HashMap<Game::Position, Game::Outcome> {
    let mut context = CreationContext::new();
    let (start_inp, start_position) = context.new_input();
    let (position_inp, non_start_positions) = context.new_input();
    let positions = start_position.concat(non_start_positions).distinct().save();
    let (pos_child_vec, immediate) = positions
        .clone()
        .map(|p: Game::Position| {
            let s = p.status();
            (p, s)
        })
        .split_by_value();
    let pos_children = pos_child_vec
        .flat_map(|(p, children)| children.into_iter().map(move |c| (p.clone(), c)))
        .save();
    let next_positions = pos_children.clone().map(|(_, c)| c);

    context.feed_once(next_positions, position_inp);

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

    let next_outcomes = immediate.concat(child_outcomes.reduce(|p: &Game::Position, outs| {
        p.get_turn()
            .best_outcome(outs.keys().map(Clone::clone))
            .backup()
    }));

    let output = outcomes.get_output();

    context.feed_once(next_outcomes, outcome_inp);

    let mut context = context.begin();
    start_inp.add(&context, g.start());

    context.commit();

    let result = output.get(&context).keys().map(Clone::clone).collect();
    result
}

#[test]
fn ttt_outcomes() {
    assert_eq!(solve(&TTT), non_loopy::solve(&TTT))
}
