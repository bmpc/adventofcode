mod utils;

//const INPUT_FILE: &str = "./input/11_input_test.txt";
const INPUT_FILE: &str = "./input/11_input.txt";

struct Galaxy {
    x: usize,
    y: usize
}

// naive solution where we the image was expanded in size
fn _expand_universe(image: &mut Vec<Vec<char>>) {
    let rows_exp: Vec<usize> = image.iter().enumerate()
        .filter(|(_, row)| row.iter().all(|c| *c == '.'))
        .map(|(i, _)| i)
        .rev()
        .collect();

    for i in rows_exp {
        image.insert(i, vec!['.'; image[i].len()]);
    }

    let cols_exp: Vec<usize> = (0..image[0].len())
        .filter(|col| image.iter().all(|row| row[*col] == '.'))
        .rev()
        .collect();

    for col in cols_exp {
        for row in image.into_iter() {
            row.insert(col, '.');
        }
    }    
}

fn find_empty_rows(image: &Vec<Vec<char>>) -> Vec<usize> {
    image.iter().enumerate()
        .filter(|(_, row)| row.iter().all(|c| *c == '.'))
        .map(|(i, _)| i)
        .rev()
        .collect()
}

fn find_empty_columns(image: &Vec<Vec<char>>) -> Vec<usize> {
    (0..image[0].len())
        .filter(|col| image.iter().all(|row| row[*col] == '.'))
        .rev()
        .collect()
}


fn sum_galaxy_pair_lengths(image: &Vec<Vec<char>>, expand_factor: usize) -> u64 {
    let mut galaxy_pairs = Vec::new();

    let empty_rows = find_empty_rows(image);
    let empty_cols = find_empty_columns(image);

    // collect galaxy pairs
    for row in 0..image.len() {
        for col in 0..image[row].len() {
            if image[row][col] == '#' {
                let next_row = if col == image[row].len() - 1 { row + 1 } else { row };
                let mut next_col = if col == image[row].len() - 1 { 0 } else { col + 1 };

                for row2 in next_row..image.len() {
                    for col2 in next_col..image[row2].len() {
                        if image[row2][col2] == '#' {
                            galaxy_pairs.push((
                                Galaxy { y: row, x: col },
                                Galaxy { y: row2, x: col2 }
                            ));
                        }
                    }
                    next_col = 0;
                }
            }
        }
    }

    // find shortest length
    galaxy_pairs.iter().map(|(g1, g2)| {
        let y_diff: u64 = (g2.y.abs_diff(g1.y) + (empty_rows.iter().filter(|row| **row > g1.y && **row < g2.y).count() * (expand_factor - 1))) as u64;
        let x_diff: u64 = (g2.x.abs_diff(g1.x) + (empty_cols.iter().filter(|col| **col > g1.x.min(g2.x) && **col < g1.x.max(g2.x)).count() * (expand_factor - 1))) as u64;
        return y_diff + x_diff;
    }
    ).sum::<u64>() as u64

}

fn _print_universe(image: &Vec<Vec<char>>) {
    for row in 0..image.len() {
        for col in 0..image[row].len() {
            print!("{}", image[row][col]);
        }
        println!();
    }
}

fn main() {
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut image: Vec<Vec<char>> = Vec::new();

        for line in lines {
            if let Ok(text) = line {
                let mut col = Vec::new();
                for ch in text.chars() {
                    col.push(ch);
                }
                image.push(col);
            }
        }

        //expand_universe(&mut image);
        //print_universe(&image);

        let sum1 = sum_galaxy_pair_lengths(&image, 1);
        let sum2 = sum_galaxy_pair_lengths(&image, 1000000);

        println!("[Expansion factor = 1] Sum of shortest path lengths: {}", sum1);
        println!("[Expansion factor = 1000000] Sum of shortest path lengths: {}", sum2);
    } else {
        eprintln!("Could not extract image from {}", INPUT_FILE);
    }

}
