use std::{cmp::min, collections::HashMap};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

struct Range {
    destination_start: i64,
    source_start: i64,
    range_length: i64,
}

struct GardenMap {
    map_from: String,
    map_to: String,
    ranges: Vec<Range>,
}

pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day05.txt");
    let first_line = lines[0].to_string();

    let garden_map = generate_garden_map(lines);

    let seeds_p1: Vec<i64> = generate_seeds_part1(&first_line);
    let sol1: i64 = shortest_distance(seeds_p1, &garden_map);
    let sol2: i64 = solve_part2(&garden_map, &first_line);

    (Solution::from(sol1), Solution::from(sol2))
}

fn generate_seeds_part1(line: &String) -> Vec<i64> {
    let first_line = line.split(':').last();

    first_line
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect()
}

fn solve_part2(garden_map: &HashMap<String, GardenMap>, line: &String) -> i64 {
    let first_line = line.split(':').last();
    let seed_vector = Vec::from_iter(first_line.unwrap().split_whitespace());
    let mut shortest_distance: i64 = i64::MAX;
    for pair in seed_vector.windows(2).step_by(2) {
        if let [a, b] = pair {
            let orig_seed = a.parse::<i64>().unwrap();
            let more_seeds = b.parse::<i64>().unwrap();

            for s in orig_seed..orig_seed + more_seeds {
                shortest_distance = min(shortest_distance, distance_for_seed(garden_map, s));
            }
        }
    }
    shortest_distance
}

fn generate_garden_map(lines: Vec<String>) -> HashMap<String, GardenMap> {
    let mut garden_map: HashMap<String, GardenMap> = HashMap::new();
    let mut current_gm: Option<String> = None;

    for el in 1..lines.len() - 1 {
        if lines[el].contains(":") {
            let current_map = String::from(lines[el].split(' ').next().unwrap());
            let split_map: Vec<&str> = current_map.split('-').collect();
            let source = split_map[0];

            let gm = GardenMap {
                map_from: source.to_owned(),
                map_to: split_map[2].to_owned(),
                ranges: Vec::new(),
            };

            garden_map.insert(source.to_owned(), gm);
            current_gm = Some(source.to_owned());
        } else if !lines[el].is_empty() {
            let nums: Vec<&str> = lines[el].split(' ').collect();
            if let Some(gm) = &mut current_gm {
                let gm = garden_map.get_mut(gm).unwrap();
                gm.ranges.push(Range {
                    destination_start: nums[0].parse::<i64>().unwrap(),
                    source_start: nums[1].parse::<i64>().unwrap(),
                    range_length: nums[2].parse::<i64>().unwrap(),
                })
            }
        }
    }

    for item in garden_map.values_mut() {
        item.ranges.sort_by_key(|r| r.source_start);
    }
    garden_map
}

fn shortest_distance(seeds: Vec<i64>, garden_map: &HashMap<String, GardenMap>) -> i64 {
    let mut shortest_distance: i64 = i64::MAX;
    for seed in seeds {
        let dist = distance_for_seed(garden_map, seed);
        shortest_distance = min(shortest_distance, dist);
    }
    shortest_distance
}

fn distance_for_seed(garden_map: &HashMap<String, GardenMap>, seed: i64) -> i64 {
    let mut gm = garden_map.get("seed");
    let mut id = seed;
    while gm.is_some() {
        let cur_map = gm.unwrap();
        let mut new_id = id;
        for r in &cur_map.ranges {
            if id >= r.source_start && id < r.source_start + r.range_length {
                new_id = (id - r.source_start) + r.destination_start;
                break;
            }
            if id < r.source_start {
                break;
            }
        }
        id = new_id;
        gm = garden_map.get(&cur_map.map_to.to_string());
    }
    id
}

#[test]
fn it_works() {
    println!("{:?}", solve());
}
