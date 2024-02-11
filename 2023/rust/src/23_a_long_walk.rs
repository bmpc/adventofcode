mod utils;

use indexmap::IndexSet;
use std::hash::Hash;

const INPUT_FILE: &str = "./input/23_input.txt";
//const INPUT_FILE: &str = "./input/23_input_test.txt";

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

fn check_direction((row, col): (usize, usize), dir: DIR, slopes: bool, grid: &Grid) -> Option<(usize, usize)> {
    let pos_op = match dir {
        DIR::UP if row > 0 => Some((row - 1, col)),
        DIR::DOWN if row < grid.height - 1 => Some((row + 1, col)),
        DIR::LEFT if col > 0 => Some((row, col - 1)),
        DIR::RIGHT if col < grid.width - 1 => Some((row, col + 1)),
        _ => None
    };

    if let Some(p) = pos_op {
        let ch = grid.data[grid.width * p.0 + p.1];
        match ch {
            '<' if slopes && dir == DIR::RIGHT => None,
            '>' if slopes && dir == DIR::LEFT => None,
            '^' if slopes && dir == DIR::DOWN => None,
            'v' if slopes && dir == DIR::UP => None,
            '#' => None,
            _ => pos_op
        }
    } else {
        None
    }
}

fn neighbors(pos: (usize, usize), slopes: bool, grid: &Grid) -> [Option<(usize, usize)>; 4] {
    [
        check_direction(pos, DIR::UP, slopes, grid),
        check_direction(pos, DIR::DOWN, slopes, grid),
        check_direction(pos, DIR::LEFT, slopes, grid),
        check_direction(pos, DIR::RIGHT, slopes, grid)
    ]
}

fn walk(mut path: IndexSet<(usize, usize)>, slopes: bool, grid: &Grid) -> IndexSet<(usize, usize)> {
    let mut pos = path.last().unwrap().clone();
    
    let mut can_walk = true;
    while can_walk {
        let ch = grid.data[pos.0 * grid.width + pos.1];

        let choices: Vec<(usize, usize)> = match ch {
            '<' if slopes => vec![(pos.0, pos.1 - 1)],
            '>' if slopes => vec![(pos.0, pos.1 + 1)],
            '^' if slopes => vec![(pos.0 - 1, pos.1)],
            'v' if slopes => vec![(pos.0 + 1, pos.1)],
            _ => {
                let neighbors = neighbors(pos, slopes, grid);
        
                neighbors.iter()
                    .filter(|n| n.is_some() && !path.contains(&n.unwrap()))
                    .map(|n|n.unwrap())
                    .collect()
            }
        };

        if choices.len() > 0 {
            if choices.len() == 1 {
                pos = choices.first().unwrap().clone();
                path.insert(pos);
            } else {
                let mut lgst_fork_path = IndexSet::new();
                for fork in choices {
                    let mut npath = path.clone();
                    npath.insert(fork.clone());

                    let fork_path = walk(npath, slopes, grid);

                    let reached_dest = fork_path.last().is_some_and(|v|v.0 == grid.height - 1);
                    if reached_dest && fork_path.len() > lgst_fork_path.len() {
                        lgst_fork_path = fork_path;
                    }
                }

                path = lgst_fork_path;
                can_walk = false;
            }
        } else {
            can_walk = false;
        }
    }

    path
}

fn longest_path(grid: &Grid, slopes: bool) -> IndexSet<(usize, usize)> {
    // start
    let mut start = 0; 
    for i in 0..grid.width {
        if grid.data[i] == '.' {
            start = i;
            break;
        }
    }
    
    let mut path: IndexSet<(usize, usize)> = IndexSet::new();
    path.insert((0, start));
    path.insert((1, start));

    walk(path, slopes, grid)
}

fn _print_map(steps: &IndexSet<(usize, usize)>, grid: &Grid) {
    for row in 0..grid.height {
        for col in 0..grid.width {
            if steps.contains(&(row, col)) {
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

        let longest_path_1 = longest_path(&grid, true);
        println!("[Part 1] The longest hike is {} steps long ", longest_path_1.len() - 1);
        
        let longest_path_2 = longest_path(&grid, false);
        //_print_map(&longest_path_2, &grid);
        println!("[Part 2] The longest hike without slopes is {} steps long ", longest_path_2.len() - 1);
    } else {
        eprintln!("Could not load the hike map from {}", INPUT_FILE);
    }

}
