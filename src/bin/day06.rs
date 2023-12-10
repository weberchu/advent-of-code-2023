fn main() {
    let input = include_str!("./input/day06.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut product = 1;

    let lines = input.lines().collect::<Vec<&str>>();
    let times = split_input_line(lines[0]);
    let records = split_input_line(lines[1]);

    for race in 0..times.len() {
        let ways = winning_ways(times[race], records[race]);
        product *= ways;
    }

    println!("Part 1: {product}");
}

fn winning_ways(time: u64, record: u64) -> u64 {
    for press_time in 1..time {
        let distance = press_time * (time - press_time);
        if distance > record {
            let winning_ways = ((time + 1) / 2 - press_time) * 2 + (time + 1) % 2;
            return winning_ways
        }
    }

    0
}

fn split_input_line(line: &str) -> Vec<u64> {
    line[line.find(":").unwrap() + 1..]
        .split_whitespace()
        .map(|s: &str| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn part2(input: &str) {
    let lines = input.lines().collect::<Vec<&str>>();
    let time = lines[0][lines[0].find(":").unwrap() + 1..].replace(" ", "").parse::<u64>().unwrap();
    let record = lines[1][lines[1].find(":").unwrap() + 1..].replace(" ", "").parse::<u64>().unwrap();
    let ways = winning_ways(time, record);

    println!("Part 2: {ways}");
}
