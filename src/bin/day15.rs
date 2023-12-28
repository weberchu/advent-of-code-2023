fn main() {
    let input = include_str!("./input/day15.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut sum = 0;

    let line = input.lines().next().unwrap();
    let steps = line.split(",");

    for step in steps {
        sum += hash(step);
    }

    println!("Part 1: {sum}");
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: usize,
}

fn part2(input: &str) {
    let line = input.lines().next().unwrap();
    let steps = line.split(",");

    let mut boxes: Vec<Vec<Lens>> = Vec::new();
    for _i in 0..256 {
        boxes.push(Vec::new());
    }

    for step in steps {
        if step.contains("=") {
            let split = step.split("=").collect::<Vec<&str>>();
            let label = split[0];
            let label_hash = hash(label);
            let focal_length = split[1].parse().unwrap();
            let lens = Lens { label: label.to_string(), focal_length };

            let b = &mut boxes[label_hash];
            match b.iter().position(|l| l.label == label) {
                Some(i) => {
                    b[i] = lens;
                }
                _ => {
                    b.push(lens);
                }
            }
        } else {
            let split = step.split("-").collect::<Vec<&str>>();
            let label = split[0];
            let label_hash = hash(label);

            let b = &mut boxes[label_hash];
            match b.iter().position(|l| l.label == label) {
                Some(i) => {
                    b.remove(i);
                }
                _ => {}
            }
        }
    }

    let mut focusing_power = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (j, lens) in b.iter().enumerate() {
            focusing_power += (i + 1) * (j + 1) * lens.focal_length;
        }
    }

    println!("Part 2: {focusing_power}");
}

fn hash(input: &str) -> usize {
    let mut sum = 0;

    for c in input.chars() {
        sum = (sum + (c as usize)) * 17 % 256;
    }

    sum
}
