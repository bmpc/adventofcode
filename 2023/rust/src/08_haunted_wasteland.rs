use std::collections::HashMap;

mod utils;

// static INPUT_FILE: &str = "./input/08_input_test.txt";
// static INPUT_FILE: &str = "./input/08_input_test2.txt";
// static INPUT_FILE: &str = "./input/08_input_test3.txt";
const INPUT_FILE: &str = "./input/08_input.txt";

#[derive(Debug)]
struct MapEntry {
    name: String,
    left: String,
    right: String
}

fn parse_map_entry(text: &str) -> MapEntry {
    let mut parts = text.split('=');

    let name = parts.next().expect("No map entry label found!").trim();

    let paths = parts.next().expect("No map path found!").trim().replace("(", "").replace(")", "");
    let mut path_parts = paths.split(',');

    let left = path_parts.next().expect("Left path not found!").trim();
    let right = path_parts.next().expect("Right path not found!").trim();
    
    MapEntry { name: name.to_string(), left: left.to_string(), right: right.to_string() }
}

fn find_land_of_zzz(path: &Vec<char>, entries_map: &HashMap<String, MapEntry>) -> Option<u32> {
    let step_op = entries_map.get("AAA");

    match step_op {
        Some(mut step) => {
            let mut steps = 0;
            let mut found = false;
            
            while !found {
                for dir in path {
                    step = match dir {
                        'L' => &entries_map[&step.left],
                        'R' => &entries_map[&step.right],
                        _ => unreachable!()
                    };
        
                    steps += 1;
        
                    if step.name == "ZZZ" {
                        found = true;
                        break;
                    }
                }


            }
            
            Some(steps)
        }
        None => None
    }
    
}

/**
 * The naive solution of going though all the paths takes forever to calculate.
 * Solution: Each starting node takes a number of loops (through the initial input) 
 * to reach the final node. After this, the path repeats itself. The number of loops is
 * always a prime number: 43, 47, 61, 67, 73, 79 respectively for the 6 starting nodes we have.
 * Therefore, the lowest number of steps where all nodes are Z is: 
 * 43 * 47 * 61 * 67 * 73 * 79 * 281(steps of each loop).
 */
fn find_land_of_zs(path: &Vec<char>, entries_map: &HashMap<String, MapEntry>) -> u64 {
    let step_nodes = entries_map.keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| entries_map.get(k).unwrap())
        .collect::<Vec<_>>();

    let mut loops = Vec::new();

    for mut step in step_nodes {
        let mut loop_count: u64 = 0;
        let mut found = false;
        while !found {
            for dir in path {
                step = match dir {
                    'L' => &entries_map[&step.left],
                    'R' => &entries_map[&step.right],
                    _ => unreachable!()
                };
    
                if step.name.ends_with("Z") {
                    found = true;
                    break;
                }
            }
            loop_count += 1;
        }
        loops.push(loop_count);
    }

    // lowest common denominator where all loops reach the land of Z
    let lowest_common_loop = loops.iter().fold(1, |acc, l|acc * l);

    lowest_common_loop * path.len() as u64
}

fn main() {
    if let Ok(mut lines) = utils::read_lines(INPUT_FILE) {
        let mut path = Vec::new();
        let mut entries_map = HashMap::new();

        let input = lines.next().unwrap().expect("Input path not found!");
        for ch in input.chars() {
            path.push(ch);
        }

        for line in lines.into_iter().skip(1) {
            if let Ok(text) = line {
                let map_entry = parse_map_entry(&text);
                entries_map.insert(map_entry.name.clone(), map_entry);
            }
        }

        
        let steps = find_land_of_zzz(&path, &entries_map);
        let steps_all_zs = find_land_of_zs(&path, &entries_map);

        println!("Steps required to reach ZZZ: {}", steps.unwrap_or(0));
        println!("Steps required to reach all **Z: {}", steps_all_zs);
    } else {
        eprintln!("Could not extract nodes from {}", INPUT_FILE);
    }

}
