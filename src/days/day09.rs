use crate::{utils::files::lines_from_file, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day09.txt");
    let mut sum_tail = 0;
    let mut sum_head = 0;
    for line in lines {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|n| n.parse::<i32>().ok())
            .collect();

        let head_tail = calculate_next(&nums);
        sum_head += head_tail.0;
        sum_tail += head_tail.1;
    }

    (Solution::from(sum_tail), Solution::from(sum_head))
}

fn calculate_next(nums: &Vec<i32>) -> (i32, i32) {
    let mut current_seq = nums.to_vec();
    let mut tail_el = 0_i32;
    let mut heads: Vec<i32> = Vec::new();
    while !current_seq.iter().all(|n| n == &0) {
        if let Some(&tail) = current_seq.last() {
            tail_el += tail;
        }
        if let Some(&head) = current_seq.first() {
            heads.push(head);
        }
        current_seq = current_seq.windows(2).map(|w| w[1] - w[0]).collect();
    }
    let mut head_el = 0;
    while let Some(head) = heads.pop() {
        head_el = head - head_el;
    }
    (head_el, tail_el)
}
