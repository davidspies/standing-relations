use std::{collections::HashMap, iter::FromIterator};

use crate::{Context, Output};

#[test]
fn it_works() {
    let mut context = Context::new();
    let (inp, rel) = context.new_input();
    let outp: Output<_, _> = rel.get_output();
    inp.add('a');
    inp.add('b');
    inp.add('a');
    inp.add('b');
    inp.remove('b');
    assert_eq!(&*outp.get(&context), &HashMap::new());
    context.commit();
    assert_eq!(
        &*outp.get(&context),
        &HashMap::from_iter(vec![('a', 2), ('b', 1)])
    );
}
