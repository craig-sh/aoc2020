use std::env;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod utils;

fn runner(day: u8) {
    match day {
        1 => day_01::solve(),
        2 => day_02::solve(),
        3 => day_03::solve(),
        4 => day_04::solve(),
        5 => day_05::solve(),
        6 => day_06::solve(),
        _ => println!("Nothing on this day."),
    }
}

fn main() {
    let day = env::args()
        .nth(1)
        .expect("Must provide exactly 1 arg for day")
        .parse()
        .unwrap();
    runner(day);
}
