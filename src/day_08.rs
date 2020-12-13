use crate::utils;
use std::collections::HashSet;
use std::fs::read_to_string;

type Instructions<'a> = Vec<(&'a str, i32)>;

pub fn solve() {
    let raw_data = read_to_string(utils::filename(8)).unwrap();
    let instructions = make_instruction_set(&raw_data);

    print!("Day 8 part 1: {}\n", part_1(&instructions).0);
    print!("Day 8 part 1: {}\n", part_2(&instructions));
}

fn make_instruction_set(raw_instructions: &str) -> Vec<(&str, i32)> {
    raw_instructions
        .lines()
        .map(|x| x.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut split_iter = line.split(" ");
            (
                split_iter.next().unwrap(),
                split_iter.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect()
}

fn part_1(instructions: &Instructions) -> (i32, bool) {
    // return accumulator value, and if we hit a repeated statement or not
    let mut observed_lines: HashSet<usize> = HashSet::new();
    let mut acc = 0;
    let mut cur_line: usize = 0;
    let repeated = loop {
        if observed_lines.contains(&cur_line) {
            break false;
        }
        observed_lines.insert(cur_line);
        match instructions[cur_line] {
            ("acc", val) => {
                cur_line += 1;
                acc += val;
            }
            ("jmp", val) => {
                cur_line = (cur_line as i32 + val) as usize;
            }
            ("nop", _) => {
                cur_line += 1;
            }
            (ukn, _) => {
                panic!("Unknown instr {}", ukn)
            }
        }
        if cur_line >= instructions.len() {
            break true;
        }
    };
    return (acc, repeated);
}

fn part_2(instructions: &Instructions) -> i32 {
    let mut acc = 0;
    for (line_number, instr) in instructions.into_iter().enumerate() {
        match instr {
            ("acc", _) => continue,
            (cmd, val) => {
                let mut try_instructions = instructions.clone();
                let flipped = match cmd {
                    &"jmp" => "nop",
                    &"nop" => "jmp",
                    _ => "ukn",
                };
                try_instructions[line_number] = (flipped, *val);
                let (_acc, result) = part_1(&try_instructions);
                if result {
                    acc = _acc;
                    break;
                }
            }
        }
    }
    return acc;
}

#[cfg(test)]
mod tests {
    use super::*;
}
