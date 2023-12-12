use std::env;
mod advent_lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a day number");
    }

    let day: &u8 = &args[1].parse().expect("Please provide a day number");

    let day_args: Vec<String> = args[2..].to_vec();

    println!("Hello, to advent day {day}!! Let's go!");

    let result = match day {
        0 => panic!("Invalid day number"),
        1 => advent_lib::day1::run(day_args),
        2 => advent_lib::day2::run(day_args),
        3 => advent_lib::day3::run(day_args),
        4 => advent_lib::day4::run(day_args),
        5..=u8::MAX => panic!("Invalid day number"),
    };

    println!("We have the result!!!");
    println!("{}", result);
}
