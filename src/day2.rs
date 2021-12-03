use read_file::InputParser;

pub fn day2a() -> String {
    let input_lines: Vec<String> = Vec::read_input("./inputs/day2/full".to_string());
    let mut horizontal_pos = 0;
    let mut depth_pos = 0;

    for line in input_lines {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let value = parts[1].parse::<usize>().unwrap();
        match parts[0] {
            "forward" => horizontal_pos += value,
            "down" => depth_pos += value,
            "up" => depth_pos -= value,
            _ => (),
        };
    }

    (horizontal_pos * depth_pos).to_string()
}

pub fn day2b() -> String {
    let input_lines: Vec<String> = Vec::read_input("./inputs/day2/full".to_string());
    let mut horizontal_pos = 0;
    let mut depth_aim = 0;
    let mut depth_pos = 0;

    for line in input_lines {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let value = parts[1].parse::<usize>().unwrap();
        match parts[0] {
            "forward" => {
                horizontal_pos += value;
                depth_pos += value * depth_aim;
            },
            "down" => depth_aim += value,
            "up" => depth_aim -= value,
            _ => (),
        };
    }

    (horizontal_pos * depth_pos).to_string()
}