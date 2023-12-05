use std::{num, result};

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

    for (i, line) in (&lines).iter().enumerate() {
        let mut nums = written_nums(&line[..num_indices[i].0], true);
        if nums.len() > 0 {
            nums.sort_by(|a, b| a.1.cmp(&b.1));
            line_nums[i] = line_nums[i] % 10 + nums[0].0 * 10;
        }

        let mut nums = written_nums(&line[num_indices[i].1..], false);
        if nums.len() > 0 {
            nums.sort_by(|a, b| b.1.cmp(&a.1));
            line_nums[i] = (line_nums[i] / 10) * 10 + nums[0].0;
        }

    }

    let result2: i32 = line_nums.iter().sum();

    format!("Part 1: {:?}\nPart 2: {:?}", &result1, &result2)
}

pub fn num_from_char(s: &char) -> Result<i32, num::ParseIntError> {
    s.to_string().parse::<i32>()
}

pub fn nums_in_str(s: &str) -> (Vec<i32>, Vec<usize>) {
    let mut nums = Vec::new();
    let mut indices = Vec::new();
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

pub fn written_nums(s: &str, left: bool) -> Vec<(i32, i32)> {
    const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut nums = Vec::new();

    for (s, idx) in find_indices(s, &NUMBERS, left) {
        for (i, num) in (&NUMBERS).iter().enumerate() {
            if s == *num {
                nums.push((i  as i32 + 1, idx))
            }
        }
    }

    nums
}

pub fn find_indices<'a>(s: &str, reference: &[&'a str], left: bool) -> Vec<(&'a str, i32)> {
    let mut result: Vec<(&str, i32)> = Vec::new();

    let s = s.to_lowercase();
    for r in reference.iter() {
        let find = if left {s.find(r)} else {s.rfind(r)};
        let find = match find {
            None => continue,
            Some(r) => r,
        };

        result.push((r, find as i32));
    };

    result
}