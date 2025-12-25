use std::fs;

fn main() {
    let input: String = fs::read_to_string("inputs/day_03.txt").unwrap();

    let mut sum_p1 = 0;
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        let largest_num_1: char = *chars.iter().max().unwrap();
        let largest_id = chars.iter().position(|c| *c == largest_num_1).unwrap();
        let mut flip_later = false;
        let the_rest = if largest_id == chars.len() - 1 {
            flip_later = true;
            line[.. line.len() - 1].to_string()
        } else {
            line[largest_id + 1 ..].to_string()
        };
        let largest_num_2: char = the_rest.chars().max().unwrap();
        let mut joltage: Vec<char> = vec![largest_num_1, largest_num_2];
        if flip_later { joltage.reverse(); }
        let joltage: i32 = joltage.into_iter().collect::<String>().parse().unwrap();
        sum_p1 += joltage;
    }

    println!("part 1: {sum_p1}");
}
