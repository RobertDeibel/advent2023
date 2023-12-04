use std::num;

pub fn run(args: Vec<String>) -> String {
    let lines = super::read_lines(&args[0]);

    let mut line_nums: Vec<i32> = Vec::new();
    let mut num_indices: Vec<(usize, usize)>= Vec::new();

    for line in &lines {
        let (nums, indices) = nums_in_str(line);
        let num = nums[0] * 10 + nums[nums.len() - 1];
        line_nums.push(num);
        num_indices.push((indices[0], indices[indices.len() - 1]));
    }

    let result1: i32 = line_nums.iter().sum::<i32>();

    for (line, indices) in (&lines).iter().zip(num_indices) {
        let nums: Vec<i32> = writen_nums(line[0])
    }

let result2: i32 = 123;

    format!("Part 1: {:?}\nPart 2: {:?}", &result1, &result2)
}

pub fn num_from_char(s: &char) -> Result<i32, num::ParseIntError> {
    s.to_string().parse::<i32>()
}

pub fn nums_in_str(s: &str) -> (Vec<i32>, Vec<usize>) {
    let mut nums = Vec::new();
    let mut indices: Vec<usize> = Vec::new();
    for (i, char) in s.chars().enumerate() {
        let num = match num_from_char(&char) {
            Ok(num) => num,
            Err(_) => continue,
        };
        nums.push(num);
        indices.push(i);
    }

    (nums, indices)
}

pub fn writen_nums(s: &str) -> Vec<i32> {
    const numbers: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut nums: Vec<i32> = Vec::new();

    s.find(numbers);
    nums
}