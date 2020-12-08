use crate::utils;
use std::fs::read_to_string;

enum Terrain {
  Tree,
  Snow
}

struct Map {
  map: Vec<Vec<Terrain>>,
}

impl Map {
  fn new(raw_data: Vec<&str>) -> Map{
    let mut _map: Vec<Vec<Terrain>> = Vec::new();
    for line in raw_data.into_iter(){
      _map.push(
        line.split("")
        .filter(|x| !x.is_empty())
        .map(|val| {
            match val {
              "#" => Terrain::Tree,
              _ => Terrain::Snow,
            }
          }
        )
      .collect()
      );
    }
    Map{
      map: _map
    }
  }

  fn is_tree(&self, y: usize, x: usize) -> bool{
    match self.map[y][x] {
      Terrain::Tree => true,
      _ => false,
    }
  }

  fn height(&self) -> usize{
      return self.map.len();
  }

  fn width(&self) -> usize{
      if self.height() == 0{
        return 0
      }
      return self.map[0].len();
  }
}


pub fn solve() {
  let raw_data = read_to_string(utils::filename(3)).unwrap();
  let raw_data = utils::clean_lines(&raw_data);
  let map = Map::new(raw_data);
  print!("Day 3 part 1: {}\n", part_1(&map, 3, 1));
  print!("Day 3 part 2: {}\n", part_2(&map));
}

fn part_1(map: &Map, dx: usize, dy: usize) -> usize {
    let mut x:usize = 0;
    let mut y:usize = 0;
    let mut trees: usize = 0;
    //map.print_out();
    while y < map.height() {
      if map.is_tree(y, x % map.width()){
        trees += 1;
      }
      x += dx;
      y += dy;
    }
    return trees;
}

fn part_2(map: &Map) -> usize {
  return part_1(map, 1, 1) * part_1(map, 3, 1) * part_1(map, 5, 1) * part_1(map, 7, 1) * part_1(map, 1, 2);
}

