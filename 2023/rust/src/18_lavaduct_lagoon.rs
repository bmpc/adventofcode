mod utils;

use std::collections::HashMap;

const INPUT_FILE: &str = "./input/18_input.txt";
//const INPUT_FILE: &str = "./input/18_input_test.txt";
//const INPUT_FILE: &str = "./input/18_input_test2.txt";

struct Grid {
    width: usize,
    height: usize,
    data: Vec<char>
}

struct DigDirection {
    dir: (i32, i32),
    steps: u32,
    color: String
}

fn build_dig_map(dig_plan: &Vec<DigDirection>) -> Grid {

    // first build a hashmap of all the edge points    
    let mut edges = HashMap::new();
    let mut row = 0;
    let mut col = 0;
    let mut max_w = 0;
    let mut max_h = 0;
    let mut min_w = 0;
    let mut min_h = 0;
    for dd in dig_plan {
        for _ in 0..dd.steps {
            edges.insert((col, row), &dd.color);
            col += dd.dir.0;
            if col + 1 > max_w {
                max_w = col + 1;
            }
            if col < min_w {
                min_w = col;
            }
            row += dd.dir.1;
            if row + 1 > max_h {
                max_h = row + 1;
            }
            if row < min_h {
                min_h = row;
            }
        }
    }
    
    // build a grid of all the points
    let mut data = vec![];
    for r in min_h..max_h {
        for c in min_w..max_w {
            if let Some(_) = edges.get(&(c, r)) {
                data.push('#');
            } else {
                data.push('.');
            }
        }
    }
    
    let width = (max_w - min_w) as usize;
    let height = (max_h - min_h) as usize;
    
    Grid { width, height, data }
}

fn dig_area(grid: &Grid) -> u64 {
    let mut sum = 0;
    for r in 0..grid.height {
        let mut fc = 0;
        let mut up = false;
        let mut down = false;
        for c in 0..grid.width {
            let v = grid.data[r * grid.width + c];
            if v != '.' { // #
                sum += 1;
                up |= if r > 0 { grid.data[(r - 1) * grid.width + c] != '.' } else { false };
                down |= if r < grid.height - 1 { grid.data[(r + 1) * grid.width + c] != '.' } else { false };
            } else { // "."
                if up && down { // found frontier 
                    fc += 1;
                }
                up = false;
                down = false;
                
                if fc % 2 != 0 {
                    // we are inside the dig zone
                    sum += 1;
                }
            }
        }
    }

    sum
}

fn shoelace_and_picks(dig_plan: &Vec<DigDirection>) -> u128 {
    let mut row: i64 = 0;
    let mut col: i64 = 0;

    let mut b_points: i128 = 0;

    let mut points = vec![];
    for dd in dig_plan {
        b_points += dd.steps as i128;
        for _ in 0..dd.steps {
            points.push((col, row));
            col += dd.dir.0 as i64;
            row += dd.dir.1 as i64;
        }
    }

    let mut sum1: i128 = 0;
    let mut sum2: i128 = 0;
    for (i, (x, y)) in points.iter().enumerate() {
        if i < points.len() - 1 {
            sum1 = sum1 + (x * points[i+1].1) as i128;
            sum2 = sum2 + (y * points[i+1].0) as i128;
        }
    }

    //Add xn.y1
    sum1 = sum1 + (points[points.len() - 1].0 * points[0].1) as i128;
    //Add x1.yn
    sum2 = sum2 + (points[0].0 * points[points.len() - 1].1) as i128;
    
    let area = (sum1 - sum2).abs() / 2;
    
    let inner_area = area - (b_points / 2) + 1;

    (inner_area + b_points) as u128

}

fn _print_map(grid: &Grid) {
    for r in 0..grid.height {
        for c in 0..grid.width {
            print!("{}", grid.data[r * grid.width + c]);
        }
        println!();
    }
}

fn main() {
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        let mut dig_plan: Vec<DigDirection> = vec![];

        for line in lines {
            if let Ok(text) = line {
                let mut parts = text.split(" ");
                let dir =  match parts.next().unwrap() {
                    "R" => (1, 0),
                    "L" => (-1, 0),
                    "D" => (0, 1),
                    "U" => (0, -1),
                    _ => unreachable!()
                };
                let steps = parts.next().unwrap().parse::<u32>().unwrap();
                let color = parts.next().unwrap().strip_prefix("(").unwrap().strip_suffix(")").unwrap().to_string();

                dig_plan.push(DigDirection {dir, steps, color});
            }
        }

        // naive solution - build the map in memory and check the area line by line
        let map = build_dig_map(&dig_plan);
        //_print_map(&map);
        let area1 = dig_area(&map);
        println!("[Part 1] Cubic meters of lava : {}", area1);

        let mut dig_plan2: Vec<DigDirection> = vec![];
        for dd in &dig_plan {
            let raw = dd.color.strip_prefix("#").unwrap();
            let steps: u32 = u32::from_str_radix(&raw[0..5], 16).unwrap();

            let dir = match &raw[5..6] {
                "0" => (1, 0),
                "1" => (0, 1),
                "2" => (-1, 0),
                "3" => (0, -1),
                _ => unreachable!()
            };

            dig_plan2.push(DigDirection {dir, steps, color: String::new()});
        }

        // solution based on shoelace and picks theorem (shamely stolen from https://www.youtube.com/watch?v=bGWK76_e-LM)
        let area2 = shoelace_and_picks(&dig_plan2);

        println!("[Part 2] Cubic meters of lava after bug is fixed : {}", area2);
 
    } else {
        eprintln!("Could not load parameters map from {}", INPUT_FILE);
    }

}
