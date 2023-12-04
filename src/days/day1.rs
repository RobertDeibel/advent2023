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

    for (i, line) in (&lines).iter().enumerate() {
        println!("line: {}", line);
        let mut nums = written_nums(&line[0..=num_indices[i].0], true);
        if nums.len() > 0 {
            nums.sort_by(|a, b| a.1.cmp(&b.1));
            println!("{}, {:?}, {}", &line[0..=num_indices[i].0], nums, line_nums[i]);
            line_nums[i] = line_nums[i] % 10 + nums[0].0 * 10;
        }

        let mut nums = written_nums(&line[num_indices[i].1..=line.len() - 1], false);
        if nums.len() > 0 {
            nums.sort_by(|a, b| b.1.cmp(&a.1));
            println!("{}, {:?}, {}", &line[num_indices[i].1..=line.len() - 1], nums, line_nums[i]);
            line_nums[i] = (line_nums[i] / 10) * 10 + nums[0].0;
        }
        println!("final num {}", line_nums[i]);

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

    let mut nums: Vec<(i32, i32)> = Vec::new();

    let s = s.to_lowercase();
    for (i, num) in NUMBERS.iter().enumerate() {
        let find = if left {s.find(num)} else {s.rfind(num)};
        let find = match find {
            None => continue,
            Some(num) => num,
        };

        nums.push((i as i32 + 1 , find as i32));
    };

    nums
}