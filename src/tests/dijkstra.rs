use crate::CreationContext;
use std::hash::Hash;

fn dijkstra<'a, Node: Eq + Hash + Clone + 'a>(
    start: Node,
    end: Node,
    edges: impl IntoIterator<Item = (Node, Node, u64)>,
) -> Option<u64> {
    let mut context = CreationContext::new_();
    let (start_inp, starts_c) = context.new_input::<Node>();
    let (end_inp, ends_c) = context.new_input::<Node>();
    let (edge_inp, edges_c) = context.new_input::<(Node, Node, u64)>();

    let (dists_input, dists) = context.new_input::<(Node, u64)>();
    let dists = dists.save();
    context.feed(starts_c.map(|x| (x, 0)), dists_input.clone());
    context.interrupt_nonempty(dists.get().semijoin(ends_c).get_output(&context), |m| {
        m.keys().next().unwrap().1
    });
    context.feed_ordered(
        dists
            .get()
            .join(edges_c.map(|(from, to, dist)| (from, (to, dist))))
            .map(|(_, dfrom, (to, dist))| (to, dfrom + dist))
            .group_min()
            .map(|(x, dist)| (dist, (x, dist))),
        dists_input,
    );

    let mut context = context.begin();
    start_inp.add(&context, start);
    end_inp.add(&context, end);
    edge_inp.add_all(&context, edges);

    context.commit()
}

#[test]
fn test_dijkstra() {
    let dist = dijkstra(
        'A',
        'F',
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

    assert_eq!(dist, Some(5));
}
