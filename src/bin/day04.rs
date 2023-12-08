use std::collections::HashSet;
use regex::Regex;

fn main() {
    let input = include_str!("./input/day04.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut sum = 0;
    let regex = Regex::new(r"Card +(\d+):([^|]+)\|([^|]+)").unwrap();

    for line in input.lines() {
        let captures = regex.captures(line).unwrap();
        let winning_numbers = captures[2].split(" ").filter(|&s| s.len() > 0);
        let card_numbers = captures[3].split(" ").filter(|&s| s.len() > 0);

        let winning_set: HashSet<&str> = winning_numbers.collect();
        let winning_count = card_numbers.filter(|n| winning_set.contains(n)).count();

        if winning_count > 0 {
            let points = 2_u32.pow(winning_count as u32 - 1);
            sum += points;
        }
    }

    println!("Part 1: {sum}");
}

fn part2(input: &str) {
    let regex = Regex::new(r"Card +(\d+):([^|]+)\|([^|]+)").unwrap();

    let lines: Vec<&str> = input.lines().collect();
    let mut card_count = vec![1; lines.len()];

    for line in lines {
        let captures = regex.captures(line).unwrap();
        let card_number: usize = captures[1].parse().unwrap();
        let winning_numbers = captures[2].split(" ").filter(|&s| s.len() > 0);
        let card_numbers = captures[3].split(" ").filter(|&s| s.len() > 0);

        let winning_set: HashSet<&str> = winning_numbers.collect();
        let winning_count = card_numbers.filter(|n| winning_set.contains(n)).count();

        for x in 0..winning_count {
            card_count[card_number + x] = card_count[card_number + x] + card_count[card_number - 1];
        }
    }

    let total_card_count: i32 = card_count.iter().sum();

    println!("Part 2: {total_card_count}");
}
