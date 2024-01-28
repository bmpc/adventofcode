mod utils;

use std::collections::{ HashSet, VecDeque };
use std::hash::Hash;

const INPUT_FILE: &str = "./input/21_input.txt";
// const INPUT_FILE: &str = "./input/21_input_test.txt";
// const INPUT_FILE: &str = "./input/21_input_test2.txt";

const STEPS_1: usize = 64;
const STEPS_2: usize = 26501365;

struct Grid<'a> {
    width: usize,
    height: usize,
    data: &'a [char]
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
enum DIR {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

fn check_direction((row, col): (usize, usize), dir: DIR, grid: &Grid) -> Option<(usize, usize)> {
    let pos_op = match dir {
        DIR::UP if row > 0 => Some((row - 1, col)),
        DIR::DOWN if row < grid.height - 1 => Some((row + 1, col)),
        DIR::RIGHT if col > 0 => Some((row, col - 1)),
        DIR::LEFT if col < grid.width - 1 => Some((row, col + 1)),
        _ => None
    };

    if let Some(p) = pos_op {
        let ch = grid.data[grid.width * p.0 + p.1];
        if ch != '#' {
            pos_op
        } else {
            None
        }
    } else {
        None
    }
}

fn neighbors(pos: (usize, usize), grid: &Grid) -> [Option<(usize, usize)>; 4] {
    [
        check_direction(pos, DIR::UP, grid),
        check_direction(pos, DIR::DOWN, grid),
        check_direction(pos, DIR::LEFT, grid),
        check_direction(pos, DIR::RIGHT, grid)
    ]
}

fn count_plots(steps: usize, grid: &Grid) ->  VecDeque<(usize, usize)> {
    let start = grid.data.iter().position(|e| *e == 'S').unwrap();
    let s_row = start / grid.width;
    let s_col = start % grid.width;
    
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((s_row, s_col));

    let mut step_count = 0;

    while !queue.is_empty() && step_count < steps {
        let mut seen = HashSet::new();

        for _ in 0..queue.len() { // a single step processes all edges
            // NOTE: although we are changing the queue as we go, the length for a step is fixed when starting the loop
            let curr = queue.pop_front().unwrap();
    
            let neighbors = neighbors(curr, grid);
            for n_op in neighbors {
                if let Some(n) = n_op {
                    if !seen.contains(&n) {
                        seen.insert(n);
                        queue.push_back(n);
                    }
                }
            }
        }
        
        step_count += 1;
    }

    queue
}

fn _print_map(plots: &VecDeque<(usize, usize)>, grid: &Grid) {
    for row in 0..grid.height {
        for col in 0..grid.width {
            if plots.contains(&(row, col)) {
                print!("O");    
            } else {
                let ch = grid.data[grid.width * row + col];
                print!("{ch}");
            }
        }
        println!();
    }
}

fn main() {
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut data: Vec<char> = vec![];
        let mut width = 0;
        let mut height = 0;

        for line in lines {
            if let Ok(text) = line {
                if width == 0 {
                    width = text.len();
                }
                data.extend(text.chars());
                height += 1;
            }
        }

        let grid = Grid { width, height, data: data.as_slice() };
        
        let plots = count_plots(STEPS_1, &grid);
        //_print_map(&plots, &grid);
        println!("[Part 1] Number of garden plots the Elf can reach in exactly 64 steps : {}", plots.len());

    } else {
        eprintln!("Could not load the garden map from {}", INPUT_FILE);
    }

}
