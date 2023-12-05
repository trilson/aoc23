use std::collections::{HashMap, HashSet};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day04.txt");
    let intersections = intersections(&lines);
    let sol1 = solve_part1(&intersections);
    let sol2 = solve_part2(&intersections);

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_part1(intersections: &Vec<HashSet<&str>>) -> i32 {
    intersections
        .iter()
        .filter_map(|res| {
            if res.len() == 0 {
                return None;
            }
            return Some(2_i32.pow(res.len() as u32 - 1));
        })
        .sum()
}

fn solve_part2(intersections: &Vec<HashSet<&str>>) -> i32 {
    let mut scratch_cards: HashMap<usize, i32> = HashMap::new();
    let mut count_cards = 0;
    for (idx, el) in intersections.iter().enumerate() {
        let num_cards = add_card(&mut scratch_cards, idx + 1, 1);
        for i in idx + 2..idx + 2 + el.len() {
            add_card(&mut scratch_cards, i, num_cards);
        }
        count_cards += num_cards;
    }
    count_cards
}

fn add_card(scratch_cards: &mut HashMap<usize, i32>, game: usize, mult: i32) -> i32 {
    if !scratch_cards.contains_key(&game) {
        scratch_cards.insert(game, 0);
    }
    match scratch_cards.get_mut(&game) {
        Some(value) => {
            *value += 1 * mult;
            *value
        }
        None => 0,
    }
}

fn intersections(lines: &[String]) -> Vec<HashSet<&str>> {
    lines
        .iter()
        .map(|l| {
            let rhs = l.split(':').last();
            let split: Vec<&str> = rhs.unwrap().split('|').collect();

            let ticket: HashSet<&str> = HashSet::from_iter(split[0].split_whitespace());
            let draw: HashSet<&str> = HashSet::from_iter(split[1].split_whitespace());
            HashSet::from_iter(ticket.intersection(&draw).copied())
        })
        .collect()
}
