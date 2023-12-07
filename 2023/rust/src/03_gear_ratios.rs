use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

mod utils;

const INPUT_FILE: &str = "./03_input.txt";

const WIDTH: usize = 140;
const HEIGHT: usize = 140;

#[derive(Eq)]
#[derive(Clone)]
#[derive(Copy)]
struct Symbol {
    ch: char,
    row: usize,
    col: usize
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.ch == other.ch &&
        self.row == other.row &&
        self.col == other.col
    }
}

impl Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ch.hash(state);
        self.row.hash(state);
        self.col.hash(state);
    }
}

// .....
// .123.
// .....
fn get_adjacent_symbols(num: &str, pos: (usize, usize), matrix: &[[char;WIDTH];HEIGHT]) -> Vec<Symbol> {
    let mut symbols = Vec::new();

    // previous row
    if pos.0 > 0 { // check first row
        let adj_left = if pos.1 == 0 { pos.1 } else { pos.1 - 1 };
        let adj_right = if (pos.1 + num.len()) < WIDTH { pos.1 + num.len() } else { pos.1 + num.len() - 1 };

        for i in adj_left..= adj_right {
            let ch = matrix[pos.0 - 1][i];
            if !ch.is_digit(10) && ch != '.' {
                symbols.push(Symbol { ch, row: pos.0 - 1, col: i});
            }
        }
    }

    // current row
    if pos.1 > 0 {
        let ch = matrix[pos.0][pos.1 - 1];
        if !ch.is_digit(10) && ch != '.' {
            symbols.push(Symbol { ch, row: pos.0, col: pos.1 - 1});
        }
    }
    if pos.1 + num.len() < WIDTH {
        let ch = matrix[pos.0][pos.1 + num.len()];
        if !ch.is_digit(10) && ch != '.' {
            symbols.push(Symbol { ch, row: pos.0, col: pos.1 + num.len()});
        }
    }

    // next row
    if pos.0 < HEIGHT - 1 {
        let adj_left = if pos.1 == 0 { pos.1 } else { pos.1 - 1 };
        let adj_right = if (pos.1 + num.len()) < WIDTH { pos.1 + num.len() } else { pos.1 + num.len() - 1 };

        for i in adj_left..= adj_right {
            let ch = matrix[pos.0 + 1][i];
            if !ch.is_digit(10) && ch != '.' {
                symbols.push(Symbol { ch, row: pos.0 + 1, col: i});
            }
        }
    }

    symbols
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

        let mut symbol_nums_map: HashMap<Symbol, Vec<u32>> = HashMap::new();

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

                        let symbols = get_adjacent_symbols(&num, (num_row, num_col), &matrix);

                        if symbols.len() > 0 {
                            sum += num.parse::<u32>().unwrap();

                            for sym in symbols.iter().filter(|sym| sym.ch == '*') {
                                if let Some(symbol_nums) = symbol_nums_map.get_mut(sym) {
                                    symbol_nums.push(num.parse::<u32>().unwrap());
                                } else {
                                    symbol_nums_map.insert(*sym, vec![num.parse::<u32>().unwrap()]);
                                }                                
                            }
                        }
                    }
                }
            }
        }

        // calculate gear rations
        let sum_gear_rations = symbol_nums_map.iter()
            .filter(|(_, nums)| nums.len() == 2)
            .fold(0, |acc, (_, nums)| acc + nums.iter().copied().reduce(|acc, num| num * acc).unwrap());

        println!("Sum of engine schematic part numbers: {}", sum);

        println!("Sum of gear ratios: {}", sum_gear_rations);
    } else {
        eprintln!("Could not extract engine parts from {}", INPUT_FILE);
    }

}