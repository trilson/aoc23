use crate::{Solution, SolutionPair};

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day01.txt");

    let sol1: u32 = solve_part(false, &lines);
    let sol2: u32 = solve_part(true, &lines);

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_part(with_text: bool, lines: &Vec<String>) -> u32 {
    let nums = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut sol = 0;
    for line in lines {
        let mut i = 0;
        let mut j = line.len() - 1;

        while i <= j {
            match get_digit_from_str(&line[i..], &nums, with_text) {
                Some(value) => {
                    sol += 10 * value;
                    break;
                }
                None => i += 1,
            }
        }
        while j >= i {
            match get_digit_from_str(&line[j..], &nums, with_text) {
                Some(value) => {
                    sol += value;
                    break;
                }
                None => j -= 1,
            }
        }
    }
    sol
}

fn get_digit_from_str(input_str: &str, nums: &Vec<&str>, with_text: bool) -> Option<u32> {
    let i_char = input_str.chars().nth(0).unwrap();
    if i_char.is_digit(10) {
        return i_char.to_digit(10);
    }
    if !with_text {
        return None;
    }
    for (i_i, el) in nums.iter().enumerate() {
        if input_str.starts_with(el) {
            return Some(i_i as u32 + 1);
        }
    }
    None
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
