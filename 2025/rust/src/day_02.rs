use std::fs;
use std::collections::HashSet;


fn is_invalid_id_p1(id: u64) -> bool {
    let id_str = id.to_string();
    let id_str_len = id_str.len();
    if id_str_len % 2 == 1 { return false; }
    let half_len = id_str_len / 2;
    &id_str[0..half_len] == &id_str[half_len..id_str_len]
}


fn is_invalid_id_p2(id: u64) -> bool {
    let id_str = id.to_string();
    let length = id_str.len();
    for part_count in 2..=length {
        if length % part_count != 0 { continue; }
        let part_length = length / part_count;
        let mut chunks: HashSet<i32> = HashSet::new();
        for part_id in 0..part_count {
            let chunk_start = part_id * part_length;
            let chunk_end = (part_id + 1) * part_length;
            let chunk = &id_str[chunk_start..chunk_end];
            chunks.insert(chunk.parse().unwrap());
        }
        if chunks.len() == 1 { return true; }
    }
    return false;
}


fn main() {
    let input: String = fs::read_to_string("../inputs/day_02.txt").unwrap();

    let mut invalid_ids_p1: Vec<u64> = Vec::new();
    let mut invalid_ids_p2: Vec<u64> = Vec::new();

    for range in input.split(',') {
        let start_end: Vec<&str> = range.split("-").collect();
        let start: u64 = start_end[0].parse().unwrap();
        let end: u64 = start_end[1].parse().unwrap();
        for id in start..=end {
            if is_invalid_id_p1(id) { invalid_ids_p1.push(id) }
            if is_invalid_id_p2(id) { invalid_ids_p2.push(id) }
        }
    }

    let answer_p1: u64 = invalid_ids_p1.into_iter().sum();
    let answer_p2: u64 = invalid_ids_p2.into_iter().sum();

    println!("part 1: {}", answer_p1);
    println!("part 2: {}", answer_p2);
}

