use read_file::InputParser;

pub fn day2a() -> String {
    let input_lines: Vec<String> = Vec::read_input("./inputs/day2/full".to_string());
    let mut horizontal_pos = 0;
    let mut depth_pos = 0;

    for line in input_lines {
        let parts = line.split(" ").collect::<Vec<&str>>();
        match parts[0] {
            "forward" => horizontal_pos += parts[1].parse::<usize>().unwrap(),
            "down" => depth_pos += parts[1].parse::<usize>().unwrap(),
            "up" => depth_pos -= parts[1].parse::<usize>().unwrap(),
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
        match parts[0] {
            "forward" => {
                let horizontal_delta = parts[1].parse::<usize>().unwrap();
                horizontal_pos += horizontal_delta;
                depth_pos += horizontal_delta * depth_aim;
            },
            "down" => depth_aim += parts[1].parse::<usize>().unwrap(),
            "up" => depth_aim -= parts[1].parse::<usize>().unwrap(),
            _ => (),
        };
    }

    (horizontal_pos * depth_pos).to_string()
}