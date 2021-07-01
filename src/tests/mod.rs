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
