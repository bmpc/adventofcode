use std::collections::HashSet;

mod utils;

// const INPUT_FILE: &str = "./input/10_input_test.txt"; const WIDTH: usize = 5; const HEIGHT: usize = 5;
// const INPUT_FILE: &str = "./input/10_input_test2.txt"; const WIDTH: usize = 20; const HEIGHT: usize = 10;
// const INPUT_FILE: &str = "./input/10_input_test3.txt"; const WIDTH: usize = 11; const HEIGHT: usize = 9;
//const INPUT_FILE: &str = "./input/10_input_test4.txt"; const WIDTH: usize = 10; const HEIGHT: usize = 9;
//const INPUT_FILE: &str = "./input/10_input_test5.txt"; const WIDTH: usize = 20; const HEIGHT: usize = 10;
const INPUT_FILE: &str = "./input/10_input.txt"; const WIDTH: usize = 140; const HEIGHT: usize = 140;

enum DIRECTION {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize
}

fn get_path(ground: &[[char;WIDTH]; HEIGHT]) -> HashSet<Point> {
    let mut path = HashSet::new();

    let (start_l, start_c) = find_starting_point(ground).unwrap();

    // find valid path to start walking
    let mut l = start_l;
    let mut c = start_c;

    let mut dir = DIRECTION::DOWN;
    let mut valid_path = false;

    if start_c < WIDTH - 1 { // check right
        let ch = ground[l][c + 1];
        match ch {
            '-' | 'J' | '7' => { 
                valid_path = true; 
                c += 1; 
                dir = DIRECTION::RIGHT;
            },
            _ => {}
        }
    }

    if !valid_path && start_c > 0 { // check left
        let ch = ground[l][c - 1];
        match ch {
            '-' | 'J' | '7' => { 
                valid_path = true; 
                c -= 1; 
                dir = DIRECTION::LEFT;
            },
            _ => {}
        }
    }

    if !valid_path && start_l > 0 { // check up
        let ch = ground[l - 1][c];
        match ch {
            '|' | 'F' | '7' => { 
                valid_path = true; 
                l -= 1; 
                dir = DIRECTION::UP;
            },
            _ => {}
        }
    }

    if !valid_path && start_l < HEIGHT - 1 { // check down
        let ch = ground[l + 1][c];
        match ch {
            '|' | 'L' | 'J' => { 
                valid_path = true; 
                l += 1;
                dir = DIRECTION::DOWN;
            },
            _ => {}
        }
    }

    assert!(valid_path);
    path.insert(Point { x: start_c, y: start_l });

    while ground[l][c] != 'S' {
        path.insert(Point { x: c, y: l });
        match ground[l][c] {
            '|' => match dir {
                DIRECTION::DOWN => l += 1,
                DIRECTION::UP => l -= 1,
                _ => unreachable!()
            }
            '-' => match dir {
                DIRECTION::RIGHT => c += 1,
                DIRECTION::LEFT => c -= 1,
                _ => unreachable!()
            }
            'L' => match dir {
                DIRECTION::DOWN => { c += 1; dir = DIRECTION::RIGHT; } // move right
                DIRECTION::LEFT => { l -= 1; dir = DIRECTION::UP }, // move up
                _ => unreachable!()
            }
            'J' => match dir {
                DIRECTION::DOWN => { c -= 1; dir = DIRECTION::LEFT; } // move left
                DIRECTION::RIGHT => { l -= 1; dir = DIRECTION::UP }, // move up
                _ => unreachable!()
            }
            '7' => match dir {
                DIRECTION::UP => { c -= 1; dir = DIRECTION::LEFT; } // move left
                DIRECTION::RIGHT => { l += 1; dir = DIRECTION::DOWN }, // move down
                _ => unreachable!()
            }
            'F' => match dir {
                DIRECTION::UP => { c += 1; dir = DIRECTION::RIGHT; } // move right
                DIRECTION::LEFT => { l += 1; dir = DIRECTION::DOWN }, // move down
                _ => unreachable!()
            }
            _ => unreachable!()
        }
    }

    path
}


fn find_starting_point(ground: &[[char;WIDTH]; HEIGHT]) -> Option<(usize, usize)> {
    for l in 0..HEIGHT {
        for c in 0..WIDTH {
            if ground[l][c] == 'S' {
                return Some((l, c));
            }
        }
    }

    None
}

/**
 * Here we need cast an horizontal ray on the point and see if how many intersections we have.
 * A valid intersection is any vertical bar or when the line changes direction we must count 
 * the portion as 1 intersection:
 *  - '|' or
 *  - L(-)*7
 *  - F(-)*J
 */
fn is_point_inside_area(
    point: Point, 
    shape: &HashSet<Point>,
    ground: &[[char;WIDTH]; HEIGHT]) -> bool {

    if shape.contains(&point) {
        return false;
    }

    let mut intersect_count = 0;

    // horizontal ray cast
    let mut in_l = false;
    let mut in_f = false;

    for x in point.x+1..WIDTH {
        if shape.contains(&Point {x, y: point.y}) {
            let ch = ground[point.y][x];
            match ch {
                '|' => intersect_count += 1,
                'L' => in_l = true,
                'F' => in_f = true,
                '7' => if in_l { intersect_count += 1; in_l = false; } else { in_l = false; in_f = false; },
                'J' => if in_f { intersect_count += 1; in_f = false; } else { in_l = false; in_f = false; },
                '-' => {}
                _ => { in_l = false; in_f = false; }
            }
        }
    }

    intersect_count % 2 != 0
}

fn print_path(path: &HashSet<Point>, ground: &[[char;WIDTH]; HEIGHT]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if path.contains(&Point {x, y}) {
                print!("{}",ground[y][x]);
                //print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }
}


fn main() {
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut ground: [[char;WIDTH]; HEIGHT] = [['.'; WIDTH]; HEIGHT];

        for (i, line) in lines.enumerate() {
            if let Ok(text) = line {
                for (j, ch) in text.chars().enumerate() {
                    ground[i][j] = ch;
                }
            }
        }

        let path = get_path(&ground);

        print_path(&path, &ground);

        let mut area = 0;
        for l in 0..HEIGHT {
            for c in 0..WIDTH {
                if is_point_inside_area(Point {x:c, y:l}, &path, &ground) {
                    //println!("({},{})", l,c);
                    area += 1;
                }
            }
        }

        println!("Steps to de farthest point: {}", path.len() / 2);
        println!("Loop area: {}", area);
    } else {
        eprintln!("Could not extract the ground map from {}", INPUT_FILE);
    }

}
