fn main() {
    let input = include_str!("./input/day13.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut sum = 0;

    let mut pattern = Vec::new();
    for line in input.lines() {
        if !line.is_empty() {
            pattern.push(line);
        } else {
            let line = analyse_h_and_v(&pattern);
            sum += line;
            pattern.clear();
        }
    }

    let line = analyse_h_and_v(&pattern);
    sum += line;

    println!("Part 1: {sum}");
}

fn part2(input: &str) {
    let mut sum = 0;

    let mut pattern = Vec::new();
    for line in input.lines() {
        if !line.is_empty() {
            pattern.push(line);
        } else {
            let line = analyse_h_and_v_smudge(&pattern);
            sum += line;
            pattern.clear();
        }
    }

    let line = analyse_h_and_v_smudge(&pattern);
    sum += line;

    println!("Part 2: {sum}");
}

fn analyse_h_and_v(pattern: &Vec<&str>) -> usize {
    let h_line = analyse(pattern);
    if h_line > 0 {
        return 100 * h_line;
    }

    let t_pattern = transpose(pattern);
    let v_line = analyse(&t_pattern);

    if v_line > 0 {
        return v_line;
    }

    for i in 0..pattern.len() {
        println!("{}", pattern[i]);
    }
    panic!("no mirror line found");
}

fn analyse_h_and_v_smudge(pattern: &Vec<&str>) -> usize {
    let h_line = analyse_smudge(pattern);
    if h_line > 0 {
        return 100 * h_line;
    }

    let t_pattern = transpose(pattern);
    let t_pattern_str = t_pattern.iter().map(|s| s as &str).collect();
    let v_line = analyse_smudge(&t_pattern_str);

    if v_line > 0 {
        return v_line;
    }

    for i in 0..pattern.len() {
        println!("{}", pattern[i]);
    }
    panic!("no mirror line found");
}

fn transpose<'a>(pattern: &Vec<&str>) -> Vec<String> {
    let mut t_pattern = Vec::new();
    for i in 0..pattern[0].len() {
        let mut t_line = String::new();
        for line in pattern {
            t_line.push(line.chars().nth(i).unwrap());
        }
        t_pattern.push(t_line);
    }

    t_pattern
}

fn analyse<T: AsRef<str> + PartialEq>(pattern: &[T]) -> usize {
    for i in 0..pattern.len() - 1 {
        if pattern[i] == pattern[i + 1] {
            let mut is_match = true;
            for j in 1..i + 1 {
                if i + 1 + j < pattern.len() {
                    if pattern[i - j] != pattern[i + 1 + j] {
                        is_match = false;
                        break;
                    }
                } else {
                    return i + 1;
                }
            }

            if is_match {
                return i + 1;
            }
        }
    }

    0
}

fn analyse_smudge(pattern: &Vec<&str>) -> usize {
    for i in 0..pattern.len() - 1 {
        let mut total_diff = compare_reflection_line(&pattern[i], &pattern[i + 1]);
        if total_diff <= 1 {
            let mut is_match = true;
            for j in 1..i + 1 {
                if i + 1 + j < pattern.len() {
                    let diff_j = compare_reflection_line(&pattern[i - j], &pattern[i + 1 + j]);
                    total_diff += diff_j;
                    if total_diff > 1 {
                        is_match = false;
                        break;
                    }
                } else {
                    if total_diff == 1 {
                        return i + 1;
                    }
                }
            }

            if is_match && total_diff == 1 {
                return i + 1;
            }
        }
    }

    0
}

fn compare_reflection_line(line1: &str, line2: &str) -> usize {
    let mut diff = 0;
    let line1_arr = line1.chars().collect::<Vec<char>>();
    let line2_arr = line2.chars().collect::<Vec<char>>();
    for (i, &c) in line1_arr.iter().enumerate() {
        if c != line2_arr[i] {
            diff += 1;
        }
    }

    diff
}
