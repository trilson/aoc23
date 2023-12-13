use std::collections::VecDeque;

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day12.txt");
    let grids = lines.iter().fold(vec![vec![]], |mut acc, s| {
        if s.is_empty() {
            acc.push(vec![]);
        } else {
            acc.last_mut().unwrap().push(s.clone());
        }
        acc
    });

    let mut sum_pt1 = 0;
    let mut sum_pt2 = 0;
    for grid in grids {
        let transposed = transpose(&grid);
        sum_pt1 += solve_pt1(&grid, &transposed);
        sum_pt2 += solve_pt2(&grid, &transposed);
    }
    (Solution::from(sum_pt1), Solution::from(sum_pt2))
}

fn solve_pt1(rows: &Vec<String>, cols: &Vec<String>) -> i32 {
    let row_mirror = find_mirror_location(&rows);
    let mut col_mirror = 0;
    if row_mirror == 0 {
        col_mirror = find_mirror_location(&cols);
    }
    row_mirror * 100 + col_mirror
}

fn solve_pt2(rows: &Vec<String>, cols: &Vec<String>) -> i32 {
    let binary_grid = to_binary_grid(&rows);
    let row_index = mutation_index(binary_grid.as_slice());
    if row_index.is_none() {
        let transposed_binary_grid = to_binary_grid(cols);
        return mutation_index(transposed_binary_grid.as_slice()).unwrap_or(0);
    }
    row_index.unwrap_or(0) * 100
}

fn to_binary_grid(grid: &Vec<String>) -> Vec<u64> {
    grid.iter()
        .map(|s| {
            s.chars()
                .fold(0, |acc, c| (acc << 1) | if c == '#' { 1 } else { 0 })
        })
        .collect()
}

fn mutation_index(slice: &[u64]) -> Option<i32> {
    for (idx, _line) in slice.iter().enumerate() {
        let mut cast_left: i32 = idx as i32 - 1_i32;
        let mut cast_right = idx as i32;
        let mut mutations_needed = 0;
        while cast_left >= 0 && (cast_right as usize) < slice.len() {
            let diff = slice[cast_left as usize] ^ slice[cast_right as usize];
            mutations_needed += diff.count_ones();
            if mutations_needed >= 2 {
                break;
            }
            if cast_left == 0 {
                break;
            }
            cast_left -= 1;
            cast_right += 1;
        }
        if mutations_needed == 1 {
            return Some(idx as i32);
        }
    }
    None
}

fn transpose(vec: &Vec<String>) -> Vec<String> {
    (0..vec[0].len())
        .map(|i| vec.iter().map(|row| row.chars().nth(i).unwrap()).collect())
        .collect()
}

fn find_mirror_location(lines: &Vec<String>) -> i32 {
    let mut mirror_options: VecDeque<i32> = VecDeque::new();
    let mut el = 0;

    while el < lines.len() {
        let line = &lines[el];
        if let Some(mirror_location) = mirror_options.front() {
            let mirror_check = (2 * *mirror_location as i32) - 1 - el as i32;
            if mirror_check < 0 {
                return *mirror_location;
            }
            if lines[mirror_check as usize] != *line {
                mirror_options.pop_front();
                if let Some(next_mirror) = mirror_options.front() {
                    el = (next_mirror + 1) as usize;
                }
            }
        }
        if el > 0 && lines.get(el - 1).unwrap_or(&String::default()) == line {
            mirror_options.push_back(el as i32);
        }
        el += 1;
    }
    *mirror_options.back().unwrap_or(&0)
}