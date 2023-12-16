use std::{cmp::max, collections::HashSet};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone)]
struct Tile {
    energised_count: i32,
    tile_type: char,
    visited: HashSet<Direction>,
}

impl Tile {
    fn accept_beam(&mut self, moving_direction: Direction) -> Option<Vec<Direction>> {
        if self.visited.contains(&moving_direction) {
            return None;
        }
        self.visited.insert(moving_direction);

        self.energised_count += 1;
        match self.tile_type {
            '|' => match moving_direction {
                Direction::UP | Direction::DOWN => Some(vec![moving_direction]),
                Direction::LEFT | Direction::RIGHT => Some(vec![Direction::UP, Direction::DOWN]),
            },
            '-' => match moving_direction {
                Direction::LEFT | Direction::RIGHT => Some(vec![moving_direction]),
                Direction::UP | Direction::DOWN => Some(vec![Direction::LEFT, Direction::RIGHT]),
            },
            '\\' => match moving_direction {
                Direction::UP => Some(vec![Direction::LEFT]),
                Direction::DOWN => Some(vec![Direction::RIGHT]),
                Direction::LEFT => Some(vec![Direction::UP]),
                Direction::RIGHT => Some(vec![Direction::DOWN]),
            },
            '/' => match moving_direction {
                Direction::UP => Some(vec![Direction::RIGHT]),
                Direction::DOWN => Some(vec![Direction::LEFT]),
                Direction::LEFT => Some(vec![Direction::DOWN]),
                Direction::RIGHT => Some(vec![Direction::UP]),
            },
            _ => Some(vec![moving_direction]),
        }
    }
}

impl From<(char, usize, usize)> for Tile {
    fn from(spot: (char, usize, usize)) -> Self {
        Tile {
            energised_count: 0,
            tile_type: spot.0,
            visited: HashSet::new(),
        }
    }
}

pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day16.txt");
    let tile_grid: Vec<Vec<Tile>> = lines
        .iter()
        .enumerate()
        .map(|(idx, l)| {
            l.char_indices()
                .map(|(idy, ch)| Tile::from((ch, idx, idy)))
                .collect()
        })
        .collect();

    let sol1 = num_energised_from((0, 0), Direction::RIGHT, &mut tile_grid.clone());
    let mut sol2 = 0;
    for i in 0..tile_grid.len() {
        sol2 = max(
            sol2,
            num_energised_from((i as i32, 0), Direction::RIGHT, &mut tile_grid.clone()),
        );
        sol2 = max(
            sol2,
            num_energised_from(
                (i as i32, tile_grid[0].len() as i32 - 1),
                Direction::LEFT,
                &mut tile_grid.clone(),
            ),
        );
    }

    for i in 0..tile_grid[0].len() {
        sol2 = max(
            sol2,
            num_energised_from((0, i as i32), Direction::DOWN, &mut tile_grid.clone()),
        );
        sol2 = max(
            sol2,
            num_energised_from(
                (tile_grid.len() as i32 - 1, 0),
                Direction::UP,
                &mut tile_grid.clone(),
            ),
        );
    }
    (Solution::from(sol1), Solution::from(sol2))
}

fn num_energised_from(
    location: (i32, i32),
    moving_direction: Direction,
    tile_grid: &mut Vec<Vec<Tile>>,
) -> i32 {
    let count = match tile_grid
        .get_mut(location.0 as usize)
        .and_then(|t| t.get_mut(location.1 as usize))
    {
        None => 0,
        Some(tile) => match tile.accept_beam(moving_direction) {
            Some(directions) => {
                let mut count = if tile.energised_count > 1 { 0 } else { 1 };
                for dir in directions {
                    let new_loc = get_location(location.0, location.1, dir);
                    count += num_energised_from(new_loc, dir, tile_grid);
                }
                count
            }
            None => 0,
        },
    };
    count
}

fn get_location(row: i32, col: i32, dir: Direction) -> (i32, i32) {
    match dir {
        Direction::UP => (row - 1, col),
        Direction::DOWN => (row + 1, col),
        Direction::LEFT => (row, col - 1),
        Direction::RIGHT => (row, col + 1),
    }
}