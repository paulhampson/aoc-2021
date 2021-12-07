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
        if line.start.x == line.end.x {
            // vertical line
            let x_pos = line.start.x;
            // ensure start is l.t. end
            let mut start_y = line.start.y;
            let mut end_y = line.end.y;
            if start_y > end_y {
                start_y = line.end.y;
                end_y = line.start.y;
            }
            for y_pos in start_y..end_y + 1 {
                line_map[x_pos as usize][y_pos as usize] += 1;
            }
        } else if line.start.y == line.end.y {
            // horizontal line
            let y_pos = line.start.y;
            // ensure start is l.t. end
            let mut start_x = line.start.x;
            let mut end_x = line.end.x;
            if start_x > end_x {
                start_x = line.end.x;
                end_x = line.start.x;
            }
            for x_pos in start_x..end_x + 1 {
                line_map[x_pos as usize][y_pos as usize] += 1;
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
    "Nope".to_string()
}