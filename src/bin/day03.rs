fn main() {
    let input = include_str!("./input/day03.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut sum = 0;

    for (line_number, line) in input.lines().enumerate() {
        let mut current_number = 0;
        let mut number_start: usize = 0;
        for (char_number, char) in line.chars().into_iter().enumerate() {
            if is_number(char) {
                if current_number == 0 {
                    number_start = char_number;
                }
                current_number = current_number * 10 + char.to_digit(10).unwrap();

                if char_number + 1 == line.len() || !is_number(line.chars().nth(char_number + 1).unwrap()) {
                    if is_part_number(line_number, number_start, char_number, input) {
                        sum += current_number;
                    }
                }
            } else {
                current_number = 0;
            }
        }
    }

    println!("Part 1: {sum}");
}

fn is_part_number(line_number: usize, number_start: usize, number_end: usize, input: &str) -> bool {
    let check_start = if number_start >= 1 {
        number_start - 1
    } else {
        0
    };
    let check_end = if number_end < input.lines().count() - 1 {
        number_end + 1
    } else {
        number_end
    };

    // line before
    if line_number >= 1 {
        let line_before = input.lines().nth(line_number - 1).unwrap();
        for x in line_before.chars().skip(check_start).take(check_end - check_start + 1) {
            if x != '.' {
                return true;
            }
        }
    }

    // char before
    if check_start < number_start {
        let line = input.lines().nth(line_number).unwrap();
        if line.chars().nth(check_start).unwrap() != '.' {
            return true;
        }
    }

    // char after
    if check_end > number_end {
        let line = input.lines().nth(line_number).unwrap();
        if line.chars().nth(check_end).unwrap() != '.' {
            return true;
        }
    }

    // line after
    if line_number < input.lines().count() - 1 {
        let line_after = input.lines().nth(line_number + 1).unwrap();
        for x in line_after.chars().skip(check_start).take(check_end - check_start + 1) {
            if x != '.' {
                return true;
            }
        }
    }

    false
}

fn is_number(char: char) -> bool {
    char >= '0' && char <= '9'
}

fn part2(input: &str) {
    let mut sum = 0;

    let lines: Vec<&str> = input.lines().collect();
    for (line_number, line) in lines.clone().into_iter().enumerate() {
        for (char_number, char) in line.chars().into_iter().enumerate() {
            if char == '*' {
                // println!("* found at {line_number}-{char_number}");
                // whether is at the edge of the diagram
                let is_left_most = char_number == 0;
                let is_right_most = char_number == line.len();
                let is_top_most = line_number == 0;
                let is_bottom_most = line_number == lines.len();

                let mut part_count = 0;
                let mut gear_ratio: u32 = 1;

                // chars relative to current position
                let top_left_char = if !is_top_most && !is_left_most {
                    lines[line_number - 1].chars().nth(char_number - 1).unwrap()
                } else {
                    '.'
                };
                let top_char = if !is_top_most {
                    lines[line_number - 1].chars().nth(char_number).unwrap()
                } else {
                    '.'
                };
                let top_right_char = if !is_top_most && !is_right_most {
                    lines[line_number - 1].chars().nth(char_number + 1).unwrap()
                } else {
                    '.'
                };
                let left_char = if !is_left_most {
                    lines[line_number].chars().nth(char_number - 1).unwrap()
                } else {
                    '.'
                };
                let right_char = if !is_right_most {
                    lines[line_number].chars().nth(char_number + 1).unwrap()
                } else {
                    '.'
                };
                let bottom_left_char = if !is_bottom_most && !is_left_most {
                    lines[line_number + 1].chars().nth(char_number - 1).unwrap()
                } else {
                    '.'
                };
                let bottom_char = if !is_bottom_most {
                    lines[line_number + 1].chars().nth(char_number).unwrap()
                } else {
                    '.'
                };
                let bottom_right_char = if !is_bottom_most && !is_right_most {
                    lines[line_number + 1].chars().nth(char_number + 1).unwrap()
                } else {
                    '.'
                };

                // count top left
                if is_number(top_left_char) {
                    let chars: Vec<char> = lines[line_number - 1].chars().collect();
                    let mut number_str = scan_number_to_left(char_number - 1, &chars);
                    number_str.append(&mut scan_number_to_right(char_number, &chars));

                    let number = number_str.into_iter().collect::<String>();
                    gear_ratio = gear_ratio * number.parse::<u32>().unwrap();
                    part_count = part_count + 1;
                    // println!("  top left number= {number}");
                }
                // count top
                if !is_number(top_left_char) && is_number(top_char) {
                    let chars: Vec<char> = lines[line_number - 1].chars().collect();
                    let number_str = scan_number_to_right(char_number, &chars);

                    let number = number_str.into_iter().collect::<String>();
                    gear_ratio = gear_ratio * number.parse::<u32>().unwrap();
                    part_count = part_count + 1;
                    // println!("  top number= {number}");
                }
                // count top right
                if !is_number(top_char) && is_number(top_right_char) {
                    if part_count >= 2 {
                        continue;
                    }
                    let chars: Vec<char> = lines[line_number - 1].chars().collect();
                    let number_str = scan_number_to_right(char_number + 1, &chars);

                    let number = number_str.into_iter().collect::<String>();
                    gear_ratio = gear_ratio * number.parse::<u32>().unwrap();
                    part_count = part_count + 1;
                    // println!("  top right number= {number}");
                }
                // count left
                if is_number(left_char) {
                    if part_count >= 2 {
                        continue;
                    }
                    let chars: Vec<char> = lines[line_number].chars().collect();
                    let number_str = scan_number_to_left(char_number - 1, &chars);

                    let number = number_str.into_iter().collect::<String>();
                    gear_ratio = gear_ratio * number.parse::<u32>().unwrap();
                    part_count = part_count + 1;
                    // println!("  left number= {number}");
                }
                // count right
                if is_number(right_char) {
                    if part_count >= 2 {
                        continue;
                    }
                    let chars: Vec<char> = lines[line_number].chars().collect();
                    let number_str = scan_number_to_right(char_number + 1, &chars);

                    let number = number_str.into_iter().collect::<String>();
                    gear_ratio = gear_ratio * number.parse::<u32>().unwrap();
                    part_count = part_count + 1;
                    // println!("  right number= {number}");
                }
                // count bottom left
                if is_number(bottom_left_char) {
                    if part_count >= 2 {
                        continue;
                    }
                    let chars: Vec<char> = lines[line_number + 1].chars().collect();
                    let mut number_str = scan_number_to_left(char_number - 1, &chars);
                    number_str.append(&mut scan_number_to_right(char_number, &chars));

                    let number = number_str.into_iter().collect::<String>();
                    gear_ratio = gear_ratio * number.parse::<u32>().unwrap();
                    part_count = part_count + 1;
                    // println!("  bottom left number= {number}");
                }
                // count bottom
                if !is_number(bottom_left_char) && is_number(bottom_char) {
                    if part_count >= 2 {
                        continue;
                    }
                    let chars: Vec<char> = lines[line_number + 1].chars().collect();
                    let number_str = scan_number_to_right(char_number, &chars);

                    let number = number_str.into_iter().collect::<String>();
                    gear_ratio = gear_ratio * number.parse::<u32>().unwrap();
                    part_count = part_count + 1;
                    // println!("  bottom number= {number}");
                }
                // count bottom right
                if !is_number(bottom_char) && is_number(bottom_right_char) {
                    if part_count >= 2 {
                        continue;
                    }
                    let chars: Vec<char> = lines[line_number + 1].chars().collect();
                    let number_str = scan_number_to_right(char_number + 1, &chars);

                    let number = number_str.into_iter().collect::<String>();
                    gear_ratio = gear_ratio * number.parse::<u32>().unwrap();
                    part_count = part_count + 1;
                    // println!("  bottom right number= {number}");
                }

                if part_count == 2 {
                    sum = sum + gear_ratio;
                    // println!(" +gear_ratio = {gear_ratio}");
                    // println!(" +sum = {sum}");
                }
            }
        }
    }

    println!("Part 2: {sum}");
}

fn scan_number_to_left(char_number: usize, chars: &Vec<char>) -> Vec<char> {
    for next_pos in (0..char_number).rev() {
        if !is_number(chars[next_pos]) {
            return chars[next_pos + 1..char_number + 1].to_vec();
        }
    }

    return chars[0..char_number+1].to_vec();
}

fn scan_number_to_right(char_number: usize, chars: &Vec<char>) -> Vec<char> {
    for next_pos in char_number..chars.len() {
        if !is_number(chars[next_pos]) {
            return chars[char_number..next_pos].to_vec();
        }
    }

    return chars[char_number..chars.len()].to_vec();
}
