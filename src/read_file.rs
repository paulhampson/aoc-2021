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
}