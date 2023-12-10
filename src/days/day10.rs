use std::collections::HashSet;

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

#[derive(Debug)]
struct PipeSection {
    y_from: i32,
    y_to: i32,
    x_from: i32,
    x_to: i32,
}

impl PipeSection {
    fn new(y_from: i32, y_to: i32, x_from: i32, x_to: i32) -> PipeSection {
        PipeSection {
            y_from,
            y_to,
            x_from,
            x_to,
        }
    }

    fn from_char(value: char) -> Option<Self> {
        match value {
            '|' => Some(PipeSection::new(-1, 1, 0, 0)),
            '-' => Some(PipeSection::new(0, 0, -1, 1)),
            'L' => Some(PipeSection::new(-1, 0, 0, 1)),
            'J' => Some(PipeSection::new(0, -1, -1, 0)),
            '7' => Some(PipeSection::new(1, 0, 0, -1)),
            'F' => Some(PipeSection::new(1, 0, 0, 1)),
            'S' => Some(PipeSection::new(0, 0, 0, 0)),
            _ => None,
        }
    }
}

pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day10.txt");
    let (grid, start) = parse_grid(&lines);

    let mut main_loop = HashSet::new();
    let mut previous = start;
    let mut current = get_pipe_start(&grid, (start.0, start.1));
    // This should be much much cleaner...
    main_loop.insert(previous);
    main_loop.insert(current);

    let mut steps = 0;

    while current != start {
        if let Some(pipe) = get_pipe_element(&grid, current) {
            let (x_offset, y_offset) =
                if previous == (pipe.x_from + current.0, pipe.y_from + current.1) {
                    (pipe.x_to, pipe.y_to)
                } else {
                    (pipe.x_from, pipe.y_from)
                };

            previous = current;
            current = (x_offset + current.0, y_offset + current.1);
            main_loop.insert(current);
            steps += 1;
        }
    }

    let mut captured_points = 0;
    for (idx, line) in lines.iter().enumerate() {
        let mut pipe_count = 0;
        for (idx2, ch) in line.chars().enumerate() {
            let coord = (idx2 as i32, idx as i32);
            if main_loop.contains(&coord) && ['L', '|', 'J', 'S'].contains(&ch) {
                pipe_count += 1;
            }
            if !main_loop.contains(&coord) && pipe_count % 2 != 0 {
                captured_points += 1;
            }
        }
    }

    (
        Solution::from((steps + 1) / 2),
        Solution::from(captured_points),
    )
}

fn get_pipe_element(grid: &Vec<Vec<Option<PipeSection>>>, x_y: (i32, i32)) -> &Option<PipeSection> {
    let x = grid.get(x_y.0 as usize);
    if let Some(x_vec) = x {
        if let Some(y) = x_vec.get(x_y.1 as usize) {
            return y;
        }
    }
    &None
}

fn get_pipe_start(grid: &Vec<Vec<Option<PipeSection>>>, start: (i32, i32)) -> (i32, i32) {
    for tile in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
        let x_try = start.0 + tile.0;
        let y_try = start.1 + tile.1;

        if x_try < 0 || y_try < 0 {
            continue;
        }

        let result = grid.get(x_try as usize)
            .and_then(|x_vec| x_vec.get(y_try as usize))
            .and_then(|section| section.as_ref())
            .filter(|pipe| pipe.x_from == tile.0 as i32 || pipe.y_from == tile.1 as i32);

        if result.is_some() {
            return (x_try, y_try);
        }
    }
    (0, 0)
}

fn parse_grid(lines: &Vec<String>) -> (Vec<Vec<Option<PipeSection>>>, (i32, i32)) {
    let mut map: Vec<Vec<Option<PipeSection>>> = Vec::new();
    let mut start = (0, 0);
    for (idy, line) in lines.iter().enumerate() {
        for (idx, char) in line.chars().enumerate() {
            let section = PipeSection::from_char(char);
            if let Some(x) = map.get_mut(idx) {
                x.push(section);
            } else {
                map.push(vec![section]);
            }
            if char == 'S' {
                start = (idx as i32, idy as i32);
            }
        }
    }
    (map, start)
}
