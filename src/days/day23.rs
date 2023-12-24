use petgraph::{
    graph::{NodeIndex, UnGraph},
    visit::EdgeRef,
};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

type PathState = (i32, i32, (i32, i32), Vec<(i32, i32)>);

pub fn solve() -> SolutionPair {
    let input: Vec<Vec<char>> = lines_from_file("input/day23.txt")
        .iter()
        .map(|s| s.chars().collect())
        .collect();

    let start: (i32, i32) = (0, 1);
    let target: (i32, i32) = (input.len() as i32 - 1, input[0].len() as i32 - 2);

    let sol1 = solve_pt1(start, target, &input);
    let sol2 = solve_pt2(start, target, &input);
    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_pt2(start: (i32, i32), target: (i32, i32), input: &Vec<Vec<char>>) -> i32 {
    let compressed = compress(&input, start, target);

    let mut sol2 = 0;
    find_longest_path(
        &compressed.0,
        compressed.1,
        compressed.2,
        &mut HashSet::new(),
        &mut Vec::new(),
        0,
        &mut sol2,
    );

    sol2
}

fn compress(
    grid: &Vec<Vec<char>>,
    st: (i32, i32),
    end: (i32, i32),
) -> (UnGraph<(i32, i32), i32>, NodeIndex, NodeIndex) {
    let mut graph = UnGraph::<(i32, i32), i32>::new_undirected();
    let mut node_map = HashMap::new();

    let start = graph.add_node(st);
    node_map.insert(st, start);
    let mut q = VecDeque::new();
    q.push_back((st, start, 0, (1, 0)));

    while let Some(tile) = q.pop_front() {
        let location = tile.0;
        let previous_node = graph[tile.1];
        if let Some(current_node) = node_map.get(&location) {
            if graph[*current_node] != previous_node {
                if !graph.contains_edge(*current_node, tile.1) {
                    graph.add_edge(*current_node, tile.1, tile.2);
                }
                continue;
            }
        }
        if grid
            .get(location.0 as usize)
            .and_then(|r| r.get(location.1 as usize))
            .unwrap_or(&'#')
            == &'#'
        {
            continue;
        }
        let mut routes = 0;
        for dir in [(0_i32, 1_i32), (0, -1), (1, 0), (-1, 0)] {
            let next_location = (location.0 + dir.0, location.1 + dir.1);
            let t = grid
                .get(next_location.0 as usize)
                .and_then(|c| c.get(next_location.1 as usize))
                .unwrap_or(&'#');
            if t != &'#' {
                routes += 1;
            }
        }
        let mut next_node = tile.1;
        let mut distance = tile.2;
        if routes > 2 || routes == 1 {
            next_node = *node_map
                .entry(location)
                .or_insert_with(|| graph.add_node(location));
            if !graph.contains_edge(tile.1, next_node) {
                graph.add_edge(tile.1, next_node, tile.2);
            }
            distance = 0;
        }
        for dir in [(0_i32, 1_i32), (0, -1), (1, 0), (-1, 0)] {
            if dir.0 == -tile.3 .0 && dir.1 == -tile.3 .1 {
                continue;
            }
            let next_location = (location.0 + dir.0, location.1 + dir.1);
            q.push_back((next_location, next_node, distance + 1, dir));
        }
    }
    (graph, start, *node_map.get(&end).expect("End must exist"))
}

fn find_longest_path(
    graph: &UnGraph<(i32, i32), i32>,
    current: NodeIndex,
    end: NodeIndex,
    visited: &mut HashSet<NodeIndex>,
    path: &mut Vec<NodeIndex>,
    current_length: i32,
    max_length: &mut i32,
) {
    if current == end {
        if current_length > *max_length {
            *max_length = current_length;
        }
        return;
    }

    visited.insert(current);
    path.push(current);

    for edge in graph.edges(current) {
        let neighbor = edge.target();
        if !visited.contains(&neighbor) {
            let edge_weight = *edge.weight();
            find_longest_path(
                graph,
                neighbor,
                end,
                visited,
                path,
                current_length + edge_weight,
                max_length,
            );
        }
    }

    path.pop();
    visited.remove(&current);
}

fn solve_pt1(start: (i32, i32), target: (i32, i32), input: &Vec<Vec<char>>) -> i32 {
    let mut q = BinaryHeap::<PathState>::new();
    q.push((0, 0, start, Vec::new()));

    let mut results = HashSet::new();
    while let Some((_cost, distance, location, journey)) = q.pop() {
        if journey.contains(&location) {
            continue;
        }
        if location == target {
            results.insert(distance);
        }
        if let Some(path) = input
            .get(location.0 as usize)
            .and_then(|r| r.get(location.1 as usize))
        {
            if path == &'#' {
                continue;
            }
            let mut new_journey = Vec::new();
            new_journey.extend(journey);
            new_journey.push(location);

            // Try A*?
            let mut next = Vec::new();
            match path {
                '^' => next.push((location.0 - 1, location.1)),
                '>' => next.push((location.0, location.1 + 1)),
                'v' => next.push((location.0 + 1, location.1)),
                '<' => next.push((location.0, location.1 - 1)),
                _ => {
                    next.push((location.0 - 1, location.1));
                    next.push((location.0, location.1 + 1));
                    next.push((location.0 + 1, location.1));
                    next.push((location.0, location.1 - 1));
                }
            }
            for n in next {
                let manhattan_dist = (n.1 - location.1).abs() + (n.0 - location.0);
                q.push((
                    distance + 1 + manhattan_dist,
                    distance + 1,
                    n,
                    new_journey.clone(),
                ));
            }
        }
    }
    *results
        .iter()
        .max()
        .expect("There is, inexplicably, not a path")
}