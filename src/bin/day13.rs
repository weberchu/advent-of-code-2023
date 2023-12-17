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
            println!("line = {line}");
            sum += line;
            pattern.clear();
        }
    }

    let line = analyse_h_and_v(&pattern);
    println!("line = {line}");
    sum += line;

    println!("Part 1: {sum}");
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
            // println!("i = {i}");
            let mut is_match = true;
            for j in 1..i+1 {
                if i+1+j < pattern.len() {
                    if pattern[i-j] != pattern[i+1+j] {
                        // println!("  break at j = {j}");
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

fn part2(input: &str) {
    let mut sum = 0;

    for line in input.lines() {
        sum += line.len();
    }

    println!("Part 2: {sum}");
}
