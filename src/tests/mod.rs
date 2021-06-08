use std::{collections::HashMap, iter::FromIterator};

use crate::{CreationContext, Output};

mod game;
mod solve;
mod ttt;

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
