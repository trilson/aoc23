use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher}, cmp::min,
};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let grid = transpose(&lines_from_file("input/day14.txt"));
    let sol1 = solve_pt1(&grid);
    let sol2 = solve_pt2(&grid, 1_000_000_000);
    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_pt1(start_grid: &Vec<String>) -> i32 {
    let (weight, _tilted) = tilt_grid(&start_grid);
    weight
}

fn solve_pt2(start_grid: &Vec<String>, target: i64) -> i32 {
    let mut grid = start_grid.clone();
    let mut rot: i64 = 0;
    let mut memo: HashMap<u64, i64> = HashMap::new();

    while rot < 4 * target {
        let hash = grid_hash(&grid);

        if rot % 4 == 0 && memo.contains_key(&hash) {
            let previous_rotation = memo.get(&hash).unwrap();
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

fn transpose(vec: &Vec<String>) -> Vec<String> {
    (0..vec[0].len())
        .map(|i| vec.iter().map(|row| row.chars().nth(i).unwrap()).collect())
        .collect()
}

fn grid_hash(grid: &Vec<String>) -> u64 {
    let mut hasher = DefaultHasher::new();
    grid.hash(&mut hasher);
    hasher.finish()
}

fn tilt_grid(grid: &Vec<String>) -> (i32, Vec<String>) {
    let grid_length = grid.len() as i32;
    let mut weight = 0;

    let tilted: Vec<String> = grid
        .iter()
        .map(|el| {
            let mut consecutive_rocks = 0;
            let mut last_block = 0;
            let mut new_state = vec!['.'; grid_length as usize];
            for (chidx, ch) in el.char_indices() {
                if ch == '#' {
                    new_state[chidx] = '#';
                    last_block = chidx as i32 + 1;
                    consecutive_rocks = 0;
                } else if ch == 'O' {
                    new_state[(last_block + consecutive_rocks) as usize] = 'O';
                    consecutive_rocks += 1;
                    weight += (grid_length + 1) - last_block - consecutive_rocks;
                }
            }
            new_state.into_iter().collect()
        })
        .collect();

    (weight, tilted)
}

fn simple_weight(grid: &Vec<String>) -> i32 {
    let mut weight = 0;
    for (idx, r) in transpose(&grid).iter().enumerate() {
        let s = r.as_bytes().iter().filter(|&s| s == &b'O').count() as i32;
        weight += s * (grid.len() - idx) as i32;
    }
    weight
}

fn rotate_grid(grid: &Vec<String>) -> Vec<String> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut rotated_grid = vec![String::with_capacity(rows); cols];

    for i in (0..cols).rev() {
        for row in grid {
            rotated_grid[cols - 1 - i].push(row.chars().nth(i).unwrap());
        }
    }

    rotated_grid
}