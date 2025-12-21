fn main() {
    let input: String = String::from("328412-412772,1610-2974,163-270,7693600637-7693779967,352-586,65728-111612,734895-926350,68-130,183511-264058,8181752851-8181892713,32291-63049,6658-12472,720-1326,21836182-21869091,983931-1016370,467936-607122,31-48,6549987-6603447,8282771161-8282886238,7659673-7828029,2-18,7549306131-7549468715,3177-5305,20522-31608,763697750-763835073,5252512393-5252544612,6622957-6731483,9786096-9876355,53488585-53570896");

    let ranges: Vec<&str> = input.split(',').collect();
    let mut invalid_ids_p1: Vec<u64> = Vec::new();
    let mut invalid_ids_p2: Vec<u64> = Vec::new();

    for range in &ranges {
        let start_end: Vec<&str> = range.split("-").collect();
        let start: u64 = start_end[0].parse().unwrap();
        let end: u64 = start_end[1].parse().unwrap();
        for id in start..=end {
            if is_invalid_id(id) {
                invalid_ids_p1.push(id)
            }
            if is_invalid_id_p2(id) {
                invalid_ids_p2.push(id)
            }
        }
    }

    let answer_p1: u64 = invalid_ids_p1.into_iter().sum();
    let answer_p2: u64 = invalid_ids_p2.into_iter().sum();

    println!("part 1: {}", answer_p1);
    println!("part 2: {}", answer_p2);
}


fn is_invalid_id(id: u64) -> bool {
    let id_str = id.to_string();
    let id_str_len = id_str.len();
    if id_str_len % 2 == 1 { return false; }
    let half_len = id_str_len / 2;
    &id_str[0..half_len] == &id_str[half_len..id_str_len]
}

fn is_invalid_id_p2(id: u64) -> bool {
    let id_str = id.to_string();
    let id_str_len = id_str.len();
    let half_len = id_str_len / 2;
    // println!("half: {half_len}");
    let mut invalid = true;

    for n in 1..half_len {
        // println!("n: {n}");
        if id_str_len % n != 0 { continue; }
        let chunks = id_str_len / n;
        // println!("n: {n}, chunks: {chunks}");
        let first_chunk = &id_str[0..n];
        for i in 1..chunks {
            if id_str[i*n..(i+1)*n] != *first_chunk {
                invalid = false
            }
        }
    }
    return invalid
}
