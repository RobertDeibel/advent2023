use std::num;

pub fn run(args: Vec<String>) -> String {
    let lines = super::read_lines(&args[0]);

    let mut line_nums: Vec<i32> = Vec::new();
    for line in lines {
        let nums = nums_in_str(&line);
        let num = nums[0] * 10 + nums[nums.len() - 1];
        line_nums.push(num)
    }

    let result: i32 = line_nums.iter().sum::<i32>();

    return format!("{:?}", &result);
}

pub fn num_from_char(s: &char) -> Result<i32, num::ParseIntError> {
    return s.to_string().parse::<i32>();
}

pub fn nums_in_str(s: &str) -> Vec<i32> {
    let mut nums = Vec::new();
    for char in s.chars() {
        let num = match num_from_char(&char) {
            Ok(num) => num,
            Err(_) => continue,
        };
        nums.push(num);
    }
    return  nums;
}
