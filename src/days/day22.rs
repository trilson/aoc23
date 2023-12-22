use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap, HashSet},
};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

impl Brick {
    fn collides_with(&self, other_brick: &Brick) -> bool {
        let (x_start, x_end) = self.x;
        let (other_x_start, other_x_end) = other_brick.x;
        if x_end < other_x_start || x_start > other_x_end {
            return false;
        }
        let (y_start, y_end) = self.y;
        let (other_y_start, other_y_end) = other_brick.y;

        return y_end >= other_y_start && y_start <= other_y_end;
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.z.1.cmp(&self.z.1) {
            Ordering::Equal => self.id.cmp(&other.id),
            other_ordering => other_ordering,
        }
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<(usize, &String)> for Brick {
    fn from(value: (usize, &String)) -> Self {
        let components: Vec<&str> = value.1.split("~").collect();
        let from = components.get(0).expect("No from component found");
        let to = components.get(1).expect("No to component found");
        let coords: Vec<(i32, i32)> = from
            .split(",")
            .zip(to.split(","))
            .map(|(f, t)| (f.parse::<i32>().unwrap_or(0), t.parse::<i32>().unwrap_or(0)))
            .collect();

        Brick {
            x: *coords.get(0).expect("Must have x co-ordinate"),
            y: *coords.get(1).expect("Must have y co-ordinate"),
            z: *coords.get(2).expect("Must have z co-ordinate"),
            id: value.0,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Brick {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
    id: usize,
}

pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day22.txt");
    let mut bricks: Vec<Brick> = lines.iter().enumerate().map(|l| l.into()).collect();
    bricks.sort_by_key(|b| b.z.0);

    let (supporting, sitting) = extract_graph(bricks);
    let load_bearing = calculate_load_bearing(&sitting);

    let sol1 = &lines.len() - load_bearing.len();
    let sol2 = solve_pt2(load_bearing, sitting, supporting);

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_pt2(
    load_bearing: HashSet<usize>,
    sitting: HashMap<usize, HashSet<usize>>,
    supporting: HashMap<usize, HashSet<usize>>,
) -> i64 {
    let default = HashSet::new();
    let mut sol2 = 0;
    for b in load_bearing {
        let destroyed = sitting.get(&b).unwrap_or_else(|| &default);
        let r = chain_reaction(
            &HashSet::from([b]),
            &supporting,
            &sitting,
            &mut destroyed.clone(),
        ) - 1;
        sol2 += r;
    }
    sol2
}

fn calculate_load_bearing(sitting: &HashMap<usize, HashSet<usize>>) -> HashSet<usize> {
    let mut load_bearing = HashSet::new();
    for x in sitting.iter().filter(|s| s.1.len() == 1) {
        load_bearing.insert(x.1.iter().next().unwrap_or(&0));
    }
    load_bearing.into_iter().cloned().collect()
}

fn extract_graph(
    bricks: Vec<Brick>,
) -> (
    HashMap<usize, HashSet<usize>>,
    HashMap<usize, HashSet<usize>>,
) {
    let mut pile: BTreeSet<Brick> = BTreeSet::new();
    let mut supporting: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut sitting: HashMap<usize, HashSet<usize>> = HashMap::new();

    for mut falling in bricks {
        let (z, collisions) = collision_points(falling, &pile);

        let new_z = z + 1;
        falling.z = (new_z, new_z + (falling.z.1 - falling.z.0));
        pile.insert(falling);

        for collision_point in collisions {
            supporting
                .entry(collision_point.id)
                .or_insert(HashSet::new())
                .insert(falling.id);
            sitting
                .entry(falling.id)
                .or_insert(HashSet::new())
                .insert(collision_point.id);
        }
    }
    (supporting, sitting)
}

fn chain_reaction(
    brick_ids: &HashSet<usize>,
    supporting: &HashMap<usize, HashSet<usize>>,
    sitting: &HashMap<usize, HashSet<usize>>,
    destroyed: &mut HashSet<usize>,
) -> i64 {
    let mut collapsible: Vec<_> = brick_ids.iter().copied().collect();
    collapsible.retain(|&b| sitting.get(&b).map_or(true, |si| si.is_subset(destroyed)));
    let destroyable = collapsible.len() as i64;

    destroyed.extend(collapsible.iter().copied());

    let mut next_layer = HashSet::new();
    for &collapsed_brick in &collapsible {
        if let Some(support) = supporting.get(&collapsed_brick) {
            next_layer.extend(support);
        }
    }

    if next_layer.is_empty() {
        destroyable
    } else {
        destroyable + chain_reaction(&next_layer, supporting, sitting, destroyed)
    }
}

fn collision_points(falling_brick: Brick, pile: &BTreeSet<Brick>) -> (i32, Vec<Brick>) {
    let mut collisions = Vec::new();
    let mut z = 0;
    for stable_brick in pile {
        if stable_brick.z.1 < z {
            break;
        }
        if falling_brick.collides_with(&stable_brick) {
            z = stable_brick.z.1;
            collisions.push(*stable_brick);
        }
    }
    (z, collisions)
}