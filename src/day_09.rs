use crate::utils;
use std::fs::read_to_string;

pub fn solve() {
    let input = utils::clean_lines(&read_to_string(utils::filename(9)).unwrap())
        .into_iter()
        .map(|n| n.parse().unwrap())
        .collect();

    print!("Day 9 part 1: {}\n", part_1(&input, 25));
    print!("Day 9 part 2: {}\n", part_2(&input, 25));
}

fn is_valid(target: u64, numbers: &Vec<u64>) -> bool {
    for (i, first_num) in numbers.into_iter().enumerate() {
        for second_num in numbers[i..].into_iter() {
            if first_num + second_num == target {
                return true;
            }
        }
    }
    return false;
}

fn calc_continguous(target: u64, numbers: &Vec<u64>) -> Vec<&u64> {
    for (i, first_num) in numbers.into_iter().enumerate() {
        let mut acc = first_num.clone();
        let mut answers = vec![first_num];
        for num in numbers[i + 1..].into_iter() {
            acc += num;
            answers.push(num);
            if acc == target {
                return answers;
            }
        }
    }
    return vec![];
}

fn part_1(numbers: &Vec<u64>, preamble: u64) -> u64 {
    *numbers[preamble as usize..]
        .into_iter()
        .skip_while(|num| is_valid(**num, numbers))
        .next()
        .unwrap()
}

fn part_2(numbers: &Vec<u64>, preamble: u64) -> u64 {
    let target = part_1(numbers, preamble);
    let answers = calc_continguous(target, numbers);
    return *answers.iter().min().unwrap() + *answers.iter().max().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
}
