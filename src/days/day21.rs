use std::{
    collections::{HashSet, VecDeque},
    ops::Add,
};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

#[derive(PartialEq, Eq, Hash)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn default() -> Self {
        Point { row: 0, col: 0 }
    }
}

impl From<(usize, usize)> for Point {
    fn from(t: (usize, usize)) -> Self {
        Point {
            row: t.0 as i32,
            col: t.1 as i32,
        }
    }
}

impl From<(i32, i32)> for Point {
    fn from(t: (i32, i32)) -> Self {
        Point { row: t.0, col: t.1 }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day21.txt");
    let mut position: Point = Point::default();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for (id_row, line) in lines.iter().enumerate() {
        let mut l_chars = Vec::new();
        for (id_col, ch) in line.char_indices() {
            if ch == 'S' {
                position = (id_row, id_col).into();
            }
            l_chars.push(ch);
        }
        grid.push(l_chars);
    }

    let target = 64;
    let mut visited = HashSet::new();
    let mut state_queue = VecDeque::new();
    let point_offsets: [Point; 4] = [(1, 0).into(), (-1, 0).into(), (0, 1).into(), (0, -1).into()];
    state_queue.push_back((position, 0));

    while let Some(state) = state_queue.pop_front() {
        if let Some(spot) = grid
            .get(state.0.row as usize)
            .and_then(|r| r.get(state.0.col as usize))
        {
            if spot == &'#' {
                continue;
            }
            if state.1 == target {
                visited.insert(state.0);
                continue;
            }
            // Otherwise check surrounding squares
            for offset in &point_offsets {
                state_queue.push_back((state.0 + offset, state.1 + 1));
            }
        }
    }

    println!("Visited {} spots", visited.len());
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

const OFFSETS: [Point; 4] = [(1, 0).into(), (-1, 0).into(), (0, 1).into(), (0, -1).into()];

fn solve_recursive(grid: &Vec<Vec<char>>, point: Point, steps: i32) -> HashSet<Point> {
    if let Some(spot) = grid
        .get(point.row as usize)
        .and_then(|r| r.get(point.col as usize))
    {
        if spot == &'#' {
            return HashSet::default();
        }
        if steps == 0 {
            return HashSet::from([point]);
        }
        let mut points = HashSet::default();
        for offset in OFFSETS {
            let res = solve_recursive(&grid, offset + point, steps - 1);
        }
    }
    return HashSet::default();
}
#[test]
fn run_me() {
    let r = solve();
    println!("{r:?}");
}
