use crate::{CreationContext, Op, Relation};
use std::hash::Hash;

fn dijkstra<'a, Node: Eq + Hash + Clone + 'a>(
    context: &mut CreationContext<'a, u64>,
    starts: Relation<impl Op<D = Node> + 'a>,
    ends: Relation<impl Op<D = Node> + 'a>,
    edges: Relation<impl Op<D = (Node, Node, u64)> + 'a>,
) {
    let (dists_input, dists) = context.new_input::<(Node, u64)>();
    let dists = dists.save();
    context.feed(starts.map(|x| (x, 0)), dists_input.clone());
    context.interrupt_nonempty(dists.get().semijoin(ends).get_output(context), |m| {
        m.keys().next().unwrap().1
    });
    context.feed_ordered(
        dists
            .get()
            .join(edges.map(|(from, to, dist)| (from, (to, dist))))
            .map(|(_, dfrom, (to, dist))| (to, dfrom + dist))
            .group_min()
            .map(|(x, dist)| (dist, (x, dist))),
        dists_input,
    );
}

#[test]
fn test_dijkstra() {
    let mut context = CreationContext::new_();
    let (start_inp, starts) = context.new_input::<char>();
    let (end_inp, ends) = context.new_input::<char>();
    let (edge_inp, edges) = context.new_input::<(char, char, u64)>();
    dijkstra(&mut context, starts, ends, edges);

    let mut context = context.begin();
    start_inp.add(&context, 'A');
    end_inp.add(&context, 'F');
    edge_inp.add_all(
        &context,
        vec![
            ('A', 'B', 1),
            ('A', 'C', 2),
            ('A', 'F', 7),
            ('B', 'D', 2),
            ('C', 'E', 3),
            ('D', 'A', 1),
            ('D', 'E', 1),
            ('E', 'A', 1),
            ('E', 'F', 1),
        ],
    );

    assert_eq!(context.commit(), Some(5));
}
