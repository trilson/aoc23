use crate::{utils::files::lines_from_file, Solution, SolutionPair};
use itertools::Itertools;

pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day11.txt");
    let grid: Vec<String> = explode_grid(lines);
    let transposed = transpose(grid);
    let exploded = explode_grid(transposed);

    let mut galaxies: Vec<(i32, i32)> = Vec::new();
    for (idx, elx) in exploded.iter().enumerate() {
        for (idy, ely) in elx.chars().enumerate() {
            if ely == '#' {
                galaxies.push((idx as i32, idy as i32));
            }
        }
    }
    let distance_sum: i32 = galaxies.into_iter()
        .combinations(2)
        .map(|pair| {
            (pair.get(0).unwrap().0 - pair.get(1).unwrap().0).abs() +
            (pair.get(0).unwrap().1 - pair.get(1).unwrap().1).abs() 
        })
        .sum();
    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn explode_grid(source: Vec<String>) -> Vec<String> {
    let mut grid: Vec<String> = Vec::new();
    let mut idx_offset = 0;
    for row in source {
        grid.insert(idx_offset, row.clone());
        idx_offset += 1;
        if row.chars().all(|a| a == '.') {
            grid.insert(idx_offset, row);
            idx_offset += 1;
        }
    }
    grid
}

fn transpose(vec: Vec<String>) -> Vec<String> {
    (0..vec[0].len()).map(|i| {
        vec.iter()
            .map(|row| row.chars().nth(i).unwrap())
            .collect()
    }).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = super::solve();
        println!("{result:?}");
    }
}
