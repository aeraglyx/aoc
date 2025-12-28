use std::fs;


fn get_joltage(line: &str, n: usize) -> i64 {
    let mut digits: Vec<u8> = line.chars().map(|c| c as u8 - 0x30).collect();
    let mut digits_found: Vec<u8> = Vec::new();
    for i in 1..=n {
        let total_bats = digits.len();
        let end_id: usize = total_bats + i - n;
        let found_digit: u8 = *digits[0..end_id].iter().max().unwrap();
        let found_id = digits.iter().position(|c| *c == found_digit).unwrap();
        digits.drain(0..=found_id);
        digits_found.push(found_digit);
    }
    digits_found.into_iter()
        .fold(0, |accum, digit| accum * 10 + digit as i64)
}


fn main() {
    let input: String = fs::read_to_string("../inputs/day_03.txt").unwrap();

    let mut sum_p1: i64 = 0;
    let mut sum_p2: i64 = 0;
    for line in input.lines() {
        sum_p1 += get_joltage(line, 2);
        sum_p2 += get_joltage(line, 12);
    }

    println!("part 1: {sum_p1}");
    println!("part 2: {sum_p2}");
}

