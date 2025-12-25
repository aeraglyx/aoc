use std::fs;

fn main() {
    let input: String = fs::read_to_string("inputs/day_01.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut accum = 50;
    let mut answer_p1 = 0;
    let mut answer_p2 = 0;

    for line in &lines {
        let offset: i32 = line[1..].parse().unwrap();
        let mut direction: i32 = 1;
        if line.chars().nth(0).unwrap() == 'L' { direction = -1; }

        for _ in 1..=offset {
            accum += direction;
            accum = accum % 100;
            if accum == 0 { answer_p2 += 1; }
        }

        if accum == 0 { answer_p1 += 1; }
    }

    println!("part 1: {}", answer_p1);
    println!("part 2: {}", answer_p2);
}
