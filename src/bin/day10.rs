use std::collections::HashMap;

fn main() {
    let input = include_str!("./input/day10.txt");
    part1_and_2(input);
}

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Debug)]
enum Turn {
    No,
    Left,
    Right
}

fn part1_and_2(input: &str) {
    let possible_up = HashMap::from([
        ('S', (Direction::Up, Turn::No)),
        ('|', (Direction::Up, Turn::No)),
        ('F', (Direction::Right, Turn::Right)),
        ('7', (Direction::Left, Turn::Left))
    ]);
    let possible_down = HashMap::from([
        ('S', (Direction::Down, Turn::No)),
        ('|', (Direction::Down, Turn::No)),
        ('L', (Direction::Right, Turn::Left)),
        ('J', (Direction::Left, Turn::Right))
    ]);
    let possible_left = HashMap::from([
        ('S', (Direction::Left, Turn::No)),
        ('-', (Direction::Left, Turn::No)),
        ('F', (Direction::Down, Turn::Left)),
        ('L', (Direction::Up, Turn::Right))
    ]);
    let possible_right = HashMap::from([
        ('S', (Direction::Right, Turn::No)),
        ('-', (Direction::Right, Turn::No)),
        ('7', (Direction::Down, Turn::Right)),
        ('J', (Direction::Up, Turn::Left))
    ]);

    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let map_height = map.len();
    let map_width = map[0].len();
    let starting_point = find_starting_point(&map);
    let mut current_point = starting_point.clone();

    let mut current_pipe = 'S';
    let mut next_direction = &Direction::Up;
    let mut travelled_locations = Vec::new();
    let mut travelled_directions = Vec::new();
    let mut turns_made = Vec::from([&Turn::No]);
    if current_point.0 > 0 && possible_up.contains_key(&map[current_point.0 - 1][current_point.1]) {
        current_point = (current_point.0 - 1, current_point.1);
        current_pipe = map[current_point.0][current_point.1];
        travelled_directions.push(&Direction::Up);
        next_direction = &possible_up[&current_pipe].0;
        turns_made.push(&possible_up[&current_pipe].1);
    } else if current_point.0 < map_height - 1 && possible_down.contains_key(&map[current_point.0 + 1][current_point.1]) {
        current_point = (current_point.0 + 1, current_point.1);
        current_pipe = map[current_point.0][current_point.1];
        travelled_directions.push(&Direction::Down);
        next_direction = &possible_down[&current_pipe].0;
        turns_made.push(&possible_down[&current_pipe].1);
    } else if current_point.1 > 0 && possible_left.contains_key(&map[current_point.0][current_point.1 - 1]) {
        current_point = (current_point.0, current_point.1 - 1);
        current_pipe = map[current_point.0][current_point.1];
        travelled_directions.push(&Direction::Left);
        next_direction = &possible_left[&current_pipe].0;
        turns_made.push(&possible_left[&current_pipe].1);
    } else if current_point.1 < map_width - 1 && possible_right.contains_key(&map[current_point.0][current_point.1 + 1]) {
        current_point = (current_point.0, current_point.1 + 1);
        current_pipe = map[current_point.0][current_point.1];
        travelled_directions.push(&Direction::Right);
        next_direction = &possible_right[&current_pipe].0;
        turns_made.push(&possible_right[&current_pipe].1);
    }
    travelled_locations.push(current_point);
    travelled_directions.push(next_direction);

    let mut steps = 1;
    while current_pipe != 'S' {
        match next_direction {
            Direction::Up => {
                current_point = (current_point.0 - 1, current_point.1);
                current_pipe = map[current_point.0][current_point.1];
                next_direction = &possible_up[&current_pipe].0;
                turns_made.push(&possible_up[&current_pipe].1);
            },
            Direction::Down => {
                current_point = (current_point.0 + 1, current_point.1);
                current_pipe = map[current_point.0][current_point.1];
                next_direction = &possible_down[&current_pipe].0;
                turns_made.push(&possible_down[&current_pipe].1);
            },
            Direction::Left => {
                current_point = (current_point.0, current_point.1 - 1);
                current_pipe = map[current_point.0][current_point.1];
                next_direction = &possible_left[&current_pipe].0;
                turns_made.push(&possible_left[&current_pipe].1);
            },
            Direction::Right => {
                current_point = (current_point.0, current_point.1 + 1);
                current_pipe = map[current_point.0][current_point.1];
                next_direction = &possible_right[&current_pipe].0;
                turns_made.push(&possible_right[&current_pipe].1);
            }
        }
        travelled_locations.push(current_point);
        travelled_directions.push(next_direction);
        steps += 1;
    }

    let half_steps = steps / 2;
    println!("Part 1: {half_steps}");

    let total_left_turns = turns_made.iter().filter(|&&t| *t == Turn::Left).count();
    let total_right_turns = turns_made.iter().filter(|&&t| *t == Turn::Right).count();
    let is_clockwise = total_right_turns > total_left_turns;
    let mut inside_directions = match travelled_directions[0] {
        Direction::Down => {
            if is_clockwise {
                Vec::from([Direction::Left])
            } else {
                Vec::from([Direction::Right])
            }
        },
        Direction::Up => {
            if is_clockwise {
                Vec::from([Direction::Right])
            } else {
                Vec::from([Direction::Left])
            }
        },
        Direction::Left => {
            if is_clockwise {
                Vec::from([Direction::Up])
            } else {
                Vec::from([Direction::Down])
            }
        },
        Direction::Right => {
            if is_clockwise {
                Vec::from([Direction::Down])
            } else {
                Vec::from([Direction::Up])
            }
        }
    };

    // . means unknown yet, O means outside, I means inside
    let mut visited_map: Vec<Vec<char>> = Vec::new();
    for _i in 0..map_height {
        let mut row = Vec::new();
        for _j in 0..map_width {
            row.push('.');
        }
        visited_map.push(row);
    }

    for &(r, c) in travelled_locations.iter() {
        // mark pipe as outside
        visited_map[r][c] = 'O';
    }

    for (i, &(r, c)) in travelled_locations.iter().enumerate() {
        let turn = turns_made[i+1];
        let travelled_direction = travelled_directions[i];
        inside_directions = match inside_directions[0] {
            Direction::Up => match travelled_direction {
                Direction::Left => {
                    match turn {
                        Turn::Left => Vec::from([Direction::Left, Direction::Up]),
                        Turn::Right => Vec::from([Direction::Right]),
                        Turn::No => Vec::from([Direction::Up])
                    }
                },
                Direction::Right => {
                    match turn {
                        Turn::Left => Vec::from([Direction::Left]),
                        Turn::Right => Vec::from([Direction::Right, Direction::Up]),
                        Turn::No => Vec::from([Direction::Up])
                    }
                },
                _ => panic!("Doesn't make sense")
            },
            Direction::Down => match travelled_direction {
                Direction::Left => {
                    match turn {
                        Turn::Left => Vec::from([Direction::Right]),
                        Turn::Right => Vec::from([Direction::Left, Direction::Down]),
                        Turn::No => Vec::from([Direction::Down])
                    }
                },
                Direction::Right => {
                    match turn {
                        Turn::Left => Vec::from([Direction::Right, Direction::Down]),
                        Turn::Right => Vec::from([Direction::Left]),
                        Turn::No => Vec::from([Direction::Down])
                    }
                },
                _ => panic!("Doesn't make sense")
            },
            Direction::Left => match travelled_direction {
                Direction::Up => {
                    match turn {
                        Turn::Left => Vec::from([Direction::Down]),
                        Turn::Right => Vec::from([Direction::Up, Direction::Left]),
                        Turn::No => Vec::from([Direction::Left])
                    }
                },
                Direction::Down => {
                    match turn {
                        Turn::Left => Vec::from([Direction::Down, Direction::Left]),
                        Turn::Right => Vec::from([Direction::Up]),
                        Turn::No => Vec::from([Direction::Left])
                    }
                },
                _ => panic!("Doesn't make sense")
            },
            Direction::Right => match travelled_direction {
                Direction::Up => {
                    match turn {
                        Turn::Left => Vec::from([Direction::Up, Direction::Right]),
                        Turn::Right => Vec::from([Direction::Down]),
                        Turn::No => Vec::from([Direction::Right])
                    }
                },
                Direction::Down => {
                    match turn {
                        Turn::Left => Vec::from([Direction::Up]),
                        Turn::Right => Vec::from([Direction::Down, Direction::Right]),
                        Turn::No => Vec::from([Direction::Right])
                    }
                },
                _ => panic!("Doesn't make sense")
            }
        };

        for inside_direction in inside_directions.iter() {
            let inside_location = match inside_direction {
                Direction::Up => if r > 0 {
                    Some((r - 1, c))
                } else {
                    None
                },
                Direction::Down => if r < map_height - 1 {
                    Some((r + 1, c))
                } else {
                    None
                },
                Direction::Left => if c > 0 {
                    Some((r, c - 1))
                } else {
                    None
                },
                Direction::Right => if c < map_width - 1 {
                    Some((r, c + 1))
                } else {
                    None
                }
            };

            if let Some(inside) = inside_location {
                mark_inside(&mut visited_map, inside);
            }
        }
    }

    for (r, row) in visited_map.iter().enumerate() {
        for (c, &col) in row.iter().enumerate() {
            print!("{}", if col == 'O' {
                map[r][c]
            } else if col == '.' {
                ' '
            } else if col == 'I' {
                'X'
            } else {
                visited_map[r][c]
            });
        }
        println!();
    }

    let inside_count: usize = visited_map.iter()
        .map(|r|
            r.iter().filter(|&&c| c == 'I').count()
        ).sum();

    println!("Part 2: {inside_count}");
}

fn mark_inside(map: &mut Vec<Vec<char>>, location: (usize, usize)) {
    let mut location_stack = Vec::from([location]);

    while !location_stack.is_empty() {
        let next_location = location_stack.pop().unwrap();
        if map[next_location.0][next_location.1] == '.' {
            map[next_location.0][next_location.1] = 'I';
            // don't worry about out of map because the inside must be surrounded by pipe
            location_stack.push((next_location.0 + 1, next_location.1));
            location_stack.push((next_location.0 - 1, next_location.1));
            location_stack.push((next_location.0, next_location.1 + 1));
            location_stack.push((next_location.0, next_location.1 - 1));
        }
    }

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
