use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graph::UnGraph;
use rustworkx_core::Result;
use std::collections::HashMap;

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let input = lines_from_file("input/day25.txt");
    let mut graph = UnGraph::<(), ()>::new_undirected();
    let mut node_map = HashMap::new();

    for line in &input {
        let mut spl = line.split(": ");
        let from = spl.next().expect("Must have a start node");
        let to = spl
            .next()
            .expect("Must have target nodes")
            .split_whitespace();

        let from_node = *node_map.entry(from).or_insert_with(|| graph.add_node(()));

        for n in to {
            let to_node = node_map.entry(n).or_insert_with(|| graph.add_node(()));

            if !graph.contains_edge(from_node, *to_node) {
                graph.add_edge(from_node, *to_node, ());
            }
        }
    }

    let min_cut_res: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&graph, |_| Ok(1));

    let (min_cut, partition) = min_cut_res
        .expect("Must be a mininmum cut available")
        .expect("There must be 2 partitions");
    
    let sol1 = partition.len() * (graph.node_count() - partition.len());
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
