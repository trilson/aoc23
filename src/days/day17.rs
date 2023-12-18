use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};
pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day17.txt");

    let blocks: Vec<Vec<u32>> = lines
        .iter()
        .map(|l| l.chars().filter_map(|ch| ch.to_digit(10)).collect())
        .collect();

        let sol1 = min_loss_djikstra(&blocks, 0, 3);
        let sol2 = min_loss_djikstra(&blocks, 4, 10);
    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(PartialEq, Eq)]
struct PathState {
    location: (i32, i32),
    direction: (i32, i32),
    cont_moves: i32,
    heat_loss: i32,
}

impl Ord for PathState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn min_loss_djikstra(blocks: &Vec<Vec<u32>>, min: i32, max: i32) -> i32 {
    let mut queue = BinaryHeap::new();

    queue.push(PathState {
        location: (0, 0),
        direction: (0, 0),
        cont_moves: 0,
        heat_loss: 0,
    });

    let mut memo = HashSet::new();
    while let Some(st) = queue.pop() {
        let memo_key = (st.location, st.direction, st.cont_moves);
        if st.location.0 == blocks.len() as i32 - 1 && st.location.1 == blocks[0].len() as i32 - 1 {
            return st.heat_loss;
        }
        if memo.contains(&memo_key) {
            continue;
        }
        memo.insert(memo_key);

        if st.cont_moves < max && st.direction != (0, 0) {
            let next_loc = (
                st.location.0 + st.direction.0,
                st.location.1 + st.direction.1,
            );
            if let Some(hl) = blocks
                .get(next_loc.0 as usize)
                .and_then(|r| r.get(next_loc.1 as usize))
            {
                queue.push(PathState {
                    location: next_loc,
                    direction: st.direction,
                    cont_moves: st.cont_moves + 1,
                    heat_loss: *hl as i32 + st.heat_loss,
                });
            }
            if st.cont_moves < min {
                continue;
            }
        }
        for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if dir != st.direction && !(dir.0 == -st.direction.0 && dir.1 == -st.direction.1) {
                let next_loc = (st.location.0 + dir.0, st.location.1 + dir.1);
                if let Some(hl) = blocks
                    .get(next_loc.0 as usize)
                    .and_then(|r| r.get(next_loc.1 as usize))
                {
                    queue.push(PathState {
                        location: next_loc,
                        direction: dir,
                        cont_moves: 1,
                        heat_loss: *hl as i32 + st.heat_loss,
                    });
                }
            }
        }
    }
    0
}