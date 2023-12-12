use std::iter::repeat;

use itertools::Itertools;
use crate::{utils::files::lines_from_file, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day12.txt");

    let mut poss = 0;
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let springs = parts[0]; // repeat(parts[0]).take(5).collect::<Vec<&str>>().join("?");
        let damaged = parts[1]; // repeat(parts[1]).take(5).collect::<Vec<&str>>().join(",");

        let answers: Vec<i32> = springs
            .chars()
            .enumerate()
            .filter_map(|(idx, s)| {
                if s == '?' {
                    return Some(idx as i32);
                } else {
                    return None;
                }
            }).collect();

        let cont: Vec<i32> = damaged
            .split(",")
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let questions: i32 =
            cont.iter().sum::<i32>() - line.chars().filter(|&c| c == '#').count() as i32;

        for comb in answers
            .into_iter()
            .combinations(questions as usize) {
                let mut contiguous = 0;
                let mut value_check = 0_i32;
                for (idx, ch) in parts[0].chars().enumerate() {
                    if ch == '#' || comb.contains(&(idx as i32)) {
                        contiguous += 1;
                    } else if contiguous != 0 {
                        let exp = cont.get(value_check as usize).unwrap_or(&-1);
                        if exp != &contiguous {
                            value_check = -1;
                            break;
                        } else {
                            value_check += 1;
                            contiguous = 0;
                        }
                    }
                }
                if value_check == -1 {
                    continue;
                }
                let exp = cont.get(value_check as usize).unwrap_or(&-1);
                if contiguous == 0 || exp == &contiguous {
                    poss += 1;
                }
            }
        // count number of #, subtract from expected number (1+1+3) = C
        // collect co-ordinates of ? and apply combinations (C |comb| ?)
        // On each combination, check 1,1,3 progression
    }
    (Solution::from(poss), Solution::from(0))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = super::solve();
        println!("{result:?}");
    }
}