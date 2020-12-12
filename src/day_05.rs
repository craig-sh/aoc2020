use crate::utils;
use std::fs::read_to_string;

pub fn solve() {
    let raw_data = read_to_string(utils::filename(5)).unwrap();
    let input = utils::clean_lines(&raw_data);
    print!("Day 5 part 1: {}\n", part_1(&input));
    print!("Day 2 part 2: {}\n", part_2(&input));
}

fn binary_search(instructions: &str, high: usize, low: usize) -> usize {
    let mut chars = instructions.chars();
    let mut _high = high;
    let mut _low = low;

    loop {
        let step = (_high - _low) / 2;
        match chars.next().unwrap() {
            'F' | 'L' => {
                _high -= step + 1;
            }
            'B' | 'R' => {
                _low += step + 1;
            }
            _ => (),
        }
        if _low == _high {
            break _low;
        }
    }
}

fn calc_seatid(boarding_pass: &str) -> usize {
    let row_id = binary_search(&String::from(boarding_pass)[0..7], 127, 0);
    let seat_id = binary_search(&String::from(boarding_pass)[7..10], 7, 0);

    return row_id * 8 + seat_id;
}

fn part_1(boarding_passes: &Vec<&str>) -> usize {
    let mut value = 0;
    for boarding_pass in boarding_passes.into_iter() {
        value = std::cmp::max(calc_seatid(boarding_pass), value);
    }
    return value;
}

fn part_2(boarding_passes: &Vec<&str>) -> usize {
    let mut all_seats: Vec<usize> = boarding_passes
        .into_iter()
        .map(|x| calc_seatid(x))
        .collect();
    all_seats.sort();
    for (i, seat_id) in all_seats.clone().into_iter().enumerate() {
        if all_seats[i + 1] == seat_id + 2 {
            return seat_id + 1;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::calc_seatid;

    #[test]
    fn test_calcseatid() {
        assert_eq!(calc_seatid("FBFBBFFRLR"), 357);
    }
}
