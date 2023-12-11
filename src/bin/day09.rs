fn main() {
    let input = include_str!("./input/day09.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut sum = 0;

    for line in input.lines() {
        let number_stacks = create_stacks(line);
        let extrapolate_value: i64 = number_stacks.iter().map(|stack| stack.last().unwrap()).sum();
        sum += extrapolate_value;
    }

    println!("Part 1: {sum}");
}

fn part2(input: &str) {
    let mut sum = 0;

    for line in input.lines() {
        let number_stacks = create_stacks(line);
        let extrapolate_value: i64 = number_stacks.iter()
            .map(|stack| stack.first().unwrap())
            .enumerate()
            .map(|(i, &n)|
                if i as i32 % 2 == 0 {
                    n
                } else {
                    -1 * n
                }
            ).sum();
        sum += extrapolate_value;
    }

    println!("Part 2: {sum}");
}

fn create_stacks(line: &str) -> Vec<Vec<i64>> {
    let numbers: Vec<i64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut number_stacks = Vec::new();
    let mut current_stack = numbers.clone();
    number_stacks.push(numbers);

    while !current_stack.iter().all(|&n| n == 0) {
        let mut next_stack = Vec::new();
        for i in 1..current_stack.len() {
            next_stack.push(current_stack[i] - current_stack[i - 1]);
        }
        current_stack = next_stack.clone();
        number_stacks.push(next_stack);
    }
    number_stacks
}
