use read_file::InputParser;

pub fn parse_initial_population(input: Vec<String>) -> Vec<i32> {
    let separate_values: Vec<&str> = input[0].split(",").collect();
    separate_values
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn lantern_fish_day(population: Vec<i32>) -> Vec<i32> {
    let mut new_population = Vec::new();
    for fish in population {
        if fish == 0 {
            // produces new fish - with 8 days to reproduction, parent fish resets to 6 days
            new_population.push(8);
            new_population.push(6);
        } else {
            new_population.push(fish - 1);
        }
    }
    return new_population;
}

fn run_simulation(population: Vec<i32>, days: i32) -> Vec<i32> {
    let mut interim_population = population;
    for day in 0..days {
        println!("{}", day);
        interim_population = lantern_fish_day(interim_population);
    }
    return interim_population;
}

pub fn day6a() -> String {
    let input_lines: Vec<String> = Vec::read_input("./inputs/day6/full".to_string());
    let mut fish_population = parse_initial_population(input_lines);
    fish_population = run_simulation(fish_population, 80);
    fish_population.len().to_string()
}
