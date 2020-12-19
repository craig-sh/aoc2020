use crate::utils;
use std::fs::read_to_string;

pub fn solve() {
    let mut input: Vec<u32> = utils::clean_lines(&read_to_string(utils::filename(10)).unwrap())
        .into_iter()
        .map(|n| n.parse().unwrap())
        .collect();
    input.sort();
    let highest_adapter = input.iter().max().unwrap() + 3;
    input.push(highest_adapter);
    print!("Day 10 part 1: {}\n", part_1(&input));
    print!("Day 10 part 2: {}\n", part_2(&input));
}

fn part_1(input: &Vec<u32>) -> u32 {
    let mut index = 0;
    let mut cur_joltage = 0;
    let len = input.len();
    let mut jump_counts: [u32; 3] = [0, 0, 0];

    while index < len {
        let jolt_diff = input[index] - cur_joltage;
        jump_counts[(jolt_diff - 1) as usize] += 1;
        index += 1;
        cur_joltage += jolt_diff;
    }
    return jump_counts[0] * jump_counts[2];
}

fn part_2(input: &Vec<u32>) -> u64 {
    let highest_adapter = *input.iter().max().unwrap();
    let mut solved = vec![0; (highest_adapter + 1) as usize];
    solved[0] = 1;
    for adapter in input.iter() {
        let mut total: u64 = 0;
        let mut position: i32 = match *adapter {
            0 => 0,
            x => x as i32,
        };
        while position >= 0 && (*adapter as i32 - position) <= 3 {
            total += solved[position as usize];
            position -= 1;
        }
        solved[*adapter as usize] = total;
    }
    return solved[highest_adapter as usize];
}

#[cfg(test)]
mod tests {
    use super::*;
}
