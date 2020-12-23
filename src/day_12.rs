use crate::utils;
use std::fs::read_to_string;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

const EAST: Point = Point { x: 1, y: 0 };
const WEST: Point = Point { x: -1, y: 0 };
const NORTH: Point = Point { x: 0, y: 1 };
const SOUTH: Point = Point { x: 0, y: -1 };

impl Point {
    fn abs(&self) -> u64 {
        (self.x.abs() + self.y.abs()) as u64
    }
    fn mul(&self, scalar: i64) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
    fn add(&self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    fn sub(&self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    fn rotate(&self, origin: Point, angle: f64) -> Point {
        let s = angle.to_radians().sin();
        let c = angle.to_radians().cos();
        let mut final_point = self.sub(origin);
        let dx = final_point.x as f64 * c - final_point.y as f64 * s;
        let dy = final_point.x as f64 * s + final_point.y as f64 * c;
        final_point.x = dx.round() as i64 + origin.x;
        final_point.y = dy.round() as i64 + origin.y;
        return final_point;
    }
}

fn dir_from_char(val: char) -> Point {
    match val {
        'E' => EAST,
        'S' => SOUTH,
        'W' => WEST,
        'N' => NORTH,
        _ => panic!(),
    }
}

fn dir_from_degree(val: u64) -> Point {
    match val {
        0 => EAST,
        90 => SOUTH,
        180 => WEST,
        270 => NORTH,
        _ => panic!("Panic {}", val),
    }
}
fn degree_from_dir(val: Point) -> u64 {
    match val {
        EAST => 0,
        SOUTH => 90,
        WEST => 180,
        NORTH => 270,
        _ => panic!(),
    }
}

pub fn solve() {
    let input: Vec<(char, u64)> = utils::clean_lines(&read_to_string(utils::filename(12)).unwrap())
        .into_iter()
        .map(|n| (n.chars().next().unwrap(), n[1..].parse::<u64>().unwrap()))
        .collect();
    print!("Day 12 part 1: {}\n", part_1(&input));
    print!("Day 12 part 2: {}\n", part_2(&input));
}

fn part_1(input: &Vec<(char, u64)>) -> u64 {
    let mut pos = Point { x: 0, y: 0 };
    let mut direction = EAST;
    for (action, val) in input.iter() {
        match action {
            'R' => direction = dir_from_degree((degree_from_dir(direction) + val) % 360),
            'L' => {
                direction = dir_from_degree(
                    ((degree_from_dir(direction) as i64 - *val as i64) + 360) as u64 % 360,
                )
            }
            'F' => pos = pos.add(direction.mul(*val as i64)),
            move_dir => pos = pos.add(dir_from_char(*move_dir).mul(*val as i64)),
        };
    }
    return pos.abs();
}

fn part_2(input: &Vec<(char, u64)>) -> u64 {
    let mut ship_pos = Point { x: 0, y: 0 };
    let mut waypoint_pos = Point { x: 10, y: 1 };
    for (action, val) in input.iter() {
        match action {
            'L' => waypoint_pos = waypoint_pos.rotate(Point { x: 0, y: 0 }, *val as f64),
            'R' => waypoint_pos = waypoint_pos.rotate(Point { x: 0, y: 0 }, (360 - *val) as f64),
            'F' => ship_pos = ship_pos.add(waypoint_pos.mul(*val as i64)),
            move_dir => waypoint_pos = waypoint_pos.add(dir_from_char(*move_dir).mul(*val as i64)),
        };
    }
    return ship_pos.abs();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec![('F', 10), ('N', 3), ('F', 7), ('R', 90), ('F', 11)];
        assert_eq!(part_1(&input), 25)
    }
    #[test]
    fn test_part_2() {
        let input = vec![('F', 10), ('N', 3), ('F', 7), ('R', 90), ('F', 11)];
        assert_eq!(part_2(&input), 286)
    }
}
