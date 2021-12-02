use read_file::InputParser;

pub fn day1a() -> String {
    let input_data: Vec<usize> = Vec::read_input("./inputs/day1/full".to_string());
    let mut increased_measurement_count = 0;

    let mut previous_measurement: Option<usize> = None;
    for measurement in input_data {
        if previous_measurement != None {
            if measurement > previous_measurement.unwrap() {
                increased_measurement_count += 1;
            }
        }
        previous_measurement = Some(measurement);
    }

    increased_measurement_count.to_string()
}

pub fn day1b() -> String {
    let input_data: Vec<usize> = Vec::read_input("./inputs/day1/full".to_string());
    let input_windows = input_data.windows(3);
    let mut increased_measurement_count = 0;

    let mut previous_measurement: Option<usize> = None;
    for measurement_window in input_windows {
        let window_sum = measurement_window.iter().sum();
        if previous_measurement != None {
            if window_sum > previous_measurement.unwrap() {
                increased_measurement_count += 1;
            }
        }
        previous_measurement = Some(window_sum);
    }

    increased_measurement_count.to_string()
}