use crate::{CreationContext, Output};

#[test]
fn readme() {
    let context = CreationContext::new();

    let (input1, relation1) = context.new_input::<(char, usize)>();
    let (input2, relation2) = context.new_input::<(char, String)>();

    let foo = relation2.save();
    let bar = relation1.join(foo.get());
    let baz = foo
        .get()
        .map(|(_, s)| (s.as_str().chars().next().unwrap_or('x'), s.len()));
    let qux = bar.map(|(c, n, s)| (c, n + s.len())).concat(baz).distinct();
    let arrangement: Output<(char, usize), _> = qux.get_output(&context);

    let mut context = context.begin();

    input1.add(&context, ('a', 5));
    input1.add(&context, ('b', 6));
    input2.add(&context, ('b', "Hello".to_string()));
    input2.add(&context, ('b', "world".to_string()));

    context.commit();

    assert_eq!(
        &*arrangement.get(&context),
        &vec![(('H', 5), 1), (('b', 11), 1), (('w', 5), 1)]
            .into_iter()
            .collect()
    );

    input1.remove(&context, ('b', 6));
    input2.add(&context, ('a', "Goodbye".to_string()));
    context.commit();

    assert_eq!(
        &*arrangement.get(&context),
        &vec![(('G', 7), 1), (('H', 5), 1), (('a', 12), 1), (('w', 5), 1)]
            .into_iter()
            .collect()
    );
}
