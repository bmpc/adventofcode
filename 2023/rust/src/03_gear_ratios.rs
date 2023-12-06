mod utils;

const INPUT_FILE: &str = "./03_input.txt";

const WIDTH: usize = 140;
const HEIGHT: usize = 140;

// *****
// *123*
// *****
fn is_adjacent_to_symbol(num: &str, pos: (usize, usize), matrix: &[[char;WIDTH];HEIGHT]) -> bool {
    // previous row
    if pos.0 > 0 { // check first row
        let adj_left = if pos.1 == 0 { pos.1 } else { pos.1 - 1 };
        let adj_right = if (pos.1 + num.len()) < WIDTH { pos.1 + num.len() } else { pos.1 + num.len() - 1 };

        for i in adj_left..= adj_right {
            let ch = matrix[pos.0 - 1][i];
            if !ch.is_digit(10) && ch != '.' {
                // found symbol
                return true;
            }
        }
    }

    // current row
    if pos.1 > 0 {
        let ch = matrix[pos.0][pos.1 - 1];
        if !ch.is_digit(10) && ch != '.' {
            // found symbol
            return true;
        }
    }
    if pos.1 + num.len() < WIDTH {
        let ch = matrix[pos.0][pos.1 + num.len()];
        if !ch.is_digit(10) && ch != '.' {
            // found symbol
            return true;
        }
    }

    // next row
    if pos.0 < HEIGHT - 1 {
        let adj_left = if pos.1 == 0 { pos.1 } else { pos.1 - 1 };
        let adj_right = if (pos.1 + num.len()) < WIDTH { pos.1 + num.len() } else { pos.1 + num.len() - 1 };

        for i in adj_left..= adj_right {
            let ch = matrix[pos.0 + 1][i];
            if !ch.is_digit(10) && ch != '.' {
                // found symbol
                return true;
            }
        }
    }

    false
}

fn main() {
    
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut sum: u32 = 0;

        let mut matrix:[[char;WIDTH];HEIGHT] = [['.';WIDTH];HEIGHT];

        for (i, line) in lines.enumerate() {
            for (j, c) in line.unwrap().chars().enumerate() {
                assert!(i < WIDTH && j < HEIGHT);
                matrix[i][j] = c;
            }
        }

        let mut num = String::new();
        let mut in_num = false;

        for (i, l) in matrix.iter().enumerate() {
            for (j, c) in l.iter().enumerate() {
                if c.is_digit(10) {
                    if !in_num {
                        num = String::new();
                    }
                    in_num = true;
                    num.push(*c);
                } else {
                    if in_num {
                        in_num = false;
                        // if j == 0, number is at the end of the previous line
                        let num_col = if j == 0 { WIDTH - num.len()} else { j - num.len() };
                        let num_row = if j == 0 { i - 1 } else { i };
                        if is_adjacent_to_symbol(&num, (num_row, num_col), &matrix) {
                            sum += num.parse::<u32>().unwrap();
                        }
                    }
                }
            }
        }

        println!("Sum of engine schematic part numbers: {}", sum);
    } else {
        eprintln!("Could not extract engine parts from {}", INPUT_FILE);
    }

}