mod utils;

// const INPUT_FILE: &str = "./input/13_input_test.txt";
const INPUT_FILE: &str = "./input/13_input.txt";

struct Chunk {
    width: usize,
    height: usize,
    data: Vec<char>
}

fn process_chunk(chunk: &Chunk, smudge: bool) -> u32 {
    if smudge {
        let mut count = process_chunk_row(chunk, true);
        if count.1 {
            return count.0 * 100;
        } else {
            let transposed = transpose_right_chunk(chunk);
            count = process_chunk_row(&transposed, true);
            return count.0;
        }
    } else {
        let mut count = process_chunk_row(chunk, smudge);
        if count.0 > 0 {
            return count.0 * 100;
        }

        let transposed = transpose_right_chunk(chunk);

        count = process_chunk_row(&transposed, smudge);
        return count.0;
    }
}

fn transpose_right_chunk(chunk: &Chunk) -> Chunk {
    let mut data = vec![];
    for col in 0..chunk.width {
        for row in (0..chunk.height).rev() {
            data.push(chunk.data[(row * chunk.width) + col]);
        }
    }

    Chunk { width: chunk.height, height: chunk.width, data }
}

fn compare_rows(row1: &[char], row2: &[char], smudge: bool) -> (bool, bool) {
    let mut count_diff = 0;

    if smudge {
        for i in 0..row1.len() {
            if row1[i] != row2[i] {
                count_diff += 1;
            }
        }

        if count_diff < 2 {
            (true, count_diff == 1)
        } else {
            (false, false)
        }
    } else {
        return (row1.eq(row2), false);
    }
}

fn process_chunk_row(chunk: &Chunk, smudge: bool) -> (u32, bool) {
    for row in 1..chunk.height {
        let i = row * chunk.width;
        let row1 = &chunk.data[(i - chunk.width)..i];
        let row2 = &chunk.data[i..(i + chunk.width)];
        let (eq, mut found_smudge) = compare_rows(row1, row2, smudge);
        if eq {
            if row == 1 || row == chunk.height - 1 {
                if smudge {
                    if found_smudge {
                        return (row as u32, found_smudge);
                    } else {
                        continue;
                    }
                } else {
                    return (row as u32, false);
                }
            }
            let mut up = (row - 2) as i32;
            let mut down = row + 1;
            let mut reflect = true;
            while up >= 0 && down < chunk.height {
                let r1 = &chunk.data[(up as usize * chunk.width)..(up as usize * chunk.width + chunk.width)];
                let r2 = &chunk.data[(down * chunk.width)..(down * chunk.width + chunk.width)];
                let (eq, _found_smudge) = compare_rows(r1, r2, smudge && !found_smudge);
                if _found_smudge {
                    found_smudge = true;
                }
                if !eq {
                    reflect = false;
                    break;
                }

                up -= 1;
                down += 1;
            }

            if reflect {
                if smudge {
                    if found_smudge {
                        return (row as u32, found_smudge);
                    }
                } else {
                    return (row as u32, false);
                }
            }
        }
    }
    (0, false)
}

fn main() {
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut sum1 = 0;
        let mut sum2 = 0;

        let mut data: Vec<char> = vec![];
        let mut width = 0;
        let mut height = 0;

        for line in lines {
            if let Ok(text) = line {
                if text.is_empty() { // expects an empty line at the end of the file
                    let chunk = Chunk { width, height, data };
                    sum1 += process_chunk(&chunk, false);
                    sum2 += process_chunk(&chunk, true);  
                    data = vec![];
                    height = 0;
                    width = 0;
                } else {
                    if data.is_empty() {
                        width = text.len();
                    }
                    height += 1;

                    data.extend(text.chars());
                }
            }
        }

        // process last chunk
        if data.len() > 0 {
            let chunk = Chunk { width, height, data };
            sum1 += process_chunk(&chunk, false);
            sum2 += process_chunk(&chunk, true);
        }

        println!("[Part 1] Sum of all reflection patterns: {}", sum1);
        println!("[Part 2] Sum of all reflection patterns after fixing the smudge: {}", sum2);
    } else {
        eprintln!("Could not load reflection patterns from {}", INPUT_FILE);
    }

}
