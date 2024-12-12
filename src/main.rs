mod day12;

use chrono::{Datelike, FixedOffset, Utc};
use std::env;
use std::fs;

// Adapted from the documentation
fn read_lines(day: i32) -> Vec<String> {
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

fn get_answers(input: Vec<String>, day: i32) -> (String, String) {
    match day {
        12 => convert_ans_to_string!(day12::answers(input)),
        _ => unimplemented!(),
    }
}

fn main() {
    let eastern = FixedOffset::west_opt(5 * 3600).unwrap();
    let date = Utc::now().with_timezone(&eastern).day() as i32;

    let day: i32 = {
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
