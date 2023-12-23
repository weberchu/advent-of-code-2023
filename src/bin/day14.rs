fn main() {
    let input = include_str!("./input/day14.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect();
    let load = north_load_after_north_tilt(&map);

    println!("Part 1: {load}");
}

fn north_load_after_north_tilt(map: &Vec<Vec<char>>) -> usize {
    let mut load = 0;
    let map_height = map.len();
    for c in 0..map[0].len() {
        let mut top_empty_row = 0;
        for r in 0..map_height {
            match map[r][c] {
                'O' => {
                    load += map_height - top_empty_row;
                    top_empty_row += 1;
                }
                '#' => {
                    top_empty_row = r + 1;
                }
                _ => {}
            }
        }
    }

    load
}

fn part2(input: &str) {
    let total_cycle = 1000000000;
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect();

    let mut map_stack: Vec<Vec<Vec<char>>> = Vec::new();

    for c in 0..total_cycle {
        cycle(&mut map);
        for (i, prev_map) in map_stack.iter().enumerate() {
            if prev_map == &map {
                let cycle_before_loop = i;
                let loop_length = c - i;
                let remainder_in_loop = (total_cycle - cycle_before_loop) % loop_length;
                let final_index = cycle_before_loop - 1 + remainder_in_loop;
                let load = north_load(&map_stack[final_index]);
                println!("Part 2: {load}");

                return;
            }
        }
        map_stack.push(map.clone());
    }

    panic!("loop not found");
}

fn north_load(map: &Vec<Vec<char>>) -> usize {
    let mut load = 0;

    let map_height = map.len();
    for r in 0..map_height {
        for c in 0..map[r].len() {
            if map[r][c] == 'O' {
                load += map_height - r;
            }
        }
    }

    load
}

const DIRECTIONS: [(i32, i32); 4] = [
    (-1, 0),
    (0, -1),
    (1, 0),
    (0, 1)
];

fn cycle(map: &mut Vec<Vec<char>>) {
    for direction in DIRECTIONS {
        if direction == (1, 0) || direction == (0, 1) {
            for r in (0..map.len()).rev() {
                for c in (0..map[r].len()).rev() {
                    move_rock(map, r, c, direction);
                }
            }
        } else {
            for r in 0..map.len() {
                for c in 0..map[r].len() {
                    move_rock(map, r, c, direction);
                }
            }
        }
    }
}

fn move_rock(map: &mut Vec<Vec<char>>, r: usize, c: usize, direction: (i32, i32)) {
    if map[r][c] != 'O' {
        return;
    }


    let map_height = map.len() as i32;
    let map_width = map[0].len() as i32;

    let mut to_slot = (r as i32, c as i32);
    loop {
        let potential_slot = (
            to_slot.0 + direction.0,
            to_slot.1 + direction.1
        );

        if potential_slot.0 < 0 || potential_slot.0 >= map_height ||
            potential_slot.1 < 0 || potential_slot.1 >= map_width ||
            map[potential_slot.0 as usize][potential_slot.1 as usize] != '.' {
            break;
        }

        to_slot = potential_slot;
    }

    if (to_slot.0 as usize) != r || (to_slot.1 as usize) != c {
        map[r][c] = '.';
        map[to_slot.0 as usize][to_slot.1 as usize] = 'O';
    }
}
