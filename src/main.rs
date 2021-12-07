extern crate aoc_2021;

use aoc_2021::day1::{day1a, day1b};
use aoc_2021::day2::{day2a, day2b};
use aoc_2021::day3::{day3a, day3b};
use aoc_2021::day4::{day4a, day4b};
use aoc_2021::day5::{day5a, day5b};
use aoc_2021::day6::day6a;
use aoc_2021::day6b::day6b;

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
        "day3a" => day3a(),
        "day3b" => day3b(),
        "day4a" => day4a(),
        "day4b" => day4b(),
        "day5a" => day5a(),
        "day5b" => day5b(),
        "day6a" => day6a(),
        "day6b" => day6b(),
        _ => "Problem solution not available".to_string(),
    };

    println!("{} result is {}", problem, result);
}