use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

struct Hand {
    bid: i32,
    cards: String,
}

pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day07.txt");
    (
        Solution::from(solve_part(&lines, card_rank("AKQJT98765432"), false)),
        Solution::from(solve_part(&lines, card_rank("AKQT98765432J"), true)),
    )
}

fn solve_part(lines: &[String], card_rank_pt1: HashMap<char, usize>, j_wildcard: bool) -> i32 {
    let mut hands: Vec<Hand> = parse_hands(&lines);
    hands.sort_by(|a, b| compare_hands(a, b, &card_rank_pt1, j_wildcard));
    let mut sum = 0;
    for (idx, hand) in hands.iter().enumerate() {
        sum += (idx + 1) as i32 * hand.bid;
    }
    sum
}

fn compare_hands(
    a: &Hand,
    b: &Hand,
    card_rank: &HashMap<char, usize>,
    j_wildcard: bool,
) -> Ordering {
    let a_type = hand_type(a, j_wildcard);
    let b_type = hand_type(b, j_wildcard);
    if a_type != b_type {
        return a_type.cmp(&b_type);
    }
    return a
        .cards
        .chars()
        .zip(b.cards.chars())
        .find_map(|(c1, c2)| {
            let rank_a = card_rank.get(&c1);
            let rank_b = card_rank.get(&c2);

            match (rank_a, rank_b) {
                (Some(&r1), Some(&r2)) if r1 != r2 => Some(r2.cmp(&r1)),
                _ => None,
            }
        })
        .unwrap_or(Ordering::Equal);
}

fn card_rank(order: &str) -> HashMap<char, usize> {
    order.chars().zip(1..=13).collect()
}

fn hand_type(hand: &Hand, j_wildcard: bool) -> u8 {
    let mut card_count = HashMap::new();
    let mut max_key = '&';
    let mut max_non_j = 0;
    for card in hand.cards.chars() {
        let new_count = card_count
            .entry(card)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        if card != 'J' && *new_count > max_non_j {
            max_key = card;
            max_non_j = *new_count
        }
    }

    if j_wildcard {
        reassign_j(&mut card_count, &max_key);
    }

    assign_type(&card_count)
}

fn reassign_j(card_count: &mut HashMap<char, i32>, target_char: &char) {
    if let Some(&j_value) = card_count.get(&'J') {
        if j_value == 5 {
            return;
        }
        if let Some(target) = card_count.get_mut(&target_char) {
            *target += j_value;
        }
        card_count.remove(&'J');
    }
}

fn assign_type(card_count: &HashMap<char, i32>) -> u8 {
    let count_set: HashSet<&i32> = card_count.values().collect();
    match card_count.len() {
        1 => 7,
        2 => {
            if count_set.contains(&4) {
                6
            } else {
                5
            }
        }
        3 => {
            if count_set.contains(&3) {
                4
            } else {
                3
            }
        }
        4 => 2,
        _ => 1,
    }
}

fn parse_hands(lines: &[String]) -> Vec<Hand> {
    return lines
        .iter()
        .filter_map(|l| {
            let parts = l.split_whitespace().collect::<Vec<&str>>();
            if parts.len() == 2 {
                if let Some(bid) = parts[1].parse::<i32>().ok() {
                    return Some(Hand {
                        bid: bid,
                        cards: parts[0].to_owned(),
                    });
                }
            }
            None
        })
        .collect();
}
