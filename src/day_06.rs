use crate::utils;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

pub fn solve() {
    let raw_data = read_to_string(utils::filename(6)).unwrap();
    let input = utils::chunk_newlines(&raw_data);
    print!("Day 5 part 1: {}\n", part_1(&input));
    print!("Day 2 part 2: {}\n", part_2(&input));
}

fn unique_answers(group_answer: &Vec<&str>) -> u32 {
    group_answer
        .into_iter()
        .map(|answer| answer.chars())
        .flatten()
        .unique()
        .collect::<Vec<char>>()
        .len() as u32
}

fn concencus_answers(group_answer: &Vec<&str>) -> u32 {
    let mut group_iter = group_answer.into_iter();
    let mut concencus: HashSet<char> = group_iter.next().unwrap().chars().collect();

    for answer in group_iter {
        let set: HashSet<char> = answer.chars().collect();
        concencus = concencus.intersection(&set).map(|x| x.clone()).collect();
    }
    return concencus.len() as u32;
}

fn part_1(answers: &Vec<Vec<&str>>) -> u32 {
    answers
        .into_iter()
        .map(|answer| unique_answers(answer))
        .fold(0, |acc, x| acc + x)
}

fn part_2(answers: &Vec<Vec<&str>>) -> u32 {
    answers
        .into_iter()
        .map(|answer| concencus_answers(answer))
        .fold(0, |acc, x| acc + x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_answers() {
        let group_answer = vec!["abc", "abc", "abcd"];
        assert_eq!(unique_answers(&group_answer), 4);
    }
}
