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
    (Solution::from(part1(&lines)), Solution::from(part2(&lines)))
}

fn part1(lines: &[String]) -> i32 {
    let card_rank_pt1: HashMap<char, usize> = card_rank_pt1();
    solve_part(lines, card_rank_pt1, false)
}


fn part2(lines: &[String]) -> i32 {
    let card_rank_pt1: HashMap<char, usize> = card_rank_pt2();
    solve_part(lines, card_rank_pt1, true)
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

fn card_rank_pt1() -> HashMap<char, usize> {
    [
        ('A', 1),
        ('K', 2),
        ('Q', 3),
        ('J', 4),
        ('T', 5),
        ('9', 6),
        ('8', 7),
        ('7', 8),
        ('6', 9),
        ('5', 10),
        ('4', 11),
        ('3', 12),
        ('2', 13),
    ]
    .iter()
    .cloned()
    .collect()
}

fn card_rank_pt2() -> HashMap<char, usize> {
    [
        ('A', 1),
        ('K', 2),
        ('Q', 3),
        ('T', 4),
        ('9', 5),
        ('8', 6),
        ('7', 7),
        ('6', 8),
        ('5', 9),
        ('4', 10),
        ('3', 11),
        ('2', 12),
        ('J', 13),
    ]
    .iter()
    .cloned()
    .collect()
}

fn hand_type(hand: &Hand, j_wildcard: bool) -> u8 {
    let mut card_count = HashMap::new();
    for card in hand.cards.chars() {
        *card_count.entry(card).or_insert(0) += 1;
    }

    if j_wildcard {
        reassign_j(&mut card_count);
    }

    assign_type(&card_count)
}

fn reassign_j(card_count: &mut HashMap<char, i32>) {
    if let Some(&j_value) = card_count.get(&'J') {
        if j_value == 5 {
            return;
        }

        if let Some(target_char) = find_max_by_value_except_j(card_count) {
            if let Some(target) = card_count.get_mut(&target_char) {
                *target += j_value;
            }
            card_count.remove(&'J');
        }
    }
}

fn find_max_by_value_except_j(card_count: &HashMap<char, i32>) -> Option<char> {
    card_count.iter()
        .filter(|&(k, _v)| *k != 'J')
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| *k)
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