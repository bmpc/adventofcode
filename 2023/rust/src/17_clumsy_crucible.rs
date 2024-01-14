mod utils;

use std::collections::HashMap;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::collections::BinaryHeap;

const INPUT_FILE: &str = "./input/17_input.txt";
//const INPUT_FILE: &str = "./input/17_input_test.txt";

struct Map {
    width: usize,
    height: usize,
    data: Vec<u32>
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
enum DIR {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

fn dir_value(dir: DIR) -> (i32, i32) {
    match dir {
        DIR::UP => (0, -1),
        DIR::DOWN => (0, 1),
        DIR::RIGHT => (1, 0),
        DIR::LEFT => (-1, 0)
    }
}

#[derive(Debug, Copy, Clone, Eq)]
struct Node {
    pos: (usize, usize),
    dir: DIR,
    dir_steps: u32,
    score: u32,
}

// implement ordering based on score
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.score.partial_cmp(&self.score) // reversed
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score) // reversed
    }
}

// implement equality based on pos and dir
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos 
        && self.dir == other.dir 
        && self.dir_steps == other.dir_steps
    }
}
impl Hash for Node {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.pos.hash(hasher);
        self.dir.hash(hasher);
        self.dir_steps.hash(hasher);
    }
}

fn successors(curr: Node, (min_steps, max_steps): (usize, usize), map: &Map) -> Vec<(usize, usize, DIR, u32)> {
    
    if curr.dir_steps < min_steps as u32 { // continue to move forward
        let (x, y) = dir_value(curr.dir);
        let np = (curr.pos.0 as i32 + x, curr.pos.1 as i32 + y);

        return vec![(np.0 as usize, np.1 as usize, curr.dir, curr.dir_steps + 1)];    
    }
    
    let mut succs = vec![];
    
    let (x, y) = curr.pos;

    // up
    if curr.dir != DIR::DOWN && y > 0 {
        if curr.dir == DIR::UP {
            if curr.dir_steps < max_steps as u32 {
                succs.push((x, y - 1, DIR::UP, curr.dir_steps + 1));
            }
        } else {
            if y as i32 - min_steps as i32 >= 0 {
                succs.push((x, y - 1, DIR::UP, 1)); // switch dir
            }
        }
    }
    // down
    if curr.dir != DIR::UP && y < map.height - 1 {
        if curr.dir == DIR::DOWN {
            if curr.dir_steps < max_steps as u32 {
                succs.push((x, y + 1, DIR::DOWN, curr.dir_steps + 1));
            }
        } else {
            if y + min_steps < map.height {
                succs.push((x, y + 1, DIR::DOWN, 1)); // switch dir
            }
        }
    }
    // left
    if curr.dir != DIR::RIGHT && x > 0 {
        if curr.dir == DIR::LEFT {
            if curr.dir_steps < max_steps as u32 {
                succs.push((x - 1, y, DIR::LEFT, curr.dir_steps + 1));
            }
        } else {
            if x as i32 - min_steps as i32 >= 0 {
                succs.push((x - 1, y, DIR::LEFT, 1)); // switch dir
            }
        }
    }
    // right
    if curr.dir != DIR::LEFT && x < map.width - 1 {
        if curr.dir == DIR::RIGHT {
            if curr.dir_steps < max_steps as u32 {
                succs.push((x + 1, y, DIR::RIGHT, curr.dir_steps + 1));
            }
        } else {
            if x + min_steps < map.width {
                succs.push((x + 1, y, DIR::RIGHT, 1)); // switch dir
            }
        }
    }

    succs
}

fn dijkstra(start: (usize, usize), goal: (usize, usize), step_rules: (usize, usize), map: &Map) -> Option<u32> {
    // The set of discovered nodes that may need to be (re-)expanded.
    // Initially, only the start node is known.
    // This is usually implemented as a min-heap or priority queue rather than a hash-set.
    let mut open_set = BinaryHeap::new();
    open_set.push(Node { pos: start, dir: DIR::RIGHT, dir_steps: 0, score: 0 });
    open_set.push(Node { pos: start, dir: DIR::DOWN, dir_steps: 0, score: 0 });

    let mut came_from: HashMap<Node, Node> = HashMap::new();

    while !open_set.is_empty() {
        // This operation can occur in O(Log(N)) time if openSet is a min-heap or a priority queue
        // current := the node in openSet having the lowest gScore value
        let current = open_set.pop().unwrap();

        if current.pos == goal {
            // _print_map(current, &open_set, &came_from, &map);

            return Some(current.score);
        }

        for succ in successors(current, step_rules, map) {
            let val = map.data[map.width * succ.1 + succ.0];
            let tentative_g_score: u32 = &current.score + val;

            let nn = Node { pos: (succ.0, succ.1), dir: succ.2, dir_steps: succ.3, score: tentative_g_score};
            
            let prev_score = match came_from.get_key_value(&nn) {
                Some((en, _)) => en.score,
                None => u32::MAX
            };

            if tentative_g_score < prev_score {
                // This path to neighbor is better than any previous one. Record it!
                came_from.insert(nn, current);
                open_set.push(nn);
            }
        }
    }

    return None
}

fn _print_map(curr: Node, open_set: &BinaryHeap<Node>, came_from: &HashMap<Node, Node>, map: &Map) {
    let mut path = HashMap::new();
    let mut i = curr;
    while i.pos != (0, 0) {
        i = came_from[&i];
        path.insert(i.pos, i.score);
    }

    for y in 0..map.height {
        for x in 0..map.width {
            let score = if path.contains_key(&(x, y)) { path[&(x, y)] } else { 0 };

            if curr.pos == (x, y) {
                print!("\u{001B}[32m");
                print!(" {:03} ", score);
                print!("\u{001B}[0m");
            } else if path.contains_key(&(x, y)) {
                print!("\u{001B}[31m");
                print!(" {:03} ", score);
                print!("\u{001B}[0m");
            } else if open_set.iter().find(|p| p.pos == (x, y)) != None {
                print!("\u{001B}[34m");
                print!(" {:03} ", score);
                print!("\u{001B}[0m");
            } else {
                print!(" {:03} ", score);
            }
        }
        println!();
    }
}

fn main() {
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut data: Vec<u32> = vec![];
        let mut width = 0;
        let mut height = 0;

        for line in lines {
            if let Ok(text) = line {
                if width == 0 {
                    width = text.len();
                }
                data.extend(text.chars().map(|ch| ch.to_digit(10).unwrap()));
                height += 1;
            }
        }

        let map = Map { width, height, data };

        let goal = (width - 1, height - 1);

        let result1 = dijkstra(
            (0, 0),
            goal,
            (0, 3),
            &map);
        
        if let Some(sum) = result1 {
            println!("[Part 1] Heat loss in optimal path: {}", sum);
        } else {
            println!("[Part 1] No optimal path found!");
        }

        let result2 = dijkstra(
            (0, 0),
            goal,
            (4, 10),
            &map);

        if let Some(sum) = result2 {
            println!("[Part 2] Heat loss in optimal path: {}", sum);
        } else {
            println!("[Part 2] No optimal path found!");
        }
    } else {
        eprintln!("Could not load parameters map from {}", INPUT_FILE);
    }

}
