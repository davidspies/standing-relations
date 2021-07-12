use goldenfile::Mint;

use crate::{
    tests::{
        game::{Either, IsGame, IsOutcome, IsPlayer, IsPosition},
        solve::non_loopy,
        ttt::TTT,
    },
    CreationContext,
};
use std::{
    collections::HashMap,
    io::Write,
    iter,
    sync::{Arc, Barrier},
    thread,
};

pub fn solve<Game: IsGame>(
    g: &Game,
    stats_file: impl Write + Send + 'static,
) -> HashMap<Game::Position, Game::Outcome> {
    let draw = IsOutcome::draw();

    let mut context = CreationContext::new();

    let (position_inp, positions_dupped) = context.new_input();
    let positions_dupped = positions_dupped.named("positions_dupped");
    let positions = positions_dupped.distinct().named("positions");
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
    let pos_children = pos_children.named("pos_children").collect();
    let starting_values = starting_values.named("starting_values");
    let next_positions = pos_children.get().map(|(_, c)| c).named("next_positions");

    context.feed(next_positions, position_inp.clone());

    let (outcome_inp, outcomes) = context.new_input();
    let outcomes = outcomes.named("outcomes");
    context.feed(starting_values, outcome_inp.clone());

    let child_outcomes = pos_children
        .get()
        .map(|(p, c)| (c, p))
        .join(outcomes)
        .map(|(_, p, o)| (p, o))
        .named("child_outcomes");

    let output_probe = child_outcomes
        .reduce(|p: &Game::Position, outs| {
            p.get_turn()
                .best_outcome(outs.keys().map(Clone::clone))
                .backup()
        })
        .probe(&context);
    let nonterminal_outcomes = output_probe.get_relation().named("nonterminal_outcomes");

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

    //Testing that ContextTracker can be sent between threads
    let context_tracker = context.get_tracker().clone();
    let barrier = Arc::new(Barrier::new(2));
    let barrier_clone = Arc::clone(&barrier);
    thread::spawn(move || {
        barrier_clone.wait();
        context_tracker.dump_dot(stats_file).unwrap();
    });

    position_inp.add(&context, g.start());

    context.commit();

    let result = output_probe.get(&context).clone();

    barrier.wait();

    result
}

#[test]
fn ttt_outcomes() {
    let mut mint = Mint::new("tests/goldenfiles");
    let stats_file = mint.new_goldenfile("ttt_stats.dot").unwrap();
    assert_eq!(solve(&TTT, stats_file), non_loopy::solve(&TTT))
}
