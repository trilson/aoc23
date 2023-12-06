use std::cmp::{max, min};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

#[derive(Clone)]
struct Range {
    destination_start: i64,
    source_start: i64,
    range_length: i64,
}

#[derive(Clone)]
struct Block {
    ranges: Vec<Range>,
}

pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day05.txt");
    let first_line = lines[0].to_string();

    let blocks = generate_blocks(lines);
    let seeds_p1: Vec<i64> = get_seeds(&first_line);

    let sol1: i64 = shortest_distance(&seeds_p1, &blocks);

    let seeds_p2: Vec<(i64, i64)> = get_seed_ranges(&seeds_p1);
    let sol2: i64 = solve_part2(&blocks, &seeds_p2);

    (Solution::from(sol1), Solution::from(sol2))
}

fn get_seed_ranges(seeds: &Vec<i64>) -> Vec<(i64, i64)> {
    let mut ranges = Vec::new();
    for pair in seeds.windows(2).step_by(2) {
        if let [a, b] = pair {
            ranges.push((a.to_owned(), a + b));
        }
    }
    ranges
}

fn get_seeds(line: &String) -> Vec<i64> {
    let first_line = line.split(':').last();

    first_line
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect()
}

fn solve_part2(blocks: &Vec<Block>, seed_pairs: &Vec<(i64, i64)>) -> i64 {
    let mut seeds = seed_pairs.clone(); // Clone the initial seed pairs
    let mut new_seeds;
    let mut ranges;

    for block in blocks {
        new_seeds = Vec::new(); // Create a new vector for the next iteration
        ranges = block.ranges.clone();
        while let Some((s, e)) = seeds.pop() {
            let mut no_seeds_pushed = true;
            for range in &ranges {
                let os = max(s, range.source_start);
                let oe = min(e, range.source_start + range.range_length);

                if os < oe {
                    new_seeds.push((os - range.source_start + range.destination_start, oe - range.source_start + range.destination_start));
                    if os > s {
                        seeds.push((s, os));
                    }
                    if e > oe {
                        seeds.push((oe, e));
                    }
                    no_seeds_pushed = false;
                    break;
                }
            }
            if no_seeds_pushed {                
                new_seeds.push((s, e));
            }
        }

        seeds = new_seeds; // Update seeds for the next block
    }
    println!("{:?}", seeds);
    let min = seeds.iter().map(|(first, _)| first).min();
    min.unwrap().to_owned()
}

fn generate_blocks(lines: Vec<String>) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();

    for line in lines.iter().skip(1) {
        if line.contains(":") {
            // Start a new block when the line contains ":"
            blocks.push(Block { ranges: Vec::new() });
        } else if !line.is_empty() && !blocks.is_empty() {
            // Split the line into numbers and add a range to the last block
            let nums: Vec<&str> = line.split(' ').collect();
            if let Some(last_block) = blocks.last_mut() {
                last_block.ranges.push(Range {
                    destination_start: nums[0].parse::<i64>().unwrap(),
                    source_start: nums[1].parse::<i64>().unwrap(),
                    range_length: nums[2].parse::<i64>().unwrap(),
                });
            }
        }
    }
    blocks
}

fn shortest_distance(seeds: &Vec<i64>, blocks: &Vec<Block>) -> i64 {
    let mut shortest_distance: i64 = i64::MAX;
    for seed in seeds {
        let dist = distance_for_seed(&blocks, seed.to_owned());
        shortest_distance = min(shortest_distance, dist);
    }
    shortest_distance
}

fn distance_for_seed(blocks: &Vec<Block>, seed: i64) -> i64 {
    let mut id = seed;
    for block in blocks {
        let mut new_id = id;
        for r in &block.ranges {
            if id >= r.source_start && id < r.source_start + r.range_length {
                new_id = (id - r.source_start) + r.destination_start;
                break;
            }
        }
        id = new_id;
    }
    id
}