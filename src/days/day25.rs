use petgraph::visit::Dfs;
use petgraph::{
    dot::{Config, Dot},
    graph::UnGraph,
};
use std::collections::HashSet;
use std::io::Write;
use std::{collections::HashMap, fs::File};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let input = lines_from_file("input/day25.txt");
    let mut graph = UnGraph::<String, ()>::new_undirected();
    let mut node_map = HashMap::new();

    for line in &input {
        let mut spl = line.split(": ");
        let from = spl.next().expect("Must have a start node");
        let to = spl
            .next()
            .expect("Must have target nodes")
            .split_whitespace();

        let from_node = *node_map
            .entry(from)
            .or_insert_with(|| graph.add_node(from.to_owned()));

        for n in to {
            let to_node = node_map
                .entry(n)
                .or_insert_with(|| graph.add_node(n.to_owned()));

            if !graph.contains_edge(from_node, *to_node) {
                graph.add_edge(from_node, *to_node, ());
            }
        }
    }

    // Analyze the output of this using GraphViz...
    // At some point, we might want to revisit this and implement Kargers
    write_graph_dot(&graph);
    let njn = node_map.get("njn").expect("njn must exist");
    let xtx = node_map.get("xtx").expect("xtx must exist");

    let rhh = node_map.get("rhh").expect("rhh must exist");
    let mtc = node_map.get("mtc").expect("mtc must exist");

    let tmb = node_map.get("tmb").expect("tmb must exist");
    let gpj = node_map.get("gpj").expect("gpj must exist");

    let e1 = graph.find_edge(*njn, *xtx).expect("e1 must exist");
    let e2 = graph.find_edge(*rhh, *mtc).expect("e2 must exist");
    let e3 = graph.find_edge(*tmb, *gpj).expect("e3 must exist");

    graph.remove_edge(e1);
    graph.remove_edge(e2);
    graph.remove_edge(e3);

    let sol1: i32 = solve_pt1(&graph);
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_pt1(graph: &petgraph::prelude::Graph<String, (), petgraph::prelude::Undirected>) -> i32 {
    let mut component_sizes = Vec::new();
    let mut dfs = Dfs::empty(&graph);
    let mut visited = HashSet::new();

    for node_index in graph.node_indices() {
        if !visited.contains(&node_index) {
            let mut size = 0;
            dfs.move_to(node_index);
            visited.insert(node_index);
            while let Some(nx) = dfs.next(&graph) {
                size += 1;
                visited.insert(nx);
            }
            component_sizes.push(size);
        }
    }

    component_sizes[0] * component_sizes[1]
}

fn write_graph_dot(graph: &petgraph::prelude::Graph<String, (), petgraph::prelude::Undirected>) {
    let graph_dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);

    let mut file = File::create("input/day25_graph.dot").expect("Could not create file");
    writeln!(file, "{:?}", graph_dot).expect("Error writing to file");
}