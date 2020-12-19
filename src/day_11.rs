use crate::utils;
use std::fmt;
use std::fs::read_to_string;

#[derive(Clone, Copy, PartialEq)]
enum Terrain {
    EmptySeat,
    OccupiedSeat,
    Floor,
}

impl fmt::Debug for Terrain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Terrain::EmptySeat => write!(f, "L"),
            Terrain::OccupiedSeat => write!(f, "#"),
            Terrain::Floor => write!(f, "."),
        }
    }
}

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (0, -1),
    (0, 1),
    (1, 0),
    (1, 1),
    (-1, 1),
    (1, -1),
];

#[derive(Clone, Debug)]
struct Map {
    map: Vec<Vec<Terrain>>,
}

impl Map {
    fn new(raw_data: Vec<&str>) -> Map {
        let mut map: Vec<Vec<Terrain>> = Vec::new();
        for line in raw_data.into_iter() {
            map.push(
                line.split("")
                    .filter(|x| !x.is_empty())
                    .map(|val| match val {
                        "L" => Terrain::EmptySeat,
                        "#" => Terrain::OccupiedSeat,
                        _ => Terrain::Floor,
                    })
                    .collect(),
            );
        }
        return Map { map: map };
    }

    fn height(&self) -> usize {
        return self.map.len();
    }

    fn width(&self) -> usize {
        if self.height() == 0 {
            return 0;
        }
        return self.map[0].len();
    }

    fn get_terrain(&self, x: i32, y: i32) -> Terrain {
        match (x, y) {
            (_, _) if x >= self.width() as i32 || x < 0 || y >= self.height() as i32 || y < 0 => {
                Terrain::Floor
            }
            (x, y) => self.map[y as usize][x as usize].clone(),
        }
    }

    fn adjacent_terrain(&self, x: usize, y: usize) -> [Terrain; 8] {
        let mut result = [Terrain::Floor; 8];
        for (pos, dir) in DIRECTIONS.iter().enumerate() {
            result[pos] = self.get_terrain(x as i32 + dir.0, y as i32 + dir.1)
        }
        return result;
    }

    fn simulate(&mut self) -> (bool, Map) {
        let mut changed = false;
        let mut new_map: Vec<Vec<Terrain>> = Vec::new();
        for (y, row) in self.map.iter().enumerate() {
            new_map.push(
                row.into_iter()
                    .enumerate()
                    .map(|(x, terrain)| {
                        let adj = self.adjacent_terrain(x, y);
                        match terrain {
                            Terrain::EmptySeat
                                if adj.iter().all(|p| *p != Terrain::OccupiedSeat) =>
                            {
                                changed = true;
                                Terrain::OccupiedSeat
                            }
                            Terrain::OccupiedSeat
                                if adj.iter().filter(|p| **p == Terrain::OccupiedSeat).count()
                                    >= 4 =>
                            {
                                changed = true;
                                Terrain::EmptySeat
                            }
                            no_change => no_change.clone(),
                        }
                    })
                    .collect(),
            );
        }
        return (changed, Map { map: new_map });
    }
}

pub fn solve() {
    let raw_data = read_to_string(utils::filename(11)).unwrap();
    let raw_data = utils::clean_lines(&raw_data);
    let map = Map::new(raw_data);
    print!("Day 11 part 1: {}\n", part_1(map.clone()));
    //print!("Day 11 part 2: {}\n", part_2(map.clone()));
}

fn part_1(map: Map) -> u32 {
    let mut cur_map: Map = map;
    loop {
        let result = cur_map.simulate();
        cur_map = result.1;
        if !result.0 {
            break;
        }
    }
    return cur_map
        .map
        .iter()
        .flatten()
        .filter(|p| **p == Terrain::OccupiedSeat)
        .count() as u32;
}

fn part_2(map: Map) -> u32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let raw_data = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let map = Map::new(utils::clean_lines(raw_data));
        assert_eq!(part_1(map.clone()), 37);
    }
}
