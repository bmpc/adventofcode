mod utils;

use indexmap::IndexSet;
use std::hash::Hash;
use std::collections::{HashMap,HashSet};

const INPUT_FILE: &str = "./input/23_input.txt";
// const INPUT_FILE: &str = "./input/23_input_test.txt";

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
    let start = grid.data.iter().position(|ch| *ch == '.').unwrap();
    
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

fn edge_contraction(grid: &Grid) -> i32 {
    let start = grid.data.iter().position(|ch| *ch == '.').unwrap();

    let lr = grid.width * (grid.height - 1);
    let end_index = grid.data.iter().skip(lr).position(|ch| *ch == '.').unwrap();
    let end = (grid.height - 1, end_index);
    
    // first get junction points    
    let mut junction_points = HashSet::new();
    junction_points.insert((0, start));
    junction_points.insert(end);
    
    for row in 0..grid.height {
        for col in 0..grid.width {
            let ch = grid.data[grid.width * row + col];
            if ch != '#' {
                let neighbors = neighbors((row, col), false, grid);
                
                let neighbors_count = neighbors.iter().filter(|n| n.is_some()).map(|n| n.unwrap()).count();
                if neighbors_count >= 3 {
                    junction_points.insert((row, col));
                }
            }
        }
    }
    
    // build a weighted graph contracting edges 
    let mut graph: HashMap<(usize, usize), Vec<(usize, usize, usize)>> = HashMap::new();
    
    for (sr, sc) in &junction_points {
        let mut nodes = vec![(*sr, *sc, 0)];
        let mut seen = HashSet::new();
        seen.insert((*sr, *sc));

        while let Some((r, c, n)) = nodes.pop() {
            if n != 0 && junction_points.contains(&(r, c)) {
                graph.entry((*sr, *sc)).or_default().push((r, c, n));
                continue;
            }

            let neighbors: Vec<(usize, usize)> = neighbors((r, c), false, grid)
                .iter()
                .filter(|ng| ng.is_some() && !seen.contains(&ng.unwrap()))
                .map(|ng| ng.unwrap())
                .collect();

            for nb in neighbors {
                nodes.push((nb.0, nb.1, n + 1));
                seen.insert(nb);
            }
        }
    }
    
    // calculate longest path using brute force
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    dfs((0, start), end, &mut seen, &graph, grid)

}

fn dfs(pos: (usize, usize), end: (usize ,usize), seen: &mut HashSet<(usize, usize)>, graph: &HashMap<(usize, usize), Vec<(usize, usize, usize)>>, grid: &Grid) -> i32 {
    if pos == end {
        return 0;
    }
    
    seen.insert(pos);
    
    let mut m = i32::MIN;

    for nx in &graph[&pos] {
        if !seen.contains(&(nx.0, nx.1)) {
            let bm = dfs((nx.0, nx.1), end, seen, graph, grid) + nx.2 as i32;
            if bm > m {
                m = bm;
            }
        }
    }
    seen.remove(&pos);

    m
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
        
        // the part 1 algorithm will not finish in a reasonable amount of time
        // we need to simplify the graph by removing edges that do not branch
        // once again, based on HyperNeurtrino's solution: https://www.youtube.com/watch?v=NTLYL7Mg2jU
        let longest_path_2 = edge_contraction(&grid);
        println!("[Part 2] The longest hike without slopes is {} steps long ", longest_path_2);
    } else {
        eprintln!("Could not load the hike map from {}", INPUT_FILE);
    }

}
