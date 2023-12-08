use std::ops::Range;
use regex::Regex;

fn main() {
    let input = include_str!("./input/day05.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let lines: Vec<&str> = input.lines().collect();

    let mut current_cat_numbers = lines[0]
        .split(" ")
        .filter(|s| s.parse::<u64>().is_ok())
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let regex = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();

    let mut map_to = "seed";
    let mut current_map: Vec<(Range<u64>, Range<u64>)> = Vec::new();
    for i in 2..lines.len() {
        let line = lines[i];

        if line.is_empty() {
            current_cat_numbers = map_cat(&current_cat_numbers, &current_map);
            current_map.clear();
            continue;
        }

        match regex.captures(line) {
            Some(captures) => {
                let new_map_from = captures.get(1).unwrap().as_str();
                let new_map_to = captures.get(2).unwrap().as_str();
                assert_eq!(map_to, new_map_from);
                map_to = new_map_to;
            }
            None => {
                let split: Vec<&str> = line.split(" ").collect();
                let destination_range_start = split[0].parse::<u64>().unwrap();
                let source_range_start = split[1].parse::<u64>().unwrap();
                let range_length = split[2].parse::<u64>().unwrap();
                current_map.push((
                    source_range_start..source_range_start + range_length,
                    destination_range_start..destination_range_start + range_length
                ));
            }
        }
    }

    assert_eq!(map_to, "location");
    current_cat_numbers = map_cat(&current_cat_numbers, &current_map);

    let min_location = current_cat_numbers.iter().min().unwrap();

    println!("Part 1: {min_location}");
}

fn map_cat(current_cat_numbers: &Vec<u64>, cat_map: &Vec<(Range<u64>, Range<u64>)>) -> Vec<u64> {
    current_cat_numbers.iter()
        .map(|&n| {
            match cat_map.iter().find(|map| map.0.contains(&n)) {
                Some(&ref map) => {
                    map.1.start + (n - map.0.start)
                }
                None => {
                    n
                }
            }
        }).collect()
}

fn part2(input: &str) {
    let mut sum = 0;

    println!("Part 2: {sum}");
}
