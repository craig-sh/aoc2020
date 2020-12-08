use crate::utils;
use std::fs::read_to_string;

pub fn solve() {
  let raw_data = read_to_string(utils::filename(1)).unwrap();
  let input = utils::clean_lines(&raw_data).into_iter()
      .map(|l| l.parse().unwrap())
      .collect();
  print!("Day 1 part 1: {}\n", part_1(&input));
  print!("Day 1 part 2: {}\n", part_2(&input));
}


fn part_1(v: &Vec<u32>) -> u32{
    for (i, num1) in v.iter().enumerate(){
        for num2 in v[i..].iter(){
            if num1 + num2 == 2020 {
                return num1 * num2;
            }
        }
    }
    return v[1]
}


fn part_2(v: &Vec<u32>) -> u32{
    for (i, num1) in v.iter().enumerate(){
        for (j, num2) in v[i..].iter().enumerate(){
            for num3 in v[j..].iter(){
                if num1 + num2 + num3 == 2020 {
                    return num1 * num2 * num3;
                }
            }
        }
    }
    return v[1]
}
