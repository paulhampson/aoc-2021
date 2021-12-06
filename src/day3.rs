use read_file::InputParser;

fn most_common_bit_value(input_strings: &Vec<String>, bit_idx: usize) -> Option<i32> {
    let mut bit_counter = 0;
    for s in input_strings {
        let b = s.as_bytes()[bit_idx] as char;
        if b == '0' {
            bit_counter -= 1;
        } else if b == '1' {
            bit_counter += 1;
        }
    }

    if bit_counter > 0 {
        Some(1)
    } else if bit_counter < 0 {
        Some(0)
    } else {
        None
    }
}

fn least_common_bit_value(input_strings: &Vec<String>, bit_idx: usize) -> Option<i32> {
    let most_common = most_common_bit_value(input_strings, bit_idx);

    if most_common.is_some() {
        let least_common_value = (!most_common.unwrap()) & 0x1;
        Some(least_common_value)
    } else {
        None
    }
}

fn get_gamma_value(input_strings: &Vec<String>) -> (i32, usize) {
    let bit_count = input_strings[0].len();
    let mut gamma = 0;
    for bit_idx in 0..bit_count {
        gamma = gamma << 1;
        let bit_value = most_common_bit_value(&input_strings, bit_idx);
        if bit_value.is_some() {
            gamma |= bit_value.unwrap();
        }
    }
    return (gamma, bit_count);
}

fn calc_power(gamma: i32, bit_count: usize) -> i32 {
    let mask = (1 << bit_count) - 1;
    let epsilon = (!gamma) & mask;
    gamma * epsilon
}

fn oxygen_rating(input_strings: &Vec<String>) -> i32 {
    let bit_count = input_strings[0].len();
    let mut number_list = input_strings.clone();
    let mut new_number_list: Vec<String> = Vec::new();

    for bit_idx in 0..bit_count {
        let common_bit_value = most_common_bit_value(&number_list, bit_idx).unwrap_or(1);
        for number in number_list {
            // if the most common bit matches the current bit index then keep the value in the list
            let bit = number.as_bytes()[bit_idx] as char;
            if bit == common_bit_value.to_string().as_bytes()[0] as char {
                new_number_list.push(number);
            }
        }
        number_list = new_number_list.clone();
        new_number_list = Vec::new();

        println!("{:?}", number_list);

        // if there's only one entry left then we're done
        if number_list.len() == 1 {
            break;
        }
    }
    i32::from_str_radix(number_list[0].as_str(), 2).unwrap()
}

fn co2_rating(input_strings: &Vec<String>) -> i32 {
    let bit_count = input_strings[0].len();
    let mut number_list = input_strings.clone();
    let mut new_number_list: Vec<String> = Vec::new();

    for bit_idx in 0..bit_count {
        let common_bit_value = least_common_bit_value(&number_list, bit_idx).unwrap_or(0);
        for number in number_list {
            // if the most common bit matches the current bit index then keep the value in the list
            let bit = number.as_bytes()[bit_idx] as char;
            if bit == common_bit_value.to_string().as_bytes()[0] as char {
                new_number_list.push(number);
            }
        }
        number_list = new_number_list.clone();
        new_number_list = Vec::new();

        println!("{:?}", number_list);

        // if there's only one entry left then we're done
        if number_list.len() == 1 {
            break;
        }
    }
    i32::from_str_radix(number_list[0].as_str(), 2).unwrap()
}

pub fn day3a() -> String {
    let input_lines: Vec<String> = Vec::read_input("./inputs/day3/full".to_string());
    let (gamma, bit_count) = get_gamma_value(&input_lines);
    calc_power(gamma, bit_count).to_string()
}

pub fn day3b() -> String {
    let input_lines: Vec<String> = Vec::read_input("./inputs/day3/full".to_string());
    let co2_rating = co2_rating(&input_lines);
    let o2_rating = oxygen_rating(&input_lines);
    (co2_rating * o2_rating).to_string()
}