use std::{
    cmp::{max, min},
    collections::HashMap,
};

use itertools::Itertools;

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

#[derive(Clone)]
struct Block {
    rel_location: (i64, i64),
    flooded: bool,
}

pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day18.txt");
    let mut walls = Vec::new();
    let mut hwalls = HashMap::new();
    let mut current_location = (0, 0);

    let mut vertices = Vec::new();
    for line in &lines {
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
        // TEMP
        let dir = sp[0].chars().nth(0).unwrap_or('R');
        let dist = sp[1].parse::<i64>().unwrap_or(0);
        let delta = dir_delta(dir);

        println!("{} {:?}", dist, delta);
        // Jump ahead to the next location
        let next_location = (
            current_location.0 + (dist * delta.0),
            current_location.1 + (dist * delta.1),
        );
        vertices.push(next_location);
        // We're going along columns (creating a horizontal wall). We don't care about vertical walls
        if delta.0 == 0 {
            hwalls
                .entry(current_location.0)
                .or_insert(Vec::new())
                .push(((
                    min(current_location.1, next_location.1),
                    max(current_location.1, next_location.1)),
                    0
                ));

            walls.push((
                current_location.0,
                min(current_location.1, next_location.1),
                max(current_location.1, next_location.1),
                0, // for later use
            ));
            println!("HORIZONTAL WALL @ {:?}", walls.last().unwrap());
        }

        current_location = next_location;
    }

    let area = shoelace_formula(&vertices);
    
    let mut total = 0;
    let mut hkeys: Vec<&i64> = hwalls.keys().collect_vec();
    hkeys.sort();
    
    let mut prev_index = 0;
    let mut prev = hwalls[hkeys[prev_index]].clone();

    while true {
        let next_idx = prev_index + 1;
        println!("Comparing lines @ y={} to y={}", hkeys[prev_index], hkeys[next_idx]);        
        println!("Lines (synth) @ y={}: {:?}", hkeys[prev_index], prev);
        let next = &hwalls[hkeys[next_idx]];
        println!("Lines @ y={}: {:?}", hkeys[next_idx], next);

        let mut next_state = Vec::new();
        for prev_wall in prev {
            for next_wall in next {
                let mut intersection = intersect_ranges(&prev_wall.0, &next_wall.0);
                if let Some(intersected) = intersection.0 {
                    let points = (hkeys[next_idx] - hkeys[prev_index]) * (intersected.1 - intersected.0);
                    println!("Adding {intersected:?}, points: {points}");
                    // next_state.push((intersected, prev_wall.1 + 1));
                }
                for inter in intersection.1 {
                    next_state.push((inter, prev_wall.1 + 1));
                }
            }
        }

        println!("We've created a new state... {:?}", next_state);
        prev = next_state;
        prev_index = next_idx;
    }
    (Solution::from(0), Solution::from(0))
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

fn intersect_ranges(
    range1: &(i64, i64),
    range2: &(i64, i64),
) -> (Option<(i64, i64)>, Vec<(i64, i64)>) {
    let (start1, end1) = range1.to_owned();
    let (start2, end2) = range2.to_owned();

    // Calculate the intersection
    let max_start = start1.max(start2);
    let min_end = end1.min(end2);

    // Check if there is an intersection
    let intersection = if max_start <= min_end {
        Some((max_start, min_end))
    } else {
        None
    };

    // Determine non-intersecting ranges
    let mut non_intersecting = Vec::new();
    if start1 < max_start {
        non_intersecting.push((start1, max_start - 1));
    }
    if end1 > min_end {
        non_intersecting.push((min_end + 1, end1));
    }
    if start2 < max_start {
        non_intersecting.push((start2, max_start - 1));
    }
    if end2 > min_end {
        non_intersecting.push((min_end + 1, end2));
    }

    // Return the tuple
    (intersection, non_intersecting)
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

#[test]
fn run() {
    println!("{:?}", solve());
}
