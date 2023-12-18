use std::cmp::{max, min};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

#[derive(Clone)]
struct Block {
    rel_location: (i64, i64),
    flooded: bool,
}

pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day18.txt");
    let sol1 = solve_pt1(&lines);
    let sol2 = solve_pt2(&lines);
    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_pt2(lines: &Vec<String>) -> i64 {
    let mut current_location = (0, 0);
    let mut path_length = 0;
    let mut vertices = Vec::new();
    for line in lines {
        let sp: Vec<&str> = line.split_whitespace().collect();
        let dist = i64::from_str_radix(&sp[2][2..7], 16).unwrap_or(0);
        let delta: (i64, i64) = sp[2]
            .chars()
            .nth(7)
            .map(|c| match c {
                '0' => (0, 1),
                '1' => (1, 0),
                '2' => (0, -1),
                '3' => (-1, 0),
                _ => panic!("Not a valid direction"),
            })
            .unwrap();
        let next_location = (
            current_location.0 + (dist * delta.0),
            current_location.1 + (dist * delta.1),
        );
        vertices.push(next_location);
        path_length += (dist * delta.0).abs() + (dist * delta.1).abs();
        current_location = next_location;
    }

    let area = shoelace_formula(&vertices);
    area + (path_length / 2) + 1
}

fn shoelace_formula(vertices: &[(i64, i64)]) -> i64 {
    let mut area = 0_i64;
    let n = vertices.len();

    for i in 0..n {
        let j = (i + 1) % n;
        area += vertices[i].0 * vertices[j].1;
        area -= vertices[i].1 * vertices[j].0;
    }

    area.abs() / 2
}

fn solve_pt1(lines: &Vec<String>) -> i64 {
    let mut path: Vec<Block> = Vec::new();

    let mut loc = (0, 0);
    let mut max_row = i64::MIN;
    let mut min_row = i64::MAX;
    let mut max_col = i64::MIN;
    let mut min_col = i64::MAX;

    for line in lines {
        let sp: Vec<&str> = line.split_whitespace().collect();
        let dir = sp[0].chars().nth(0).unwrap_or('R');
        let dist = sp[1].parse::<i64>().unwrap_or(0);
        let delta = dir_delta(dir);

        for _i in 1..dist + 1 {
            loc = (loc.0 + delta.0, loc.1 + delta.1);
            max_row = max(max_row, loc.0);
            min_row = min(min_row, loc.0);
            max_col = max(max_col, loc.1);
            min_col = min(min_col, loc.1);

            path.push(Block {
                rel_location: loc,
                flooded: true,
            })
        }
    }

    let mut grid: Vec<Vec<Block>> = Vec::with_capacity((max_row - min_row) as usize + 1);
    for _row in 0..(max_row - min_row) + 1 {
        grid.push(vec![
            Block {
                rel_location: (0, 0),
                flooded: false
            };
            (max_col - min_col) as usize + 1
        ]);
    }

    let path_length = path.len();
    for block in path {
        let (row_id, col_id) = block.rel_location;
        let abs_row_id = row_id - min_row;
        let abs_col_id = col_id - min_col;
        if let Some(row) = grid.get_mut(abs_row_id as usize) {
            row[abs_col_id as usize] = block;
        }
    }
    flood_fill(&mut grid, 246, 224) + path_length as i64
}

fn dir_delta(dir: char) -> (i64, i64) {
    match dir {
        'R' => (0, 1),
        'L' => (0, -1),
        'U' => (-1, 0),
        'D' => (1, 0),
        _ => panic!("Not a valid direction"),
    }
}

fn flood_fill(grid: &mut Vec<Vec<Block>>, start_x: i64, start_y: i64) -> i64 {
    let mut stack = vec![(start_x, start_y)];
    let mut area = 0;

    while let Some((x, y)) = stack.pop() {
        if x < 0
            || y < 0
            || x as usize >= grid.len()
            || y as usize >= grid[0].len()
            || grid[x as usize][y as usize].flooded
        {
            continue;
        }

        grid[x as usize][y as usize].flooded = true;
        area += 1;

        stack.push((x + 1, y));
        stack.push((x, y + 1));
        stack.push((x - 1, y));
        stack.push((x, y - 1));
    }
    area
}