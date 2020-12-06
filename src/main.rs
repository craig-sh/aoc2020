use std::io::{BufRead, BufReader, Lines};
use std::fs::File;


fn read_file(day: u8) -> Lines<BufReader<File>>{
    let f_name = format!("src/inputs/day{day}.txt", day = day);
    let reader = BufReader::new(File::open(f_name).unwrap());
    return reader.lines()
}

fn day1_p1() -> u32{
    let f_name = "src/inputs/day1.txt";
    let reader = BufReader::new(File::open(f_name).expect("Cannot open file"));
    let mut v: Vec<u32> = Vec::new();
    for line in reader.lines() {
        for word in line.unwrap().split_whitespace() {
            v.push(word.parse::<u32>().unwrap());
        }
    }
    for (i, num1) in v.iter().enumerate(){
        for num2 in v[i..].iter(){
            if num1 + num2 == 2020 {
                return num1 * num2;
            }
        }
    }


    return v[1]

}


fn day1_p2() -> u32{
    let f_name = "src/inputs/day1.txt";
    let reader = BufReader::new(File::open(f_name).expect("Cannot open file"));
    let mut v: Vec<u32> = Vec::new();
    for line in reader.lines() {
        for word in line.unwrap().split_whitespace() {
            v.push(word.parse::<u32>().unwrap());
        }
    }
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

fn day2_p1() -> u32 {
    let f_name = "src/inputs/day2.txt";
    let reader = BufReader::new(File::open(f_name).expect("Cannot open file"));
    let mut correct: u32 = 0;
    for _line in reader.lines() {
        let line = _line.unwrap().clone();
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

fn day2_p2() -> u32 {
    let mut correct: u32 = 0;
    for _line in read_file(2) {
        let line = _line.unwrap();
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


fn main() {
    let res = day2_p2();
    println!("{}", res);
}
