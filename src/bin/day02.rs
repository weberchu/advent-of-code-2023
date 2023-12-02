use std::cmp::max;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = include_str!("./input/day02.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut sum = 0;

    let regex = Regex::new(r"Game (\d+):(.+)").unwrap();

    for line in input.lines() {
        let captures = regex.captures(line).unwrap();
        let game_number = captures.get(1).unwrap().as_str();
        let turns = captures.get(2).unwrap().as_str();
        let mut is_possible = true;
        for turn in turns.split(";") {
            for cube in turn.split(",") {
                let split = cube.trim().split(" ").collect::<Vec<&str>>();

                let count: i32 = split[0].parse().unwrap();
                let color = split[1];

                if color == "red" {
                    if count > 12 {
                        is_possible = false;
                        break;
                    }
                } else if color == "green" {
                    if count > 13 {
                        is_possible = false;
                        break;
                    }
                } else if color == "blue" {
                    if count > 14 {
                        is_possible = false;
                        break;
                    }
                }
            }

            if !is_possible {
                break;
            }
        }

        if is_possible {
            sum += game_number.parse::<i32>().unwrap();
        }
    }

    println!("Part 1: {sum}");
}

fn part2(input: &str) {
    // let mut sum = 0;
    let mut sum = 0;

    let regex = Regex::new(r"Game (\d+):(.+)").unwrap();

    for line in input.lines() {
        let captures = regex.captures(line).unwrap();
        let turns = captures.get(2).unwrap().as_str();

        let mut cube_count: HashMap<&str, i32> = HashMap::new();

        for turn in turns.split(";") {
            for cube in turn.split(",") {
                let split = cube.trim().split(" ").collect::<Vec<&str>>();

                let count: i32 = split[0].parse().unwrap();
                let color = split[1];

                if cube_count.contains_key(color) {
                    cube_count.insert(color, max(cube_count[color], count));
                } else {
                    cube_count.insert(color, count);
                }
            }
        }

        let values = cube_count.values().into_iter();
        let power: i32 = values.product();

        sum += power;
    }

    println!("Part 2: {sum}");
}
