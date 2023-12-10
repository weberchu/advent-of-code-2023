use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = include_str!("./input/day08.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut lines = input.lines();
    let instruction = lines.next().unwrap();
    let mut nodes = HashMap::new();

    lines.next();

    let node_regex = Regex::new(r"(...) = \((...), (...)\)").unwrap();
    for line in lines {
        let captures = node_regex.captures(line).unwrap();
        nodes.insert(
            captures.get(1).unwrap().as_str(),
            (captures.get(2).unwrap().as_str(), captures.get(3).unwrap().as_str()),
        );
    }

    let mut current_location = "AAA";
    let mut step_count = 0;
    while current_location != "ZZZ" {
        for command in instruction.chars() {
            let current_node = nodes.get(current_location).unwrap();
            current_location = if command == 'L' {
                current_node.0
            } else {
                current_node.1
            };
            step_count += 1;
        }
    }

    println!("Part 1: {step_count}");
}

fn part2(input: &str) {
    let mut lines = input.lines();
    let instruction = lines.next().unwrap();
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    lines.next();

    let node_regex = Regex::new(r"(...) = \((...), (...)\)").unwrap();
    for line in lines {
        let captures = node_regex.captures(line).unwrap();
        nodes.insert(
            captures.get(1).unwrap().as_str(),
            (captures.get(2).unwrap().as_str(), captures.get(3).unwrap().as_str()),
        );
    }

    let current_locations: Vec<&str> = nodes.iter().clone()
        .map(|k| *(k.0))
        .filter(|node| node.ends_with("A"))
        .collect();

    let mut all_step_counts = Vec::new();

    for location in current_locations {
        let mut step_count: i64 = 0;
        let mut current_location = location;
        while !current_location.ends_with("Z") {
            for command in instruction.chars() {
                let current_node = nodes.get(current_location).unwrap();
                current_location = if command == 'L' {
                    current_node.0
                } else {
                    current_node.1
                };
                step_count += 1;
            }
        }

        all_step_counts.push(step_count);
    }

    let lcm_step_count = all_step_counts.iter().map(|s| *s).reduce(|s1, s2| lcm(s1, s2)).unwrap();
    println!("Part 2: {lcm_step_count}");
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

