use std::collections::HashMap;

use read_file::InputParser;

// fn count_crabs_at_positions(crab_positions: &Vec<usize>) -> HashMap<usize, usize> {
//     let mut grouped_positions = HashMap::new();
//     for crab in crab_positions {
//         *grouped_positions.entry(*crab).or_insert(0) += 1;
//     }
//     return grouped_positions;
// }


fn find_desired_position(crab_positions: &Vec<usize>) -> usize {
    // Calculate the median value
    let mut sorted_crab_positions = crab_positions.clone();
    sorted_crab_positions.sort();
    let mid_point = sorted_crab_positions.len() / 2;
    sorted_crab_positions[mid_point]
}

fn get_min_fuel_to_align_crabs(crab_positions: Vec<usize>) -> usize {
    let desired_pos = find_desired_position(&crab_positions);
    println!("Desired position: {}", desired_pos);

    let mut fuel_used: usize = 0;
    for crab_position in crab_positions {
        let diff: isize = crab_position as isize - desired_pos as isize;
        fuel_used += diff.abs() as usize;
    }

    return fuel_used;
}

pub fn day7a() -> String {
    let crab_positions: Vec<usize> = Vec::read_single_line_csv("./inputs/day7/full".to_string());
    get_min_fuel_to_align_crabs(crab_positions).to_string()
}

pub fn day7b() -> String {
    "Nope".to_string()
}