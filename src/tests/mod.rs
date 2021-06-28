use std::{collections::HashMap, iter::FromIterator};

use crate::CreationContext;

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
    assert_eq!(&*outp.get(&context), &HashMap::new());
    context.commit();
    assert_eq!(
        &*outp.get(&context),
        &HashMap::from_iter(vec![('a', 4), ('b', 2)])
    );
}

#[test]
fn undo_feedback_changes() {
    let mut context = CreationContext::new();
    let (inp1, rel1) = context.new_input::<char>();
    let (inp2, rel2) = context.new_input::<char>();
    let rel2 = rel2.save();
    context.feed(rel1.set_minus(rel2.get()).get_output(&context), inp2);
    let output = rel2.get().get_output(&context);

    let mut context = context.begin();
    context
        .with(|context| inp1.add(context, 'a'))
        .go(|context, conflict| {
            assert_eq!(conflict, None);
            assert_eq!(
                &*output.get(&context),
                &vec![('a', 1)].into_iter().collect()
            );
        });
    context.commit();
    assert_eq!(&*output.get(&context), &HashMap::new());
}
