mod day11;
mod day12;
mod day13;
mod day14;

use chrono::{Datelike, FixedOffset, Utc};
use std::env;
use std::fs;

// Adapted from the documentation
fn read_lines(day: i64) -> Vec<String> {
    let path = format!("input/day{day}.txt");
    let message = format!("Couldn't open input for Day {day}");

    fs::read_to_string(&path)
        .expect(&message)
        .lines()
        .map(String::from)
        .collect()
}

macro_rules! convert_ans_to_string {
    ($ans:expr) => {{
        let ans = $ans;
        (ans.0.to_string(), ans.1.to_string())
    }};
}

fn get_answers(input: Vec<String>, day: i64) -> (String, String) {
    match day {
        11 => convert_ans_to_string!(day11::answers(input)),
        12 => convert_ans_to_string!(day12::answers(input)),
        13 => convert_ans_to_string!(day13::answers(input)),
        14 => convert_ans_to_string!(day14::answers(input)),
        _ => unimplemented!(),
    }
}

fn main() {
    let eastern = FixedOffset::west_opt(5 * 3600).unwrap();
    let date = Utc::now().with_timezone(&eastern).day() as i64;

    let day: i64 = {
        if let Some(arg) = env::args().nth(1) {
            let arg = arg.parse().unwrap();
            if arg > date {
                panic!("It isn't Day {arg} yet!");
            } else {
                arg
            }
        } else {
            date
        }
    };

    let input = read_lines(day);
    let (ans1, ans2) = get_answers(input, day);

    println!("Answer 1: {ans1}");
    println!("Answer 2: {ans2}");
}
