fn main() {
    let input: String = std::fs::read_to_string("../inputs/day_04.txt").unwrap();

    let matrix: Vec<Vec<i32>> = input.lines()
        .map(|line| line.chars().map(|c| if c == '@' {1} else {0}).collect())
        .collect();

    let mut sum: u32 = 0;
    for (y, line) in matrix.iter().enumerate() {
        for (x, current_cell) in line.iter().enumerate() {
            if *current_cell == 0 { continue; }
            let x = x as isize;
            let y = y as isize;
            let mut cell_accum = 0;
            for offset_x in -1isize..=1isize {
                for offset_y in -1isize..=1isize {
                    if offset_x == 0 && offset_y == 0 {
                        continue;
                    }
                    let matrix_width = matrix.len().try_into().unwrap();
                    if y + offset_y < 0 || y + offset_y >= matrix_width {
                        continue;
                    }
                    let matrix_height = matrix[0].len().try_into().unwrap();
                    if x + offset_x < 0 || x + offset_x >= matrix_height {
                        continue;
                    }
                    cell_accum += matrix[(y+offset_y) as usize][(x+offset_x) as usize];
                }
            }
            if cell_accum < 4 {
                sum += 1;
            }
        }
    }

    println!("part 1: {sum}");
}

