use std::{collections::HashMap, iter::FromIterator};

use crate::CreationContext;

mod dijkstra;
mod fingers;
mod game;
mod player_outcome;
mod readme;
mod solve;
mod ttt;

#[test]
fn it_works() {
    let context = CreationContext::new();
    let (inp, rel) = context.new_input::<char>();
    let rel = rel.save();
    let concatted = rel.get().concat(rel.get()).t::<char>();
    let outp = concatted.get_output(&context);

    let mut context = context.begin();
    inp.add(&context, 'a');
    inp.add(&context, 'b');
    inp.add(&context, 'a');
    inp.add(&context, 'b');
    inp.remove(&context, 'b');
    assert_eq!(&*outp.get(&context), &HashMap::from_iter(vec![]));
    context.commit();
    assert_eq!(
        &*outp.get(&context),
        &HashMap::from_iter(vec![('a', 4), ('b', 2)])
    );
}

#[test]
fn feed_ordered() {
    let mut context = CreationContext::new();
    let (inp, rel) = context.new_input::<((), usize)>();
    let rel = rel.group_min().save();
    let outp = rel.get().get_output(&context);
    context.feed_ordered(rel.get().map(|(c, i)| (i, (c, i + 1))), inp.clone());

    let mut context = context.begin();
    inp.add(&context, ((), 0));
    context.commit();
    assert_eq!(
        &*outp.get(&context),
        &HashMap::from_iter(vec![(((), 0), 1)])
    );
    inp.remove(&context, ((), 0));
    context.commit();
    assert_eq!(&*outp.get(&context), &HashMap::from_iter(vec![]));
}
