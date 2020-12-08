use crate::utils;
use std::fs::read_to_string;

pub fn solve() {
  let raw_data = read_to_string(utils::filename(2)).unwrap();
  let input = utils::clean_lines(&raw_data);
  print!("Day 2 part 1: {}\n", part_1(&input));
  print!("Day 2 part 2: {}\n", part_2(&input));
}

fn part_1(input: &Vec<&str>) -> u32 {
    let mut correct: u32 = 0;
    for line in input{
        let v: Vec<&str> = line.split(|c| c == '-' || c == ' ' || c == ':').collect();
        let target_min_count = v[0].parse::<usize>().unwrap();
        let target_max_count = v[1].parse::<usize>().unwrap();
        let target = v[2];
        let actual_count = v[4].matches(target).count();
        if actual_count >= target_min_count && actual_count <= target_max_count{
            correct += 1;
        }

    }
    return correct;

}

fn part_2(input: &Vec<&str>) -> u32 {
    let mut correct: u32 = 0;
    for line in input {
        let v: Vec<&str> = line.split(|c| c == '-' || c == ' ' || c == ':').collect();
        let first_pos = v[0].parse::<usize>().unwrap();
        let second_pos = v[1].parse::<usize>().unwrap();
        let target = v[2];
        let in_pos_1 = v[4].as_bytes()[first_pos - 1] == target.as_bytes()[0];
        let in_pos_2 = v[4].as_bytes()[second_pos - 1] == target.as_bytes()[0];
        if in_pos_2 ^ in_pos_1{
            correct += 1;
        }

    }
    return correct;
}

