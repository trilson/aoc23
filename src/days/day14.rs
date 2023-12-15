use std::{
    cmp::min,
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let grid = transpose(&lines_from_file("input/day14.txt"));
    let sol1 = solve_pt1(&grid);
    let sol2 = solve_pt2(&grid, 1_000_000_000);
    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_pt1(start_grid: &[String]) -> i32 {
    let (weight, _tilted) = tilt_grid(start_grid);
    weight
}

fn solve_pt2(start_grid: &[String], target: i64) -> i32 {
    let mut grid = start_grid.to_vec();
    let mut rot: i64 = 0;
    let mut memo: HashMap<u64, i64> = HashMap::new();

    while rot < 4 * target {
        let hash = grid_hash(&grid);

        if let Some(&previous_rotation) = memo.get(&hash) {
            let rotation_gap = rot - previous_rotation;
            let cycles = rotation_gap / 4;

            if cycles > 0 {
                let remaining_full_cycles = (4 * target - rot) / 4;
                let skip_cycles = min(remaining_full_cycles / cycles, target / cycles);
                rot += skip_cycles * cycles * 4;
            }
        }

        if rot % 4 == 0 {
            memo.insert(hash, rot);
        }

        let (_tilted_weight, grid_state) = tilt_grid(&grid);
        grid = rotate_grid(&grid_state);
        rot += 1;
    }
    simple_weight(&grid)
}

fn transpose(vec: &[String]) -> Vec<String> {
    (0..vec[0].len())
        .map(|i| vec.iter().map(|row| row.as_bytes()[i] as char).collect())
        .collect()
}

fn grid_hash(grid: &[String]) -> u64 {
    let mut hasher = DefaultHasher::new();
    grid.hash(&mut hasher);
    hasher.finish()
}

fn tilt_grid(grid: &[String]) -> (i32, Vec<String>) {
    let grid_length = grid.len() as i32;
    let mut weight = 0;

    let tilted: Vec<String> = grid
        .iter()
        .map(|row| {
            let mut new_state = vec!['.'; grid_length as usize];
            let mut last_block = 0;
            let mut consecutive_rocks = 0;

            for (chidx, &ch) in row.as_bytes().iter().enumerate() {
                match ch as char {
                    '#' => {
                        new_state[chidx] = '#';
                        last_block = chidx as i32 + 1;
                        consecutive_rocks = 0;
                    }
                    'O' => {
                        new_state[(last_block + consecutive_rocks) as usize] = 'O';
                        consecutive_rocks += 1;
                        weight += (grid_length + 1) - last_block - consecutive_rocks;
                    }
                    _ => {}
                }
            }

            new_state.into_iter().collect()
        })
        .collect();

    (weight, tilted)
}

fn simple_weight(grid: &[String]) -> i32 {
    let mut weight = 0;
    for (idx, row) in transpose(grid).iter().enumerate() {
        let s = row.bytes().filter(|&s| s == b'O').count() as i32;
        weight += s * (grid.len() - idx) as i32;
    }
    weight
}

fn rotate_grid(grid: &[String]) -> Vec<String> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut rotated_grid = vec![String::with_capacity(rows); cols];

    for i in (0..cols).rev() {
        for row in grid {
            rotated_grid[cols - 1 - i].push(row.as_bytes()[i] as char);
        }
    }

    rotated_grid
}
