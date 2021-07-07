# Standing Relations
## Overview
This crate provides an interface vaguely similar to differential-dataflow for creating standing
relations over a shifting dataset.
Critically, _unlike_ differential-dataflow the operators here are single-threaded and optimized for
fast turnaround rather than high throughput. That is, this package is intended to be used in
"feedback loop" scenarios where the calling code uses the output to determine which inputs to feed
in next.

## Getting Started
To get started, create a `CreationContext`:

```rust
use standing_relations::CreationContext;

let mut context = CreationContext::new();
```

And some inputs:

```rust
let (input1, relation1) = context.new_input::<(char, usize)>();
let (input2, relation2) = context.new_input::<(char, String)>();
```

Set up your relational operations:

```rust
let foo = relation2.save();
let bar = relation1.join(foo.get());
let baz = foo
    .get()
    .map(|(_, s)| (s.as_str().chars().next().unwrap_or('x'), s.len()));
let qux = bar.map(|(c, n, s)| (c, n + s.len())).concat(baz).distinct();
let arrangement: Output<(char, usize), _> = qux.get_output(&context);
```

Begin inserting data. To do this, you must first change your `CreationContext` into an
`ExecutionContext` by calling `CreationContext::begin`. This tells the system that your relational
graph is fully built and you won't be making any more changes to it:

```rust
let mut context = context.begin();

input1.add(&context, ('a', 5));
input1.add(&context, ('b', 6));
input2.add(&context, ('b', "Hello".to_string()));
input2.add(&context, ('b', "world".to_string()));
```

Commit your changes:

```rust
context.commit();
```

Read the output:

```rust
assert_eq!(
    &*arrangement.get(&context),
    &HashMap::from_iter(vec![(('H', 5), 1), (('b', 11), 1), (('w', 5), 1)])
);
```

Make some more changes (and commit them):

```rust
input1.remove(&context, ('b', 6));
input2.add(&context, ('a', "Goodbye".to_string()));
context.commit();
```

Read the new output:
```rust
assert_eq!(
    &*arrangement.get(&context),
    &HashMap::from_iter(vec![
        (('G', 7), 1),
        (('H', 5), 1),
        (('a', 12), 1),
        (('w', 5), 1)
    ])
);
```

## Tips and Gotchas
* If the compiler is complaining, running slowly, or using too much memory, consider using
`Relation::dynamic` to simplify your type signatures. `Relation::t` is useful for keeping track of
a complex relation's item type.

* Changes are propagated lazily from inputs to outputs only when `Output::get` is
called. Calling `ExecutionContext::commit` only marks pending changes as "ready" to be propagated
and any downstream outputs as dirty.

* If for some reason you have created multiple `CreationContext`s, any function call involving
`Input`s, `Output`s, or `Relation`s from different contexts should result in a runtime panic with
the message "Context mismatch".

## Feedback Operators
In addition to creating acyclic relational graphs, it is also possible to create _cyclic_ graphs
where relational outputs feed back into inputs until all collections stabilize. To create a feedback
loop in this way, use one of the three `CreationContext::feed`, `CreationContext::feed_once`, or
`CreationContext::feed_ordered` methods.

TODO Explain these
