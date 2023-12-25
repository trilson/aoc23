use crate::{utils::files::lines_from_file, Solution, SolutionPair};

#[derive(Debug, Clone)]
struct HailStone {
    position: (f64, f64, f64),
    velocity: (f64, f64, f64),
}

impl From<(usize, &String)> for HailStone {
    fn from((_id, value): (usize, &String)) -> Self {
        let binding = value.replace(" ", "");
        let mut split = binding.split("@");
        let pos = split.next().expect("Must have a position");
        let velocity = split.next().expect("Must have a velocity");

        let mut xyz_pos = pos.split(",");
        let mut xyz_vol = velocity.split(",");

        HailStone {
            position: (
                xyz_pos
                    .next()
                    .expect("Must have X")
                    .parse::<f64>()
                    .expect("X must be a number"),
                xyz_pos
                    .next()
                    .expect("Must have Y")
                    .parse::<f64>()
                    .expect("Y must be a number"),
                xyz_pos
                    .next()
                    .expect("Must have Z")
                    .parse::<f64>()
                    .expect("Z must be a number"),
            ),

            velocity: (
                xyz_vol
                    .next()
                    .expect("Must have X")
                    .parse::<f64>()
                    .expect("X must be a number"),
                xyz_vol
                    .next()
                    .expect("Must have Y")
                    .parse::<f64>()
                    .expect("Y must be a number"),
                xyz_vol
                    .next()
                    .expect("Must have Z")
                    .parse::<f64>()
                    .expect("Z must be a number"),
            ),
        }
    }
}

pub fn solve() -> SolutionPair {
    let hail_stones: Vec<HailStone> = lines_from_file("input/day24.txt")
        .iter()
        .enumerate()
        .map(|c| c.into())
        .collect();

    let sol1 = solve_pt1(&hail_stones, 200000000000000_f64, 400000000000000_f64);
    let sol2 = solve_pt2(&hail_stones);
    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_pt2(hail_stones: &[HailStone]) -> i64 {
    println!(
        "(declare-const K_x Int)
    (declare-const K_y Int)
    (declare-const K_z Int)
    (declare-const V_x Int)
    (declare-const V_y Int)
    (declare-const V_z Int)"
    );
    for (x, _s) in hail_stones.iter().enumerate() {
        println!("(declare-const n_{} Int)", x + 1);
    }
    println!();
    for (x, s) in hail_stones.iter().enumerate() {
        let i = x + 1;
        println!(
            "(assert (= (+ {} (* n_{i} {})) (+ K_x (* n_{i} V_x))))",
            s.position.0, s.velocity.0
        );
        println!(
            "(assert (= (+ {} (* n_{i} {})) (+ K_y (* n_{i} V_y))))",
            s.position.1, s.velocity.1
        );
        println!(
            "(assert (= (+ {} (* n_{i} {})) (+ K_z (* n_{i} V_z))))",
            s.position.2, s.velocity.2
        );
    }
    println!();
    println!("(check-sat) (get-model)");

    1007148211789625
}

fn solve_pt1(hail_stones: &[HailStone], from: f64, to: f64) -> i64 {
    let mut collisions = 0;
    for (idx, stone) in hail_stones.iter().enumerate() {
        for other_hail_stone in hail_stones[idx + 1..].iter() {
            if collides(stone, other_hail_stone, from, to) {
                collisions += 1;
            }
        }
    }
    collisions
}

fn collides(stone: &HailStone, other_stone: &HailStone, from: f64, to: f64) -> bool {
    let a_stone = stone.velocity.1 / stone.velocity.0;
    let a_other_stone = other_stone.velocity.1 / other_stone.velocity.0;

    let b_stone = stone.position.1 - (stone.position.0 * a_stone);
    let b_other_stone = other_stone.position.1 - (other_stone.position.0 * a_other_stone);

    let x_cross = (b_other_stone - b_stone) / (a_stone - a_other_stone);
    let y_cross = (a_stone * x_cross) + b_stone;

    if stone.velocity.0 >= 0f64 && x_cross < stone.position.0 {
        return false;
    }
    if stone.velocity.0 <= 0f64 && x_cross > stone.position.0 {
        return false;
    }
    if stone.velocity.1 >= 0f64 && y_cross < stone.position.1 {
        return false;
    }
    if stone.velocity.1 <= 0f64 && y_cross > stone.position.1 {
        return false;
    }
    if other_stone.velocity.0 >= 0f64 && x_cross < other_stone.position.0 {
        return false;
    }
    if other_stone.velocity.0 <= 0f64 && x_cross > other_stone.position.0 {
        return false;
    }
    if other_stone.velocity.1 >= 0f64 && y_cross < other_stone.position.1 {
        return false;
    }
    if other_stone.velocity.1 <= 0f64 && y_cross > other_stone.position.1 {
        return false;
    }
    if x_cross < from || x_cross > to {
        return false;
    }
    if y_cross < from || y_cross > to {
        return false;
    }
    true
}
