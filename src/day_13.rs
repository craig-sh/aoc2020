use crate::utils;
use std::fs::read_to_string;

pub fn solve() {
    let raw_data = &read_to_string(utils::filename(13)).unwrap();
    let mut input_iter = utils::clean_lines(raw_data).into_iter();
    let depart_time: u64 = input_iter.next().unwrap().parse().unwrap();
    let bus_times: Vec<u64> = input_iter
        .next()
        .unwrap()
        .split(',')
        .map(|n| if n == "x" { 0 } else { n.parse().unwrap() })
        .collect();

    print!("Day 13 part 1: {}\n", part_1(depart_time, &bus_times));
    print!("Day 13 part 2: {}\n", part_2(&bus_times));
}

fn part_1(depart_time: u64, bus_times: &Vec<u64>) -> u64 {
    let res: (u64, u64) = bus_times
        .into_iter()
        .filter(|time| **time != 0)
        .map(|time| {
            (
                *time,
                ((depart_time as f64 / *time as f64).ceil() as u64 * time) - depart_time,
            )
        })
        .fold((depart_time, depart_time), |acc, val| {
            if val.1 < acc.1 {
                (val.0, val.1)
            } else {
                (acc.0, acc.1)
            }
        });
    return res.0 * res.1;
}

fn valid_sequence(start_time: u64, bus_times: &Vec<u64>) -> bool {
    bus_times
        .into_iter()
        .enumerate()
        .all(|(i, time)| *time == 0 || ((start_time + i as u64) % time == 0))
}

fn part_2(bus_times: &Vec<u64>) -> u64 {
    let delta = bus_times[0];
    let mut start_time = bus_times[0];
    loop {
        if valid_sequence(start_time, bus_times){
            return start_time;
        }
        start_time += delta;
    } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_seq() {
        let input = vec![4,5,0,7,8];
        assert_eq!(valid_sequence(4, &input), true)
    }
    #[test]
    fn test_part_2() {
        let input = vec![7,13,0,0,59,0,31,19];
        assert_eq!(part_2(&input), 1068781)
    }
}
