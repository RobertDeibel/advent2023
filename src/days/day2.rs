const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;


/// Calculates solution of day 2 in string format.
/// Args:
///     args - input arguments for problem
/// Returns:
///     Solution of day 2 as string
pub fn run(args: Vec<String>) -> String {
    let lines = super::read_lines(&args[0]);

    let mut sum = 0;
    for line in lines {
        let split: Vec<&str> = line.split(':').collect();

        let line = split[1];
        // game number is only number in the string
        let game = super::day1::nums_in_str(split[0]).0[0];

        let mut valid = false;

        for draw in line.split(';') {
            
        }

        if valid {
            sum += game;
        }
    }

    let result1 = String::new();
    super::make_output(&[result1])
}