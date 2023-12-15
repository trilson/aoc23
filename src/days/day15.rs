use linked_hash_map::LinkedHashMap;
use std::str;

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let line = &lines_from_file("input/day15.txt")[0];
    let sol1 = solve_pt1(&line);
    let sol2 = solve_pt2(&line);
    
    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_pt1(input: &String) -> i32 {
    input.split(',').map(hash).sum()
}

fn solve_pt2(line: &String) -> u32 {
    let mut boxes: Vec<LinkedHashMap<String, u32>> = vec![LinkedHashMap::new(); 256];
    for step in line.split(',') {
        match step.chars().last().expect("Help, invalid input!") {
            '-' => {
                let lens_label = &step[0..step.len() - 1];
                let box_match = hash(lens_label);
                if let Some(bx) = boxes.get_mut(box_match as usize) {
                    bx.remove(lens_label);
                }
            }
            digit => {                
                let lens_label = &step[0..step.len() - 2];
                let box_match = hash(lens_label);

                let focal_length = digit.to_digit(10).unwrap();
                if let Some(bx) = boxes.get_mut(box_match as usize) {
                    *bx.entry(lens_label.to_owned()).or_insert(0) = focal_length; 
                }
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(bx_idx, bx)| {
            bx.iter()
                .enumerate()
                .map(|(idx, v)| (bx_idx as u32 + 1) * (idx as u32 + 1) * v.1)
                .sum::<u32>()
        })
        .sum()
}

fn hash(input: &str) -> i32 {
    input.as_bytes().iter().fold(0_i32, |acc, &ch| (acc + ch as i32) * 17 % 256)
}