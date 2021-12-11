use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub trait InputParser<T> {
    fn read_input(filename: String) -> Vec<T>;
    fn read_single_line_csv(filename: String) -> Vec<T>;
}

impl InputParser<String> for Vec<String> {
    fn read_input(filename: String) -> Vec<String> {
        let mut all_lines: Vec<String> = Vec::new();
        if let Ok(lines) = read_lines(filename) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    all_lines.push(ip);
                }
            }
        }
        return all_lines;
    }

    fn read_single_line_csv(_filename: String) -> Vec<String> {
        // Not yet implemented
        Vec::new()
    }
}

impl InputParser<usize> for Vec<usize> {
    fn read_input(filename: String) -> Vec<usize> {
        let mut all_lines: Vec<usize> = Vec::new();
        if let Ok(lines) = read_lines(filename) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    all_lines.push(ip.parse().unwrap());
                }
            }
        }
        return all_lines;
    }

    fn read_single_line_csv(filename: String) -> Vec<usize> {
        let mut result: Vec<usize> = vec![];
        if let Ok(mut lines) = read_lines(filename) {
            if let Ok(first_line) = lines.next().unwrap() {
                let separate_values: Vec<&str> = first_line.split(',').collect();
                result = separate_values
                    .iter()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();
            }
        }
        return result;
    }
}