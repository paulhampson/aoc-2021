use grid::Grid;

use read_file::InputParser;

struct Position {
    x: i32,
    y: i32,
}

struct Line {
    start: Position,
    end: Position,
}

fn pos_from_str(s: String) -> Position {
    let xy_str: Vec<&str> = s.split(",").collect();
    Position {
        x: xy_str[0].parse::<i32>().unwrap(),
        y: xy_str[1].parse::<i32>().unwrap(),
    }
}

fn parse_line_input(input_data: Vec<String>) -> Vec<Line>
{
    let mut lines = Vec::new();
    for entry in input_data {
        let start_end_str: Vec<&str> = entry.split(" -> ").collect();
        let line = Line {
            start: pos_from_str(start_end_str[0].to_string()),
            end: pos_from_str(start_end_str[1].to_string()),
        };
        lines.push(line);
    }
    return lines;
}

fn horiz_vert_lines_only(lines: Vec<Line>) -> Vec<Line> {
    lines.into_iter().filter(|s| (s.start.x == s.end.x) || (s.start.y == s.end.y)).collect()
}

fn get_grid_size(lines: &Vec<Line>) -> (i32, i32) {
    let mut max_x = 0;
    let mut max_y = 0;

    for line in lines {
        if line.start.x > max_x {
            max_x = line.start.x;
        }
        if line.start.y > max_y {
            max_y = line.start.y;
        }
        if line.end.x > max_x {
            max_x = line.end.x;
        }
        if line.end.y > max_y {
            max_y = line.end.y;
        }
    }

    return (max_x + 1, max_y + 1);
}

fn populate_grid(lines: &Vec<Line>) -> Grid<i32> {
    let (x, y) = get_grid_size(lines);
    let mut line_map = Grid::from_vec(vec![0; (x * y) as usize], x as usize);

    for line in lines {
        // Determine which way we need to move in the X and Y axes
        let mut x_step = 0;
        let mut y_step = 0;
        if line.start.x < line.end.x {
            x_step = 1;
        }
        if line.start.x > line.end.x {
            x_step = -1;
        }
        if line.start.y < line.end.y {
            y_step = 1;
        }
        if line.start.y > line.end.y {
            y_step = -1;
        }

        let x_start = line.start.x;
        let y_start = line.start.y;
        let x_end = line.end.x;
        let y_end = line.end.y;

        let mut x_pos = x_start;
        let mut y_pos = y_start;
        let mut x_finished = false;
        let mut y_finished = false;

        while !x_finished || !y_finished {
            line_map[x_pos as usize][y_pos as usize] += 1;

            if x_pos != x_end {
                x_pos += x_step;
            } else {
                x_finished = true;
            }

            if y_pos != y_end {
                y_pos += y_step;
            } else {
                y_finished = true;
            }
        }
    }

    return line_map;
}

fn overlap_count(g: &Grid<i32>) -> i32 {
    let mut overlap_count = 0;
    for entry in g.iter() {
        if *entry >= 2 {
            overlap_count += 1;
        }
    }
    return overlap_count;
}

pub fn day5a() -> String {
    let input_lines: Vec<String> = Vec::read_input("./inputs/day5/full".to_string());
    let all_lines = parse_line_input(input_lines);
    let horizontal_vertical_lines_only = horiz_vert_lines_only(all_lines);
    let map = populate_grid(&horizontal_vertical_lines_only);
    let overlap_count = overlap_count(&map);

    overlap_count.to_string()
}

pub fn day5b() -> String {
    let input_lines: Vec<String> = Vec::read_input("./inputs/day5/full".to_string());
    let all_lines = parse_line_input(input_lines);
    let map = populate_grid(&all_lines);
    let overlap_count = overlap_count(&map);

    overlap_count.to_string()
}