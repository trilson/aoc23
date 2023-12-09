use num_integer::gcd;
use std::collections::HashMap;

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day08.txt");
    let mut step_map = HashMap::new();

    for line in lines.iter().skip(2) {
        if let (Some(key), Some(l), Some(r)) = (line.get(0..3), line.get(7..10), line.get(12..15)) {
            step_map.insert(key, (l, r));
        }
    }

    // let sol1 = solve_part1(&step_map, &lines[0]);
    let sol1 = calculate(&step_map, &lines[0], |a| a == "AAA", |b| b == "ZZZ");
    let sol2 = calculate(
        &step_map,
        &lines[0],
        |a| a.ends_with("A"),
        |b| b.ends_with("Z"),
    );

    (Solution::from(sol1), Solution::from(sol2))
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

fn lcm_of_list(numbers: &[i64]) -> i64 {
    numbers.iter().cloned().reduce(lcm).unwrap_or(1)
}

fn calculate<F, G>(
    step_map: &HashMap<&str, (&str, &str)>,
    instructions: &String,
    start_predicate: F,
    end_predicate: G,
) -> i64
where
    F: Fn(&str) -> bool,
    G: Fn(&str) -> bool,
{
    let loc_keys: Vec<&str> = step_map
        .keys()
        .filter(|p| start_predicate(p))
        .cloned()
        .collect();

    let steps_max: Vec<i64> = loc_keys
        .iter()
        .map(|lk| num_steps(step_map, instructions, lk, |a| end_predicate(a)) as i64)
        .collect();

    lcm_of_list(&steps_max)
}

fn num_steps<F>(
    step_map: &HashMap<&str, (&str, &str)>,
    instructions: &String,
    start_loc: &str,
    end_predicate: F,
) -> i32
where
    F: Fn(&str) -> bool,
{
    let mut iter = instructions.chars().cycle();
    let mut loc = start_loc;
    let mut steps = 0;
    while !end_predicate(loc) {
        if let Some(step) = step_map.get(loc) {
            steps += 1;
            if let Some(instr) = iter.next() {
                if instr == 'L' {
                    loc = step.0;
                } else {
                    loc = step.1;
                }
            }
        }
    }
    steps
}
