use crate::{utils::files::lines_from_file, Solution, SolutionPair};
use regex::Regex;
use std::cmp::max;
use std::str::FromStr;

#[derive(Debug)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    bags: Vec<Bag>,
}

pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day02.txt");
    let sol1 = solve_part1(&lines);
    let sol2 = solve_part2(&lines);

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_part1(lines: &Vec<String>) -> u32 {
    let games = extract_games(lines);
    games
        .iter()
        .filter_map(|game| {
            if game
                .bags
                .iter()
                .all(|bag| bag.red <= 12 && bag.green <= 13 && bag.blue <= 14)
            {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

fn solve_part2(lines: &Vec<String>) -> u32 {
    let games = extract_games(lines);
    games
        .iter()
        .map(|game| {
            let mut r = 0;
            let mut b = 0;
            let mut g = 0;
            for bag in &game.bags {
                r = max(r, bag.red);
                b = max(b, bag.blue);
                g = max(g, bag.green);
            }
            r * b * g
        })
        .sum()
}

fn extract_games(lines: &Vec<String>) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    let re = Regex::new(r"(?P<Red>\d+)\s*red|(?P<Green>\d+)\s*green|(?P<Blue>\d+)\s*blue").unwrap();

    for (i, line) in lines.iter().enumerate() {
        let mut bags: Vec<Bag> = Vec::new();
        let game_split: Vec<&str> = line.split(':').collect();
        let bags_split = game_split[1].split(';');

        for bag_line in bags_split {
            let mut bag = Bag {
                red: 0,
                green: 0,
                blue: 0,
            };
            for cap in re.captures_iter(bag_line) {
                if let Some(red) = cap.name("Red") {
                    bag.red = u32::from_str(red.as_str()).unwrap_or(0);
                }
                if let Some(green) = cap.name("Green") {
                    bag.green = u32::from_str(green.as_str()).unwrap_or(0);
                }
                if let Some(blue) = cap.name("Blue") {
                    bag.blue = u32::from_str(blue.as_str()).unwrap_or(0);
                }
            }
            bags.push(bag);
        }
        games.push(Game {
            id: (i + 1) as u32,
            bags,
        })
    }
    games
}
