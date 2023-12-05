use std::collections::HashSet;

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

#[derive(Debug, PartialEq, Clone)]
enum PointType {
    Symbol,
    Number,
    Star,
}

#[derive(Debug, Clone)]
struct Point {
    x: u16,
    x_to: u16,
    y: u16,
    point_type: PointType,
}

pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day03.txt");

    let max_y = lines.len() as i16;
    let max_x = lines[0].len() as i16;

    let points = generate_points(&lines, max_x, max_y);
    let res = sum_engine_parts(points, &lines);

    (Solution::from(res.0), Solution::from(res.1))
}

fn sum_engine_parts(points: Vec<Vec<Option<Point>>>, lines: &Vec<String>) -> (u32, u32) {
    let mut seen: HashSet<(u16, u16)> = HashSet::new();
    let mut part_number_sum = 0;
    let mut gear_mult = 0;

    for line in &points {
        for element in line {
            if let Some(el) = element {
                if el.point_type == PointType::Symbol || el.point_type == PointType::Star {
                    let mut point_vals = Vec::new();
                    for offset in [
                        (-1, -1), (-1, 0), (-1, 1),
                        ( 0, -1), /* .. */ ( 0, 1),
                        ( 1, -1), ( 1, 0), ( 1, 1),
                    ] {
                        let val = extract_number(
                            &mut seen, &points, el, lines, offset.0, offset.1
                        );
                        if let Some(num) = val {
                            point_vals.push(num);
                        }
                    }
                    if el.point_type == PointType::Star && point_vals.len() == 2 {
                        gear_mult += point_vals[0] * point_vals[1];
                    }
                    part_number_sum += point_vals.iter().sum::<u32>();
                }
            }
        }
    }
    (part_number_sum, gear_mult)
}

fn extract_number(
    seen: &mut HashSet<(u16, u16)>,
    points: &Vec<Vec<Option<Point>>>,
    el: &Point,
    lines: &Vec<String>,
    y_offset: i16,
    x_offset: i16
) -> Option<u32> {
    let x_co = el.x as i16 + x_offset;
    let y_co = el.y as i16 + y_offset;

    if let Some(point) = &points[x_co as usize][y_co as usize] {
        if point.point_type == PointType::Number && !seen.contains(&(point.x, point.y)) {
            let number_str = lines[y_co as usize].get(point.x as usize..point.x_to as usize);
            if let Some(number_s) = number_str {
                let number = number_s.parse::<u32>();
                seen.insert((point.x, point.y));
                return number.ok();
            }
        }
    }
    None
}

fn generate_points(lines: &Vec<String>, max_x: i16, max_y: i16) -> Vec<Vec<Option<Point>>> {
    let mut points: Vec<Vec<Option<Point>>> = vec![vec![None; max_y as usize]; max_x as usize];

    for (i, el) in lines.iter().enumerate() {
        let mut current_idx: Option<u16> = None;

        for (j, line_char) in el.chars().enumerate() {
            if line_char.is_digit(10) {
                if current_idx.is_none() {
                    current_idx = Some(j as u16);
                }
            } else {
                if let Some(idx) = current_idx {
                    for el in idx..(j as u16) {
                        points[el as usize][i] = Some(Point {
                            x: idx,
                            x_to: j as u16,
                            y: i as u16,
                            point_type: PointType::Number,
                        });
                    }
                    current_idx = None;
                }
                if !line_char.eq_ignore_ascii_case(&'.') {
                    let pt = if line_char == '*' {
                        PointType::Star
                    } else {
                        PointType::Symbol
                    };
                    points[j][i] = Some(Point {
                        x: j as u16,
                        x_to: j as u16,
                        y: i as u16,
                        point_type: pt,
                    });
                }
            }
        }

        // Collect up the remaining numbers
        if let Some(idx) = current_idx {
            for el in idx..max_x as u16 {
                points[el as usize][i] = Some(Point {
                    x: idx,
                    x_to: max_x as u16,
                    y: i as u16,
                    point_type: PointType::Number,
                });
            }
        }
    }
    points
}
