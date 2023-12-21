use std::collections::{HashMap, HashSet, VecDeque};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

const GRID_SIZE: usize = 131;
const GRID_LENGTH: usize = GRID_SIZE * GRID_SIZE;

pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day21.txt");
    let grid_str: String = lines.iter().flat_map(|row| row.chars()).collect();
    let grid_chars: Vec<char> = grid_str.chars().collect();
    let grid_char_arr: [char; GRID_LENGTH] = grid_chars.try_into().expect("Help, array fail");
    let start = grid_char_arr
        .iter()
        .position(|s| s == &'S')
        .expect("Must have a starting point") as i32;

    let res1 = solve_iterative(&grid_char_arr, start, 64);

    println!("Visited {} spots", res1.len());
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

const OFFSETS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn solve_iterative(
    grid: &[char],
    start_point: i32,
    steps: i32,
) -> HashSet<i32> {
    let mut memo = HashMap::new(); // Memoization for overlapping subproblems
    let mut queue = VecDeque::new(); // Queue for iterative exploration
    queue.push_back((start_point, steps));

    let mut reachable_points = HashSet::new();

    // Can we simplify. For each spot in our flood, set to visited
    while let Some((point, remaining_steps)) = queue.pop_front() {
        if remaining_steps < 0 {
            continue;
        }
        if point > grid.len() as i32 {
            continue;
        }
        if grid[point as usize] == '#' {
            continue;
        }
        let memo_key = (point, remaining_steps);
        if let Some(existing) = memo.get(&memo_key) {
            reachable_points.extend(existing);
            continue;
        }
        if remaining_steps == 0 {
            reachable_points.insert(point);
        } else {
            for offset in &OFFSETS {
                let next_spot = point + (offset.0 * grid_rows) + offset.1;
                queue.push_back((next_spot, remaining_steps - 1));
            }
        }
        memo.insert(memo_key, reachable_points.clone()); // Memoize results
    }

    reachable_points
}

#[test]
fn run_me() {
    let r = solve();
    println!("{r:?}");
}
