extern crate aoc_2021;

use aoc_2021::day1::{day1a, day1b};
use aoc_2021::day2::{day2a, day2b};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1)
        .map(|s| s.as_str())
        .unwrap_or("None");

    let result = match problem {
        "day1a" => day1a(),
        "day1b" => day1b(),
        "day2a" => day2a(),
        "day2b" => day2b(),
        _ => "Problem solution not available".to_string(),
    } ;

    println!("{} result is {}", problem, result);
}