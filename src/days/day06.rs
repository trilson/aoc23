use crate::{Solution, SolutionPair, utils::files::lines_from_file};

pub fn solve() -> SolutionPair {    
    let lines = lines_from_file("input/day06.txt");
    let races = parse_races_pt1(&lines);
    let mut mult = 1;

    for race in races {
        let count = eval_maths(race);
        mult *= count;
    }
        
    let race2 = parse_races_pt2(&lines);
    let sol2 = eval_maths(race2); 

    (Solution::from(mult), Solution::from(sol2))
}

fn eval_simple(race: (i64, i64)) -> i64 {   
    let mut count = 0;
    for i in 1..race.0-1 {
        let distance = i * (race.0-i);
        if distance > race.1 {
            count += 1;
        }
    }
    count
}

fn eval_maths(race: (i64, i64)) -> i64 {
    let mid_point = race.0 as f64 / 2.0;

    let a: f64 = -1_f64;
    let b: f64 = race.0 as f64;
    let c: f64 = -race.1 as f64;

    let t_0 = (-b + (b.powf(2.0) - (4.0 * a * c)).sqrt()) / (2.0 * a);
    let count_above = (mid_point as i64 - (t_0 as i64)) * 2;

    if mid_point.fract() != 0.0 {
        count_above
    } else {
        count_above - 1
    }
}

fn parse_races_pt1(lines: &Vec<String>) -> Vec<(i64, i64)> {
    let times = lines[0].split_whitespace().skip(1);
    let distances = lines[1].split_whitespace().skip(1);

    let pairs = times.zip(distances)
        .filter_map(|(t, d)| {
            let time = t.parse::<i64>().ok()?;
            let distance = d.parse::<i64>().ok()?;
            Some((time, distance))
        })
        .collect::<Vec<(i64, i64)>>();

    pairs
}

fn parse_races_pt2(lines: &Vec<String>) -> (i64, i64) {
    let time: Vec<&str> = lines[0].split(':').last().unwrap().split_whitespace().collect();
    let time_i = time.concat().parse::<i64>().unwrap();

    let dist: Vec<&str> = lines[1].split(':').last().unwrap().split_whitespace().collect();
    let dist_i = dist.concat().parse::<i64>().unwrap();

    return (time_i, dist_i);
}