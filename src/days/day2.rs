use crate::advent_lib;


#[derive(PartialEq, Eq)]
struct Color<'a> {
    value: i32,
    name: &'a str,
}

const RED: Color = Color {value: 12, name: "red"};
const GREEN: Color = Color {value: 13, name: "green"};
const BLUE: Color = Color {value: 14, name: "blue"};

struct Power {
    red: i32,
    green: i32,
    blue: i32,
}

impl Power {
    fn new() -> Power {
        Power {
            red: 0,
            green: 0,
            blue: 0
        }
    }

    fn get_col(&self, col: &Color<'_>) -> i32 {
        match *col {
            RED => self.red,
            GREEN => self.green,
            BLUE => self.blue,
            _ => panic!("SHIT IS DOWN")
        }
    }

    fn set_col(&mut self, col: &Color<'_>, value: i32) {
        match *col {
            RED => self.red = value,
            GREEN => self.green = value,
            BLUE => self.blue = value,
            _ => panic!("SHIT IS DOWN")
        }
    }

    fn calc_power(&self) -> i32 {
        self.red * self.blue * self.green
    }
}

/// Calculates solution of day 2 in string format.
/// Args:
///     args - input arguments for problem
/// Returns:
///     Solution of day 2 as string
pub fn run(args: Vec<String>) -> String {
    let lines = super::read_lines(&args[0]);

    let mut sum = 0;
    let mut power_sum = 0;
    for line in lines {
        let split: Vec<&str> = line.split(':').collect();

        let line = split[1];
        // game number is only number in the string
        let values = super::day1::nums_in_str(split[0]).0;
        let game = reduce_to_num(values);

        let mut valid = true;

        let mut power = Power::new();
        for draw in line.split(';') {
            for col in draw.split(',') {
                let count = reduce_to_num(super::day1::nums_in_str(col).0);
                for c in &[RED, GREEN, BLUE] {
                    if col.contains(&c.name) {
                        if valid && count > c.value {
                            valid = false;
                        }
                        if power.get_col(c) < count {
                            power.set_col(c, count)
                        }
                    }
                }
            }
        }

        if valid {
            sum += game;
            advent_lib::debug_msg(&line, &args);
            advent_lib::debug_msg(&format!("added {} to {} for {}", game, sum - game, sum), &args);
        } else {
            advent_lib::debug_msg(&line, &args);
            advent_lib::debug_msg(&format!("NOT added {} to {}", game, sum), &args);
        }
        power_sum += power.calc_power();
    }

    let result1 = format!("{:?}", sum);
    let result2 = format!("{:?}", power_sum);
    super::make_output(&[result1, result2])
}


/// Reduces integer vector to number by summation
pub fn reduce_to_num(values: Vec<i32>) -> i32 {
    let mut result = 0;
    for value in values {
        result *= 10;
        result += value;
    }
    result
}

