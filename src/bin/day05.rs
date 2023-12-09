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
        .filter(|s| s.parse::<i64>().is_ok())
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let regex = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();

    let mut map_to = "seed";
    let mut current_map: Vec<(Range<i64>, Range<i64>)> = Vec::new();
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
                let destination_range_start = split[0].parse::<i64>().unwrap();
                let source_range_start = split[1].parse::<i64>().unwrap();
                let range_length = split[2].parse::<i64>().unwrap();
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

fn map_cat(current_cat_numbers: &Vec<i64>, cat_map: &Vec<(Range<i64>, Range<i64>)>) -> Vec<i64> {
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
    let lines: Vec<&str> = input.lines().collect();

    let cat_inputs = lines[0]
        .split(" ")
        .filter(|s| s.parse::<i64>().is_ok())
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut cat_ranges: Vec<Range<i64>> = Vec::new();

    for x in (0..cat_inputs.len()).step_by(2) {
        cat_ranges.push(cat_inputs[x]..(cat_inputs[x]) + cat_inputs[x + 1]);
    }

    let regex = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();

    let mut map_to = "seed";
    let mut current_map: Vec<(Range<i64>, Range<i64>)> = Vec::new();
    for i in 2..lines.len() {
        let line = lines[i];

        if line.is_empty() {
            cat_ranges = map_cat_2(&cat_ranges, &current_map);
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
                let destination_range_start = split[0].parse::<i64>().unwrap();
                let source_range_start = split[1].parse::<i64>().unwrap();
                let range_length = split[2].parse::<i64>().unwrap();
                current_map.push((
                    source_range_start..source_range_start + range_length,
                    destination_range_start..destination_range_start + range_length
                ));
            }
        }
    }

    assert_eq!(map_to, "location");
    cat_ranges = map_cat_2(&cat_ranges, &current_map);

    let min_location = cat_ranges.iter().map(|c| c.start).min().unwrap();

    println!("Part 2: {min_location}");
}

fn map_cat_2(cat_ranges_in: &Vec<Range<i64>>, current_map: &Vec<(Range<i64>, Range<i64>)>) -> Vec<Range<i64>> {
    let mut new_cat_ranges: Vec<Range<i64>> = Vec::new();
    let mut cat_ranges = cat_ranges_in.clone();

    while !cat_ranges.is_empty() {
        let cat_range = cat_ranges.pop().unwrap();
        let mut has_mapped = false;
        for (source, destination) in current_map {
            let map_offset = destination.start - source.start;

            // map mappable to new_cat_range, split remaining back to cat_range to map later
            if cat_range.start >= source.end || cat_range.end <= source.start {
                // no overlap
            } else if cat_range.start >= source.start && cat_range.end <= source.end {
                // cat is completely within source
                new_cat_ranges.push(cat_range.start + map_offset..cat_range.end + map_offset);
                has_mapped = true;
                break;
            } else if cat_range.start < source.start && cat_range.end > source.end {
                // source is completely within cat
                cat_ranges.push(cat_range.start..source.start);
                new_cat_ranges.push(source.start + map_offset..source.end + map_offset);
                cat_ranges.push(source.end..cat_range.end);
                has_mapped = true;
                break;
            } else if cat_range.start < source.start && cat_range.end <= source.end {
                cat_ranges.push(cat_range.start..source.start);
                new_cat_ranges.push(source.start + map_offset..cat_range.end + map_offset);
                has_mapped = true;
                break;
            } else if cat_range.end > source.end && cat_range.start >= source.start {
                cat_ranges.push(source.end..cat_range.end);
                new_cat_ranges.push(cat_range.start + map_offset..source.end + map_offset);
                has_mapped = true;
                break;
            } else {
                panic!("Should leave anything here {:?} {:?}", cat_range, source);
            }
        }

        if !has_mapped {
            new_cat_ranges.push(cat_range);
        }
    }
    new_cat_ranges
}
