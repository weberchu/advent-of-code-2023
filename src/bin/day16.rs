use std::collections::HashSet;

fn main() {
    let input = include_str!("./input/day16.txt");
    part1(input);
    part2(input);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Beam {
    direction: Direction,
    position: (usize, usize),
}

fn part1(input: &str) {
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        map.push(line.chars().collect());
    }

    let beam = Beam {
        direction: Direction::Right,
        position: (0, 0),
    };

    let visited_count = visited_count(&map, beam);

    println!("Part 1: {visited_count}");
}

fn part2(input: &str) {
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        map.push(line.chars().collect());
    }

    let mut max_visited_count = 0;

    let mut starting_beams = Vec::new();

    for i in 0..map.len() {
        starting_beams.push(Beam {
            direction: Direction::Right,
            position: (i, 0),
        });
        starting_beams.push(Beam {
            direction: Direction::Left,
            position: (i, map[0].len() - 1),
        })
    }
    for i in 0..map[0].len() {
        starting_beams.push(Beam {
            direction: Direction::Down,
            position: (0, i),
        });
        starting_beams.push(Beam {
            direction: Direction::Up,
            position: (map.len() - 1, i),
        })
    }

    for beam in starting_beams {
        let visited_count = visited_count(&map, beam);
        if visited_count > max_visited_count {
            max_visited_count = visited_count;
        }
    }

    println!("Part 2: {max_visited_count}");
}

fn visited_count(map: &Vec<Vec<char>>, initial_beam: Beam) -> i32 {
    let mut visited_map: Vec<Vec<HashSet<Direction>>> = Vec::new();

    for _ in 0..map.len() {
        let mut visited_line = Vec::new();
        for _ in 0..map[0].len() {
            visited_line.push(HashSet::new());
        }
        visited_map.push(visited_line);
    }

    let height = map.len();
    let width = map[0].len();

    let mut beams = vec![initial_beam];

    while !beams.is_empty() {
        let beam = beams.pop().unwrap();
        let tile = map[beam.position.0][beam.position.1];

        let visited_tile = &mut visited_map[beam.position.0][beam.position.1];
        if visited_tile.contains(&beam.direction) {
            continue;
        }

        match tile {
            '.' => {
                match beam.direction {
                    Direction::Up => {
                        if beam.position.0 > 0 {
                            beams.push(Beam {
                                direction: Direction::Up,
                                position: (beam.position.0 - 1, beam.position.1),
                            });
                        }
                    }
                    Direction::Down => {
                        if beam.position.0 < height - 1 {
                            beams.push(Beam {
                                direction: Direction::Down,
                                position: (beam.position.0 + 1, beam.position.1),
                            });
                        }
                    }
                    Direction::Left => {
                        if beam.position.1 > 0 {
                            beams.push(Beam {
                                direction: Direction::Left,
                                position: (beam.position.0, beam.position.1 - 1),
                            });
                        }
                    }
                    Direction::Right => {
                        if beam.position.1 < width - 1 {
                            beams.push(Beam {
                                direction: Direction::Right,
                                position: (beam.position.0, beam.position.1 + 1),
                            });
                        }
                    }
                };
            }
            '\\' => {
                match beam.direction {
                    Direction::Up => {
                        if beam.position.1 > 0 {
                            beams.push(Beam {
                                direction: Direction::Left,
                                position: (beam.position.0, beam.position.1 - 1),
                            });
                        }
                    }
                    Direction::Down => {
                        if beam.position.1 < width - 1 {
                            beams.push(Beam {
                                direction: Direction::Right,
                                position: (beam.position.0, beam.position.1 + 1),
                            });
                        }
                    }
                    Direction::Left => {
                        if beam.position.0 > 0 {
                            beams.push(Beam {
                                direction: Direction::Up,
                                position: (beam.position.0 - 1, beam.position.1),
                            });
                        }
                    }
                    Direction::Right => {
                        if beam.position.0 < height - 1 {
                            beams.push(Beam {
                                direction: Direction::Down,
                                position: (beam.position.0 + 1, beam.position.1),
                            });
                        }
                    }
                }
            }
            '/' => {
                match beam.direction {
                    Direction::Up => {
                        if beam.position.1 < width - 1 {
                            beams.push(Beam {
                                direction: Direction::Right,
                                position: (beam.position.0, beam.position.1 + 1),
                            });
                        }
                    }
                    Direction::Down => {
                        if beam.position.1 > 0 {
                            beams.push(Beam {
                                direction: Direction::Left,
                                position: (beam.position.0, beam.position.1 - 1),
                            });
                        }
                    }
                    Direction::Left => {
                        if beam.position.0 < height - 1 {
                            beams.push(Beam {
                                direction: Direction::Down,
                                position: (beam.position.0 + 1, beam.position.1),
                            });
                        }
                    }
                    Direction::Right => {
                        if beam.position.0 > 0 {
                            beams.push(Beam {
                                direction: Direction::Up,
                                position: (beam.position.0 - 1, beam.position.1),
                            });
                        }
                    }
                }
            }
            '|' => {
                match beam.direction {
                    Direction::Left | Direction::Right => {
                        if beam.position.0 > 0 {
                            beams.push(Beam {
                                direction: Direction::Up,
                                position: (beam.position.0 - 1, beam.position.1),
                            });
                        }
                        if beam.position.0 < height - 1 {
                            beams.push(Beam {
                                direction: Direction::Down,
                                position: (beam.position.0 + 1, beam.position.1),
                            });
                        }
                    }
                    Direction::Up => {
                        if beam.position.0 > 0 {
                            beams.push(Beam {
                                direction: Direction::Up,
                                position: (beam.position.0 - 1, beam.position.1),
                            });
                        }
                    }
                    Direction::Down => {
                        if beam.position.0 < height - 1 {
                            beams.push(Beam {
                                direction: Direction::Down,
                                position: (beam.position.0 + 1, beam.position.1),
                            });
                        }
                    }
                }
            }
            '-' => {
                match beam.direction {
                    Direction::Up | Direction::Down => {
                        if beam.position.1 > 0 {
                            beams.push(Beam {
                                direction: Direction::Left,
                                position: (beam.position.0, beam.position.1 - 1),
                            });
                        }
                        if beam.position.1 < width - 1 {
                            beams.push(Beam {
                                direction: Direction::Right,
                                position: (beam.position.0, beam.position.1 + 1),
                            });
                        }
                    }
                    Direction::Left => {
                        if beam.position.1 > 0 {
                            beams.push(Beam {
                                direction: Direction::Left,
                                position: (beam.position.0, beam.position.1 - 1),
                            });
                        }
                    }
                    Direction::Right => {
                        if beam.position.1 < width - 1 {
                            beams.push(Beam {
                                direction: Direction::Right,
                                position: (beam.position.0, beam.position.1 + 1),
                            });
                        }
                    }
                }
            }
            _ => {
                panic!("unknown tile {tile}");
            }
        }

        visited_tile.insert(beam.direction);
    }

    let mut visited_count = 0;

    for r in visited_map.iter() {
        for c in r.iter() {
            if !c.is_empty() {
                visited_count += 1;
            }
        }
    }
    visited_count
}
