mod utils;

use std::collections::{ HashSet, VecDeque };
use std::hash::Hash;

const INPUT_FILE: &str = "./input/21_input.txt";
//const INPUT_FILE: &str = "./input/21_input_test.txt";

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

fn count_plots((s_row, s_col): (usize, usize), steps: usize, grid: &Grid) ->  HashSet<(usize, usize)> {
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    queue.push_back((s_row, s_col, steps));

    let mut seen = HashSet::new();
    seen.insert((s_row, s_col));
    
    let mut ans: HashSet<(usize, usize)> = HashSet::new();

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        let steps = curr.2;
        let cn = (curr.0, curr.1);

        if steps % 2 == 0 {
            ans.insert(cn);
        }

        if steps == 0 {
            continue;
        }
    
        let neighbors = neighbors(cn, grid);
        for n_op in neighbors {
            if let Some(n) = n_op {
                if !seen.contains(&n) {
                    seen.insert(n);
                    queue.push_back((n.0, n.1, steps - 1));
                }
            }
        }
    }

    ans
}

/* slower solution */
fn _count_plots2((s_row, s_col): (usize, usize), steps: usize, grid: &Grid) ->  VecDeque<(usize, usize)> {
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

/* Shamefully borrowed from HyperNeutrino (https://www.youtube.com/watch?v=9UOMZSL0JTg) */
fn infinite_plots((sr, sc): (usize, usize), steps: usize, grid: &Grid) -> usize {
    let size = grid.width;
    let grid_width = steps / size - 1;

    let odd = (grid_width / 2 * 2 + 1).pow(2);
    let even = ((grid_width + 1) / 2 * 2).pow(2);

    let odd_points = count_plots((sr, sc), size * 2 + 1, grid).len();
    let even_points = count_plots((sr, sc), size * 2, grid).len();

    let corner_t = count_plots((size - 1, sc), size - 1, grid).len();
    let corner_r = count_plots((sr, 0), size - 1, grid).len();
    let corner_b = count_plots((0, sc), size - 1, grid).len();
    let corner_l = count_plots((sr, size - 1), size - 1, grid).len();

    let small_tr = count_plots((size - 1, 0), size / 2 - 1, grid).len();
    let small_tl = count_plots((size - 1, size - 1), size / 2 - 1, grid).len();
    let small_br = count_plots((0, 0), size / 2 - 1, grid).len();
    let small_bl = count_plots((0, size - 1), size / 2 - 1, grid).len();

    let large_tr = count_plots((size - 1, 0), size * 3 / 2 - 1, grid).len();
    let large_tl = count_plots((size - 1, size - 1), size * 3 / 2 - 1, grid).len();
    let large_br = count_plots((0, 0), size * 3 / 2 - 1, grid).len();
    let large_bl = count_plots((0, size - 1), size * 3 / 2 - 1, grid).len();

    odd * odd_points +
    even * even_points +
    corner_t + corner_r + corner_b + corner_l +
    (grid_width + 1) * (small_tr + small_tl + small_br + small_bl) +
    grid_width * (large_tr + large_tl + large_br + large_bl)
}

fn _print_map(plots: &HashSet<(usize, usize)>, grid: &Grid) {
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
        
        let start = grid.data.iter().position(|e| *e == 'S').unwrap();
        let s_row = start / grid.width;
        let s_col = start % grid.width;

        let plots = count_plots((s_row, s_col), STEPS_1, &grid);
        //_print_map(&plots, &grid);
        println!("[Part 1] Number of garden plots the Elf can reach in exactly {} steps : {}", STEPS_1, plots.len());

        let plots2 = infinite_plots((s_row, s_col), STEPS_2, &grid);
        println!("[Part 2] Number of garden plots the Elf can reach in exactly {} steps : {}", STEPS_2, plots2);
    } else {
        eprintln!("Could not load the garden map from {}", INPUT_FILE);
    }

}
