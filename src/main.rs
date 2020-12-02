use std::io::{BufRead, BufReader};
use std::fs::File;


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


fn main() {
    let res = day1_p2();
    println!("{}", res);
}
