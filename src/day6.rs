use std::thread;

use read_file::InputParser;

fn parse_initial_population(input: Vec<String>) -> Vec<i32> {
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

pub fn day6b() -> String {
    let input_lines: Vec<String> = Vec::read_input("./inputs/day6/basic".to_string());
    let mut fish_population = parse_initial_population(input_lines);

    let mut cores = usize::min(fish_population.len(), 12);
    let chunk_size = fish_population.len() / cores;
    // split the population into number of core parts so that we can run it across multiple threads, we can then combine the results
    let population_chunks = fish_population.chunks_mut(chunk_size);

    let mut thread_handles = Vec::new();
    for chunk in population_chunks {
        let thread_chunk = chunk.to_vec().clone();
        let handle = thread::spawn(move || run_simulation(thread_chunk.to_vec(), 256));
        thread_handles.push(handle);
    }

    /* wait for thread completion */
    let mut full_population = Vec::new();
    for handle in thread_handles {
        full_population.append(&mut handle.join().unwrap());
    }

    full_population.len().to_string()
}
