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
        let sol = calculate_recursive(&nums);
        sum_head += sol.0;
        sum_tail += sol.1;
    }

    (Solution::from(sum_tail), Solution::from(sum_head))
}

fn calculate_recursive(nums: &Vec<i32>) -> (i32, i32) {
    if nums.len() == 0 {
        return (0, 0);
    }

    let p1 = calculate_recursive(&nums.windows(2).map(|w| w[1] - w[0]).collect());
    return (
        nums.first().unwrap_or(&0) - p1.0,
        nums.last().unwrap_or(&0) + p1.1,
    );
}
