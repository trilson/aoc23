use std::{
    cmp::{max, min},
    collections::{HashMap, VecDeque},
};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

const GRID_LEN: usize = 131;
const GRID_FLAT_SIZE: usize = GRID_LEN * GRID_LEN;

pub fn solve() -> SolutionPair {
    let lines: Vec<Vec<char>> = lines_from_file("input/day21.txt")
        .iter()
        .map(|s| s.chars().collect())
        .collect();

    let start_p = lines
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|c| c == &'S').map(|j| (i, j)))
        .unwrap_or((0, 0));

    let grid_str: String = lines.iter().flatten().collect();
    let grid_chars: Vec<char> = grid_str.chars().collect();
    let grid_char_arr: [char; GRID_FLAT_SIZE] = grid_chars.try_into().expect("Help, array fail");
    let start = grid_char_arr
        .iter()
        .position(|s| s == &'S')
        .expect("Must have a starting point") as i32;

    let sol1 = solve_iterative(&grid_char_arr, start, 64);

    let even = solve_iterative(&grid_char_arr, start, 130);
    let odd = solve_iterative(&grid_char_arr, start - 1, 130);

    let sol2 = solve_pt2(start_p, 26501365, &even.1, &odd.1);

    (Solution::from(sol1.0), Solution::from(sol2))
}

const OFFSETS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn solve_iterative(grid: &[char], start_point: i32, steps: i32) -> (i64, Vec<Vec<char>>) {
    let mut memo = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start_point, steps));

    // Can we simplify. For each spot in our flood, set to visited
    while let Some((point, remaining_steps)) = queue.pop_front() {
        if remaining_steps < 0 {
            continue;
        }
        if point > grid.len() as i32 {
            continue;
        }
        if grid.get(point as usize).is_none() {
            continue;
        }
        if grid[point as usize] == '#' {
            continue;
        }
        if memo.contains_key(&point) {
            continue;
        }
        for offset in &OFFSETS {
            let next_spot = point + offset.0 * GRID_LEN as i32 + offset.1;
            queue.push_back((next_spot, remaining_steps - 1));
        }
        memo.insert(point, remaining_steps % 2 == 0); // Memoize results
    }

    let mut rows = Vec::new();
    let mut vec = Vec::new();
    for (idc, ch) in grid.iter().enumerate() {
        if let Some(r) = memo.get(&(idc as i32)) {
            if *r {
                vec.push('0');
            } else {
                vec.push(*ch);
            }
        } else {
            vec.push(*ch);
        }
        if (idc + 1) % GRID_LEN == 0 {
            rows.push(vec.clone());
            vec = Vec::new();
        }
    }
    (memo.iter().filter(|s| *s.1).count() as i64, rows)
}

fn solve_pt2(
    start_point: (usize, usize),
    steps: i32,
    odd: &Vec<Vec<char>>,
    even: &Vec<Vec<char>>,
) -> i64 {
    let row_mid = start_point.0 as i32;
    let len_i32 = GRID_LEN as i32;

    let mut count: i64 = 0;
    let mut map = HashMap::new();

    // Start at the pointy end of the diamond and go to the other pointy end
    for i in -steps..=steps {
        if i % 100_000 == 0 {
            println!("{i}");
        }
        let row_offset = (row_mid + i) % len_i32;
        let synth_row = if row_offset < 0 {
            row_offset + len_i32
        } else {
            row_offset
        };

        let grid_offset = (row_mid + i.abs()) / len_i32;
        let offset = steps - i.abs();
        let row_from = row_mid - offset;
        let row_to = row_mid + offset - 1;

        let flipped = grid_offset % 2 == 0;

        let right_to = min(len_i32 - 1, row_to + 1) + 1;
        let left_from = max(0, row_from);
        let num_lengths = (1 + row_to - right_to) / len_i32;

        let use_current = num_lengths / 2;
        let use_inverse = num_lengths - use_current;

        let even_mult = use_current as i64
            * 2
            * get_hits_memo(even, odd, flipped, synth_row, 0, len_i32, &mut map);
        let odd_mult = use_inverse as i64
            * 2
            * get_hits_memo(even, odd, !flipped, synth_row, 0, len_i32, &mut map);

        let rem = 1 + ((1 + row_to - right_to) % len_i32).abs();
        {
            let mut right_hits =
                get_hits_memo(even, odd, flipped, synth_row, row_mid, right_to, &mut map);

            let mut left_hits =
                get_hits_memo(even, odd, flipped, synth_row, left_from, row_mid, &mut map);

            let use_even_grid = (num_lengths % 2 == 0) ^ flipped;
            if row_to + 1 >= len_i32 {
                let remainder_right =
                    get_hits_memo(even, odd, use_even_grid, synth_row, 0, rem, &mut map);
                let remainder_left = get_hits_memo(
                    even,
                    odd,
                    use_even_grid,
                    synth_row,
                    len_i32 - rem,
                    len_i32,
                    &mut map,
                );
                right_hits += remainder_right;
                left_hits += remainder_left;
            }
            count += right_hits + left_hits + even_mult + odd_mult
        }
    }
    count
}

fn get_hits_memo(
    grid_even: &Vec<Vec<char>>,
    grid_odd: &Vec<Vec<char>>,
    even: bool,
    row: i32,
    row_from: i32,
    row_to: i32,
    memo: &mut HashMap<(bool, i32, i32, i32), i64>,
) -> i64 {
    let memo_key = (even, row, row_from, row_to);
    if let Some(cached) = memo.get(&memo_key) {
        return *cached;
    }

    let grid = if even { grid_even } else { grid_odd };
    let result = grid[row as usize][row_from as usize..row_to as usize]
        .iter()
        .filter(|&c| c == &'0')
        .count() as i64;

    memo.insert(memo_key, result);
    result
}