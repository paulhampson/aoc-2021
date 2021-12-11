use read_file::InputParser;

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

fn find_desired_position_b(crab_positions: &Vec<usize>) -> usize {
    // Calculate the mean value
    let sum = crab_positions.iter().sum::<usize>();
    let mean = sum as f32 / crab_positions.len() as f32;
    println!("True mean: {}", mean);
    mean.round() as usize
}

fn calc_fuel_use_for_pos(crab_positions: &Vec<usize>, desired_pos: usize) -> usize {
    let mut fuel_used: usize = 0;
    for crab_position in crab_positions {
        let position_delta: isize = *crab_position as isize - desired_pos as isize;
        // sum of natural numbers n(n+1)/2 where n=position_delta
        fuel_used += ((position_delta.abs() * (position_delta.abs() + 1)) / 2) as usize;
    }
    return fuel_used;
}

fn get_min_fuel_to_align_crabs_b(crab_positions: Vec<usize>) -> usize {
    let desired_pos = find_desired_position_b(&crab_positions);
    println!("Desired position (mean): {}", desired_pos);

    // A bit of experimentation shows that apparently there's some margin around it being the mean.
    // This threw me for a while, so I just brute force around the mean.
    let mut fuel_use_exploration = Vec::new();
    let exploration_range = 5;
    let lowest: usize;
    if desired_pos < exploration_range {
        lowest = 0;
    } else {
        lowest = desired_pos - 5;
    }

    let highest = desired_pos + exploration_range;
    for pos in lowest..highest {
        let fuel_used = calc_fuel_use_for_pos(&crab_positions, pos);
        println!("{} => {}", pos, fuel_used);
        fuel_use_exploration.push(fuel_used);
    }
    fuel_use_exploration.sort();

    return fuel_use_exploration[0];
}

pub fn day7a() -> String {
    let crab_positions: Vec<usize> = Vec::read_single_line_csv("./inputs/day7/full".to_string());
    get_min_fuel_to_align_crabs(crab_positions).to_string()
}

pub fn day7b() -> String {
    let crab_positions: Vec<usize> = Vec::read_single_line_csv("./inputs/day7/full".to_string());
    get_min_fuel_to_align_crabs_b(crab_positions).to_string()
}