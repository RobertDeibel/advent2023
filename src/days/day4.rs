pub fn run(argv: Vec<String>) -> String {
    let lines = super::read_lines(&argv[0]);
    let mut game_sum = 0;
    let mut all_counts = vec![1; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        //  split game string and extract game number
        let (game, card) = extract_game(line);
        super::debug_msg(&format!("game {game}"), &argv);

        let split: Vec<&str> = card.split('|').collect();
        let winning_numbers = numbers_by_seperator(split[0], ' ');
        let game_numbers = numbers_by_seperator(split[1], ' ');

        let mut winnings = 0;
        let mut count = 0;
        for number in winning_numbers {
            if game_numbers.contains(&number) {
                winnings = match winnings {
                    0 => 1,
                    _ => winnings << 1,
                };
                count += 1;
            }
        }

        for won in 1..=count {
            if i + won >= lines.len() {
                break;
            }
            all_counts[i + won] += all_counts[i];
        }

        game_sum += winnings;

    }
    let mut card_counts = 0;
    for count in all_counts {
        card_counts += count;
    }

    let result1 = format!("{}", game_sum);
    let result2 = format!("{}", card_counts);

    super::make_output(&[result1, result2])
}

pub fn extract_game(line: &String) -> (i32, String) {
    //  split game string and extract game number
    let split: Vec<&str> = line.split(':').collect();
    let game = super::day1::nums_in_str(split[0]).0;
    let game = super::day2::reduce_to_num(game);
    (game, split[1].to_owned())
}

pub fn numbers_by_seperator(line: &str, sep: char) -> Vec<i32> {
    line
        .trim()
        .split(sep)
        .filter(|&num| num.len() > 0)
        .map(|num| num.parse::<i32>().unwrap())
        .collect()

}