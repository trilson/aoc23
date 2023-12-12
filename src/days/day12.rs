use std::{collections::HashMap, iter::repeat};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day12.txt");
    let sol1 = solve_recursive(&lines, 1);
    let sol2 = solve_recursive(&lines, 5);

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_recursive(lines: &Vec<String>, multiple: usize) -> i64 {
    let mut sol = 0;
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let springs = repeat(parts[0])
            .take(multiple)
            .collect::<Vec<&str>>()
            .join("?");
        let damaged = repeat(parts[1])
            .take(multiple)
            .collect::<Vec<&str>>()
            .join(",");

        let contiguous_parts: Vec<i32> = damaged
            .split(",")
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let mut memo: HashMap<(&[u8], usize), i64> = HashMap::new();
        sol += count(springs.as_bytes(), &contiguous_parts, &mut memo);
    }
    sol
}

fn count<'a>(line: &'a [u8], parts: &[i32], memo: &mut HashMap<(&'a [u8], usize), i64>) -> i64 {
    if line.is_empty() {
        return if parts.is_empty() { 1 } else { 0 };
    }

    if parts.is_empty() {
        return if line.contains(&b'#') { 0 } else { 1 };
    }

    let memo_key = (line, parts.len());
    if let Some(&cached_result) = memo.get(&memo_key) {
        return cached_result;
    }

    let mut result = 0;
    let first_char = line[0];

    if first_char == b'.' || first_char == b'?' {
        result += count(&line[1..], parts, memo);
    }

    if first_char == b'#' || first_char == b'?' {
        if let Some(&part_size) = parts.first() {
            if part_size as usize <= line.len()
                && !line[..part_size as usize].contains(&b'.')
                && (part_size as usize == line.len() || line[part_size as usize] != b'#')
            {
                let remainder: &[u8] = if part_size < line.len() as i32 {
                    &line[(part_size as usize) + 1..]
                } else {
                    &[]
                };
                result += count(remainder, &parts[1..], memo);
            }
        }
    }

    memo.insert(memo_key, result);
    result
}
