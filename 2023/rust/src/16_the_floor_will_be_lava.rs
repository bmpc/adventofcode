mod utils;

use std::collections::HashMap;

// const INPUT_FILE: &str = "./input/16_input_test.txt";
const INPUT_FILE: &str = "./input/16_input.txt";

struct Contraption {
    width: usize,
    height: usize,
    data: Vec<char>
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum DIR {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

fn get_energized_tiles(cont: &Contraption, (mut x, mut y): (i32, i32), mut dir: DIR, visited: &mut HashMap<(i32, i32), DIR>) {
    while x >= 0 && (x as usize) < cont.width && y >= 0 && (y as usize) < cont.height {
        if let Some(d) = visited.get(&(x, y)) {
            if *d == dir {
                break;
            }
        }

        let ch = cont.data[cont.width*(y as usize) + (x as usize)];
        visited.insert((x,y), dir);
        
        match ch {
            '|' => match dir {
                DIR::LEFT | DIR::RIGHT => {
                    if y > 0 {
                        get_energized_tiles(cont, (x, y - 1), DIR::UP, visited);
                    }
                    if (y as usize) < cont.height - 1 {
                        get_energized_tiles(cont, (x, y + 1), DIR::DOWN, visited);
                    }
                },
                DIR::UP => y -= 1,
                DIR::DOWN => y += 1,
            }
            '-' => match dir {
                DIR::UP | DIR::DOWN => {
                    if (x as usize) < cont.width - 1 {
                        get_energized_tiles(cont, (x + 1, y), DIR::RIGHT, visited);
                    }
                    if x > 0 {
                        get_energized_tiles(cont, (x - 1, y), DIR::LEFT, visited);
                    }
                },
                DIR::RIGHT => x += 1,
                DIR::LEFT => x -= 1,
            }
            '\\' => match dir {
                DIR::UP => {
                    dir = DIR::LEFT;
                    x -= 1;
                }
                DIR::RIGHT => {
                    dir = DIR::DOWN;
                    y += 1;
                }
                DIR::DOWN => {
                    dir = DIR::RIGHT;
                    x += 1;
                }
                DIR::LEFT => {
                    dir = DIR::UP;
                    y -= 1;
                }
            },
            '/' => match dir {
                DIR::UP => {
                    dir = DIR::RIGHT;
                    x += 1;
                }
                DIR::RIGHT => {
                    dir = DIR::UP;
                    y -= 1;
                }
                DIR::DOWN => {
                    dir = DIR::LEFT;
                    x -= 1;
                }
                DIR::LEFT => {
                    dir = DIR::DOWN;
                    y += 1;
                }
            },
            '.' => match dir {
                DIR::UP => y -= 1,
                DIR::RIGHT => x += 1,
                DIR::LEFT => x -= 1,
                DIR::DOWN => y += 1,
            },
            _ => unreachable!()
        }
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

        let contraption = Contraption { width, height, data };

        let mut visited: HashMap<(i32, i32), DIR> = HashMap::new();
        get_energized_tiles(&contraption, (0, 0), DIR::RIGHT, &mut visited);

        let sum1 = visited.keys().count();

        println!("[Part 1] Sum of energized tiles: {}", sum1);
    } else {
        eprintln!("Could not contraption layout from {}", INPUT_FILE);
    }

}
