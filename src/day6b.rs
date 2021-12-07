/* Part B requires a different approach due to the otherwise exponential computation required. What
 * I've done here is to just count how many fish are of a certain age then through each day those
 * groups move down until they hit 0, the number of fish in the 0 group is the number of fish that
 * then are grouped in age 8 and the ones in 0 then move back to group 6 - added of course to any that
 * were 7 on the last day.
 */

use std::collections::HashMap;

use day6::parse_initial_population;
use read_file::InputParser;

fn parse_initial_population_groups(input: Vec<String>) -> HashMap<i32, usize> {
    let mut population = HashMap::new();
    let population_age_list = parse_initial_population(input);

    // age can be the values 0 through to 8 inclusive
    for age in 0..9 {
        population.insert(age,
                          population_age_list.iter().filter(|fish_age| **fish_age == age).count());
    }
    return population;
}

fn lantern_fish_day(population: HashMap<i32, usize>) -> HashMap<i32, usize> {
    let mut new_population = HashMap::new();
    for age in 0..9 {
        if age == 0 {
            // gestation complete, give birth, new fish need 8 days to mature and gestate
            new_population.insert(8, population[&0]);
            // existing fish restart gestation
            new_population.insert(6, population[&0]);
        } else {
            let new_age = age - 1;
            if new_population.contains_key(&new_age) {
                new_population.insert(new_age, new_population[&new_age] + population[&age]);
            } else {
                new_population.insert(new_age, population[&age]);
            }
        }
    }
    return new_population;
}

fn run_simulation(population: HashMap<i32, usize>, days: i32) -> HashMap<i32, usize> {
    let mut interim_population = population;
    for day in 0..days {
        println!("{}", day);
        interim_population = lantern_fish_day(interim_population);
    }
    return interim_population;
}

pub fn day6b() -> String {
    let input_lines: Vec<String> = Vec::read_input("./inputs/day6/full".to_string());
    let mut fish_population = parse_initial_population_groups(input_lines);
    fish_population = run_simulation(fish_population, 256);
    fish_population.values().into_iter().sum::<usize>().to_string()
}