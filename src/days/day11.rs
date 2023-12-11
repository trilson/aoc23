use std::collections::HashSet;

use crate::{utils::files::lines_from_file, Solution, SolutionPair};
use itertools::Itertools;

pub fn solve() -> SolutionPair {
    let lines: Vec<String> = lines_from_file("input/day11.txt");
    let empty_rows = empty_lines(&lines);
    let transposed = transpose(&lines);
    let empty_cols = empty_lines(&transposed);

    let mut galaxies: Vec<(i64, i64)> = Vec::new();
    for (idx, elx) in lines.iter().enumerate() {
        for (idy, ely) in elx.chars().enumerate() {
            if ely == '#' {
                galaxies.push((idx as i64, idy as i64));
            }
        }
    }
    let sol1 = distance(&galaxies, &empty_cols, &empty_rows, 2);
    let sol2 = distance(&galaxies, &empty_cols, &empty_rows, 1_000_000);

    (Solution::from(sol1), Solution::from(sol2))
}

fn distance(
    galaxies: &Vec<(i64, i64)>,
    empty_cols: &HashSet<i64>,
    empty_rows: &HashSet<i64>,
    explosion_factor: i64,
) -> i64 {
    galaxies
        .into_iter()
        .combinations(2)
        .map(|pair| {
            let mut sum: i64 = 0;

            let row_from = pair.get(0).unwrap().0;
            let row_to = pair.get(1).unwrap().0;
            let col_from = pair.get(0).unwrap().1;
            let col_to = pair.get(1).unwrap().1;

            for empty_row in empty_rows {
                if empty_row > &row_from.min(row_to) && empty_row < &row_from.max(row_to) {
                    sum += explosion_factor - 1;
                }
            }
            sum += (row_to - row_from).abs();
            for empty_col in empty_cols {
                if empty_col > &col_from.min(col_to) && empty_col < &col_from.max(col_to) {
                    sum += explosion_factor - 1;
                }
            }
            sum += (col_to - col_from).abs();
            sum
        })
        .sum()
}

fn empty_lines(source: &Vec<String>) -> HashSet<i64> {
    source
        .iter()
        .enumerate()
        .filter_map(|(idx, row)| {
            if row.chars().all(|a| a == '.') {
                return Some(idx as i64);
            }
            None
        })
        .collect()
}

fn transpose(vec: &Vec<String>) -> Vec<String> {
    (0..vec[0].len())
        .map(|i| vec.iter().map(|row| row.chars().nth(i).unwrap()).collect())
        .collect()
}
