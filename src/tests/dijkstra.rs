use crate::CreationContext;
use std::hash::Hash;

fn dijkstra<'a, Node: Eq + Hash + Clone + 'a>(
    start: Node,
    end: Node,
    edges: impl IntoIterator<Item = (Node, Node, u64)>,
) -> Option<u64> {
    let mut context = CreationContext::new_();
    // A relation containing a single element: the start node
    let (start_inp, start_relation) = context.new_input::<Node>();
    // A relation containing a single element: the end node
    let (end_inp, end_relation) = context.new_input::<Node>();
    let (edge_inp, edge_relation) = context.new_input::<(Node, Node, u64)>();

    let (dists_input, dists) = context.new_input::<(Node, u64)>();
    let dists = dists.save();
    // The start node has distance 0
    context.feed(start_relation.map(|x| (x, 0)), dists_input.clone());
    // Stop as soon as the dists collection contains the end node
    context.interrupt(dists.get().semijoin(end_relation).get_output(&context), |m| {
        m.keys().next().unwrap().1
    });
    // Discover new connections via a `Relation::join` and feed them back in. Lower distances take
    // priority over higher ones.
    // Note that if we change this from `feed_ordered` to `feed`, this will become to a
    // breadth-first search and not necessarily find the lowest-weight path. This could be rectified
    // by moving the `interrupt` call to come after all the `feed` calls giving it lower priority,
    // but that would force the entire graph to be discovered before any answer can be returned. 
    context.feed_ordered(
        dists
            .get()
            .join(edge_relation.map(|(from, to, dist)| (from, (to, dist))))
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
