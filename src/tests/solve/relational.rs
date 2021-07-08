use crate::{
    tests::{
        game::{Either, IsGame, IsOutcome, IsPlayer, IsPosition},
        solve::non_loopy,
        ttt::TTT,
    },
    CreationContext,
};
use std::{collections::HashMap, iter};

pub fn solve<Game: IsGame>(g: &Game) -> HashMap<Game::Position, Game::Outcome> {
    let draw = IsOutcome::draw();

    let mut context = CreationContext::new();
    let (position_inp, positions_dupped) = context.new_input();
    let positions = positions_dupped.distinct();
    let (pos_children, starting_values) = positions
        .map(|p: Game::Position| {
            let (it, p_clone, outcome) = match p.status() {
                Either::Left(moves) => (moves, Some(p.clone()), IsOutcome::draw()),
                Either::Right(outcome) => (Default::default(), None, outcome),
            };
            (
                it.into_iter().map(move |x| (p_clone.clone().unwrap(), x)),
                iter::once((p, outcome)),
            )
        })
        .split();
    let pos_children = pos_children.collect();
    let next_positions = pos_children.get().map(|(_, c)| c);

    context.feed(next_positions, position_inp.clone());

    let (outcome_inp, outcomes) = context.new_input();
    context.feed(starting_values, outcome_inp.clone());

    let child_outcomes = pos_children
        .get()
        .map(|(p, c)| (c, p))
        .join(outcomes)
        .map(|(_, p, o)| (p, o));

    let output_probe = child_outcomes
        .reduce(|p: &Game::Position, outs| {
            p.get_turn()
                .best_outcome(outs.keys().map(Clone::clone))
                .backup()
        })
        .probe(&context);
    let nonterminal_outcomes = output_probe.get_relation();

    context.feed(
        nonterminal_outcomes.flat_map_(|((x, o), count)| {
            if &o == &draw {
                Vec::new()
            } else {
                vec![((x.clone(), o), count), ((x, IsOutcome::draw()), -count)]
            }
        }),
        outcome_inp,
    );

    let mut context = context.begin();
    position_inp.add(&context, g.start());

    context.commit();

    let result = output_probe.get(&context).clone();
    result
}

#[test]
fn ttt_outcomes() {
    assert_eq!(solve(&TTT), non_loopy::solve(&TTT))
}
