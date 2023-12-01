use regex::Regex;

fn main() {
    let input = include_str!("./input/day01.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut sum = 0;

    for line in input.lines() {
        let first_digit = line.chars()
            .find(|c| (&'0'..=&'9').contains(&c))
            .unwrap()
            .to_digit(10)
            .unwrap();
        let last_digit = line.chars()
            .rfind(|c| (&'0'..=&'9').contains(&c))
            .unwrap()
            .to_digit(10)
            .unwrap();

        let number = first_digit * 10 + last_digit;

        sum += number;
    }

    println!("Part 1: {sum}");
}

fn part2(input: &str) {
    let mut sum = 0;

    let all_number_words = "one|two|three|four|five|six|seven|eight|nine";
    let all_number_words_reversed = all_number_words.chars().rev().collect::<String>();

    let first_regex = Regex::new(&format!("{all_number_words}|\\d")).unwrap();
    let last_regex = Regex::new(&format!("{all_number_words_reversed}|\\d")).unwrap();

    for line in input.lines() {
        let first_matched = first_regex.find(line).unwrap().as_str();
        let first_digit = convert_to_number(first_matched);

        let reversed_line = line.chars().rev().collect::<String>();
        let last_matched = last_regex.find(&reversed_line).unwrap().as_str();
        let last_digit = convert_to_number(last_matched);

        let number = first_digit * 10 + last_digit;

        sum += number;
    }

    println!("Part 2: {sum}");
}

fn convert_to_number(s: &str) -> u32 {
    match s {
        "one" => 1,
        "eno" => 1,
        "two" => 2,
        "owt" => 2,
        "three" => 3,
        "eerht" => 3,
        "four" => 4,
        "ruof" => 4,
        "five" => 5,
        "evif" => 5,
        "six" => 6,
        "xis" => 6,
        "seven" => 7,
        "neves" => 7,
        "eight" => 8,
        "thgie" => 8,
        "nine" => 9,
        "enin" => 9,
        _ => s.parse().unwrap(),
    }
}
