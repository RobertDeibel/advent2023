use std::fs;
use std::io;
use std::io::BufRead;

#[path = "days/day1.rs"] pub mod day1;
#[path = "days/day2.rs"] pub mod day2;

/// Reads and returns lines from a file
/// Args:
///     path to file
pub fn read_lines(path: &str) -> Vec<String> {
    let file = fs::File::open(path).expect("Unable to open file");
    let reader = io::BufReader::new(file);
    return reader.lines().map(|l| l.expect("Unable to read line")).collect()
}

/// Builds output from result strings.
/// Args:
///     strings as array
/// Returns:
///     combined result string
pub fn make_output(results: &[String]) -> String {
    let mut output: String = String::new();
    for (i, result) in results.iter().enumerate() {
        if !output.is_empty(){
            output += "\n";
        }
        output += &format!("part {i}: {}", result);
    }

    output
}