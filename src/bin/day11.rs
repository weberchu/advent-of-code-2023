fn main() {
    let input = include_str!("./input/day11.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let sum = simulation(input, 1);
    println!("Part 1: {sum}");
}

fn part2(input: &str) {
    let sum = simulation(input, 1000000-1);
    println!("Part 2: {sum}");
}

fn simulation(input: &str, expand_speed: usize) -> i64 {
    let universe: Vec<Vec<char>> = input.lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let mut galaxies = find_galaxies(&universe);
    expand(&mut galaxies, &universe, expand_speed);
    sum_of_distances(&galaxies)
}

fn find_galaxies(universe: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();
    for (r, row) in universe.iter().enumerate() {
        for (c, &point) in row.iter().enumerate() {
            if point == '#' {
                galaxies.push((r, c));
            }
        }
    }
    return galaxies;
}

fn expand(galaxies: &mut Vec<(usize, usize)>, universe: &Vec<Vec<char>>, expand_speed: usize) {
    let mut expanded_rows = Vec::new();
    for (r, row) in universe.iter().enumerate().rev() {
        if row.iter().all(|&c| c == '.') {
            expanded_rows.push(r);
        }
    }

    let mut expanded_columns = Vec::new();
    for c in 0..universe[0].len() {
        if universe.iter().all(|row| row[c] == '.') {
            expanded_columns.push(c);
        }
    }

    for galaxy in galaxies.iter_mut() {
        let row_expand_count = expanded_rows.iter().filter(|&&r| galaxy.0 >= r).count();
        galaxy.0 += row_expand_count * expand_speed;

        let column_expand_count = expanded_columns.iter().filter(|&&c| galaxy.1 >= c).count();
        galaxy.1 += column_expand_count * expand_speed;
    }
}

fn sum_of_distances(galaxies: &Vec<(usize, usize)>) -> i64 {
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (x1, y1) = galaxies[i];
            let (x2, y2) = galaxies[j];
            let distance = (x1 as i64 - x2 as i64).abs() + (y1 as i64 - y2 as i64).abs();
            sum += distance;
        }
    }

    sum
}
