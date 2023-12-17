use std::collections::HashSet;

fn main() {
    let input = include_str!("./input/day12.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut sum = 0;

    for line in input.lines() {
        println!("---- {}", line);
        let split: Vec<&str> = line.split(" ").collect();
        let visual_record = split[0];
        let number_record = split[1];

        let contiguous_damaged: Vec<usize> = number_record.split(",").map(|s| s.parse::<usize>().unwrap()).collect();

        // let possible_arrangements = possible_arrangements(&visual_record.chars().collect(), &contiguous_damaged);
        let possible_arrangements = possible_arrangements_v2(&visual_record.chars().collect(), &contiguous_damaged);
        // println!("possible_arrangements = {possible_arrangements}");
        // println!("possible_arrangements2 = {possible_arrangements2}");
        // if possible_arrangements != possible_arrangements2 {
        //     panic!("not equal");
        // }

        sum += possible_arrangements;
    }

    println!("Part 1: {sum}");
}

fn part2(input: &str) {
    let mut sum = 0;

    for line in input.lines() {
        println!("----");
        let split: Vec<&str> = line.split(" ").collect();

        let visual_record_str = split[0];
        let mut visual_record = Vec::new();
        for i in 0..5 {
            if i > 0 {
                visual_record.push('?');
            }
            for c in visual_record_str.chars() {
                visual_record.push(c);
            }
        }
        let number_record = split[1];

        let contiguous_damaged_single: Vec<usize> = number_record.split(",").map(|s| s.parse::<usize>().unwrap()).collect();
        let mut contiguous_damaged = Vec::new();
        for _ in 0..5 {
            for &c in contiguous_damaged_single.iter() {
                contiguous_damaged.push(c);
            }
        }

        println!("visual_record = {:?}", visual_record.iter().collect::<String>());
        println!("contiguous_damaged = {contiguous_damaged:?}");
        let possible_arrangements = possible_arrangements_v2(&visual_record, &contiguous_damaged);
        println!("possible_arrangements = {possible_arrangements}");

        sum += possible_arrangements;
    }

    println!("Part 2: {sum}");
}

fn possible_arrangements_v2(visual_record: &Vec<char>, contiguous_damaged: &Vec<usize>) -> usize {
    let right_most_possible_start = right_most_possible_start(visual_record, contiguous_damaged);
    // println!("visual_record={visual_record:?}, contiguous_damaged={:?}", contiguous_damaged);
    // println!("right_most_possible_start={right_most_possible_start}");

    let mut possible_arrangements = 0;
    let first_damaged_group = contiguous_damaged[0];

    for i in 0..right_most_possible_start + 1 {
        let mut is_possible = true;
        for j in i..i + first_damaged_group {
            if visual_record[j] == '.' {
                is_possible = false;
                break;
            }
        }
        if contiguous_damaged.len() == 1 {
            for j in i + first_damaged_group..visual_record.len() {
                // cannot have more damaged spring after last group
                if  visual_record[j] == '#' {
                    is_possible = false;
                    break;
                }
            }
        } else {
            if i + first_damaged_group < visual_record.len() && visual_record[i + first_damaged_group] == '#' {
                is_possible = false;
            }
        }

        if is_possible {
            if contiguous_damaged.len() == 1 {
                // println!(" p record={:?} finding{:?}@{i} = 1", visual_record.iter().collect::<String>(), contiguous_damaged);
                possible_arrangements += 1;
            } else {
                let remaining_visual_record = Vec::from(visual_record.split_at(i + first_damaged_group + 1).1);
                let remaining_contiguous_damaged = Vec::from(contiguous_damaged.split_first().unwrap().1);

                // println!("-record={:?} finding{:?}@{i} delegating", visual_record.iter().collect::<String>(), contiguous_damaged);
                let p = possible_arrangements_v2(&remaining_visual_record, &remaining_contiguous_damaged);
                // println!(" p record={:?} finding{:?}@{i} = {p}", visual_record.iter().collect::<String>(), contiguous_damaged);
                possible_arrangements += p;
            }
        } else {
            // println!(" p record={:?} finding{:?}@{i} = 0", visual_record.iter().collect::<String>(), contiguous_damaged);
        }

        if visual_record[i] == '#' {
            // cannot skip a damaged spring
            break;
        }
    }

    possible_arrangements
}

fn right_most_possible_start(visual_record: &Vec<char>, contiguous_damaged: &Vec<usize>) -> usize {
    let mut right_most_possible_start = visual_record.len() - 1;
    for (d_i, &d) in contiguous_damaged.iter().enumerate().rev() {
        let mut is_current_found = false;
        while !is_current_found && right_most_possible_start >= 0 {
            let mut is_damaged_found = true;
            for j in right_most_possible_start + 1 - d..right_most_possible_start + 1 {
                if visual_record[j] == '.' {
                    right_most_possible_start -= 1;
                    is_damaged_found = false;
                    break;
                }
            }
            if is_damaged_found {
                is_current_found = true;
            }
        }

        if !is_current_found {
            panic!("impossible arrangement. visual_record={:?}, contiguous_damaged={:?}", visual_record, contiguous_damaged);
        }

        if d_i > 0 {
            right_most_possible_start -= d + 1;
        }
    }

    right_most_possible_start + 1 - contiguous_damaged[0]
}

fn possible_arrangements(visual_record: &Vec<char>, contiguous_damaged: &Vec<usize>) -> usize {
    let spring_count = visual_record.len();
    let operational_group_count = contiguous_damaged.len() + 1; // always with a leading and a trailing group
    let operational_count = spring_count + 2 - contiguous_damaged.iter().sum::<usize>();

    let mut padded_visual_record = visual_record.clone();
    padded_visual_record.insert(0, '.');
    padded_visual_record.push('.');

    println!("padded_visual_record={:?}", padded_visual_record.iter().collect::<String>());
    println!("operational_count={operational_count}");
    println!("operational_group_count={operational_group_count}");

    let mut longest_functional = 0;
    let mut current_streak = 0;
    for &spring in padded_visual_record.iter() {
        if spring == '.' || spring == '?' {
            current_streak += 1;
        } else {
            if current_streak > longest_functional {
                longest_functional = current_streak;
            }
            current_streak = 0;
        }
    }
    if current_streak > longest_functional {
        longest_functional = current_streak;
    }
    println!("longest_functional={longest_functional}");

    let mut all_operational_splits = HashSet::new();
    let mut operational_splits_to_process = HashSet::new();

    let mut first_split = Vec::new();

    let a = (operational_count - operational_group_count) / (longest_functional - 1);
    let b = (operational_count - operational_group_count) % (longest_functional - 1);
    for i in 0..operational_group_count {
        first_split.push(
            if i < a {
                longest_functional
            } else if i == a {
                b + 1
            } else {
                1
            }
        );
    }
    println!("first_split={first_split:?}");

    operational_splits_to_process.insert(first_split);

    while !operational_splits_to_process.is_empty() {
        let split = operational_splits_to_process.iter().next().unwrap().clone();
        operational_splits_to_process.remove(&split);
        all_operational_splits.insert(split.clone());

        if all_operational_splits.len() % 10000 == 0 {
            println!("all_operational_splits.len()={}, todo={:?}", all_operational_splits.len(), operational_splits_to_process.len());
        }

        for i in 0..split.len() - 1 {
            if split[i] > 1 && split[i + 1] < longest_functional {
                let mut new_split = split.clone();
                new_split[i] -= 1;
                new_split[i + 1] += 1;
                if !all_operational_splits.contains(&new_split) &&
                    !operational_splits_to_process.contains(&new_split) {
                    // println!("pushing {:?}", new_split);
                    operational_splits_to_process.insert(new_split);
                }
            }
        }
    }

    all_operational_splits.iter()
        .filter(|split| is_possible(&padded_visual_record, contiguous_damaged, split))
        .count()
}

fn is_possible(visual_record: &Vec<char>, contiguous_damaged: &Vec<usize>, contiguous_operational: &Vec<usize>) -> bool {
    // println!("is_possible contiguous_operational={contiguous_operational:?}");
    if contiguous_damaged.len() != contiguous_operational.len() - 1 ||
        visual_record.len() != contiguous_damaged.iter().sum::<usize>() + contiguous_operational.iter().sum::<usize>() {
        eprintln!("visual_record={visual_record:?}");
        eprintln!("contiguous_damaged={contiguous_damaged:?}");
        eprintln!("contiguous_operational={contiguous_operational:?}");
        panic!("is_possible check with invalid input");
    }

    let mut pointer = 0;
    for (i, &operational) in contiguous_operational.iter().enumerate() {
        for j in 0..operational {
            // println!("o:checking {}@{}", visual_record[pointer + j], pointer + j);
            if visual_record[pointer + j] != '.' && visual_record[pointer + j] != '?' {
                return false;
            }
        }

        pointer += operational;

        if i < contiguous_damaged.len() {
            let damaged = contiguous_damaged[i];
            for j in 0..damaged {
                // println!("d:checking {}@{}", visual_record[pointer + j], pointer + j);
                if visual_record[pointer + j] != '#' && visual_record[pointer + j] != '?' {
                    return false;
                }
            }
            pointer += damaged;
        }
    }

    // println!("{contiguous_operational:?} is possible");

    true
}
