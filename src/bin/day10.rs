use std::collections::HashMap;

fn main() {
    let input = include_str!("./input/day10.txt");
    part1(input);
    part2(input);
}

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part1(input: &str) {
    let possible_up: HashMap<char, Direction> = HashMap::from([
        ('S', Direction::Up),
        ('|', Direction::Up),
        ('F', Direction::Right),
        ('7', Direction::Left)
    ]);
    let possible_down: HashMap<char, Direction> = HashMap::from([
        ('S', Direction::Down),
        ('|', Direction::Down),
        ('L', Direction::Right),
        ('J', Direction::Left)
    ]);
    let possible_left: HashMap<char, Direction> = HashMap::from([
        ('S', Direction::Left),
        ('-', Direction::Left),
        ('F', Direction::Down),
        ('L', Direction::Up)
    ]);
    let possible_right: HashMap<char, Direction> = HashMap::from([
        ('S', Direction::Right),
        ('-', Direction::Right),
        ('7', Direction::Down),
        ('J', Direction::Up)
    ]);

    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let map_height = map.len();
    let map_width = map[0].len();
    let starting_point = find_starting_point(&map);
    let mut current_point = starting_point.clone();

    let mut current_pipe = 'S';
    let mut next_direction = &Direction::Up;
    if current_point.0 > 0 && possible_up.contains_key(&map[current_point.0 - 1][current_point.1]) {
        current_point = (current_point.0 - 1, current_point.1);
        current_pipe = map[current_point.0][current_point.1];
        next_direction = &possible_up[&current_pipe];
    } else if current_point.0 < map_height - 1 && possible_down.contains_key(&map[current_point.0 + 1][current_point.1]) {
        current_point = (current_point.0 + 1, current_point.1);
        current_pipe = map[current_point.0][current_point.1];
        next_direction = &possible_down[&current_pipe];
    } else if current_point.1 > 0 && possible_left.contains_key(&map[current_point.0][current_point.1 - 1]) {
        current_point = (current_point.0, current_point.1 - 1);
        current_pipe = map[current_point.0][current_point.1];
        next_direction = &possible_left[&current_pipe];
    } else if current_point.1 < map_width - 1 && possible_right.contains_key(&map[current_point.0][current_point.1 + 1]) {
        current_point = (current_point.0, current_point.1 + 1);
        current_pipe = map[current_point.0][current_point.1];
        next_direction = &possible_right[&current_pipe];
    }

    let mut steps = 1;
    while current_pipe != 'S' {
        match next_direction {
            Direction::Up => {
                current_point = (current_point.0 - 1, current_point.1);
                current_pipe = map[current_point.0][current_point.1];
                next_direction = &possible_up[&current_pipe];
            },
            Direction::Down => {
                current_point = (current_point.0 + 1, current_point.1);
                current_pipe = map[current_point.0][current_point.1];
                next_direction = &possible_down[&current_pipe];
            },
            Direction::Left => {
                current_point = (current_point.0, current_point.1 - 1);
                current_pipe = map[current_point.0][current_point.1];
                next_direction = &possible_left[&current_pipe];
            },
            Direction::Right => {
                current_point = (current_point.0, current_point.1 + 1);
                current_pipe = map[current_point.0][current_point.1];
                next_direction = &possible_right[&current_pipe];
            }
        }
        steps += 1;

        println!("{current_pipe} {current_point:?} {next_direction:?}");
    }

    let half_steps = steps / 2;
    println!("Part 1: {half_steps}");
}

fn part2(input: &str) {
    let mut sum = 0;

    for line in input.lines() {
        sum += line.len();
    }

    println!("Part 2: {sum}");
}

fn find_starting_point(map: &Vec<Vec<char>>) -> (usize, usize) {
    for (row_number, row) in map.iter().enumerate() {
        match row.iter().position(|&c| c == 'S') {
            Some(index) => { return (row_number, index); }
            None => (),
        }
    }

    panic!("starting point not found");
}