use std::fs;
use std::io;
use std::io::BufRead;

#[path = "days/day1.rs"] pub mod day1;

pub fn read_lines(path: &str) -> Vec<String> {
    let file = fs::File::open(path).expect("Unable to open file");
    let reader = io::BufReader::new(file);
    return reader.lines().map(|l| l.expect("Unable to read line")).collect()
}